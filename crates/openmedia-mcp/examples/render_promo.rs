use openmedia_video::{
    VideoScene, Scene, SceneElement, Position, DimensionValue, TextStyle, Anchor,
    CustomFontSpec, SceneTransition, TransitionType, Keyframe, ElementTimeline, Size,
    ShapeType, ShapeStyle
};
use std::path::Path;

#[tokio::main]
async fn main() {
    let output_path = Path::new("openmedia_promo.mp4");
    
    // Construct Custom Font Spec
    let fonts = vec![CustomFontSpec {
        family: "RobotoRegular".to_string(),
        src: "https://github.com/google/fonts/raw/main/ofl/roboto/Roboto-Regular.ttf".to_string(),
    }];

    // Slide 0: Title Card
    let slide0 = Scene {
        id: "slide_0".to_string(),
        start: 0.0,
        end: 6.0,
        elements: vec![
            SceneElement::Shape {
                shape: ShapeType::Rect,
                size: Size {
                    width: DimensionValue::Pixels(140.0),
                    height: DimensionValue::Pixels(140.0),
                },
                position: Position {
                    x: DimensionValue::Pixels(640.0),
                    y: DimensionValue::Pixels(200.0),
                },
                style: ShapeStyle {
                    fill: None,
                    stroke: Some("#00adb5".to_string()),
                    stroke_width: Some(3.0),
                    border_radius: Some(15.0),
                    opacity: Some(0.6),
                },
                timeline: Some(ElementTimeline {
                    keyframes: vec![
                        Keyframe {
                            time: 0.0,
                            opacity: Some(0.0),
                            x: Some("0".to_string()),
                            y: Some("0".to_string()),
                            scale: Some(0.0),
                            scale_x: None,
                            scale_y: None,
                            rotation: Some(0.0),
                            easing: None,
                        },
                        Keyframe {
                            time: 1.5,
                            opacity: Some(0.6),
                            x: Some("0".to_string()),
                            y: Some("0".to_string()),
                            scale: Some(1.0),
                            scale_x: None,
                            scale_y: None,
                            rotation: Some(90.0),
                            easing: Some("ease_out".to_string()),
                        },
                        Keyframe {
                            time: 6.0,
                            opacity: Some(0.0),
                            x: Some("0".to_string()),
                            y: Some("0".to_string()),
                            scale: Some(1.3),
                            scale_x: None,
                            scale_y: None,
                            rotation: Some(360.0),
                            easing: None,
                        },
                    ],
                }),
            },
            SceneElement::Text {
                content: "OpenMedia-RS".to_string(),
                style: TextStyle {
                    font_family: "RobotoRegular".to_string(),
                    font_size: 64.0,
                    font_weight: 700,
                    color: "#00adb5".to_string(), // Sleek teal
                    text_align: "center".to_string(),
                    line_height: None,
                    letter_spacing: None,
                },
                position: Position {
                    x: DimensionValue::Pixels(640.0),
                    y: DimensionValue::Pixels(200.0),
                },
                anchor: Anchor::Center,
                timeline: Some(ElementTimeline {
                    keyframes: vec![
                        Keyframe {
                            time: 0.0,
                            opacity: Some(0.0),
                            x: Some("0".to_string()),
                            y: Some("50".to_string()), // starts 50 pixels lower
                            scale: Some(0.5),
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: Some("ease_out".to_string()),
                        },
                        Keyframe {
                            time: 2.0,
                            opacity: Some(1.0),
                            x: Some("0".to_string()),
                            y: Some("0".to_string()), // moves back to original position
                            scale: Some(1.0),
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: None,
                        },
                    ],
                }),
            },
            SceneElement::Text {
                content: "Local Media Generation MCP Server".to_string(),
                style: TextStyle {
                    font_family: "RobotoRegular".to_string(),
                    font_size: 28.0,
                    font_weight: 400,
                    color: "#eeeeee".to_string(),
                    text_align: "center".to_string(),
                    line_height: None,
                    letter_spacing: None,
                },
                position: Position {
                    x: DimensionValue::Pixels(640.0),
                    y: DimensionValue::Pixels(320.0),
                },
                anchor: Anchor::Center,
                timeline: Some(ElementTimeline {
                    keyframes: vec![
                        Keyframe {
                            time: 1.0,
                            opacity: Some(0.0),
                            x: None,
                            y: None,
                            scale: None,
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: None,
                        },
                        Keyframe {
                            time: 2.5,
                            opacity: Some(1.0),
                            x: None,
                            y: None,
                            scale: None,
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: None,
                        },
                    ],
                }),
            },
            SceneElement::Text {
                content: "Local, Free, Parallel & Agent-Ready".to_string(),
                style: TextStyle {
                    font_family: "RobotoRegular".to_string(),
                    font_size: 20.0,
                    font_weight: 400,
                    color: "#888888".to_string(),
                    text_align: "center".to_string(),
                    line_height: None,
                    letter_spacing: None,
                },
                position: Position {
                    x: DimensionValue::Pixels(640.0),
                    y: DimensionValue::Pixels(420.0),
                },
                anchor: Anchor::Center,
                timeline: Some(ElementTimeline {
                    keyframes: vec![
                        Keyframe {
                            time: 2.0,
                            opacity: Some(0.0),
                            x: None,
                            y: None,
                            scale: None,
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: None,
                        },
                        Keyframe {
                            time: 3.5,
                            opacity: Some(1.0),
                            x: None,
                            y: None,
                            scale: None,
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: None,
                        },
                    ],
                }),
            },
        ],
    };

    // Slide 1: Diagrams
    let slide1 = Scene {
        id: "slide_1".to_string(),
        start: 6.0,
        end: 12.0,
        elements: vec![
            SceneElement::Text {
                content: "1. Vector Diagram Engine".to_string(),
                style: TextStyle {
                    font_family: "RobotoRegular".to_string(),
                    font_size: 40.0,
                    font_weight: 700,
                    color: "#00adb5".to_string(),
                    text_align: "left".to_string(),
                    line_height: None,
                    letter_spacing: None,
                },
                position: Position {
                    x: DimensionValue::Pixels(100.0),
                    y: DimensionValue::Pixels(150.0),
                },
                anchor: Anchor::TopLeft,
                timeline: Some(ElementTimeline {
                    keyframes: vec![
                        Keyframe {
                            time: 0.0,
                            opacity: Some(0.0),
                            x: Some("-150".to_string()),
                            y: Some("0".to_string()),
                            scale: None,
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: Some("ease_out".to_string()),
                        },
                        Keyframe {
                            time: 1.0,
                            opacity: Some(1.0),
                            x: Some("0".to_string()),
                            y: Some("0".to_string()),
                            scale: None,
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: None,
                        },
                    ],
                }),
            },
            SceneElement::Text {
                content: "• Compile raw JSON arrays to styled vector shape canvas".to_string(),
                style: TextStyle {
                    font_family: "RobotoRegular".to_string(),
                    font_size: 24.0,
                    font_weight: 400,
                    color: "#eeeeee".to_string(),
                    text_align: "left".to_string(),
                    line_height: None,
                    letter_spacing: None,
                },
                position: Position {
                    x: DimensionValue::Pixels(100.0),
                    y: DimensionValue::Pixels(250.0),
                },
                anchor: Anchor::TopLeft,
                timeline: Some(ElementTimeline {
                    keyframes: vec![
                        Keyframe {
                            time: 0.3,
                            opacity: Some(0.0),
                            x: Some("-150".to_string()),
                            y: Some("0".to_string()),
                            scale: None,
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: Some("ease_out".to_string()),
                        },
                        Keyframe {
                            time: 1.3,
                            opacity: Some(1.0),
                            x: Some("0".to_string()),
                            y: Some("0".to_string()),
                            scale: None,
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: None,
                        },
                    ],
                }),
            },
            SceneElement::Text {
                content: "• Dynamic mathematical bar, bezier-line, and pie charts".to_string(),
                style: TextStyle {
                    font_family: "RobotoRegular".to_string(),
                    font_size: 24.0,
                    font_weight: 400,
                    color: "#eeeeee".to_string(),
                    text_align: "left".to_string(),
                    line_height: None,
                    letter_spacing: None,
                },
                position: Position {
                    x: DimensionValue::Pixels(100.0),
                    y: DimensionValue::Pixels(330.0),
                },
                anchor: Anchor::TopLeft,
                timeline: Some(ElementTimeline {
                    keyframes: vec![
                        Keyframe {
                            time: 0.6,
                            opacity: Some(0.0),
                            x: Some("-150".to_string()),
                            y: Some("0".to_string()),
                            scale: None,
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: Some("ease_out".to_string()),
                        },
                        Keyframe {
                            time: 1.6,
                            opacity: Some(1.0),
                            x: Some("0".to_string()),
                            y: Some("0".to_string()),
                            scale: None,
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: None,
                        },
                    ],
                }),
            },
            SceneElement::Text {
                content: "• Render native Mermaid text graphs completely offline".to_string(),
                style: TextStyle {
                    font_family: "RobotoRegular".to_string(),
                    font_size: 24.0,
                    font_weight: 400,
                    color: "#eeeeee".to_string(),
                    text_align: "left".to_string(),
                    line_height: None,
                    letter_spacing: None,
                },
                position: Position {
                    x: DimensionValue::Pixels(100.0),
                    y: DimensionValue::Pixels(410.0),
                },
                anchor: Anchor::TopLeft,
                timeline: Some(ElementTimeline {
                    keyframes: vec![
                        Keyframe {
                            time: 0.9,
                            opacity: Some(0.0),
                            x: Some("-150".to_string()),
                            y: Some("0".to_string()),
                            scale: None,
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: Some("ease_out".to_string()),
                        },
                        Keyframe {
                            time: 1.9,
                            opacity: Some(1.0),
                            x: Some("0".to_string()),
                            y: Some("0".to_string()),
                            scale: None,
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: None,
                        },
                    ],
                }),
            },
            // PIE CHART
            SceneElement::Chart {
                chart_type: "pie".to_string(),
                data: serde_json::json!([
                    { "label": "Charts", "value": 35.0 },
                    { "label": "Flows", "value": 45.0 },
                    { "label": "Shapes", "value": 20.0 }
                ]),
                position: Position {
                    x: DimensionValue::Pixels(750.0),
                    y: DimensionValue::Pixels(200.0),
                },
                size: Size {
                    width: DimensionValue::Pixels(450.0),
                    height: DimensionValue::Pixels(350.0),
                },
                theme: "dark".to_string(),
                timeline: Some(ElementTimeline {
                    keyframes: vec![
                        Keyframe {
                            time: 0.5,
                            opacity: Some(0.0),
                            x: None,
                            y: None,
                            scale: Some(0.2),
                            scale_x: None,
                            scale_y: None,
                            rotation: Some(-45.0),
                            easing: None,
                        },
                        Keyframe {
                            time: 2.0,
                            opacity: Some(1.0),
                            x: None,
                            y: None,
                            scale: Some(1.0),
                            scale_x: None,
                            scale_y: None,
                            rotation: Some(0.0),
                            easing: Some("ease_out".to_string()),
                        },
                    ],
                }),
            },
        ],
    };

    // Slide 2: Image Filters & Shaders
    let slide2 = Scene {
        id: "slide_2".to_string(),
        start: 12.0,
        end: 18.0,
        elements: vec![
            SceneElement::Text {
                content: "2. Image Processing & Shaders".to_string(),
                style: TextStyle {
                    font_family: "RobotoRegular".to_string(),
                    font_size: 40.0,
                    font_weight: 700,
                    color: "#00adb5".to_string(),
                    text_align: "left".to_string(),
                    line_height: None,
                    letter_spacing: None,
                },
                position: Position {
                    x: DimensionValue::Pixels(100.0),
                    y: DimensionValue::Pixels(150.0),
                },
                anchor: Anchor::TopLeft,
                timeline: Some(ElementTimeline {
                    keyframes: vec![
                        Keyframe {
                            time: 0.0,
                            opacity: Some(0.0),
                            x: Some("-150".to_string()),
                            y: Some("0".to_string()),
                            scale: None,
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: Some("ease_out".to_string()),
                        },
                        Keyframe {
                            time: 1.0,
                            opacity: Some(1.0),
                            x: Some("0".to_string()),
                            y: Some("0".to_string()),
                            scale: None,
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: None,
                        },
                    ],
                }),
            },
            SceneElement::Text {
                content: "• GPU-accelerated WGSL compute shaders for ultra-fast filtering".to_string(),
                style: TextStyle {
                    font_family: "RobotoRegular".to_string(),
                    font_size: 24.0,
                    font_weight: 400,
                    color: "#eeeeee".to_string(),
                    text_align: "left".to_string(),
                    line_height: None,
                    letter_spacing: None,
                },
                position: Position {
                    x: DimensionValue::Pixels(100.0),
                    y: DimensionValue::Pixels(250.0),
                },
                anchor: Anchor::TopLeft,
                timeline: Some(ElementTimeline {
                    keyframes: vec![
                        Keyframe {
                            time: 0.3,
                            opacity: Some(0.0),
                            x: Some("-150".to_string()),
                            y: Some("0".to_string()),
                            scale: None,
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: Some("ease_out".to_string()),
                        },
                        Keyframe {
                            time: 1.3,
                            opacity: Some(1.0),
                            x: Some("0".to_string()),
                            y: Some("0".to_string()),
                            scale: None,
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: None,
                        },
                    ],
                }),
            },
            SceneElement::Text {
                content: "• Multi-threaded CPU fallback processor using Rayon".to_string(),
                style: TextStyle {
                    font_family: "RobotoRegular".to_string(),
                    font_size: 24.0,
                    font_weight: 400,
                    color: "#eeeeee".to_string(),
                    text_align: "left".to_string(),
                    line_height: None,
                    letter_spacing: None,
                },
                position: Position {
                    x: DimensionValue::Pixels(100.0),
                    y: DimensionValue::Pixels(330.0),
                },
                anchor: Anchor::TopLeft,
                timeline: Some(ElementTimeline {
                    keyframes: vec![
                        Keyframe {
                            time: 0.6,
                            opacity: Some(0.0),
                            x: Some("-150".to_string()),
                            y: Some("0".to_string()),
                            scale: None,
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: Some("ease_out".to_string()),
                        },
                        Keyframe {
                            time: 1.6,
                            opacity: Some(1.0),
                            x: Some("0".to_string()),
                            y: Some("0".to_string()),
                            scale: None,
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: None,
                        },
                    ],
                }),
            },
            SceneElement::Text {
                content: "• Format converters supporting AVIF, WebP, PNG, JPEG".to_string(),
                style: TextStyle {
                    font_family: "RobotoRegular".to_string(),
                    font_size: 24.0,
                    font_weight: 400,
                    color: "#eeeeee".to_string(),
                    text_align: "left".to_string(),
                    line_height: None,
                    letter_spacing: None,
                },
                position: Position {
                    x: DimensionValue::Pixels(100.0),
                    y: DimensionValue::Pixels(410.0),
                },
                anchor: Anchor::TopLeft,
                timeline: Some(ElementTimeline {
                    keyframes: vec![
                        Keyframe {
                            time: 0.9,
                            opacity: Some(0.0),
                            x: Some("-150".to_string()),
                            y: Some("0".to_string()),
                            scale: None,
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: Some("ease_out".to_string()),
                        },
                        Keyframe {
                            time: 1.9,
                            opacity: Some(1.0),
                            x: Some("0".to_string()),
                            y: Some("0".to_string()),
                            scale: None,
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: None,
                        },
                    ],
                }),
            },
            // BAR CHART SHOWING PERFORMANCE
            SceneElement::Chart {
                chart_type: "bar".to_string(),
                data: serde_json::json!([
                    { "label": "1 Thread", "value": 1.0 },
                    { "label": "4 Threads", "value": 3.4 },
                    { "label": "8 Threads", "value": 6.2 },
                    { "label": "WGSL GPU", "value": 12.5 }
                ]),
                position: Position {
                    x: DimensionValue::Pixels(750.0),
                    y: DimensionValue::Pixels(200.0),
                },
                size: Size {
                    width: DimensionValue::Pixels(450.0),
                    height: DimensionValue::Pixels(350.0),
                },
                theme: "dark".to_string(),
                timeline: Some(ElementTimeline {
                    keyframes: vec![
                        Keyframe {
                            time: 0.5,
                            opacity: Some(0.0),
                            x: None,
                            y: Some("100".to_string()),
                            scale: Some(0.8),
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: None,
                        },
                        Keyframe {
                            time: 2.0,
                            opacity: Some(1.0),
                            x: None,
                            y: Some("0".to_string()),
                            scale: Some(1.0),
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: Some("ease_out".to_string()),
                        },
                    ],
                }),
            },
        ],
    };

    // Render Mermaid Graph for Slide 3
    let mermaid_code = "graph TD\n  Agent[LLM Agent] --> Engine[openmedia-rs]\n  Engine --> Vector[Vector Engine]\n  Engine --> Audio[Rayon Mixer]\n  Vector --> Render[MP4 Output]\n  Audio --> Render";
    let mermaid_svg = openmedia_svg::render_mermaid(mermaid_code, None, None)
        .unwrap_or_else(|e| format!("<svg><text y=\"20\">Mermaid Error: {}</text></svg>", e));

    // Slide 3: Video Scene & Audio
    let slide3 = Scene {
        id: "slide_3".to_string(),
        start: 18.0,
        end: 24.0,
        elements: vec![
            SceneElement::Text {
                content: "3. Video DSL & Audio Mixer".to_string(),
                style: TextStyle {
                    font_family: "RobotoRegular".to_string(),
                    font_size: 40.0,
                    font_weight: 700,
                    color: "#00adb5".to_string(),
                    text_align: "left".to_string(),
                    line_height: None,
                    letter_spacing: None,
                },
                position: Position {
                    x: DimensionValue::Pixels(100.0),
                    y: DimensionValue::Pixels(150.0),
                },
                anchor: Anchor::TopLeft,
                timeline: Some(ElementTimeline {
                    keyframes: vec![
                        Keyframe {
                            time: 0.0,
                            opacity: Some(0.0),
                            x: Some("-150".to_string()),
                            y: Some("0".to_string()),
                            scale: None,
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: Some("ease_out".to_string()),
                        },
                        Keyframe {
                            time: 1.0,
                            opacity: Some(1.0),
                            x: Some("0".to_string()),
                            y: Some("0".to_string()),
                            scale: None,
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: None,
                        },
                    ],
                }),
            },
            SceneElement::Text {
                content: "• Complex Video Scene DSL layer compiler".to_string(),
                style: TextStyle {
                    font_family: "RobotoRegular".to_string(),
                    font_size: 24.0,
                    font_weight: 400,
                    color: "#eeeeee".to_string(),
                    text_align: "left".to_string(),
                    line_height: None,
                    letter_spacing: None,
                },
                position: Position {
                    x: DimensionValue::Pixels(100.0),
                    y: DimensionValue::Pixels(250.0),
                },
                anchor: Anchor::TopLeft,
                timeline: Some(ElementTimeline {
                    keyframes: vec![
                        Keyframe {
                            time: 0.3,
                            opacity: Some(0.0),
                            x: Some("-150".to_string()),
                            y: Some("0".to_string()),
                            scale: None,
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: Some("ease_out".to_string()),
                        },
                        Keyframe {
                            time: 1.3,
                            opacity: Some(1.0),
                            x: Some("0".to_string()),
                            y: Some("0".to_string()),
                            scale: None,
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: None,
                        },
                    ],
                }),
            },
            SceneElement::Text {
                content: "• Pixel-level frames transitions (crossfades, slide wipes)".to_string(),
                style: TextStyle {
                    font_family: "RobotoRegular".to_string(),
                    font_size: 24.0,
                    font_weight: 400,
                    color: "#eeeeee".to_string(),
                    text_align: "left".to_string(),
                    line_height: None,
                    letter_spacing: None,
                },
                position: Position {
                    x: DimensionValue::Pixels(100.0),
                    y: DimensionValue::Pixels(330.0),
                },
                anchor: Anchor::TopLeft,
                timeline: Some(ElementTimeline {
                    keyframes: vec![
                        Keyframe {
                            time: 0.6,
                            opacity: Some(0.0),
                            x: Some("-150".to_string()),
                            y: Some("0".to_string()),
                            scale: None,
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: Some("ease_out".to_string()),
                        },
                        Keyframe {
                            time: 1.6,
                            opacity: Some(1.0),
                            x: Some("0".to_string()),
                            y: Some("0".to_string()),
                            scale: None,
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: None,
                        },
                    ],
                }),
            },
            SceneElement::Text {
                content: "• Multi-track audio mixer (delays, volumes, and fades)".to_string(),
                style: TextStyle {
                    font_family: "RobotoRegular".to_string(),
                    font_size: 24.0,
                    font_weight: 400,
                    color: "#eeeeee".to_string(),
                    text_align: "left".to_string(),
                    line_height: None,
                    letter_spacing: None,
                },
                position: Position {
                    x: DimensionValue::Pixels(100.0),
                    y: DimensionValue::Pixels(410.0),
                },
                anchor: Anchor::TopLeft,
                timeline: Some(ElementTimeline {
                    keyframes: vec![
                        Keyframe {
                            time: 0.9,
                            opacity: Some(0.0),
                            x: Some("-150".to_string()),
                            y: Some("0".to_string()),
                            scale: None,
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: Some("ease_out".to_string()),
                        },
                        Keyframe {
                            time: 1.9,
                            opacity: Some(1.0),
                            x: Some("0".to_string()),
                            y: Some("0".to_string()),
                            scale: None,
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: None,
                        },
                    ],
                }),
            },
            // MERMAID DIAGRAM
            SceneElement::Svg {
                content: mermaid_svg,
                position: Position {
                    x: DimensionValue::Pixels(750.0),
                    y: DimensionValue::Pixels(180.0),
                },
                size: Some(Size {
                    width: DimensionValue::Pixels(450.0),
                    height: DimensionValue::Pixels(400.0),
                }),
                timeline: Some(ElementTimeline {
                    keyframes: vec![
                        Keyframe {
                            time: 0.5,
                            opacity: Some(0.0),
                            x: Some("150".to_string()),
                            y: None,
                            scale: None,
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: None,
                        },
                        Keyframe {
                            time: 2.0,
                            opacity: Some(1.0),
                            x: Some("0".to_string()),
                            y: None,
                            scale: None,
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: Some("ease_out".to_string()),
                        },
                    ],
                }),
            },
        ],
    };

    // Slide 4: Outro
    let slide4 = Scene {
        id: "slide_4".to_string(),
        start: 24.0,
        end: 30.0,
        elements: vec![
            SceneElement::Shape {
                shape: ShapeType::Circle,
                size: Size {
                    width: DimensionValue::Pixels(300.0),
                    height: DimensionValue::Pixels(300.0),
                },
                position: Position {
                    x: DimensionValue::Pixels(640.0),
                    y: DimensionValue::Pixels(300.0),
                },
                style: ShapeStyle {
                    fill: None,
                    stroke: Some("#00adb5".to_string()),
                    stroke_width: Some(2.0),
                    border_radius: None,
                    opacity: Some(0.3),
                },
                timeline: Some(ElementTimeline {
                    keyframes: vec![
                        Keyframe {
                            time: 0.0,
                            opacity: Some(0.0),
                            x: None,
                            y: None,
                            scale: Some(0.5),
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: None,
                        },
                        Keyframe {
                            time: 2.0,
                            opacity: Some(0.3),
                            x: None,
                            y: None,
                            scale: Some(1.2),
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: None,
                        },
                        Keyframe {
                            time: 4.0,
                            opacity: Some(0.1),
                            x: None,
                            y: None,
                            scale: Some(1.5),
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: None,
                        },
                        Keyframe {
                            time: 6.0,
                            opacity: Some(0.0),
                            x: None,
                            y: None,
                            scale: Some(1.8),
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: None,
                        },
                    ],
                }),
            },
            SceneElement::Text {
                content: "Powered by OpenMedia-RS".to_string(),
                style: TextStyle {
                    font_family: "RobotoRegular".to_string(),
                    font_size: 48.0,
                    font_weight: 700,
                    color: "#00adb5".to_string(),
                    text_align: "center".to_string(),
                    line_height: None,
                    letter_spacing: None,
                },
                position: Position {
                    x: DimensionValue::Pixels(640.0),
                    y: DimensionValue::Pixels(200.0),
                },
                anchor: Anchor::Center,
                timeline: Some(ElementTimeline {
                    keyframes: vec![
                        Keyframe {
                            time: 0.0,
                            opacity: Some(0.0),
                            x: Some("0".to_string()),
                            y: Some("-100".to_string()),
                            scale: None,
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: Some("ease_out".to_string()),
                        },
                        Keyframe {
                            time: 1.5,
                            opacity: Some(1.0),
                            x: Some("0".to_string()),
                            y: Some("0".to_string()),
                            scale: None,
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: None,
                        },
                    ],
                }),
            },
            SceneElement::Text {
                content: "Exposing 33 robust tools for LLM agent integration.".to_string(),
                style: TextStyle {
                    font_family: "RobotoRegular".to_string(),
                    font_size: 24.0,
                    font_weight: 400,
                    color: "#eeeeee".to_string(),
                    text_align: "center".to_string(),
                    line_height: None,
                    letter_spacing: None,
                },
                position: Position {
                    x: DimensionValue::Pixels(640.0),
                    y: DimensionValue::Pixels(300.0),
                },
                anchor: Anchor::Center,
                timeline: Some(ElementTimeline {
                    keyframes: vec![
                        Keyframe {
                            time: 1.0,
                            opacity: Some(0.0),
                            x: None,
                            y: None,
                            scale: Some(0.5),
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: None,
                        },
                        Keyframe {
                            time: 2.5,
                            opacity: Some(1.0),
                            x: None,
                            y: None,
                            scale: Some(1.0),
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: None,
                        },
                    ],
                }),
            },
            SceneElement::Text {
                content: "Built on Rust. Engineered for Speed.".to_string(),
                style: TextStyle {
                    font_family: "RobotoRegular".to_string(),
                    font_size: 18.0,
                    font_weight: 400,
                    color: "#888888".to_string(),
                    text_align: "center".to_string(),
                    line_height: None,
                    letter_spacing: None,
                },
                position: Position {
                    x: DimensionValue::Pixels(640.0),
                    y: DimensionValue::Pixels(400.0),
                },
                anchor: Anchor::Center,
                timeline: Some(ElementTimeline {
                    keyframes: vec![
                        Keyframe {
                            time: 2.0,
                            opacity: Some(0.0),
                            x: None,
                            y: None,
                            scale: None,
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: None,
                        },
                        Keyframe {
                            time: 3.5,
                            opacity: Some(1.0),
                            x: None,
                            y: None,
                            scale: None,
                            scale_x: None,
                            scale_y: None,
                            rotation: None,
                            easing: None,
                        },
                    ],
                }),
            },
        ],
    };

    // Transitions
    let transitions = vec![
        SceneTransition {
            from: "slide_0".to_string(),
            to: "slide_1".to_string(),
            transition_type: TransitionType::Crossfade,
            duration: 0.5,
            easing: Some("ease_in_out".to_string()),
        },
        SceneTransition {
            from: "slide_1".to_string(),
            to: "slide_2".to_string(),
            transition_type: TransitionType::SlideLeft,
            duration: 0.5,
            easing: Some("ease_in_out".to_string()),
        },
        SceneTransition {
            from: "slide_2".to_string(),
            to: "slide_3".to_string(),
            transition_type: TransitionType::SlideRight,
            duration: 0.5,
            easing: Some("ease_in_out".to_string()),
        },
        SceneTransition {
            from: "slide_3".to_string(),
            to: "slide_4".to_string(),
            transition_type: TransitionType::SlideUp,
            duration: 0.5,
            easing: Some("ease_in_out".to_string()),
        },
    ];

    let scene = VideoScene {
        width: 1280,
        height: 720,
        fps: 15,
        duration: 30.0,
        background: "#1a1a2e".to_string(),
        scenes: vec![slide0, slide1, slide2, slide3, slide4],
        transitions,
        audio: None,
        custom_fonts: Some(fonts),
    };

    println!("Starting render of openmedia_promo.mp4 (30s, 15fps, 1280x720)...");
    let spec = openmedia_video::render_video_scene(&scene, output_path).await.unwrap();
    println!("SUCCESS: Video generated at {:?}", spec.path);
}
