use std::sync::Arc;
use tokio::sync::RwLock;
use openmedia_core::{Config, HardwareInfo, ModelRegistry, Result as CoreResult};
use openmedia_image::DiffusionPipeline;
use openmedia_process::DummyGpuPipeline;
use openmedia_improve::{ClipScorer, AestheticScorer, GenerationHistory, PromptRefiner};
use rmcp::{tool, tool_router, handler::server::wrapper::{Parameters, Json}};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

/// Main MCP server for OpenMedia
#[derive(Clone)]
pub struct OpenMediaServer {
    pub config: Arc<Config>,
    pub hardware: Arc<HardwareInfo>,
    pub model_registry: Arc<ModelRegistry>,
    pub history: Arc<GenerationHistory>,
    pub clip_scorer: Arc<Option<ClipScorer>>,
    pub aesthetic_scorer: Arc<Option<AestheticScorer>>,
    pub gpu_pipeline: Arc<Option<DummyGpuPipeline>>,
    pub prompt_refiner: Arc<PromptRefiner>,
    pub active_backend: Arc<RwLock<Option<Box<dyn DiffusionPipeline>>>>,
}

impl OpenMediaServer {
    pub async fn new(config: Config) -> CoreResult<Self> {
        let hardware = HardwareInfo::detect().await;
        let model_registry = ModelRegistry::scan(&config.paths.model_dir).await?;
        let history = GenerationHistory::open(&config.paths.history_db)?;

        let clip_scorer = if config.improve.enable_clip_scoring {
            ClipScorer::load(&config.paths.model_dir.join("clip")).await.ok()
        } else {
            None
        };

        let aesthetic_scorer = if config.improve.enable_aesthetic_scoring {
            AestheticScorer::load(&config.paths.model_dir.join("clip/aesthetic-predictor.onnx")).await.ok()
        } else {
            None
        };

        let gpu_pipeline = if config.compute.gpu_processing {
            Some(DummyGpuPipeline::new())
        } else {
            None
        };

        Ok(Self {
            config: Arc::new(config),
            hardware: Arc::new(hardware),
            model_registry: Arc::new(model_registry),
            history: Arc::new(history),
            clip_scorer: Arc::new(clip_scorer),
            aesthetic_scorer: Arc::new(aesthetic_scorer),
            gpu_pipeline: Arc::new(gpu_pipeline),
            prompt_refiner: Arc::new(PromptRefiner::new()),
            active_backend: Arc::new(RwLock::new(None)),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RasterizeSvgRequest {
    /// Raw SVG XML string or file path to .svg
    pub svg: String,
    /// Target width (maintains aspect ratio if omitted)
    pub width: Option<u32>,
    /// Target height
    pub height: Option<u32>,
    /// Optional background color hex (e.g. #ffffff). Default is transparent.
    pub background_color: Option<String>,
    /// Output format (png, jpeg, webp). Default is png.
    pub output_format: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HtmlToImageRequest {
    /// Raw HTML string or file path to .html
    pub html: String,
    /// Viewport width. Default is 1920.
    pub width: Option<u32>,
    /// Viewport height. Default is 1080.
    pub height: Option<u32>,
    /// Display density (DPI scaler). Default is 1.0.
    pub device_scale_factor: Option<f64>,
    /// Output format (png, jpeg, webp). Default is png.
    pub output_format: Option<String>,
}

#[tool_router(server_handler)]
impl OpenMediaServer {
    #[tool(description = "Ping the media generation server to check status and health")]
    pub async fn ping(&self) -> String {
        format!(
            "pong (CPU: {}, GPU: {:?})",
            self.hardware.cpu.brand,
            self.hardware.gpu.as_ref().map(|g| &g.name)
        )
    }

    #[tool(
        name = "rasterize_svg",
        description = "Rasterize an SVG string or file path into a PNG, JPEG, or WebP image."
    )]
    pub async fn rasterize_svg(
        &self,
        params: Parameters<RasterizeSvgRequest>,
    ) -> Result<Json<serde_json::Value>, String> {
        let req = params.0;
        let format = req.output_format.unwrap_or_else(|| "png".to_string());
        let filename = format!("{}.{}", uuid::Uuid::now_v7(), format);
        let output_path = self.config.paths.output_dir.join(filename);
        
        let _ = std::fs::create_dir_all(&self.config.paths.output_dir);
        
        let svg_content = if req.svg.trim().starts_with('<') {
            req.svg
        } else {
            let path = std::path::Path::new(&req.svg);
            if path.exists() && path.is_file() {
                std::fs::read_to_string(path).map_err(|e| e.to_string())?
            } else {
                req.svg
            }
        };

        let output = openmedia_svg::rasterize(
            &svg_content,
            req.width,
            req.height,
            req.background_color.as_deref(),
            &format,
            &output_path,
        ).map_err(|e| e.to_string())?;

        serde_json::to_value(output)
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "html_to_image",
        description = "Render an HTML/CSS string or file path into a PNG, JPEG, or WebP screenshot."
    )]
    pub async fn html_to_image(
        &self,
        params: Parameters<HtmlToImageRequest>,
    ) -> Result<Json<serde_json::Value>, String> {
        let req = params.0;
        let format = req.output_format.unwrap_or_else(|| "png".to_string());
        let filename = format!("{}.{}", uuid::Uuid::now_v7(), format);
        let output_path = self.config.paths.output_dir.join(filename);
        
        let _ = std::fs::create_dir_all(&self.config.paths.output_dir);

        let output = openmedia_video::html_to_image(
            &req.html,
            req.width,
            req.height,
            req.device_scale_factor,
            &format,
            &output_path,
        ).await.map_err(|e| e.to_string())?;

        serde_json::to_value(output)
            .map(Json)
            .map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mcp_ping() {
        let mut config = Config::default();
        let temp_dir = std::env::temp_dir();
        config.paths.model_dir = temp_dir.join("openmedia_test_models");
        config.paths.output_dir = temp_dir.join("openmedia_test_output");
        config.paths.history_db = temp_dir.join("openmedia_test_history.db");

        let _ = std::fs::create_dir_all(&config.paths.model_dir);
        let _ = std::fs::create_dir_all(&config.paths.output_dir);
        let _ = std::fs::remove_file(&config.paths.history_db);

        let server = OpenMediaServer::new(config).await.unwrap();
        let response = server.ping().await;
        assert!(response.starts_with("pong"));
    }

    #[tokio::test]
    async fn test_mcp_rasterize_svg() {
        let mut config = Config::default();
        let temp_dir = std::env::temp_dir();
        config.paths.model_dir = temp_dir.join("openmedia_test_models_svg");
        config.paths.output_dir = temp_dir.join("openmedia_test_output_svg");
        config.paths.history_db = temp_dir.join("openmedia_test_history_svg.db");

        let _ = std::fs::create_dir_all(&config.paths.output_dir);
        let server = OpenMediaServer::new(config).await.unwrap();

        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100">
            <rect width="100" height="100" fill="red"/>
        </svg>"#.to_string();

        let params = Parameters(RasterizeSvgRequest {
            svg,
            width: Some(200),
            height: None,
            background_color: Some("#ffffff".to_string()),
            output_format: Some("png".to_string()),
        });

        let result = server.rasterize_svg(params).await;
        assert!(result.is_ok());
        let val = result.unwrap().0;
        let output: openmedia_core::ImageOutput = serde_json::from_value(val).unwrap();
        assert_eq!(output.width, 200);
        assert_eq!(output.height, 200);
        assert!(output.path.exists());
        let _ = std::fs::remove_file(output.path);
    }

    #[tokio::test]
    async fn test_mcp_html_to_image() {
        let mut config = Config::default();
        let temp_dir = std::env::temp_dir();
        config.paths.model_dir = temp_dir.join("openmedia_test_models_html");
        config.paths.output_dir = temp_dir.join("openmedia_test_output_html");
        config.paths.history_db = temp_dir.join("openmedia_test_history_html.db");

        let _ = std::fs::create_dir_all(&config.paths.output_dir);
        let server = OpenMediaServer::new(config).await.unwrap();

        let html = "<html><body><h1>Hello World</h1></body></html>".to_string();

        let params = Parameters(HtmlToImageRequest {
            html,
            width: Some(800),
            height: Some(600),
            device_scale_factor: Some(1.0),
            output_format: Some("png".to_string()),
        });

        let result = server.html_to_image(params).await;
        match result {
            Ok(val) => {
                let output: openmedia_core::ImageOutput = serde_json::from_value(val.0).unwrap();
                assert_eq!(output.width, 800);
                assert_eq!(output.height, 600);
                assert!(output.path.exists());
                let _ = std::fs::remove_file(output.path);
            }
            Err(e) => {
                assert!(
                    e.contains("ChromeNotFound") || e.contains("Chrome not found") || e.contains("headless-chrome") || e.contains("oneshot canceled"),
                    "Unexpected error: {}",
                    e
                );
            }
        }
    }
}
