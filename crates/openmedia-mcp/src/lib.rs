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

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AnimateSvgRequest {
    /// Raw SVG XML string or file path to .svg
    pub svg: String,
    /// Target element ID to animate
    pub element_id: String,
    /// Preset animation (fade_in, fade_out, slide_in_left, bounce, pulse, spin, typewriter, draw_path, etc.)
    pub preset: String,
    /// Duration of animation in seconds (default 1.0)
    pub duration: Option<f64>,
    /// Delay of animation in seconds (default 0.0)
    pub delay: Option<f64>,
    /// Easing function name (default linear)
    pub easing: Option<String>,
    /// Repeat count (infinite, 1, 2, etc. default 1)
    pub repeat_count: Option<String>,
    /// Optional preset parameters
    pub params: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AnimateTimelineRequest {
    /// Raw SVG XML string or file path to .svg
    pub svg: String,
    /// Timeline mode (parallel | sequential | staggered)
    pub mode: String,
    /// Delay for stagger mode in seconds (default 0.2)
    pub stagger_delay: Option<f64>,
    /// Timeline entries
    pub entries: Vec<TimelineEntryRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TimelineEntryRequest {
    /// Target element ID
    pub element_id: String,
    /// Preset animation
    pub preset: String,
    /// Duration of animation in seconds
    pub duration: f64,
    /// Offset/delay in seconds relative to timeline sequence
    pub offset: f64,
    /// Easing function name (default linear)
    pub easing: Option<String>,
    /// Optional preset parameters
    pub params: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AnimateMorphRequest {
    /// Source path data string (d attribute)
    pub from_path: String,
    /// Target path data string (d attribute)
    pub to_path: String,
    /// Duration of morph animation in seconds (default 3.0)
    pub duration: Option<f64>,
    /// Easing function name (default ease_in_out)
    pub easing: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GenerateSpinnerRequest {
    /// Spinner style (ring | dots | border | bars)
    pub spinner_type: String,
    /// Color of spinner (e.g. #8b5cf6)
    pub color: Option<String>,
    /// Size in pixels (default 60)
    pub size: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LottieToSvgRequest {
    /// Lottie JSON content or file path to Lottie JSON
    pub lottie_json: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SvgToLottieRequest {
    /// Raw SVG XML string or file path to .svg
    pub svg: String,
}

// Helper functions for SVG Animation MCP Tools
fn parse_easing(s: Option<&str>) -> openmedia_animate::Easing {
    let name = s.unwrap_or("linear");
    match name.to_lowercase().as_str() {
        "linear" => openmedia_animate::Easing::Linear,
        "ease_in" | "easein" | "ease-in" => openmedia_animate::Easing::EaseInQuad,
        "ease_out" | "easeout" | "ease-out" => openmedia_animate::Easing::EaseOutQuad,
        "ease_in_out" | "easeinout" | "ease-in-out" => openmedia_animate::Easing::EaseInOutQuad,
        "ease_in_cubic" | "ease-in-cubic" => openmedia_animate::Easing::EaseInCubic,
        "ease_out_cubic" | "ease-out-cubic" => openmedia_animate::Easing::EaseOutCubic,
        "ease_in_out_cubic" | "ease-in-out-cubic" => openmedia_animate::Easing::EaseInOutCubic,
        "ease_in_expo" | "ease-in-expo" => openmedia_animate::Easing::EaseInExpo,
        "ease_out_expo" | "ease-out-expo" => openmedia_animate::Easing::EaseOutExpo,
        "ease_in_out_expo" | "ease-in-out-expo" => openmedia_animate::Easing::EaseInOutExpo,
        "bounce" | "ease_out_bounce" | "ease-out-bounce" => openmedia_animate::Easing::EaseOutBounce,
        "elastic" | "ease_out_elastic" | "ease-out-elastic" => openmedia_animate::Easing::EaseOutElastic,
        "spring" => openmedia_animate::Easing::Spring { stiffness: 100.0, damping: 10.0, mass: 1.0 },
        _ => {
            if name.starts_with("cubic-bezier(") && name.ends_with(')') {
                let content = &name["cubic-bezier(".len() .. name.len() - 1];
                let parts: Vec<&str> = content.split(',').map(|p| p.trim()).collect();
                if parts.len() == 4 {
                    let x1 = parts[0].parse::<f64>().unwrap_or(0.25);
                    let y1 = parts[1].parse::<f64>().unwrap_or(0.1);
                    let x2 = parts[2].parse::<f64>().unwrap_or(0.25);
                    let y2 = parts[3].parse::<f64>().unwrap_or(1.0);
                    return openmedia_animate::Easing::CubicBezier(x1, y1, x2, y2);
                }
            }
            openmedia_animate::Easing::Linear
        }
    }
}

fn parse_preset(s: &str) -> openmedia_animate::AnimationPreset {
    match s.to_lowercase().as_str() {
        "fade_in" | "fadein" | "fade-in" => openmedia_animate::AnimationPreset::FadeIn,
        "fade_out" | "fadeout" | "fade-out" => openmedia_animate::AnimationPreset::FadeOut,
        "slide_in_left" | "slide-in-left" => openmedia_animate::AnimationPreset::SlideInLeft,
        "slide_in_right" | "slide-in-right" => openmedia_animate::AnimationPreset::SlideInRight,
        "slide_in_up" | "slide-in-up" => openmedia_animate::AnimationPreset::SlideInUp,
        "slide_in_down" | "slide-in-down" => openmedia_animate::AnimationPreset::SlideInDown,
        "bounce" => openmedia_animate::AnimationPreset::Bounce,
        "pulse" => openmedia_animate::AnimationPreset::Pulse,
        "spin" => openmedia_animate::AnimationPreset::Spin,
        "shake" => openmedia_animate::AnimationPreset::Shake,
        "wobble" => openmedia_animate::AnimationPreset::Wobble,
        "typewriter" => openmedia_animate::AnimationPreset::Typewriter,
        "draw_path" | "draw-path" | "drawpath" => openmedia_animate::AnimationPreset::DrawPath,
        "morph" => openmedia_animate::AnimationPreset::Morph,
        "gradient_shift" | "gradient-shift" => openmedia_animate::AnimationPreset::GradientShift,
        "parallax_scroll" | "parallax-scroll" => openmedia_animate::AnimationPreset::ParallaxScroll,
        "stagger" => openmedia_animate::AnimationPreset::Stagger,
        _ => openmedia_animate::AnimationPreset::FadeIn,
    }
}

fn inject_css_class(svg: &str, element_id: &str, class_name: &str) -> String {
    let clean_id = element_id.trim_start_matches('#');
    let patterns = [
        format!("id=\"{}\"", clean_id),
        format!("id='{}'", clean_id),
    ];
    
    let mut found_pos = None;
    for pat in &patterns {
        if let Some(pos) = svg.find(pat) {
            found_pos = Some((pos, pat.len()));
            break;
        }
    }
    
    let (pos, _pat_len) = match found_pos {
        Some(p) => p,
        None => return svg.to_string(),
    };
    
    let start_tag_idx = match svg[..pos].rfind('<') {
        Some(idx) => idx,
        None => return svg.to_string(),
    };
    
    let end_tag_idx = match svg[pos..].find('>') {
        Some(idx) => pos + idx,
        None => return svg.to_string(),
    };
    
    let mut tag_content = svg[start_tag_idx..=end_tag_idx].to_string();
    let class_pat_double = "class=\"";
    let class_pat_single = "class='";
    
    if let Some(c_pos) = tag_content.find(class_pat_double) {
        let insert_idx = c_pos + class_pat_double.len();
        tag_content.insert_str(insert_idx, &format!("{} ", class_name));
    } else if let Some(c_pos) = tag_content.find(class_pat_single) {
        let insert_idx = c_pos + class_pat_single.len();
        tag_content.insert_str(insert_idx, &format!("{} ", class_name));
    } else {
        if let Some(space_pos) = tag_content.find(' ') {
            tag_content.insert_str(space_pos, &format!(" class=\"{}\"", class_name));
        } else {
            let insert_pos = if tag_content.ends_with("/>") {
                tag_content.len() - 2
            } else {
                tag_content.len() - 1
            };
            tag_content.insert_str(insert_pos, &format!(" class=\"{}\" ", class_name));
        }
    }
    
    let mut result = svg.to_string();
    result.replace_range(start_tag_idx..=end_tag_idx, &tag_content);
    result
}

fn inject_style_or_xml(mut svg: String, content_to_inject: &str) -> String {
    let lower = svg.to_lowercase();
    if let Some(close_idx) = lower.rfind("</svg>") {
        svg.insert_str(close_idx, content_to_inject);
    } else {
        svg.push_str(content_to_inject);
    }
    svg
}

fn parse_svg_dimensions(svg: &str) -> (u32, u32) {
    let mut width = 800;
    let mut height = 600;
    
    if let Some(pos) = svg.find("width=\"") {
        let start = pos + "width=\"".len();
        if let Some(end) = svg[start..].find('"') {
            if let Ok(val) = svg[start..start+end].parse::<f64>() {
                width = val as u32;
            }
        }
    }
    
    if let Some(pos) = svg.find("height=\"") {
        let start = pos + "height=\"".len();
        if let Some(end) = svg[start..].find('"') {
            if let Ok(val) = svg[start..start+end].parse::<f64>() {
                height = val as u32;
            }
        }
    }
    
    (width, height)
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

    #[tool(
        name = "animate_svg",
        description = "Apply animation presets (such as fade_in, spin, bounce, pulse, typewriter, draw_path) to SVG elements."
    )]
    pub async fn animate_svg(
        &self,
        params: Parameters<AnimateSvgRequest>,
    ) -> Result<Json<serde_json::Value>, String> {
        use openmedia_animate::*;
        let req = params.0;
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

        let preset = parse_preset(&req.preset);
        let duration = req.duration.unwrap_or(1.0);
        let delay = req.delay.unwrap_or(0.0);
        let easing = parse_easing(req.easing.as_deref());
        let extra_params = req.params.clone().unwrap_or(serde_json::Value::Null);

        let output = preset.generate(duration, delay, &easing, &extra_params)
            .map_err(|e| e.to_string())?;

        let (animated_svg, animation_count) = match output {
            AnimationOutput::Smil(anims) => {
                let animation_count = anims.len() as u32;
                let mut xml_block = String::new();
                for anim in anims {
                    xml_block.push_str("  ");
                    xml_block.push_str(&anim.to_xml(Some(&req.element_id)));
                    xml_block.push_str("\n");
                }
                (inject_style_or_xml(svg_content, &xml_block), animation_count)
            }
            AnimationOutput::Css(keyframes) => {
                let animated_svg = inject_css_class(&svg_content, &req.element_id, &keyframes.name);
                let style_block = format!("  <style>\n    {}\n  </style>\n", keyframes.to_css());
                (inject_style_or_xml(animated_svg, &style_block), 1)
            }
            AnimationOutput::Combined { smil, css } => {
                let animated_svg = inject_css_class(&svg_content, &req.element_id, &css.name);
                let mut xml_block = format!("  <style>\n    {}\n  </style>\n", css.to_css());
                for anim in &smil {
                    xml_block.push_str("  ");
                    xml_block.push_str(&anim.to_xml(Some(&req.element_id)));
                    xml_block.push_str("\n");
                }
                (inject_style_or_xml(animated_svg, &xml_block), (smil.len() + 1) as u32)
            }
        };

        let filename = format!("{}.svg", uuid::Uuid::now_v7());
        let output_path = self.config.paths.output_dir.join(filename);
        let _ = std::fs::create_dir_all(&self.config.paths.output_dir);
        std::fs::write(&output_path, &animated_svg).map_err(|e| e.to_string())?;

        let file_size = std::fs::metadata(&output_path)
            .map(|m| m.len())
            .unwrap_or(0);
        let (width, height) = parse_svg_dimensions(&animated_svg);

        let result = openmedia_core::AnimatedSvgOutput {
            path: output_path,
            width,
            height,
            duration,
            animation_count,
            file_size,
            generation_id: uuid::Uuid::now_v7().to_string(),
        };

        serde_json::to_value(result)
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "animate_create_timeline",
        description = "Sequentially or concurrently coordinate animations of multiple elements."
    )]
    pub async fn animate_create_timeline(
        &self,
        params: Parameters<AnimateTimelineRequest>,
    ) -> Result<Json<serde_json::Value>, String> {
        use openmedia_animate::*;
        let req = params.0;
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

        let mode = match req.mode.to_lowercase().as_str() {
            "sequential" => TimelineMode::Sequential,
            "staggered" => TimelineMode::Staggered { delay: req.stagger_delay.unwrap_or(0.2) },
            _ => TimelineMode::Parallel,
        };

        let mut timeline = AnimationTimeline::new(mode);

        for entry in &req.entries {
            let preset = parse_preset(&entry.preset);
            let easing = parse_easing(entry.easing.as_deref());
            let entry_params = entry.params.clone().unwrap_or(serde_json::Value::Null);

            let out = preset.generate(entry.duration, entry.offset, &easing, &entry_params)
                .map_err(|e| e.to_string())?;

            match out {
                AnimationOutput::Smil(anims) => {
                    for anim in anims {
                        timeline.add(&entry.element_id, anim);
                    }
                }
                AnimationOutput::Css(_keyframes) => {
                    let anim = SmilAnimation::Animate {
                        attribute_name: "opacity".to_string(),
                        from: "0".to_string(),
                        to: "1".to_string(),
                        dur: entry.duration,
                        begin: entry.offset,
                        fill: AnimationFill::Freeze,
                        repeat_count: RepeatCount::Definite(1),
                        easing,
                    };
                    timeline.add(&entry.element_id, anim);
                }
                AnimationOutput::Combined { smil, .. } => {
                    for anim in smil {
                        timeline.add(&entry.element_id, anim);
                    }
                }
            }
        }

        let timeline_xml = timeline.to_svg();
        let animated_svg = inject_style_or_xml(svg_content, &timeline_xml);

        let filename = format!("{}.svg", uuid::Uuid::now_v7());
        let output_path = self.config.paths.output_dir.join(filename);
        let _ = std::fs::create_dir_all(&self.config.paths.output_dir);
        std::fs::write(&output_path, &animated_svg).map_err(|e| e.to_string())?;

        let file_size = std::fs::metadata(&output_path)
            .map(|m| m.len())
            .unwrap_or(0);
        let (width, height) = parse_svg_dimensions(&animated_svg);

        let result = openmedia_core::AnimatedSvgOutput {
            path: output_path,
            width,
            height,
            duration: timeline.total_duration,
            animation_count: timeline.animations.len() as u32,
            file_size,
            generation_id: uuid::Uuid::now_v7().to_string(),
        };

        serde_json::to_value(result)
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "animate_morph_paths",
        description = "Interpolate morph frames between two path data strings."
    )]
    pub async fn animate_morph_paths(
        &self,
        params: Parameters<AnimateMorphRequest>,
    ) -> Result<Json<serde_json::Value>, String> {
        let req = params.0;
        let duration = req.duration.unwrap_or(3.0);
        let easing = parse_easing(req.easing.as_deref());

        let frames = openmedia_animate::morph_paths(&req.from_path, &req.to_path, 30, &easing)
            .map_err(|e| e.to_string())?;

        let values_attr = frames.join("; ");
        let animated_svg = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 800 600\" width=\"800\" height=\"600\">\n  \
               <path d=\"{}\" fill=\"none\" stroke=\"#8b5cf6\" stroke-width=\"4\">\n    \
                 <animate attributeName=\"d\" values=\"{}\" dur=\"{}s\" repeatCount=\"indefinite\" />\n  \
               </path>\n\
             </svg>",
            req.from_path, values_attr, duration
        );

        let filename = format!("{}.svg", uuid::Uuid::now_v7());
        let output_path = self.config.paths.output_dir.join(filename);
        let _ = std::fs::create_dir_all(&self.config.paths.output_dir);
        std::fs::write(&output_path, &animated_svg).map_err(|e| e.to_string())?;

        let file_size = std::fs::metadata(&output_path)
            .map(|m| m.len())
            .unwrap_or(0);

        let result = openmedia_core::AnimatedSvgOutput {
            path: output_path,
            width: 800,
            height: 600,
            duration,
            animation_count: 1,
            file_size,
            generation_id: uuid::Uuid::now_v7().to_string(),
        };

        serde_json::to_value(result)
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "animate_generate_spinner",
        description = "Generate beautiful animated loading spinner SVGs."
    )]
    pub async fn animate_generate_spinner(
        &self,
        params: Parameters<GenerateSpinnerRequest>,
    ) -> Result<Json<serde_json::Value>, String> {
        let req = params.0;
        let color = req.color.unwrap_or_else(|| "#8b5cf6".to_string());
        let size = req.size.unwrap_or(60);

        let animated_svg = match req.spinner_type.to_lowercase().as_str() {
            "ring" => {
                format!(
                    "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 50 50\">\n  \
                       <path d=\"M 25 5 A 20 20 0 0 1 45 25\" fill=\"none\" stroke=\"{}\" stroke-width=\"4\" stroke-linecap=\"round\">\n    \
                         <animateTransform attributeName=\"transform\" type=\"rotate\" from=\"0 25 25\" to=\"360 25 25\" dur=\"1s\" repeatCount=\"indefinite\" />\n  \
                       </path>\n  \
                       <circle cx=\"25\" cy=\"25\" r=\"20\" fill=\"none\" stroke=\"{}\" stroke-width=\"4\" opacity=\"0.2\" />\n\
                     </svg>",
                    size, size, color, color
                )
            }
            "dots" => {
                format!(
                    "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 120 30\">\n  \
                       <circle cx=\"20\" cy=\"15\" r=\"8\" fill=\"{}\">\n    \
                         <animate attributeName=\"cy\" values=\"15; 5; 15\" dur=\"1s\" begin=\"0s\" repeatCount=\"indefinite\" />\n    \
                         <animate attributeName=\"opacity\" values=\"0.3; 1; 0.3\" dur=\"1s\" begin=\"0s\" repeatCount=\"indefinite\" />\n  \
                       </circle>\n  \
                       <circle cx=\"60\" cy=\"15\" r=\"8\" fill=\"{}\">\n    \
                         <animate attributeName=\"cy\" values=\"15; 5; 15\" dur=\"1s\" begin=\"0.25s\" repeatCount=\"indefinite\" />\n    \
                         <animate attributeName=\"opacity\" values=\"0.3; 1; 0.3\" dur=\"1s\" begin=\"0.25s\" repeatCount=\"indefinite\" />\n  \
                       </circle>\n  \
                       <circle cx=\"100\" cy=\"15\" r=\"8\" fill=\"{}\">\n    \
                         <animate attributeName=\"cy\" values=\"15; 5; 15\" dur=\"1s\" begin=\"0.5s\" repeatCount=\"indefinite\" />\n    \
                         <animate attributeName=\"opacity\" values=\"0.3; 1; 0.3\" dur=\"1s\" begin=\"0.5s\" repeatCount=\"indefinite\" />\n  \
                       </circle>\n\
                     </svg>",
                    size, size, color, color, color
                )
            }
            "bars" => {
                format!(
                    "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 50 50\">\n  \
                       <rect x=\"10\" y=\"15\" width=\"6\" height=\"20\" fill=\"{}\">\n    \
                         <animate attributeName=\"height\" values=\"20; 40; 20\" dur=\"1s\" begin=\"0s\" repeatCount=\"indefinite\" />\n    \
                         <animate attributeName=\"y\" values=\"15; 5; 15\" dur=\"1s\" begin=\"0s\" repeatCount=\"indefinite\" />\n  \
                       </rect>\n  \
                       <rect x=\"22\" y=\"15\" width=\"6\" height=\"20\" fill=\"{}\">\n    \
                         <animate attributeName=\"height\" values=\"20; 40; 20\" dur=\"1s\" begin=\"0.2s\" repeatCount=\"indefinite\" />\n    \
                         <animate attributeName=\"y\" values=\"15; 5; 15\" dur=\"1s\" begin=\"0.2s\" repeatCount=\"indefinite\" />\n  \
                       </rect>\n  \
                       <rect x=\"34\" y=\"15\" width=\"6\" height=\"20\" fill=\"{}\">\n    \
                         <animate attributeName=\"height\" values=\"20; 40; 20\" dur=\"1s\" begin=\"0.4s\" repeatCount=\"indefinite\" />\n    \
                         <animate attributeName=\"y\" values=\"15; 5; 15\" dur=\"1s\" begin=\"0.4s\" repeatCount=\"indefinite\" />\n  \
                       </rect>\n\
                     </svg>",
                    size, size, color, color, color
                )
            }
            _ => {
                format!(
                    "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 50 50\">\n  \
                       <circle cx=\"25\" cy=\"25\" r=\"20\" fill=\"none\" stroke=\"{}\" stroke-width=\"4\" stroke-dasharray=\"31.4 31.4\" stroke-linecap=\"round\">\n    \
                         <animateTransform attributeName=\"transform\" type=\"rotate\" from=\"0 25 25\" to=\"360 25 25\" dur=\"1.2s\" repeatCount=\"indefinite\" />\n  \
                       </circle>\n\
                     </svg>",
                    size, size, color
                )
            }
        };

        let filename = format!("{}.svg", uuid::Uuid::now_v7());
        let output_path = self.config.paths.output_dir.join(filename);
        let _ = std::fs::create_dir_all(&self.config.paths.output_dir);
        std::fs::write(&output_path, &animated_svg).map_err(|e| e.to_string())?;

        let file_size = std::fs::metadata(&output_path)
            .map(|m| m.len())
            .unwrap_or(0);

        let result = openmedia_core::AnimatedSvgOutput {
            path: output_path,
            width: size,
            height: size,
            duration: 1.0,
            animation_count: 1,
            file_size,
            generation_id: uuid::Uuid::now_v7().to_string(),
        };

        serde_json::to_value(result)
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "animate_from_lottie",
        description = "Import Lottie JSON and convert to an animated SVG."
    )]
    pub async fn animate_from_lottie(
        &self,
        params: Parameters<LottieToSvgRequest>,
    ) -> Result<Json<serde_json::Value>, String> {
        let req = params.0;
        let lottie_json = if req.lottie_json.trim().starts_with('{') {
            req.lottie_json
        } else {
            let path = std::path::Path::new(&req.lottie_json);
            if path.exists() && path.is_file() {
                std::fs::read_to_string(path).map_err(|e| e.to_string())?
            } else {
                req.lottie_json
            }
        };

        let animated_svg = openmedia_animate::lottie_to_svg(&lottie_json)
            .map_err(|e| e.to_string())?;

        let filename = format!("{}.svg", uuid::Uuid::now_v7());
        let output_path = self.config.paths.output_dir.join(filename);
        let _ = std::fs::create_dir_all(&self.config.paths.output_dir);
        std::fs::write(&output_path, &animated_svg).map_err(|e| e.to_string())?;

        let file_size = std::fs::metadata(&output_path)
            .map(|m| m.len())
            .unwrap_or(0);
        let (width, height) = parse_svg_dimensions(&animated_svg);

        let result = openmedia_core::AnimatedSvgOutput {
            path: output_path,
            width,
            height,
            duration: 3.0,
            animation_count: 1,
            file_size,
            generation_id: uuid::Uuid::now_v7().to_string(),
        };

        serde_json::to_value(result)
            .map(Json)
            .map_err(|e| e.to_string())
    }

    #[tool(
        name = "animate_to_lottie",
        description = "Export SVG to Lottie JSON."
    )]
    pub async fn animate_to_lottie(
        &self,
        params: Parameters<SvgToLottieRequest>,
    ) -> Result<Json<serde_json::Value>, String> {
        let req = params.0;
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

        let lottie_json_str = openmedia_animate::svg_to_lottie(&svg_content)
            .map_err(|e| e.to_string())?;

        let lottie_val: serde_json::Value = serde_json::from_str(&lottie_json_str)
            .map_err(|e| e.to_string())?;

        Ok(Json(lottie_val))
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

    #[tokio::test]
    async fn test_mcp_animate_svg_smil() {
        let mut config = Config::default();
        let temp_dir = std::env::temp_dir();
        config.paths.model_dir = temp_dir.join("openmedia_test_models_animate_smil");
        config.paths.output_dir = temp_dir.join("openmedia_test_output_animate_smil");
        config.paths.history_db = temp_dir.join("openmedia_test_history_animate_smil.db");

        let _ = std::fs::create_dir_all(&config.paths.output_dir);
        let server = OpenMediaServer::new(config).await.unwrap();

        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100">
            <circle id="my-circle" cx="50" cy="50" r="40" fill="blue"/>
        </svg>"#.to_string();

        let params = Parameters(AnimateSvgRequest {
            svg,
            element_id: "my-circle".to_string(),
            preset: "spin".to_string(),
            duration: Some(2.0),
            delay: Some(0.5),
            easing: Some("ease-in-out".to_string()),
            repeat_count: Some("infinite".to_string()),
            params: None,
        });

        let result = server.animate_svg(params).await;
        assert!(result.is_ok());
        let val = result.unwrap().0;
        let output: openmedia_core::AnimatedSvgOutput = serde_json::from_value(val).unwrap();
        assert_eq!(output.width, 100);
        assert_eq!(output.height, 100);
        assert_eq!(output.duration, 2.0);
        assert_eq!(output.animation_count, 1);
        assert!(output.path.exists());
        
        let file_content = std::fs::read_to_string(&output.path).unwrap();
        assert!(file_content.contains("<animateTransform"));
        assert!(file_content.contains("href=\"#my-circle\""));
        assert!(file_content.contains("dur=\"2s\""));
        assert!(file_content.contains("begin=\"0.5s\""));
        
        let _ = std::fs::remove_file(output.path);
    }

    #[tokio::test]
    async fn test_mcp_animate_svg_css() {
        let mut config = Config::default();
        let temp_dir = std::env::temp_dir();
        config.paths.model_dir = temp_dir.join("openmedia_test_models_animate_css");
        config.paths.output_dir = temp_dir.join("openmedia_test_output_animate_css");
        config.paths.history_db = temp_dir.join("openmedia_test_history_animate_css.db");

        let _ = std::fs::create_dir_all(&config.paths.output_dir);
        let server = OpenMediaServer::new(config).await.unwrap();

        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100">
            <rect id="my-rect" width="100" height="100" fill="red"/>
        </svg>"#.to_string();

        let params = Parameters(AnimateSvgRequest {
            svg,
            element_id: "my-rect".to_string(),
            preset: "pulse".to_string(),
            duration: Some(1.5),
            delay: None,
            easing: None,
            repeat_count: None,
            params: None,
        });

        let result = server.animate_svg(params).await;
        assert!(result.is_ok());
        let val = result.unwrap().0;
        let output: openmedia_core::AnimatedSvgOutput = serde_json::from_value(val).unwrap();
        assert_eq!(output.animation_count, 1);
        assert!(output.path.exists());
        
        let file_content = std::fs::read_to_string(&output.path).unwrap();
        assert!(file_content.contains("<style>"));
        assert!(file_content.contains("@keyframes pulse_preset"));
        assert!(file_content.contains("class=\"pulse_preset\""));
        
        let _ = std::fs::remove_file(output.path);
    }

    #[tokio::test]
    async fn test_mcp_animate_create_timeline() {
        let mut config = Config::default();
        let temp_dir = std::env::temp_dir();
        config.paths.model_dir = temp_dir.join("openmedia_test_models_timeline");
        config.paths.output_dir = temp_dir.join("openmedia_test_output_timeline");
        config.paths.history_db = temp_dir.join("openmedia_test_history_timeline.db");

        let _ = std::fs::create_dir_all(&config.paths.output_dir);
        let server = OpenMediaServer::new(config).await.unwrap();

        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100">
            <circle id="c1" cx="30" cy="50" r="10"/>
            <circle id="c2" cx="70" cy="50" r="10"/>
        </svg>"#.to_string();

        let entries = vec![
            TimelineEntryRequest {
                element_id: "c1".to_string(),
                preset: "fade_in".to_string(),
                duration: 1.0,
                offset: 0.0,
                easing: None,
                params: None,
            },
            TimelineEntryRequest {
                element_id: "c2".to_string(),
                preset: "fade_out".to_string(),
                duration: 2.0,
                offset: 0.5,
                easing: None,
                params: None,
            },
        ];

        let params = Parameters(AnimateTimelineRequest {
            svg,
            mode: "sequential".to_string(),
            stagger_delay: None,
            entries,
        });

        let result = server.animate_create_timeline(params).await;
        assert!(result.is_ok());
        let val = result.unwrap().0;
        let output: openmedia_core::AnimatedSvgOutput = serde_json::from_value(val).unwrap();
        assert_eq!(output.duration, 3.5);
        assert_eq!(output.animation_count, 2);
        
        let file_content = std::fs::read_to_string(&output.path).unwrap();
        assert!(file_content.contains("href=\"#c1\""));
        assert!(file_content.contains("href=\"#c2\""));
        
        let _ = std::fs::remove_file(output.path);
    }

    #[tokio::test]
    async fn test_mcp_animate_morph_paths() {
        let mut config = Config::default();
        let temp_dir = std::env::temp_dir();
        config.paths.model_dir = temp_dir.join("openmedia_test_models_morph");
        config.paths.output_dir = temp_dir.join("openmedia_test_output_morph");
        config.paths.history_db = temp_dir.join("openmedia_test_history_morph.db");

        let _ = std::fs::create_dir_all(&config.paths.output_dir);
        let server = OpenMediaServer::new(config).await.unwrap();

        let params = Parameters(AnimateMorphRequest {
            from_path: "M 0 0 L 10 10".to_string(),
            to_path: "M 10 10 L 20 20".to_string(),
            duration: Some(4.0),
            easing: Some("ease_in_out".to_string()),
        });

        let result = server.animate_morph_paths(params).await;
        assert!(result.is_ok());
        let val = result.unwrap().0;
        let output: openmedia_core::AnimatedSvgOutput = serde_json::from_value(val).unwrap();
        assert_eq!(output.duration, 4.0);
        assert_eq!(output.animation_count, 1);
        
        let file_content = std::fs::read_to_string(&output.path).unwrap();
        assert!(file_content.contains("<animate"));
        assert!(file_content.contains("attributeName=\"d\""));
        
        let _ = std::fs::remove_file(output.path);
    }

    #[tokio::test]
    async fn test_mcp_animate_generate_spinner() {
        let mut config = Config::default();
        let temp_dir = std::env::temp_dir();
        config.paths.model_dir = temp_dir.join("openmedia_test_models_spinner");
        config.paths.output_dir = temp_dir.join("openmedia_test_output_spinner");
        config.paths.history_db = temp_dir.join("openmedia_test_history_spinner.db");

        let _ = std::fs::create_dir_all(&config.paths.output_dir);
        let server = OpenMediaServer::new(config).await.unwrap();

        let params = Parameters(GenerateSpinnerRequest {
            spinner_type: "ring".to_string(),
            color: Some("#ff0000".to_string()),
            size: Some(80),
        });

        let result = server.animate_generate_spinner(params).await;
        assert!(result.is_ok());
        let val = result.unwrap().0;
        let output: openmedia_core::AnimatedSvgOutput = serde_json::from_value(val).unwrap();
        assert_eq!(output.width, 80);
        assert_eq!(output.height, 80);
        
        let file_content = std::fs::read_to_string(&output.path).unwrap();
        assert!(file_content.contains("stroke=\"#ff0000\""));
        assert!(file_content.contains("<animateTransform"));
        
        let _ = std::fs::remove_file(output.path);
    }

    #[tokio::test]
    async fn test_mcp_lottie_conversions() {
        let mut config = Config::default();
        let temp_dir = std::env::temp_dir();
        config.paths.model_dir = temp_dir.join("openmedia_test_models_lottie");
        config.paths.output_dir = temp_dir.join("openmedia_test_output_lottie");
        config.paths.history_db = temp_dir.join("openmedia_test_history_lottie.db");

        let _ = std::fs::create_dir_all(&config.paths.output_dir);
        let server = OpenMediaServer::new(config).await.unwrap();

        let lottie_json = r#"{
            "w": 120,
            "h": 120,
            "fr": 30.0,
            "ip": 0.0,
            "op": 60.0,
            "layers": [
                {
                    "ind": 1,
                    "ty": 4,
                    "nm": "test-layer",
                    "ks": {
                        "o": { "k": 100.0 },
                        "r": { "k": 0.0 },
                        "p": { "k": [60.0, 60.0, 0.0] },
                        "s": { "k": 100.0 }
                    },
                    "shapes": []
                }
            ]
        }"#.to_string();

        let params_import = Parameters(LottieToSvgRequest { lottie_json });
        let res_import = server.animate_from_lottie(params_import).await;
        assert!(res_import.is_ok());
        let val_import = res_import.unwrap().0;
        let out_import: openmedia_core::AnimatedSvgOutput = serde_json::from_value(val_import).unwrap();
        assert_eq!(out_import.width, 120);
        assert_eq!(out_import.height, 120);

        let svg_content = std::fs::read_to_string(&out_import.path).unwrap();
        let params_export = Parameters(SvgToLottieRequest { svg: svg_content });
        let res_export = server.animate_to_lottie(params_export).await;
        assert!(res_export.is_ok());
        let val_export = res_export.unwrap().0;
        assert_eq!(val_export["w"].as_u64(), Some(800));

        let _ = std::fs::remove_file(out_import.path);
    }
}
