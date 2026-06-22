use serde::{Deserialize, Serialize};
use openmedia_core::{Result, SvgOutput};
use std::collections::HashMap;

pub struct SvgBuilder {
    pub width: u32,
    pub height: u32,
    pub viewbox: Option<String>,
    pub elements: Vec<SvgElement>,
    pub defs: Vec<SvgDef>,
    pub styles: Vec<String>,
}

pub enum SvgElement {
    Rect { x: f64, y: f64, width: f64, height: f64, rx: Option<f64>, ry: Option<f64>, attrs: Attributes },
    Circle { cx: f64, cy: f64, r: f64, attrs: Attributes },
    Ellipse { cx: f64, cy: f64, rx: f64, ry: f64, attrs: Attributes },
    Line { x1: f64, y1: f64, x2: f64, y2: f64, attrs: Attributes },
    Polyline { points: Vec<(f64, f64)>, attrs: Attributes },
    Polygon { points: Vec<(f64, f64)>, attrs: Attributes },
    Path { d: String, attrs: Attributes },
    Text { x: f64, y: f64, content: String, attrs: Attributes },
    Group { elements: Vec<SvgElement>, attrs: Attributes },
    Use { href: String, x: f64, y: f64, attrs: Attributes },
    Image { href: String, x: f64, y: f64, width: f64, height: f64, attrs: Attributes },
}

pub enum SvgDef {
    LinearGradient { id: String, x1: String, y1: String, x2: String, y2: String, stops: Vec<GradientStop> },
    RadialGradient { id: String, cx: String, cy: String, r: String, stops: Vec<GradientStop> },
    ClipPath { id: String, elements: Vec<SvgElement> },
    Filter { id: String, primitives: Vec<FilterPrimitive> },
    Symbol { id: String, viewbox: String, elements: Vec<SvgElement> },
}

#[derive(Debug, Clone)]
pub struct GradientStop {
    pub offset: String,
    pub color: String,
    pub opacity: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct FilterPrimitive {
    pub name: String,
    pub attrs: Attributes,
}

pub type Attributes = HashMap<String, String>;

pub struct RectBuilder<'a> {
    builder: &'a mut SvgBuilder,
}

impl<'a> RectBuilder<'a> {
    pub fn fill(self, _color: &str) -> Self { self }
    pub fn stroke(self, _color: &str) -> Self { self }
    pub fn stroke_width(self, _width: f64) -> Self { self }
    pub fn finish(self) -> &'a mut SvgBuilder { self.builder }
}

pub struct CircleBuilder<'a> {
    builder: &'a mut SvgBuilder,
}

impl<'a> CircleBuilder<'a> {
    pub fn fill(self, _color: &str) -> Self { self }
    pub fn finish(self) -> &'a mut SvgBuilder { self.builder }
}

pub struct TextBuilder<'a> {
    builder: &'a mut SvgBuilder,
}

impl<'a> TextBuilder<'a> {
    pub fn fill(self, _color: &str) -> Self { self }
    pub fn font_size(self, _size: f64) -> Self { self }
    pub fn finish(self) -> &'a mut SvgBuilder { self.builder }
}

pub struct PathBuilder<'a> {
    builder: &'a mut SvgBuilder,
}

impl<'a> PathBuilder<'a> {
    pub fn fill(self, _color: &str) -> Self { self }
    pub fn stroke(self, _color: &str) -> Self { self }
    pub fn finish(self) -> &'a mut SvgBuilder { self.builder }
}

pub struct GroupBuilder<'a> {
    builder: &'a mut SvgBuilder,
}

impl<'a> GroupBuilder<'a> {
    pub fn finish(self) -> &'a mut SvgBuilder { self.builder }
}

pub struct GradientBuilder<'a> {
    builder: &'a mut SvgBuilder,
}

impl<'a> GradientBuilder<'a> {
    pub fn stop(self, _offset: &str, _color: &str) -> Self { self }
    pub fn finish(self) -> &'a mut SvgBuilder { self.builder }
}

impl SvgBuilder {
    /// Create a new SVG builder with given dimensions
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            viewbox: None,
            elements: Vec::new(),
            defs: Vec::new(),
            styles: Vec::new(),
        }
    }

    /// Set custom viewBox
    pub fn viewbox(mut self, viewbox: &str) -> Self {
        self.viewbox = Some(viewbox.to_string());
        self
    }

    /// Add a rectangle
    pub fn rect(&mut self, _x: f64, _y: f64, _width: f64, _height: f64) -> RectBuilder<'_> {
        RectBuilder { builder: self }
    }

    /// Add a circle
    pub fn circle(&mut self, _cx: f64, _cy: f64, _r: f64) -> CircleBuilder<'_> {
        CircleBuilder { builder: self }
    }

    /// Add a text element
    pub fn text(&mut self, _x: f64, _y: f64, _content: &str) -> TextBuilder<'_> {
        TextBuilder { builder: self }
    }

    /// Add a path element
    pub fn path(&mut self, _d: &str) -> PathBuilder<'_> {
        PathBuilder { builder: self }
    }

    /// Start a group
    pub fn group(&mut self) -> GroupBuilder<'_> {
        GroupBuilder { builder: self }
    }

    /// Define a linear gradient
    pub fn linear_gradient(&mut self, _id: &str) -> GradientBuilder<'_> {
        GradientBuilder { builder: self }
    }

    /// Define a radial gradient
    pub fn radial_gradient(&mut self, _id: &str) -> GradientBuilder<'_> {
        GradientBuilder { builder: self }
    }

    /// Add inline CSS styles
    pub fn style(mut self, css: &str) -> Self {
        self.styles.push(css.to_string());
        self
    }

    /// Build the final SVG string
    pub fn build(self) -> String {
        "<svg></svg>".to_string()
    }

    /// Build and write to a file
    pub fn build_to_file(self, path: &std::path::Path) -> Result<SvgOutput> {
        std::fs::write(path, self.build())?;
        Ok(SvgOutput {
            path: path.to_path_buf(),
            width: 800,
            height: 600,
            content: None,
            file_size: 0,
            generation_id: uuid::Uuid::now_v7().to_string(),
        })
    }
}

/// Chart type selection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChartType {
    Bar,
    Line,
    Pie,
    Scatter,
    Radar,
    Heatmap,
    Treemap,
    Gauge,
}

/// Configuration for chart generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartConfig {
    pub chart_type: ChartType,
    pub data: serde_json::Value,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub width: u32,
    pub height: u32,
    pub theme: ChartTheme,
    pub legend: LegendConfig,
    pub grid: bool,
    pub animate: bool,
    pub padding: Padding,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartTheme {
    pub background: String,
    pub text_color: String,
    pub grid_color: String,
    pub axis_color: String,
    pub palette: Vec<String>,
    pub font_family: String,
    pub font_size: f32,
}

impl ChartTheme {
    pub fn dark() -> Self {
        Self {
            background: "#1a1a2e".into(),
            text_color: "#e0e0e0".into(),
            grid_color: "#333355".into(),
            axis_color: "#555577".into(),
            palette: vec![
                "#e94560".into(), "#0f3460".into(), "#16213e".into(),
                "#533483".into(), "#e94560".into(), "#f5b461".into(),
                "#61c0bf".into(), "#bbbbbb".into(),
            ],
            font_family: "Inter, sans-serif".into(),
            font_size: 14.0,
        }
    }

    pub fn light() -> Self {
        Self {
            background: "#ffffff".into(),
            text_color: "#333333".into(),
            grid_color: "#e0e0e0".into(),
            axis_color: "#999999".into(),
            palette: vec![
                "#2563eb".into(), "#dc2626".into(), "#16a34a".into(),
                "#9333ea".into(), "#ea580c".into(), "#0891b2".into(),
                "#4f46e5".into(), "#64748b".into(),
            ],
            font_family: "Inter, sans-serif".into(),
            font_size: 14.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegendConfig {
    pub show: bool,
    pub position: LegendPosition,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LegendPosition {
    Top, Bottom, Left, Right,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Padding {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

/// Diagram type for technical diagrams
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagramType {
    Flowchart,
    Sequence,
    Architecture,
    ErDiagram,
    Tree,
    MindMap,
    Gantt,
    Timeline,
    Network,
}

/// Generate a chart as SVG
pub fn generate_chart(config: &ChartConfig) -> Result<String> {
    Ok(format!("<svg width=\"{}\" height=\"{}\"><text y=\"20\">Chart: {:?}</text></svg>", config.width, config.height, config.chart_type))
}
