use serde::{Deserialize, Serialize};
use openmedia_core::{Result, OpenMediaError, ImageOutput};
use std::path::Path;
use futures::StreamExt;
use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::handler::viewport::Viewport;
use tokio::io::AsyncWriteExt;

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

impl Default for DummyFrameRenderer {
    fn default() -> Self {
        Self::new()
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

// === Native SVG Frame Renderer ===
pub struct SvgFrameRenderer;

#[async_trait::async_trait]
impl FrameRenderer for SvgFrameRenderer {
    async fn render_frame(
        &self,
        scene: &VideoScene,
        time: f64,
        width: u32,
        height: u32,
    ) -> Result<image::RgbaImage> {
        // Check for active transition
        let mut active_trans = None;
        for trans in &scene.transitions {
            if let Some(from_s) = scene.scenes.iter().find(|s| s.id == trans.from) {
                let trans_start = from_s.end - trans.duration;
                if time >= trans_start && time <= from_s.end {
                    if let Some(to_s) = scene.scenes.iter().find(|s| s.id == trans.to) {
                        active_trans = Some((from_s, to_s, trans_start, trans));
                        break;
                    }
                }
            }
        }

        if let Some((from_s, to_s, trans_start, trans)) = active_trans {
            let progress = (time - trans_start) / trans.duration;
            
            let mut from_scene = scene.clone();
            from_scene.scenes = vec![from_s.clone()];
            from_scene.transitions = vec![];
            let img_from = self.render_frame(&from_scene, time, width, height).await?;
            
            let mut to_scene = scene.clone();
            to_scene.scenes = vec![to_s.clone()];
            to_scene.transitions = vec![];
            let img_to = self.render_frame(&to_scene, time, width, height).await?;
            
            return Ok(blend_frames(&img_from, &img_to, progress, &trans.transition_type));
        }

        let svg_str = compile_scene_to_svg(scene, time, width, height)?;
        let opt = resvg::usvg::Options::default();
        let tree = resvg::usvg::Tree::from_str(&svg_str, &opt)
            .map_err(|e| OpenMediaError::InvalidSvgInput(e.to_string()))?;
            
        let mut pixmap = tiny_skia::Pixmap::new(width, height)
            .ok_or_else(|| OpenMediaError::InvalidDimensions {
                width,
                height,
                reason: "Failed to allocate pixmap".to_string(),
            })?;
            
        let transform = tiny_skia::Transform::default();
        resvg::render(&tree, transform, &mut pixmap.as_mut());
        
        let mut pixels = pixmap.data().to_vec();
        for chunk in pixels.chunks_exact_mut(4) {
            let a = chunk[3];
            if a > 0 && a < 255 {
                let alpha_factor = 255.0 / a as f32;
                chunk[0] = (chunk[0] as f32 * alpha_factor).min(255.0) as u8;
                chunk[1] = (chunk[1] as f32 * alpha_factor).min(255.0) as u8;
                chunk[2] = (chunk[2] as f32 * alpha_factor).min(255.0) as u8;
            }
        }

        let buffer = image::ImageBuffer::from_raw(width, height, pixels)
            .ok_or_else(|| OpenMediaError::Internal("Failed to build RgbaImage".to_string()))?;
            
        Ok(buffer)
    }

    fn name(&self) -> &str {
        "svg"
    }
}

fn resolve_dimension(val: &DimensionValue, total: f64) -> f64 {
    match val {
        DimensionValue::Pixels(pixels) => *pixels,
        DimensionValue::Percentage(pct) => {
            let clean = pct.trim_end_matches('%');
            if let Ok(pct_val) = clean.parse::<f64>() {
                (pct_val / 100.0) * total
            } else {
                0.0
            }
        }
    }
}

fn interpolate_f64(t: f64, keyframes: &[Keyframe], get_val: impl Fn(&Keyframe) -> Option<f64>, default: f64) -> f64 {
    if keyframes.is_empty() {
        return default;
    }
    
    let mut sorted = keyframes.to_vec();
    sorted.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
    
    if t <= sorted[0].time {
        return get_val(&sorted[0]).unwrap_or(default);
    }
    if t >= sorted[sorted.len() - 1].time {
        return get_val(&sorted[sorted.len() - 1]).unwrap_or(default);
    }
    
    for window in sorted.windows(2) {
        let k1 = &window[0];
        let k2 = &window[1];
        if t >= k1.time && t <= k2.time {
            let v1 = get_val(k1).unwrap_or(default);
            let v2 = get_val(k2).unwrap_or(default);
            let duration = k2.time - k1.time;
            if duration == 0.0 {
                return v2;
            }
            let mut progress = (t - k1.time) / duration;
            if let Some(easing) = &k2.easing {
                progress = match easing.to_lowercase().as_str() {
                    "ease_in" | "ease-in" => progress * progress,
                    "ease_out" | "ease-out" => progress * (2.0 - progress),
                    "ease_in_out" | "ease-in-out" => {
                        if progress < 0.5 {
                            2.0 * progress * progress
                        } else {
                            -1.0 + (4.0 - 2.0 * progress) * progress
                        }
                    }
                    _ => progress,
                };
            }
            return v1 + (v2 - v1) * progress;
        }
    }
    default
}

fn interpolate_string_dimension(
    t: f64,
    keyframes: &[Keyframe],
    get_str: impl Fn(&Keyframe) -> Option<&String>,
    default_str: &str,
    total: f64,
) -> f64 {
    let parse_dim = |s: &str| -> f64 {
        if s.ends_with('%') {
            let clean = s.trim_end_matches('%');
            if let Ok(p) = clean.parse::<f64>() {
                (p / 100.0) * total
            } else {
                0.0
            }
        } else {
            s.parse::<f64>().unwrap_or(0.0)
        }
    };
    
    if keyframes.is_empty() {
        return parse_dim(default_str);
    }
    
    let mut sorted = keyframes.to_vec();
    sorted.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
    
    if t <= sorted[0].time {
        let s = get_str(&sorted[0]).map(|x| x.as_str()).unwrap_or(default_str);
        return parse_dim(s);
    }
    if t >= sorted[sorted.len() - 1].time {
        let s = get_str(&sorted[sorted.len() - 1]).map(|x| x.as_str()).unwrap_or(default_str);
        return parse_dim(s);
    }
    
    for window in sorted.windows(2) {
        let k1 = &window[0];
        let k2 = &window[1];
        if t >= k1.time && t <= k2.time {
            let s1 = get_str(k1).map(|x| x.as_str()).unwrap_or(default_str);
            let s2 = get_str(k2).map(|x| x.as_str()).unwrap_or(default_str);
            let v1 = parse_dim(s1);
            let v2 = parse_dim(s2);
            let duration = k2.time - k1.time;
            if duration == 0.0 {
                return v2;
            }
            let mut progress = (t - k1.time) / duration;
            if let Some(easing) = &k2.easing {
                progress = match easing.to_lowercase().as_str() {
                    "ease_in" | "ease-in" => progress * progress,
                    "ease_out" | "ease-out" => progress * (2.0 - progress),
                    "ease_in_out" | "ease-in-out" => {
                        if progress < 0.5 {
                            2.0 * progress * progress
                        } else {
                            -1.0 + (4.0 - 2.0 * progress) * progress
                        }
                    }
                    _ => progress,
                };
            }
            return v1 + (v2 - v1) * progress;
        }
    }
    parse_dim(default_str)
}

fn compile_scene_to_svg(scene: &VideoScene, time: f64, width: u32, height: u32) -> Result<String> {
    let mut active_scene = None;
    for s in &scene.scenes {
        if time >= s.start && time <= s.end {
            active_scene = Some(s);
            break;
        }
    }

    let mut svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">"#,
        width, height, width, height
    );
    svg.push_str(&format!(
        r#"<rect width="100%" height="100%" fill="{}"/>"#,
        scene.background
    ));

    if let Some(s) = active_scene {
        let local_t = time - s.start;
        for el in &s.elements {
            let el_svg = render_element_to_svg(el, local_t, width as f64, height as f64)?;
            svg.push_str(&el_svg);
        }
    }

    svg.push_str("</svg>");
    Ok(svg)
}

fn render_element_to_svg(el: &SceneElement, t: f64, total_w: f64, total_h: f64) -> Result<String> {
    match el {
        SceneElement::Shape { shape, size, position, style, timeline } => {
            let (opacity, x_offset, y_offset, scale_x, scale_y, rotation) = if let Some(tl) = timeline {
                let op = interpolate_f64(t, &tl.keyframes, |k| k.opacity, 1.0);
                let x = interpolate_string_dimension(t, &tl.keyframes, |k| k.x.as_ref(), "0", total_w);
                let y = interpolate_string_dimension(t, &tl.keyframes, |k| k.y.as_ref(), "0", total_h);
                let scale = interpolate_f64(t, &tl.keyframes, |k| k.scale, 1.0);
                let sx = interpolate_f64(t, &tl.keyframes, |k| k.scale_x, scale);
                let sy = interpolate_f64(t, &tl.keyframes, |k| k.scale_y, scale);
                let rot = interpolate_f64(t, &tl.keyframes, |k| k.rotation, 0.0);
                (op, x, y, sx, sy, rot)
            } else {
                (1.0, 0.0, 0.0, 1.0, 1.0, 0.0)
            };

            let base_x = resolve_dimension(&position.x, total_w);
            let base_y = resolve_dimension(&position.y, total_h);
            let final_x = base_x + x_offset;
            let final_y = base_y + y_offset;

            let w = resolve_dimension(&size.width, total_w);
            let h = resolve_dimension(&size.height, total_h);
            let fill_str = style.fill.as_deref().unwrap_or("none");
            let stroke_str = style.stroke.as_deref().unwrap_or("none");
            let stroke_w = style.stroke_width.unwrap_or(0.0);

            match shape {
                ShapeType::Rect => {
                    let rx = style.border_radius.unwrap_or(0.0);
                    Ok(format!(
                        r#"<rect x="0" y="0" width="{}" height="{}" rx="{}" fill="{}" stroke="{}" stroke-width="{}" opacity="{}" transform="translate({}, {}) rotate({}) scale({}, {}) translate({}, {})"/>"#,
                        w, h, rx, fill_str, stroke_str, stroke_w, opacity,
                        final_x + w / 2.0, final_y + h / 2.0,
                        rotation,
                        scale_x, scale_y,
                        -w / 2.0, -h / 2.0
                    ))
                }
                ShapeType::Circle => {
                    let r = w / 2.0;
                    Ok(format!(
                        r#"<circle cx="0" cy="0" r="{}" fill="{}" stroke="{}" stroke-width="{}" opacity="{}" transform="translate({}, {}) rotate({}) scale({}, {})"/>"#,
                        r, fill_str, stroke_str, stroke_w, opacity,
                        final_x + r, final_y + r,
                        rotation,
                        scale_x, scale_y
                    ))
                }
                _ => Ok(String::new()),
            }
        }
        SceneElement::Text { content, style, position, anchor, timeline } => {
            let (opacity, x_offset, y_offset, scale_x, scale_y, rotation) = if let Some(tl) = timeline {
                let op = interpolate_f64(t, &tl.keyframes, |k| k.opacity, 1.0);
                let x = interpolate_string_dimension(t, &tl.keyframes, |k| k.x.as_ref(), "0", total_w);
                let y = interpolate_string_dimension(t, &tl.keyframes, |k| k.y.as_ref(), "0", total_h);
                let scale = interpolate_f64(t, &tl.keyframes, |k| k.scale, 1.0);
                let sx = interpolate_f64(t, &tl.keyframes, |k| k.scale_x, scale);
                let sy = interpolate_f64(t, &tl.keyframes, |k| k.scale_y, scale);
                let rot = interpolate_f64(t, &tl.keyframes, |k| k.rotation, 0.0);
                (op, x, y, sx, sy, rot)
            } else {
                (1.0, 0.0, 0.0, 1.0, 1.0, 0.0)
            };

            let base_x = resolve_dimension(&position.x, total_w);
            let base_y = resolve_dimension(&position.y, total_h);
            let final_x = base_x + x_offset;
            let final_y = base_y + y_offset;

            let text_anchor = match anchor {
                Anchor::TopLeft | Anchor::CenterLeft | Anchor::BottomLeft => "start",
                Anchor::TopCenter | Anchor::Center | Anchor::BottomCenter => "middle",
                Anchor::TopRight | Anchor::CenterRight | Anchor::BottomRight => "end",
            };

            let escaped = content.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;");

            Ok(format!(
                r#"<text x="0" y="0" fill="{}" font-family="{}" font-size="{}" font-weight="{}" text-anchor="{}" opacity="{}" transform="translate({}, {}) rotate({}) scale({}, {})">{}</text>"#,
                style.color, style.font_family, style.font_size, style.font_weight, text_anchor, opacity,
                final_x, final_y + style.font_size as f64,
                rotation,
                scale_x, scale_y,
                escaped
            ))
        }
        SceneElement::Image { src, position, size, timeline, .. } => {
            let (opacity, x_offset, y_offset, scale_x, scale_y, rotation) = if let Some(tl) = timeline {
                let op = interpolate_f64(t, &tl.keyframes, |k| k.opacity, 1.0);
                let x = interpolate_string_dimension(t, &tl.keyframes, |k| k.x.as_ref(), "0", total_w);
                let y = interpolate_string_dimension(t, &tl.keyframes, |k| k.y.as_ref(), "0", total_h);
                let scale = interpolate_f64(t, &tl.keyframes, |k| k.scale, 1.0);
                let sx = interpolate_f64(t, &tl.keyframes, |k| k.scale_x, scale);
                let sy = interpolate_f64(t, &tl.keyframes, |k| k.scale_y, scale);
                let rot = interpolate_f64(t, &tl.keyframes, |k| k.rotation, 0.0);
                (op, x, y, sx, sy, rot)
            } else {
                (1.0, 0.0, 0.0, 1.0, 1.0, 0.0)
            };

            let base_x = resolve_dimension(&position.x, total_w);
            let base_y = resolve_dimension(&position.y, total_h);
            let final_x = base_x + x_offset;
            let final_y = base_y + y_offset;

            let w = resolve_dimension(&size.width, total_w);
            let h = resolve_dimension(&size.height, total_h);

            Ok(format!(
                r#"<image href="{}" x="0" y="0" width="{}" height="{}" opacity="{}" transform="translate({}, {}) rotate({}) scale({}, {}) translate({}, {})"/>"#,
                src, w, h, opacity,
                final_x + w / 2.0, final_y + h / 2.0,
                rotation,
                scale_x, scale_y,
                -w / 2.0, -h / 2.0
            ))
        }
        SceneElement::Svg { content, position, size, timeline } => {
            let (opacity, x_offset, y_offset, scale_x, scale_y, rotation) = if let Some(tl) = timeline {
                let op = interpolate_f64(t, &tl.keyframes, |k| k.opacity, 1.0);
                let x = interpolate_string_dimension(t, &tl.keyframes, |k| k.x.as_ref(), "0", total_w);
                let y = interpolate_string_dimension(t, &tl.keyframes, |k| k.y.as_ref(), "0", total_h);
                let scale = interpolate_f64(t, &tl.keyframes, |k| k.scale, 1.0);
                let sx = interpolate_f64(t, &tl.keyframes, |k| k.scale_x, scale);
                let sy = interpolate_f64(t, &tl.keyframes, |k| k.scale_y, scale);
                let rot = interpolate_f64(t, &tl.keyframes, |k| k.rotation, 0.0);
                (op, x, y, sx, sy, rot)
            } else {
                (1.0, 0.0, 0.0, 1.0, 1.0, 0.0)
            };

            let base_x = resolve_dimension(&position.x, total_w);
            let base_y = resolve_dimension(&position.y, total_h);
            let final_x = base_x + x_offset;
            let final_y = base_y + y_offset;

            let w = size.as_ref().map(|s| resolve_dimension(&s.width, total_w)).unwrap_or(100.0);
            let h = size.as_ref().map(|s| resolve_dimension(&s.height, total_h)).unwrap_or(100.0);

            Ok(format!(
                r#"<g opacity="{}" transform="translate({}, {}) rotate({}) scale({}, {}) translate({}, {})">{}</g>"#,
                opacity,
                final_x + w / 2.0, final_y + h / 2.0,
                rotation,
                scale_x, scale_y,
                -w / 2.0, -h / 2.0,
                content
            ))
        }
        SceneElement::Chart { chart_type, data, position, size, theme: _, timeline } => {
            let (opacity, x_offset, y_offset, scale_x, scale_y, rotation) = if let Some(tl) = timeline {
                let op = interpolate_f64(t, &tl.keyframes, |k| k.opacity, 1.0);
                let x = interpolate_string_dimension(t, &tl.keyframes, |k| k.x.as_ref(), "0", total_w);
                let y = interpolate_string_dimension(t, &tl.keyframes, |k| k.y.as_ref(), "0", total_h);
                let scale = interpolate_f64(t, &tl.keyframes, |k| k.scale, 1.0);
                let sx = interpolate_f64(t, &tl.keyframes, |k| k.scale_x, scale);
                let sy = interpolate_f64(t, &tl.keyframes, |k| k.scale_y, scale);
                let rot = interpolate_f64(t, &tl.keyframes, |k| k.rotation, 0.0);
                (op, x, y, sx, sy, rot)
            } else {
                (1.0, 0.0, 0.0, 1.0, 1.0, 0.0)
            };

            let base_x = resolve_dimension(&position.x, total_w);
            let base_y = resolve_dimension(&position.y, total_h);
            let final_x = base_x + x_offset;
            let final_y = base_y + y_offset;

            let w = resolve_dimension(&size.width, total_w);
            let h = resolve_dimension(&size.height, total_h);

            let chart_theme = openmedia_svg::ChartTheme::dark();
            let chart_cfg = openmedia_svg::ChartConfig {
                chart_type: match chart_type.to_lowercase().as_str() {
                    "bar" => openmedia_svg::ChartType::Bar,
                    "line" => openmedia_svg::ChartType::Line,
                    "pie" => openmedia_svg::ChartType::Pie,
                    _ => openmedia_svg::ChartType::Bar,
                },
                data: data.clone(),
                title: None,
                subtitle: None,
                width: w as u32,
                height: h as u32,
                theme: chart_theme,
                legend: openmedia_svg::LegendConfig { show: false, position: openmedia_svg::LegendPosition::Bottom },
                grid: true,
                animate: false,
                padding: openmedia_svg::Padding { top: 10.0, right: 10.0, bottom: 10.0, left: 10.0 },
            };

            let chart_xml = openmedia_svg::generate_chart(&chart_cfg)
                .map_err(|e| OpenMediaError::Internal(e.to_string()))?;

            Ok(format!(
                r#"<g opacity="{}" transform="translate({}, {}) rotate({}) scale({}, {}) translate({}, {})">{}</g>"#,
                opacity,
                final_x + w / 2.0, final_y + h / 2.0,
                rotation,
                scale_x, scale_y,
                -w / 2.0, -h / 2.0,
                chart_xml
            ))
        }
        _ => Ok(String::new()),
    }
}

// === Browser Frame Renderer (CDP Headless Chrome) ===
pub struct BrowserFrameRenderer {
    browser: Browser,
}

impl BrowserFrameRenderer {
    pub async fn launch() -> Result<Self> {
        let config = BrowserConfig::builder()
            .no_sandbox()
            .build()
            .map_err(|e| OpenMediaError::ConfigError(e.to_string()))?;
        let (browser, mut handler) = Browser::launch(config).await
            .map_err(|_| OpenMediaError::ChromeNotFound)?;

        tokio::spawn(async move {
            while let Some(h) = handler.next().await {
                if let Err(err) = h {
                    tracing::error!("BrowserFrameRenderer loop error: {:?}", err);
                    break;
                }
            }
        });

        Ok(Self { browser })
    }

    pub async fn close(mut self) {
        let _ = self.browser.close().await;
    }
}

#[async_trait::async_trait]
impl FrameRenderer for BrowserFrameRenderer {
    async fn render_frame(
        &self,
        scene: &VideoScene,
        time: f64,
        width: u32,
        height: u32,
    ) -> Result<image::RgbaImage> {
        let html_content = compile_scene_to_html(scene, time, width, height)?;
        
        let page = self.browser.new_page("about:blank").await
            .map_err(|e| OpenMediaError::Internal(e.to_string()))?;
            
        let params = chromiumoxide::cdp::browser_protocol::emulation::SetDeviceMetricsOverrideParams::builder()
            .width(width as i64)
            .height(height as i64)
            .device_scale_factor(1.0)
            .mobile(false)
            .build()
            .map_err(|e| OpenMediaError::Internal(e.to_string()))?;
        page.execute(params).await
            .map_err(|e| OpenMediaError::Internal(e.to_string()))?;

        page.set_content(html_content).await
            .map_err(|e| OpenMediaError::Internal(e.to_string()))?;

        let params = chromiumoxide::page::ScreenshotParams::builder()
            .format(chromiumoxide::cdp::browser_protocol::page::CaptureScreenshotFormat::Png)
            .build();
            
        let screenshot_bytes = page.screenshot(params).await
            .map_err(|e| OpenMediaError::Internal(e.to_string()))?;
            
        let _ = page.close().await;

        let img = image::load_from_memory(&screenshot_bytes)
            .map_err(|e| OpenMediaError::ImageDecodeError(e.to_string()))?
            .to_rgba8();

        Ok(img)
    }

    fn name(&self) -> &str {
        "browser"
    }
}

fn compile_scene_to_html(scene: &VideoScene, time: f64, width: u32, height: u32) -> Result<String> {
    let mut active_scene = None;
    let mut active_scene_to = None;
    let mut transition_progress = 0.0;
    let mut active_transition = None;

    for s in &scene.scenes {
        if time >= s.start && time <= s.end {
            active_scene = Some(s);
            break;
        }
    }
    
    for trans in &scene.transitions {
        if let Some(from_s) = scene.scenes.iter().find(|s| s.id == trans.from) {
            let trans_start = from_s.end - trans.duration;
            if time >= trans_start && time <= from_s.end {
                active_scene = Some(from_s);
                active_scene_to = scene.scenes.iter().find(|s| s.id == trans.to);
                transition_progress = (time - trans_start) / trans.duration;
                active_transition = Some(trans);
                break;
            }
        }
    }

    let bg_color = &scene.background;
    let mut html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
<style>
  body {{
    margin: 0;
    padding: 0;
    width: {}px;
    height: {}px;
    background-color: {};
    overflow: hidden;
    position: relative;
    font-family: sans-serif;
  }}
  .element {{
    position: absolute;
    transform-origin: center;
    box-sizing: border-box;
  }}
</style>
</head>
<body>"#,
        width, height, bg_color
    );

    if let Some(trans) = active_transition {
        if let (Some(s_from), Some(s_to)) = (active_scene, active_scene_to) {
            let from_html = render_scene_elements_to_html(s_from, time - s_from.start, width, height)?;
            let to_html = render_scene_elements_to_html(s_to, time - s_to.start, width, height)?;
            
            let (from_style, to_style) = match trans.transition_type {
                TransitionType::Crossfade => (
                    format!("opacity: {};", 1.0 - transition_progress),
                    format!("opacity: {};", transition_progress),
                ),
                TransitionType::SlideLeft => (
                    format!("transform: translateX(-{}px);", transition_progress * width as f64),
                    format!("transform: translateX({}px);", (1.0 - transition_progress) * width as f64),
                ),
                TransitionType::SlideRight => (
                    format!("transform: translateX({}px);", transition_progress * width as f64),
                    format!("transform: translateX(-{}px);", (1.0 - transition_progress) * width as f64),
                ),
                TransitionType::SlideUp => (
                    format!("transform: translateY(-{}px);", transition_progress * height as f64),
                    format!("transform: translateY({}px);", (1.0 - transition_progress) * height as f64),
                ),
                TransitionType::SlideDown => (
                    format!("transform: translateY({}px);", transition_progress * height as f64),
                    format!("transform: translateY(-{}px);", (1.0 - transition_progress) * height as f64),
                ),
                _ => (
                    format!("opacity: {};", 1.0 - transition_progress),
                    format!("opacity: {};", transition_progress),
                ),
            };
            
            html.push_str(&format!(
                r#"<div style="position: absolute; width: 100%; height: 100%; {}">{}</div>"#,
                from_style, from_html
            ));
            html.push_str(&format!(
                r#"<div style="position: absolute; width: 100%; height: 100%; {}">{}</div>"#,
                to_style, to_html
            ));
        }
    } else if let Some(s) = active_scene {
        let content = render_scene_elements_to_html(s, time - s.start, width, height)?;
        html.push_str(&content);
    }

    html.push_str("</body></html>");
    Ok(html)
}

fn render_scene_elements_to_html(s: &Scene, t: f64, width: u32, height: u32) -> Result<String> {
    let mut elements_html = String::new();
    let total_w = width as f64;
    let total_h = height as f64;

    for el in &s.elements {
        match el {
            SceneElement::Text { content, style, position, anchor, timeline } => {
                let (opacity, x_offset, y_offset, scale_x, scale_y, rotation) = if let Some(tl) = timeline {
                    let op = interpolate_f64(t, &tl.keyframes, |k| k.opacity, 1.0);
                    let x = interpolate_string_dimension(t, &tl.keyframes, |k| k.x.as_ref(), "0", total_w);
                    let y = interpolate_string_dimension(t, &tl.keyframes, |k| k.y.as_ref(), "0", total_h);
                    let scale = interpolate_f64(t, &tl.keyframes, |k| k.scale, 1.0);
                    let sx = interpolate_f64(t, &tl.keyframes, |k| k.scale_x, scale);
                    let sy = interpolate_f64(t, &tl.keyframes, |k| k.scale_y, scale);
                    let rot = interpolate_f64(t, &tl.keyframes, |k| k.rotation, 0.0);
                    (op, x, y, sx, sy, rot)
                } else {
                    (1.0, 0.0, 0.0, 1.0, 1.0, 0.0)
                };

                let base_x = resolve_dimension(&position.x, total_w);
                let base_y = resolve_dimension(&position.y, total_h);
                let final_x = base_x + x_offset;
                let final_y = base_y + y_offset;

                let text_align = match anchor {
                    Anchor::TopLeft | Anchor::CenterLeft | Anchor::BottomLeft => "left",
                    Anchor::TopCenter | Anchor::Center | Anchor::BottomCenter => "center",
                    Anchor::TopRight | Anchor::CenterRight | Anchor::BottomRight => "right",
                };

                elements_html.push_str(&format!(
                    r#"<div class="element" style="left: {}px; top: {}px; opacity: {}; transform: rotate({}deg) scale({}, {}); font-family: {}; font-size: {}px; color: {}; font-weight: {}; text-align: {};">{}</div>"#,
                    final_x, final_y, opacity, rotation, scale_x, scale_y, style.font_family, style.font_size, style.color, style.font_weight, text_align, content
                ));
            }
            SceneElement::Image { src, position, size, timeline, .. } => {
                let (opacity, x_offset, y_offset, scale_x, scale_y, rotation) = if let Some(tl) = timeline {
                    let op = interpolate_f64(t, &tl.keyframes, |k| k.opacity, 1.0);
                    let x = interpolate_string_dimension(t, &tl.keyframes, |k| k.x.as_ref(), "0", total_w);
                    let y = interpolate_string_dimension(t, &tl.keyframes, |k| k.y.as_ref(), "0", total_h);
                    let scale = interpolate_f64(t, &tl.keyframes, |k| k.scale, 1.0);
                    let sx = interpolate_f64(t, &tl.keyframes, |k| k.scale_x, scale);
                    let sy = interpolate_f64(t, &tl.keyframes, |k| k.scale_y, scale);
                    let rot = interpolate_f64(t, &tl.keyframes, |k| k.rotation, 0.0);
                    (op, x, y, sx, sy, rot)
                } else {
                    (1.0, 0.0, 0.0, 1.0, 1.0, 0.0)
                };

                let base_x = resolve_dimension(&position.x, total_w);
                let base_y = resolve_dimension(&position.y, total_h);
                let final_x = base_x + x_offset;
                let final_y = base_y + y_offset;

                let w = resolve_dimension(&size.width, total_w);
                let h = resolve_dimension(&size.height, total_h);

                elements_html.push_str(&format!(
                    r#"<img class="element" src="{}" style="left: {}px; top: {}px; width: {}px; height: {}px; opacity: {}; transform: rotate({}deg) scale({}, {}); object-fit: cover;"/>"#,
                    src, final_x, final_y, w, h, opacity, rotation, scale_x, scale_y
                ));
            }
            SceneElement::Shape { shape, size, position, style, timeline } => {
                let (opacity, x_offset, y_offset, scale_x, scale_y, rotation) = if let Some(tl) = timeline {
                    let op = interpolate_f64(t, &tl.keyframes, |k| k.opacity, 1.0);
                    let x = interpolate_string_dimension(t, &tl.keyframes, |k| k.x.as_ref(), "0", total_w);
                    let y = interpolate_string_dimension(t, &tl.keyframes, |k| k.y.as_ref(), "0", total_h);
                    let scale = interpolate_f64(t, &tl.keyframes, |k| k.scale, 1.0);
                    let sx = interpolate_f64(t, &tl.keyframes, |k| k.scale_x, scale);
                    let sy = interpolate_f64(t, &tl.keyframes, |k| k.scale_y, scale);
                    let rot = interpolate_f64(t, &tl.keyframes, |k| k.rotation, 0.0);
                    (op, x, y, sx, sy, rot)
                } else {
                    (1.0, 0.0, 0.0, 1.0, 1.0, 0.0)
                };

                let base_x = resolve_dimension(&position.x, total_w);
                let base_y = resolve_dimension(&position.y, total_h);
                let final_x = base_x + x_offset;
                let final_y = base_y + y_offset;

                let w = resolve_dimension(&size.width, total_w);
                let h = resolve_dimension(&size.height, total_h);
                let fill_str = style.fill.as_deref().unwrap_or("transparent");
                let stroke_str = style.stroke.as_deref().unwrap_or("none");
                let stroke_w = style.stroke_width.unwrap_or(0.0);

                let radius_str = match shape {
                    ShapeType::Circle => "border-radius: 50%;".to_string(),
                    ShapeType::RoundedRect => format!("border-radius: {}px;", style.border_radius.unwrap_or(4.0)),
                    _ => String::new(),
                };

                elements_html.push_str(&format!(
                    r#"<div class="element" style="left: {}px; top: {}px; width: {}px; height: {}px; opacity: {}; transform: rotate({}deg) scale({}, {}); background-color: {}; border: {}px solid {}; {}"></div>"#,
                    final_x, final_y, w, h, opacity, rotation, scale_x, scale_y, fill_str, stroke_w, stroke_str, radius_str
                ));
            }
            SceneElement::Svg { content, position, size, timeline } => {
                let (opacity, x_offset, y_offset, scale_x, scale_y, rotation) = if let Some(tl) = timeline {
                    let op = interpolate_f64(t, &tl.keyframes, |k| k.opacity, 1.0);
                    let x = interpolate_string_dimension(t, &tl.keyframes, |k| k.x.as_ref(), "0", total_w);
                    let y = interpolate_string_dimension(t, &tl.keyframes, |k| k.y.as_ref(), "0", total_h);
                    let scale = interpolate_f64(t, &tl.keyframes, |k| k.scale, 1.0);
                    let sx = interpolate_f64(t, &tl.keyframes, |k| k.scale_x, scale);
                    let sy = interpolate_f64(t, &tl.keyframes, |k| k.scale_y, scale);
                    let rot = interpolate_f64(t, &tl.keyframes, |k| k.rotation, 0.0);
                    (op, x, y, sx, sy, rot)
                } else {
                    (1.0, 0.0, 0.0, 1.0, 1.0, 0.0)
                };

                let base_x = resolve_dimension(&position.x, total_w);
                let base_y = resolve_dimension(&position.y, total_h);
                let final_x = base_x + x_offset;
                let final_y = base_y + y_offset;

                let w_str = size.as_ref().map(|s| format!("width: {}px;", resolve_dimension(&s.width, total_w))).unwrap_or_default();
                let h_str = size.as_ref().map(|s| format!("height: {}px;", resolve_dimension(&s.height, total_h))).unwrap_or_default();

                elements_html.push_str(&format!(
                    r#"<div class="element" style="left: {}px; top: {}px; {} {} opacity: {}; transform: rotate({}deg) scale({}, {});">{}</div>"#,
                    final_x, final_y, w_str, h_str, opacity, rotation, scale_x, scale_y, content
                ));
            }
            SceneElement::Html { content, position, size, timeline } => {
                let (opacity, x_offset, y_offset, scale_x, scale_y, rotation) = if let Some(tl) = timeline {
                    let op = interpolate_f64(t, &tl.keyframes, |k| k.opacity, 1.0);
                    let x = interpolate_string_dimension(t, &tl.keyframes, |k| k.x.as_ref(), "0", total_w);
                    let y = interpolate_string_dimension(t, &tl.keyframes, |k| k.y.as_ref(), "0", total_h);
                    let scale = interpolate_f64(t, &tl.keyframes, |k| k.scale, 1.0);
                    let sx = interpolate_f64(t, &tl.keyframes, |k| k.scale_x, scale);
                    let sy = interpolate_f64(t, &tl.keyframes, |k| k.scale_y, scale);
                    let rot = interpolate_f64(t, &tl.keyframes, |k| k.rotation, 0.0);
                    (op, x, y, sx, sy, rot)
                } else {
                    (1.0, 0.0, 0.0, 1.0, 1.0, 0.0)
                };

                let base_x = resolve_dimension(&position.x, total_w);
                let base_y = resolve_dimension(&position.y, total_h);
                let final_x = base_x + x_offset;
                let final_y = base_y + y_offset;

                let w = resolve_dimension(&size.width, total_w);
                let h = resolve_dimension(&size.height, total_h);

                elements_html.push_str(&format!(
                    r#"<div class="element" style="left: {}px; top: {}px; width: {}px; height: {}px; opacity: {}; transform: rotate({}deg) scale({}, {});">{}</div>"#,
                    final_x, final_y, w, h, opacity, rotation, scale_x, scale_y, content
                ));
            }
            SceneElement::Code { content, language: _, theme: _, position, size, font_size, timeline } => {
                let (opacity, x_offset, y_offset, scale_x, scale_y, rotation) = if let Some(tl) = timeline {
                    let op = interpolate_f64(t, &tl.keyframes, |k| k.opacity, 1.0);
                    let x = interpolate_string_dimension(t, &tl.keyframes, |k| k.x.as_ref(), "0", total_w);
                    let y = interpolate_string_dimension(t, &tl.keyframes, |k| k.y.as_ref(), "0", total_h);
                    let scale = interpolate_f64(t, &tl.keyframes, |k| k.scale, 1.0);
                    let sx = interpolate_f64(t, &tl.keyframes, |k| k.scale_x, scale);
                    let sy = interpolate_f64(t, &tl.keyframes, |k| k.scale_y, scale);
                    let rot = interpolate_f64(t, &tl.keyframes, |k| k.rotation, 0.0);
                    (op, x, y, sx, sy, rot)
                } else {
                    (1.0, 0.0, 0.0, 1.0, 1.0, 0.0)
                };

                let base_x = resolve_dimension(&position.x, total_w);
                let base_y = resolve_dimension(&position.y, total_h);
                let final_x = base_x + x_offset;
                let final_y = base_y + y_offset;

                let w = resolve_dimension(&size.width, total_w);
                let h = resolve_dimension(&size.height, total_h);

                elements_html.push_str(&format!(
                    r#"<pre class="element" style="left: {}px; top: {}px; width: {}px; height: {}px; opacity: {}; transform: rotate({}deg) scale({}, {}); font-size: {}px; overflow: auto; background: #282c34; color: #abb2bf; padding: 10px; border-radius: 4px; margin: 0;"><code>{}</code></pre>"#,
                    final_x, final_y, w, h, opacity, rotation, scale_x, scale_y, font_size, content
                ));
            }
            SceneElement::Chart { chart_type, data, position, size, theme: _, timeline } => {
                let (opacity, x_offset, y_offset, scale_x, scale_y, rotation) = if let Some(tl) = timeline {
                    let op = interpolate_f64(t, &tl.keyframes, |k| k.opacity, 1.0);
                    let x = interpolate_string_dimension(t, &tl.keyframes, |k| k.x.as_ref(), "0", total_w);
                    let y = interpolate_string_dimension(t, &tl.keyframes, |k| k.y.as_ref(), "0", total_h);
                    let scale = interpolate_f64(t, &tl.keyframes, |k| k.scale, 1.0);
                    let sx = interpolate_f64(t, &tl.keyframes, |k| k.scale_x, scale);
                    let sy = interpolate_f64(t, &tl.keyframes, |k| k.scale_y, scale);
                    let rot = interpolate_f64(t, &tl.keyframes, |k| k.rotation, 0.0);
                    (op, x, y, sx, sy, rot)
                } else {
                    (1.0, 0.0, 0.0, 1.0, 1.0, 0.0)
                };

                let base_x = resolve_dimension(&position.x, total_w);
                let base_y = resolve_dimension(&position.y, total_h);
                let final_x = base_x + x_offset;
                let final_y = base_y + y_offset;

                let w = resolve_dimension(&size.width, total_w);
                let h = resolve_dimension(&size.height, total_h);

                let chart_theme = openmedia_svg::ChartTheme::dark();
                let chart_cfg = openmedia_svg::ChartConfig {
                    chart_type: match chart_type.to_lowercase().as_str() {
                        "bar" => openmedia_svg::ChartType::Bar,
                        "line" => openmedia_svg::ChartType::Line,
                        "pie" => openmedia_svg::ChartType::Pie,
                        _ => openmedia_svg::ChartType::Bar,
                    },
                    data: data.clone(),
                    title: None,
                    subtitle: None,
                    width: w as u32,
                    height: h as u32,
                    theme: chart_theme,
                    legend: openmedia_svg::LegendConfig { show: false, position: openmedia_svg::LegendPosition::Bottom },
                    grid: true,
                    animate: false,
                    padding: openmedia_svg::Padding { top: 10.0, right: 10.0, bottom: 10.0, left: 10.0 },
                };

                let chart_xml = openmedia_svg::generate_chart(&chart_cfg)
                    .map_err(|e| OpenMediaError::Internal(e.to_string()))?;

                elements_html.push_str(&format!(
                    r#"<div class="element" style="left: {}px; top: {}px; width: {}px; height: {}px; opacity: {}; transform: rotate({}deg) scale({}, {});">{}</div>"#,
                    final_x, final_y, w, h, opacity, rotation, scale_x, scale_y, chart_xml
                ));
            }
            _ => {}
        }
    }
    Ok(elements_html)
}

// === Unified Video compiler and renderer ===
pub fn blend_frames(
    from: &image::RgbaImage,
    to: &image::RgbaImage,
    progress: f64,
    trans_type: &TransitionType,
) -> image::RgbaImage {
    let w = from.width();
    let h = from.height();
    let mut out = image::RgbaImage::new(w, h);
    
    match trans_type {
        TransitionType::Crossfade => {
            for y in 0..h {
                for x in 0..w {
                    let p1 = from.get_pixel(x, y);
                    let p2 = to.get_pixel(x, y);
                    let r = (p1[0] as f64 * (1.0 - progress) + p2[0] as f64 * progress) as u8;
                    let g = (p1[1] as f64 * (1.0 - progress) + p2[1] as f64 * progress) as u8;
                    let b = (p1[2] as f64 * (1.0 - progress) + p2[2] as f64 * progress) as u8;
                    let a = (p1[3] as f64 * (1.0 - progress) + p2[3] as f64 * progress) as u8;
                    out.put_pixel(x, y, image::Rgba([r, g, b, a]));
                }
            }
        }
        TransitionType::SlideLeft => {
            let offset = (progress * w as f64) as i32;
            for y in 0..h {
                for x in 0..w {
                    let target_x = x as i32 + offset;
                    if target_x < w as i32 {
                        out.put_pixel(x, y, *from.get_pixel(target_x as u32, y));
                    } else {
                        let to_x = target_x - w as i32;
                        out.put_pixel(x, y, *to.get_pixel(to_x as u32, y));
                    }
                }
            }
        }
        TransitionType::SlideRight => {
            let offset = (progress * w as f64) as i32;
            for y in 0..h {
                for x in 0..w {
                    let target_x = x as i32 - offset;
                    if target_x >= 0 {
                        out.put_pixel(x, y, *from.get_pixel(target_x as u32, y));
                    } else {
                        let to_x = target_x + w as i32;
                        out.put_pixel(x, y, *to.get_pixel(to_x as u32, y));
                    }
                }
            }
        }
        TransitionType::SlideUp => {
            let offset = (progress * h as f64) as i32;
            for y in 0..h {
                for x in 0..w {
                    let target_y = y as i32 + offset;
                    if target_y < h as i32 {
                        out.put_pixel(x, y, *from.get_pixel(x, target_y as u32));
                    } else {
                        let to_y = target_y - h as i32;
                        out.put_pixel(x, y, *to.get_pixel(x, to_y as u32));
                    }
                }
            }
        }
        TransitionType::SlideDown => {
            let offset = (progress * h as f64) as i32;
            for y in 0..h {
                for x in 0..w {
                    let target_y = y as i32 - offset;
                    if target_y >= 0 {
                        out.put_pixel(x, y, *from.get_pixel(x, target_y as u32));
                    } else {
                        let to_y = target_y + h as i32;
                        out.put_pixel(x, y, *to.get_pixel(x, to_y as u32));
                    }
                }
            }
        }
        TransitionType::WipeLeft => {
            let boundary = ((1.0 - progress) * w as f64) as u32;
            for y in 0..h {
                for x in 0..w {
                    if x < boundary {
                        out.put_pixel(x, y, *from.get_pixel(x, y));
                    } else {
                        out.put_pixel(x, y, *to.get_pixel(x, y));
                    }
                }
            }
        }
        TransitionType::WipeRight => {
            let boundary = (progress * w as f64) as u32;
            for y in 0..h {
                for x in 0..w {
                    if x < boundary {
                        out.put_pixel(x, y, *to.get_pixel(x, y));
                    } else {
                        out.put_pixel(x, y, *from.get_pixel(x, y));
                    }
                }
            }
        }
        _ => {
            for y in 0..h {
                for x in 0..w {
                    let p1 = from.get_pixel(x, y);
                    let p2 = to.get_pixel(x, y);
                    let r = (p1[0] as f64 * (1.0 - progress) + p2[0] as f64 * progress) as u8;
                    let g = (p1[1] as f64 * (1.0 - progress) + p2[1] as f64 * progress) as u8;
                    let b = (p1[2] as f64 * (1.0 - progress) + p2[2] as f64 * progress) as u8;
                    let a = (p1[3] as f64 * (1.0 - progress) + p2[3] as f64 * progress) as u8;
                    out.put_pixel(x, y, image::Rgba([r, g, b, a]));
                }
            }
        }
    }
    out
}

pub async fn render_video_scene(
    scene: &VideoScene,
    output_path: &Path,
) -> Result<openmedia_core::VideoSpec> {
    let start_time = std::time::Instant::now();
    let width = scene.width;
    let height = scene.height;
    let fps = scene.fps;
    let duration = scene.duration;
    let total_frames = (duration * fps as f64).round() as u32;

    let use_browser = scene.scenes.iter().any(|s| {
        s.elements.iter().any(|el| {
            matches!(el, SceneElement::Html { .. } | SceneElement::Code { .. })
        })
    });

    let renderer_name = if use_browser { "browser" } else { "svg" };
    let temp_silent = output_path.with_extension("silent.mp4");

    // Spawn FFmpeg pipe
    let mut cmd = tokio::process::Command::new("ffmpeg");
    cmd.args([
        "-y",
        "-f", "image2pipe",
        "-vcodec", "mjpeg",
        "-r", &fps.to_string(),
        "-i", "-",
        "-c:v", "libx264",
        "-pix_fmt", "yuv420p",
        "-crf", "23",
        "-preset", "medium",
    ])
    .arg(&temp_silent);

    cmd.stdin(std::process::Stdio::piped())
       .stdout(std::process::Stdio::null())
       .stderr(std::process::Stdio::null());

    let mut child = cmd.spawn().map_err(OpenMediaError::IoError)?;
    let mut stdin = child.stdin.take().ok_or_else(|| OpenMediaError::Internal("Failed to open FFmpeg stdin".into()))?;

    if use_browser {
        let renderer = BrowserFrameRenderer::launch().await?;
        for f in 0..total_frames {
            let t = f as f64 / fps as f64;
            let frame = renderer.render_frame(scene, t, width, height).await?;
            let rgb_frame = image::DynamicImage::ImageRgba8(frame).into_rgb8();
            let mut bytes = Vec::new();
            let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut bytes, 95);
            rgb_frame.write_with_encoder(encoder).map_err(|e| OpenMediaError::ImageEncodeError { format: "jpeg".to_string(), reason: e.to_string() })?;
            stdin.write_all(&bytes).await.map_err(OpenMediaError::IoError)?;
        }
        renderer.close().await;
    } else {
        let renderer = SvgFrameRenderer;
        for f in 0..total_frames {
            let t = f as f64 / fps as f64;
            let frame = renderer.render_frame(scene, t, width, height).await?;
            let rgb_frame = image::DynamicImage::ImageRgba8(frame).into_rgb8();
            let mut bytes = Vec::new();
            let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut bytes, 95);
            rgb_frame.write_with_encoder(encoder).map_err(|e| OpenMediaError::ImageEncodeError { format: "jpeg".to_string(), reason: e.to_string() })?;
            stdin.write_all(&bytes).await.map_err(OpenMediaError::IoError)?;
        }
    }

    drop(stdin);
    child.wait().await.map_err(OpenMediaError::IoError)?;

    // Mix audio if present
    if let Some(audio_cfg) = &scene.audio {
        let mut audio_cmd = tokio::process::Command::new("ffmpeg");
        audio_cmd.arg("-y").arg("-i").arg(&temp_silent);
        
        for track in &audio_cfg.tracks {
            audio_cmd.arg("-i").arg(&track.src);
        }

        // Generate filter_complex script to delay and volume blend
        let mut filter_complex = String::new();
        for (i, track) in audio_cfg.tracks.iter().enumerate() {
            let delay_ms = (track.start * 1000.0) as i32;
            let idx = i + 1;
            filter_complex.push_str(&format!(
                "[{}:a]adelay={}|{},volume={}[a{}];",
                idx, delay_ms, delay_ms, track.volume, idx
            ));
        }

        for i in 0..audio_cfg.tracks.len() {
            filter_complex.push_str(&format!("[a{}]", i + 1));
        }
        filter_complex.push_str(&format!("amix=inputs={}:duration=first[out_a]", audio_cfg.tracks.len()));

        audio_cmd.args([
            "-filter_complex", &filter_complex,
            "-map", "0:v",
            "-map", "[out_a]",
            "-c:v", "copy",
            "-c:a", "aac",
        ])
        .arg(output_path);

        audio_cmd.stdout(std::process::Stdio::null())
                 .stderr(std::process::Stdio::null());

        let mut mix_child = audio_cmd.spawn().map_err(OpenMediaError::IoError)?;
        mix_child.wait().await.map_err(OpenMediaError::IoError)?;
        
        let _ = std::fs::remove_file(temp_silent);
    } else {
        std::fs::rename(temp_silent, output_path).map_err(OpenMediaError::IoError)?;
    }

    let file_size = std::fs::metadata(output_path).map(|m| m.len()).unwrap_or(0);
    let generation_time = start_time.elapsed().as_secs_f64();

    Ok(openmedia_core::VideoSpec {
        path: output_path.to_path_buf(),
        width,
        height,
        duration,
        fps,
        codec: "h264".to_string(),
        file_size,
        generation_id: uuid::Uuid::now_v7().to_string(),
        renderer_used: renderer_name.to_string(),
        total_frames,
        generation_time,
    })
}

// === Legacy helper matching the old schema ===
pub async fn html_to_image(
    html_content: &str,
    width: Option<u32>,
    height: Option<u32>,
    device_scale_factor: Option<f64>,
    format: &str,
    output_path: &Path,
) -> Result<ImageOutput> {
    use std::time::Instant;
    use chromiumoxide::cdp::browser_protocol::page::CaptureScreenshotFormat;
    use chromiumoxide::page::ScreenshotParams;

    let start_time = Instant::now();

    let w = width.unwrap_or(1920);
    let h = height.unwrap_or(1080);
    let scale = device_scale_factor.unwrap_or(1.0);

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

    let (mut browser, mut handler) = Browser::launch(config).await
        .map_err(|_| OpenMediaError::ChromeNotFound)?;

    tokio::spawn(async move {
        while let Some(h) = handler.next().await {
            if let Err(err) = h {
                tracing::error!("Legacy browser handler error: {:?}", err);
                break;
            }
        }
    });

    let page = browser.new_page("about:blank").await
        .map_err(|e| OpenMediaError::Internal(e.to_string()))?;

    let html_path = Path::new(html_content);
    if html_path.exists() && html_path.is_file() {
        let abs_path = html_path.canonicalize()
            .map_err(OpenMediaError::IoError)?;
        let url = format!("file://{}", abs_path.to_string_lossy());
        page.goto(&url).await
            .map_err(|e| OpenMediaError::Internal(e.to_string()))?;
    } else {
        page.set_content(html_content).await
            .map_err(|e| OpenMediaError::Internal(e.to_string()))?;
    }

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

    page.save_screenshot(params, output_path).await
        .map_err(|e| OpenMediaError::ImageEncodeError {
            format: clean_format.clone(),
            reason: e.to_string(),
        })?;

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
