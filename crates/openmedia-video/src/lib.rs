use serde::{Deserialize, Serialize};
use openmedia_core::{Result, OpenMediaError, ImageOutput};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoScene {
    /// Video width in pixels
    pub width: u32,
    /// Video height in pixels
    pub height: u32,
    /// Frames per second
    pub fps: u32,
    /// Total duration in seconds
    pub duration: f64,
    /// Background color (hex)
    pub background: String,
    /// Ordered list of scenes
    pub scenes: Vec<Scene>,
    /// Transitions between scenes
    pub transitions: Vec<SceneTransition>,
    /// Audio tracks
    pub audio: Option<AudioConfig>,
}

/// A single scene within a video
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    /// Unique scene identifier
    pub id: String,
    /// Start time in seconds
    pub start: f64,
    /// End time in seconds
    pub end: f64,
    /// Elements within this scene
    pub elements: Vec<SceneElement>,
}

/// An element within a video scene
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SceneElement {
    Text {
        content: String,
        style: TextStyle,
        position: Position,
        anchor: Anchor,
        timeline: Option<ElementTimeline>,
    },
    Image {
        src: String,
        position: Position,
        size: Size,
        fit: ObjectFit,
        timeline: Option<ElementTimeline>,
    },
    Shape {
        shape: ShapeType,
        size: Size,
        position: Position,
        style: ShapeStyle,
        timeline: Option<ElementTimeline>,
    },
    Svg {
        content: String,
        position: Position,
        size: Option<Size>,
        timeline: Option<ElementTimeline>,
    },
    Group {
        elements: Vec<SceneElement>,
        position: Position,
        transform: Option<Transform>,
        timeline: Option<ElementTimeline>,
    },
    Html {
        content: String,
        position: Position,
        size: Size,
        timeline: Option<ElementTimeline>,
    },
    Code {
        content: String,
        language: String,
        theme: String,
        position: Position,
        size: Size,
        font_size: f32,
        timeline: Option<ElementTimeline>,
    },
    Chart {
        chart_type: String,
        data: serde_json::Value,
        position: Position,
        size: Size,
        theme: String,
        timeline: Option<ElementTimeline>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: DimensionValue,
    pub y: DimensionValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DimensionValue {
    Pixels(f64),
    Percentage(String),  // e.g., "50%"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Size {
    pub width: DimensionValue,
    pub height: DimensionValue,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Anchor {
    TopLeft, TopCenter, TopRight,
    CenterLeft, Center, CenterRight,
    BottomLeft, BottomCenter, BottomRight,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObjectFit {
    Cover, Contain, Fill, ScaleDown,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShapeType {
    Rect, RoundedRect, Circle, Ellipse, Polygon, Line,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextStyle {
    pub font_family: String,
    pub font_size: f32,
    pub font_weight: u16,
    pub color: String,
    pub text_align: String,
    pub line_height: Option<f32>,
    pub letter_spacing: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShapeStyle {
    pub fill: Option<String>,
    pub stroke: Option<String>,
    pub stroke_width: Option<f32>,
    pub border_radius: Option<f32>,
    pub opacity: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transform {
    pub translate: Option<(f64, f64)>,
    pub rotate: Option<f64>,
    pub scale: Option<(f64, f64)>,
}

/// Animation timeline for a scene element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementTimeline {
    pub keyframes: Vec<Keyframe>,
}

/// A single keyframe in an element's animation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keyframe {
    /// Time in seconds (relative to scene start)
    pub time: f64,
    /// Opacity (0.0–1.0)
    pub opacity: Option<f64>,
    /// X position offset
    pub x: Option<String>,
    /// Y position offset
    pub y: Option<String>,
    /// Uniform scale
    pub scale: Option<f64>,
    /// Horizontal scale
    pub scale_x: Option<f64>,
    /// Vertical scale
    pub scale_y: Option<f64>,
    /// Rotation in degrees
    pub rotation: Option<f64>,
    /// Easing function to reach this keyframe
    pub easing: Option<String>,
}

/// Transition between two scenes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneTransition {
    /// Source scene ID
    pub from: String,
    /// Target scene ID
    pub to: String,
    /// Transition type
    #[serde(rename = "type")]
    pub transition_type: TransitionType,
    /// Duration in seconds
    pub duration: f64,
    /// Easing function
    pub easing: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransitionType {
    None,
    Crossfade,
    SlideLeft,
    SlideRight,
    SlideUp,
    SlideDown,
    ZoomIn,
    ZoomOut,
    WipeLeft,
    WipeRight,
    WipeUp,
    WipeDown,
    Dissolve,
    IrisIn,
    IrisOut,
}

/// Audio configuration for a video
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    pub tracks: Vec<AudioTrack>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioTrack {
    pub src: String,
    pub start: f64,
    pub volume: f32,
    pub fade_in: Option<f64>,
    pub fade_out: Option<f64>,
}

/// Trait for rendering a single video frame from scene elements
#[async_trait::async_trait]
pub trait FrameRenderer: Send + Sync {
    /// Render a single frame at the given time
    async fn render_frame(
        &self,
        scene: &VideoScene,
        time: f64,
        width: u32,
        height: u32,
    ) -> Result<image::RgbaImage>;

    fn name(&self) -> &str;
}

pub struct DummyFrameRenderer;

impl DummyFrameRenderer {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl FrameRenderer for DummyFrameRenderer {
    async fn render_frame(
        &self,
        _scene: &VideoScene,
        _time: f64,
        _width: u32,
        _height: u32,
    ) -> Result<image::RgbaImage> {
        Err(OpenMediaError::BackendUnavailable("Dummy frame renderer".into()))
    }

    fn name(&self) -> &str {
        "dummy"
    }
}

/// Render HTML/CSS content to a screenshot image (PNG, JPEG, WebP) using headless Chromium.
pub async fn html_to_image(
    html_content: &str,
    width: Option<u32>,
    height: Option<u32>,
    device_scale_factor: Option<f64>,
    format: &str,
    output_path: &std::path::Path,
) -> Result<ImageOutput> {
    use std::time::Instant;
    use futures::StreamExt;
    use chromiumoxide::browser::{Browser, BrowserConfig};
    use chromiumoxide::handler::viewport::Viewport;
    use chromiumoxide::page::ScreenshotParams;
    use chromiumoxide::cdp::browser_protocol::page::CaptureScreenshotFormat;

    let start_time = Instant::now();

    let w = width.unwrap_or(1920);
    let h = height.unwrap_or(1080);
    let scale = device_scale_factor.unwrap_or(1.0);

    // Create the browser configuration
    let config = BrowserConfig::builder()
        .viewport(Viewport {
            width: w,
            height: h,
            device_scale_factor: Some(scale),
            emulating_mobile: false,
            is_landscape: w > h,
            has_touch: false,
        })
        .no_sandbox()
        .build()
        .map_err(|e| OpenMediaError::ConfigError(e.to_string()))?;

    // Launch the browser
    let (mut browser, mut handler) = Browser::launch(config).await
        .map_err(|_| OpenMediaError::ChromeNotFound)?;

    // Spawn the handler task to process background CDP events
    tokio::spawn(async move {
        while let Some(h) = handler.next().await {
            if let Err(err) = h {
                tracing::error!("Browser handler error: {:?}", err);
                break;
            }
        }
    });

    // Open a new page
    let page = browser.new_page("about:blank").await
        .map_err(|e| OpenMediaError::Internal(e.to_string()))?;

    // Check if html_content is a file path that exists
    let html_path = std::path::Path::new(html_content);
    if html_path.exists() && html_path.is_file() {
        let abs_path = html_path.canonicalize()
            .map_err(|e| OpenMediaError::IoError(e))?;
        let url = format!("file://{}", abs_path.to_string_lossy());
        page.goto(&url).await
            .map_err(|e| OpenMediaError::Internal(e.to_string()))?;
    } else {
        page.set_content(html_content).await
            .map_err(|e| OpenMediaError::Internal(e.to_string()))?;
    }

    // Set screenshot format parameter
    let clean_format = format.to_lowercase();
    let cdp_format = match clean_format.as_str() {
        "png" => CaptureScreenshotFormat::Png,
        "jpeg" | "jpg" => CaptureScreenshotFormat::Jpeg,
        "webp" => CaptureScreenshotFormat::Webp,
        other => {
            return Err(OpenMediaError::InvalidParameter {
                param: "output_format".to_string(),
                reason: format!("Unsupported screenshot format: {}", other),
            });
        }
    };

    let params = ScreenshotParams::builder()
        .format(cdp_format)
        .build();

    // Capture screenshot
    page.save_screenshot(params, output_path).await
        .map_err(|e| OpenMediaError::ImageEncodeError {
            format: clean_format.clone(),
            reason: e.to_string(),
        })?;

    // Close the browser to clean up processes
    let _ = browser.close().await;

    let file_size = std::fs::metadata(output_path)?.len();
    let generation_time = start_time.elapsed().as_secs_f64();

    Ok(ImageOutput {
        path: output_path.to_path_buf(),
        width: w,
        height: h,
        seed: 0,
        format: clean_format,
        file_size,
        generation_id: uuid::Uuid::now_v7().to_string(),
        clip_score: None,
        aesthetic_score: None,
        model_used: "headless-chrome".to_string(),
        backend_used: "chromiumoxide".to_string(),
        generation_time,
    })
}
