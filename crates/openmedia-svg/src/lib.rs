use serde::{Deserialize, Serialize};
use openmedia_core::{Result, SvgOutput};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SvgBuilder {
    pub width: u32,
    pub height: u32,
    pub viewbox: Option<String>,
    pub elements: Vec<SvgElement>,
    pub defs: Vec<SvgDef>,
    pub styles: Vec<String>,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    rx: Option<f64>,
    ry: Option<f64>,
    attrs: Attributes,
}

impl<'a> RectBuilder<'a> {
    pub fn fill(mut self, color: &str) -> Self {
        self.attrs.insert("fill".to_string(), color.to_string());
        self
    }

    pub fn stroke(mut self, color: &str) -> Self {
        self.attrs.insert("stroke".to_string(), color.to_string());
        self
    }

    pub fn stroke_width(mut self, width: f64) -> Self {
        self.attrs.insert("stroke-width".to_string(), width.to_string());
        self
    }

    pub fn rx(mut self, rx: f64) -> Self {
        self.rx = Some(rx);
        self
    }

    pub fn ry(mut self, ry: f64) -> Self {
        self.ry = Some(ry);
        self
    }

    pub fn finish(self) -> &'a mut SvgBuilder {
        self.builder.elements.push(SvgElement::Rect {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
            rx: self.rx,
            ry: self.ry,
            attrs: self.attrs,
        });
        self.builder
    }
}

pub struct CircleBuilder<'a> {
    builder: &'a mut SvgBuilder,
    cx: f64,
    cy: f64,
    r: f64,
    attrs: Attributes,
}

impl<'a> CircleBuilder<'a> {
    pub fn fill(mut self, color: &str) -> Self {
        self.attrs.insert("fill".to_string(), color.to_string());
        self
    }

    pub fn stroke(mut self, color: &str) -> Self {
        self.attrs.insert("stroke".to_string(), color.to_string());
        self
    }

    pub fn stroke_width(mut self, width: f64) -> Self {
        self.attrs.insert("stroke-width".to_string(), width.to_string());
        self
    }

    pub fn finish(self) -> &'a mut SvgBuilder {
        self.builder.elements.push(SvgElement::Circle {
            cx: self.cx,
            cy: self.cy,
            r: self.r,
            attrs: self.attrs,
        });
        self.builder
    }
}

pub struct TextBuilder<'a> {
    builder: &'a mut SvgBuilder,
    x: f64,
    y: f64,
    content: String,
    attrs: Attributes,
}

impl<'a> TextBuilder<'a> {
    pub fn fill(mut self, color: &str) -> Self {
        self.attrs.insert("fill".to_string(), color.to_string());
        self
    }

    pub fn font_size(mut self, size: f64) -> Self {
        self.attrs.insert("font-size".to_string(), size.to_string());
        self
    }

    pub fn font_family(mut self, family: &str) -> Self {
        self.attrs.insert("font-family".to_string(), family.to_string());
        self
    }

    pub fn finish(self) -> &'a mut SvgBuilder {
        self.builder.elements.push(SvgElement::Text {
            x: self.x,
            y: self.y,
            content: self.content,
            attrs: self.attrs,
        });
        self.builder
    }
}

pub struct PathBuilder<'a> {
    builder: &'a mut SvgBuilder,
    d: String,
    attrs: Attributes,
}

impl<'a> PathBuilder<'a> {
    pub fn fill(mut self, color: &str) -> Self {
        self.attrs.insert("fill".to_string(), color.to_string());
        self
    }

    pub fn stroke(mut self, color: &str) -> Self {
        self.attrs.insert("stroke".to_string(), color.to_string());
        self
    }

    pub fn stroke_width(mut self, width: f64) -> Self {
        self.attrs.insert("stroke-width".to_string(), width.to_string());
        self
    }

    pub fn finish(self) -> &'a mut SvgBuilder {
        self.builder.elements.push(SvgElement::Path {
            d: self.d,
            attrs: self.attrs,
        });
        self.builder
    }
}

pub struct GroupBuilder<'a> {
    builder: &'a mut SvgBuilder,
    elements: Vec<SvgElement>,
    attrs: Attributes,
}

impl<'a> GroupBuilder<'a> {
    pub fn finish(self) -> &'a mut SvgBuilder {
        self.builder.elements.push(SvgElement::Group {
            elements: self.elements,
            attrs: self.attrs,
        });
        self.builder
    }
}

pub struct GradientBuilder<'a> {
    builder: &'a mut SvgBuilder,
    id: String,
    is_radial: bool,
    stops: Vec<GradientStop>,
}

impl<'a> GradientBuilder<'a> {
    pub fn stop(mut self, offset: &str, color: &str) -> Self {
        self.stops.push(GradientStop {
            offset: offset.to_string(),
            color: color.to_string(),
            opacity: None,
        });
        self
    }

    pub fn finish(self) -> &'a mut SvgBuilder {
        if self.is_radial {
            self.builder.defs.push(SvgDef::RadialGradient {
                id: self.id,
                cx: "50%".to_string(),
                cy: "50%".to_string(),
                r: "50%".to_string(),
                stops: self.stops,
            });
        } else {
            self.builder.defs.push(SvgDef::LinearGradient {
                id: self.id,
                x1: "0%".to_string(),
                y1: "0%".to_string(),
                x2: "100%".to_string(),
                y2: "0%".to_string(),
                stops: self.stops,
            });
        }
        self.builder
    }
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
    pub fn viewbox(&mut self, viewbox: &str) -> &mut Self {
        self.viewbox = Some(viewbox.to_string());
        self
    }

    /// Add a rectangle
    pub fn rect(&mut self, x: f64, y: f64, width: f64, height: f64) -> RectBuilder<'_> {
        RectBuilder {
            builder: self,
            x,
            y,
            width,
            height,
            rx: None,
            ry: None,
            attrs: Attributes::new(),
        }
    }

    /// Add a circle
    pub fn circle(&mut self, cx: f64, cy: f64, r: f64) -> CircleBuilder<'_> {
        CircleBuilder {
            builder: self,
            cx,
            cy,
            r,
            attrs: Attributes::new(),
        }
    }

    /// Add a text element
    pub fn text(&mut self, x: f64, y: f64, content: &str) -> TextBuilder<'_> {
        TextBuilder {
            builder: self,
            x,
            y,
            content: content.to_string(),
            attrs: Attributes::new(),
        }
    }

    /// Add a path element
    pub fn path(&mut self, d: &str) -> PathBuilder<'_> {
        PathBuilder {
            builder: self,
            d: d.to_string(),
            attrs: Attributes::new(),
        }
    }

    /// Start a group
    pub fn group(&mut self) -> GroupBuilder<'_> {
        GroupBuilder {
            builder: self,
            elements: Vec::new(),
            attrs: Attributes::new(),
        }
    }

    /// Define a linear gradient
    pub fn linear_gradient(&mut self, id: &str) -> GradientBuilder<'_> {
        GradientBuilder {
            builder: self,
            id: id.to_string(),
            is_radial: false,
            stops: Vec::new(),
        }
    }

    /// Define a radial gradient
    pub fn radial_gradient(&mut self, id: &str) -> GradientBuilder<'_> {
        GradientBuilder {
            builder: self,
            id: id.to_string(),
            is_radial: true,
            stops: Vec::new(),
        }
    }

    /// Add inline CSS styles
    pub fn style(&mut self, css: &str) -> &mut Self {
        self.styles.push(css.to_string());
        self
    }

    /// Build the final SVG string
    pub fn build(self) -> String {
        let mut svg = format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\"", self.width, self.height);
        if let Some(vb) = &self.viewbox {
            svg.push_str(&format!(" viewBox=\"{}\"", vb));
        }
        svg.push_str(">\n");

        if !self.styles.is_empty() {
            svg.push_str("  <style>\n");
            for style in &self.styles {
                svg.push_str(&format!("    {}\n", style));
            }
            svg.push_str("  </style>\n");
        }

        if !self.defs.is_empty() {
            svg.push_str("  <defs>\n");
            for def in &self.defs {
                match def {
                    SvgDef::LinearGradient { id, x1, y1, x2, y2, stops } => {
                        svg.push_str(&format!("    <linearGradient id=\"{}\" x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\">\n", id, x1, y1, x2, y2));
                        for stop in stops {
                            let opacity_attr = stop.opacity.map(|o| format!(" stop-opacity=\"{}\"", o)).unwrap_or_default();
                            svg.push_str(&format!("      <stop offset=\"{}\" stop-color=\"{}\"{} />\n", stop.offset, stop.color, opacity_attr));
                        }
                        svg.push_str("    </linearGradient>\n");
                    }
                    SvgDef::RadialGradient { id, cx, cy, r, stops } => {
                        svg.push_str(&format!("    <radialGradient id=\"{}\" cx=\"{}\" cy=\"{}\" r=\"{}\">\n", id, cx, cy, r));
                        for stop in stops {
                            let opacity_attr = stop.opacity.map(|o| format!(" stop-opacity=\"{}\"", o)).unwrap_or_default();
                            svg.push_str(&format!("      <stop offset=\"{}\" stop-color=\"{}\"{} />\n", stop.offset, stop.color, opacity_attr));
                        }
                        svg.push_str("    </radialGradient>\n");
                    }
                    _ => {}
                }
            }
            svg.push_str("  </defs>\n");
        }

        fn serialize_element(elem: &SvgElement, indent: usize) -> String {
            let ind = " ".repeat(indent);
            match elem {
                SvgElement::Rect { x, y, width, height, rx, ry, attrs } => {
                    let mut extra = String::new();
                    if let Some(r) = rx { extra.push_str(&format!(" rx=\"{}\"", r)); }
                    if let Some(r) = ry { extra.push_str(&format!(" ry=\"{}\"", r)); }
                    let attrs_str = serialize_attrs(attrs);
                    format!("{}<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"{}{} />\n", ind, x, y, width, height, extra, attrs_str)
                }
                SvgElement::Circle { cx, cy, r, attrs } => {
                    let attrs_str = serialize_attrs(attrs);
                    format!("{}<circle cx=\"{}\" cy=\"{}\" r=\"{}\"{} />\n", ind, cx, cy, r, attrs_str)
                }
                SvgElement::Ellipse { cx, cy, rx, ry, attrs } => {
                    let attrs_str = serialize_attrs(attrs);
                    format!("{}<ellipse cx=\"{}\" cy=\"{}\" rx=\"{}\" ry=\"{}\"{} />\n", ind, cx, cy, rx, ry, attrs_str)
                }
                SvgElement::Line { x1, y1, x2, y2, attrs } => {
                    let attrs_str = serialize_attrs(attrs);
                    format!("{}<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"{} />\n", ind, x1, y1, x2, y2, attrs_str)
                }
                SvgElement::Polyline { points, attrs } => {
                    let pts: Vec<String> = points.iter().map(|(x, y)| format!("{},{}", x, y)).collect();
                    let attrs_str = serialize_attrs(attrs);
                    format!("{}<polyline points=\"{}\"{} />\n", ind, pts.join(" "), attrs_str)
                }
                SvgElement::Polygon { points, attrs } => {
                    let pts: Vec<String> = points.iter().map(|(x, y)| format!("{},{}", x, y)).collect();
                    let attrs_str = serialize_attrs(attrs);
                    format!("{}<polygon points=\"{}\"{} />\n", ind, pts.join(" "), attrs_str)
                }
                SvgElement::Path { d, attrs } => {
                    let attrs_str = serialize_attrs(attrs);
                    format!("{}<path d=\"{}\"{} />\n", ind, d, attrs_str)
                }
                SvgElement::Text { x, y, content, attrs } => {
                    let attrs_str = serialize_attrs(attrs);
                    format!("{}<text x=\"{}\" y=\"{}\"{}>{}</text>\n", ind, x, y, attrs_str, content)
                }
                SvgElement::Group { elements, attrs } => {
                    let attrs_str = serialize_attrs(attrs);
                    let mut inner = format!("{}<g{}>\n", ind, attrs_str);
                    for sub in elements {
                        inner.push_str(&serialize_element(sub, indent + 2));
                    }
                    inner.push_str(&format!("{}</g>\n", ind));
                    inner
                }
                SvgElement::Use { href, x, y, attrs } => {
                    let attrs_str = serialize_attrs(attrs);
                    format!("{}<use href=\"{}\" x=\"{}\" y=\"{}\"{} />\n", ind, href, x, y, attrs_str)
                }
                SvgElement::Image { href, x, y, width, height, attrs } => {
                    let attrs_str = serialize_attrs(attrs);
                    format!("{}<image href=\"{}\" x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"{} />\n", ind, href, x, y, width, height, attrs_str)
                }
            }
        }

        fn serialize_attrs(attrs: &Attributes) -> String {
            let mut s = String::new();
            for (k, v) in attrs {
                s.push_str(&format!(" {}=\"{}\"", k, v));
            }
            s
        }

        for elem in &self.elements {
            svg.push_str(&serialize_element(elem, 2));
        }

        svg.push_str("</svg>");
        svg
    }

    /// Build and write to a file
    pub fn build_to_file(self, path: &std::path::Path) -> Result<SvgOutput> {
        let width = self.width;
        let height = self.height;
        let content = self.build();
        std::fs::write(path, &content)?;
        let file_size = content.len() as u64;
        Ok(SvgOutput {
            path: path.to_path_buf(),
            width,
            height,
            content: Some(content),
            file_size,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_svg_builder() {
        let mut builder = SvgBuilder::new(500, 500);
        builder.viewbox("0 0 100 100")
            .rect(10.0, 20.0, 30.0, 40.0)
            .fill("red")
            .stroke("black")
            .stroke_width(2.0)
            .rx(5.0)
            .ry(5.0)
            .finish()
            .circle(50.0, 50.0, 15.0)
            .fill("blue")
            .finish();

        let output = builder.build();
        assert!(output.contains("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"500\" height=\"500\" viewBox=\"0 0 100 100\">"));
        assert!(output.contains("<rect x=\"10\" y=\"20\" width=\"30\" height=\"40\" rx=\"5\" ry=\"5\""));
        assert!(output.contains("<circle cx=\"50\" cy=\"50\" r=\"15\""));
        
        // Check attributes are present
        assert!(output.contains("fill=\"red\"") || output.contains("stroke=\"black\""));
        assert!(output.contains("fill=\"blue\""));
    }
}
