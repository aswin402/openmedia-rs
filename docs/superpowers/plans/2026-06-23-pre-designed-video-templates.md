# Pre-Designed Video Templates Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Expand the pre-designed video template options in `video_from_template` (`crates/openmedia-mcp/src/lib.rs`) to include `data_dashboard`, `social_media`, and `product_showcase`.

**Architecture:** Extend the match statement in `video_from_template` to parse specific parameters for each template name and construct corresponding `VideoScene` layouts (utilizing nested text elements, shapes, charts, and image timelines).

**Tech Stack:** Rust 1.82, openmedia-video, openmedia-mcp.

## Global Constraints
- Target Rust toolchain compatibility: 1.82
- All protocols are stdio JSON-RPC transport (telemetry to stderr only)

---

### Task 1: Extend Templates Match Block in MCP Server
**Files:**
- Modify: `crates/openmedia-mcp/src/lib.rs`
- Test: `crates/openmedia-mcp/src/lib.rs` (under tests module)

**Interfaces:**
- Consumes: `VideoFromTemplateRequest` parameters input.
- Produces: Expanded JSON-RPC template compilation routing inside `video_from_template`.

- [ ] **Step 1: Write the failing integration test**
  Add unit tests inside the `tests` module in `crates/openmedia-mcp/src/lib.rs` to verify template creation works for `data_dashboard`, `social_media`, and `product_showcase`.
  ```rust
  #[tokio::test]
  async fn test_mcp_video_template_data_dashboard() {
      let mut config = Config::default();
      let temp_dir = std::env::temp_dir();
      config.paths.output_dir = temp_dir.join("openmedia_test_template_dashboard");
      let _ = std::fs::create_dir_all(&config.paths.output_dir);
      let server = OpenMediaServer::new(config).await.unwrap();

      let params = Parameters(VideoFromTemplateRequest {
          template_name: "data_dashboard".to_string(),
          parameters: serde_json::json!({
              "title": "Sales Report",
              "charts": [
                  {
                      "type": "bar",
                      "title": "Q1 Performance",
                      "data": [
                          {"label": "January", "value": 150.0},
                          {"label": "February", "value": 200.0}
                      ]
                  }
              ],
              "chart_duration": 2.0,
              "width": 800,
              "height": 600,
              "fps": 10
          }),
          output_path: None,
      });

      let result = server.video_from_template(params).await;
      assert!(result.is_ok());
      let val = result.unwrap().0;
      let output: openmedia_core::VideoSpec = serde_json::from_value(val).unwrap();
      assert!(output.path.exists());
      let _ = std::fs::remove_file(output.path);
  }
  ```

- [ ] **Step 2: Run tests to verify they fail**
  Run: `cargo test -p openmedia-mcp`
  Expected: FAIL or fallback placeholder screen without chart content.

- [ ] **Step 3: Modify `video_from_template` match branches**
  Add `data_dashboard`, `social_media`, and `product_showcase` template match branches in `crates/openmedia-mcp/src/lib.rs` around line 1928:
  ```rust
            "data_dashboard" => {
                let title = req.parameters["title"].as_str().unwrap_or("Data Dashboard").to_string();
                let charts_arr = req.parameters["charts"].as_array()
                    .ok_or_else(|| "Missing parameters.charts array".to_string())?;
                let duration = req.parameters["chart_duration"].as_f64().unwrap_or(3.0);
                let width = req.parameters["width"].as_u64().unwrap_or(1920) as u32;
                let height = req.parameters["height"].as_u64().unwrap_or(1080) as u32;
                let fps = req.parameters["fps"].as_u64().unwrap_or(30) as u32;

                let mut scenes = Vec::new();
                let mut transitions = Vec::new();

                scenes.push(openmedia_video::Scene {
                    id: "scene_0".to_string(),
                    start: 0.0,
                    end: 2.0,
                    elements: vec![openmedia_video::SceneElement::Text {
                        content: title.clone(),
                        style: openmedia_video::TextStyle {
                            font_family: "sans-serif".to_string(),
                            font_size: 64.0,
                            font_weight: 700,
                            color: "#ffffff".to_string(),
                            text_align: "center".to_string(),
                            line_height: None,
                            letter_spacing: None,
                        },
                        position: openmedia_video::Position {
                            x: openmedia_video::DimensionValue::Pixels((width / 2) as f64),
                            y: openmedia_video::DimensionValue::Pixels((height / 2) as f64),
                        },
                        anchor: openmedia_video::Anchor::Center,
                        timeline: None,
                    }],
                });

                for (i, chart_val) in charts_arr.iter().enumerate() {
                    let chart_type = chart_val["type"].as_str().unwrap_or("bar").to_string();
                    let chart_title = chart_val["title"].as_str().unwrap_or("Statistics").to_string();
                    let chart_data = chart_val["data"].clone();

                    let scene_id = format!("scene_{}", i + 1);
                    let start = 2.0 + i as f64 * duration;
                    let end = start + duration;

                    scenes.push(openmedia_video::Scene {
                        id: scene_id.clone(),
                        start,
                        end,
                        elements: vec![
                            openmedia_video::SceneElement::Text {
                                content: chart_title,
                                style: openmedia_video::TextStyle {
                                    font_family: "sans-serif".to_string(),
                                    font_size: 40.0,
                                    font_weight: 600,
                                    color: "#ffffff".to_string(),
                                    text_align: "center".to_string(),
                                    line_height: None,
                                    letter_spacing: None,
                                },
                                position: openmedia_video::Position {
                                    x: openmedia_video::DimensionValue::Pixels((width / 2) as f64),
                                    y: openmedia_video::DimensionValue::Pixels(80.0),
                                },
                                anchor: openmedia_video::Anchor::Center,
                                timeline: None,
                            },
                            openmedia_video::SceneElement::Chart {
                                chart_type,
                                data: chart_data,
                                position: openmedia_video::Position {
                                    x: openmedia_video::DimensionValue::Pixels((width / 2) as f64),
                                    y: openmedia_video::DimensionValue::Pixels((height / 2 + 30) as f64),
                                },
                                size: openmedia_video::Size {
                                    width: openmedia_video::DimensionValue::Pixels((width - 400) as f64),
                                    height: openmedia_video::DimensionValue::Pixels((height - 300) as f64),
                                },
                                theme: "dark".to_string(),
                                timeline: None,
                            },
                        ],
                    });

                    let from = format!("scene_{}", i);
                    let to = scene_id;
                    transitions.push(openmedia_video::SceneTransition {
                        from,
                        to,
                        transition_type: openmedia_video::TransitionType::Crossfade,
                        duration: 0.5,
                        easing: None,
                    });
                }

                let total_duration = 2.0 + charts_arr.len() as f64 * duration;

                openmedia_video::VideoScene {
                    width,
                    height,
                    fps,
                    duration: total_duration,
                    background: "#0f172a".to_string(),
                    scenes,
                    transitions,
                    audio: None,
                }
            }
            "social_media" => {
                let title = req.parameters["title"].as_str().unwrap_or("Top Facts").to_string();
                let content_arr = req.parameters["content"].as_array()
                    .ok_or_else(|| "Missing parameters.content array".to_string())?;
                let duration = req.parameters["scene_duration"].as_f64().unwrap_or(3.0);
                let bg_color = req.parameters["background_color"].as_str().unwrap_or("#1e1b4b").to_string();
                let width = 1080;
                let height = 1920;
                let fps = req.parameters["fps"].as_u64().unwrap_or(30) as u32;

                let mut scenes = Vec::new();
                let mut transitions = Vec::new();

                scenes.push(openmedia_video::Scene {
                    id: "scene_0".to_string(),
                    start: 0.0,
                    end: 3.0,
                    elements: vec![openmedia_video::SceneElement::Text {
                        content: title.clone(),
                        style: openmedia_video::TextStyle {
                            font_family: "sans-serif".to_string(),
                            font_size: 72.0,
                            font_weight: 800,
                            color: "#fbbf24".to_string(),
                            text_align: "center".to_string(),
                            line_height: None,
                            letter_spacing: None,
                        },
                        position: openmedia_video::Position {
                            x: openmedia_video::DimensionValue::Pixels((width / 2) as f64),
                            y: openmedia_video::DimensionValue::Pixels((height / 2) as f64),
                        },
                        anchor: openmedia_video::Anchor::Center,
                        timeline: Some(openmedia_video::ElementTimeline {
                            keyframes: vec![
                                openmedia_video::Keyframe {
                                    time: 0.0,
                                    opacity: Some(0.0),
                                    x: None, y: None,
                                    scale: Some(0.5), scale_x: None, scale_y: None,
                                    rotation: None, easing: Some("ease_out".to_string()),
                                },
                                openmedia_video::Keyframe {
                                    time: 1.0,
                                    opacity: Some(1.0),
                                    x: None, y: None,
                                    scale: Some(1.0), scale_x: None, scale_y: None,
                                    rotation: None, easing: None,
                                }
                            ]
                        }),
                    }],
                });

                for (i, content_val) in content_arr.iter().enumerate() {
                    let point_text = content_val.as_str().unwrap_or("").to_string();
                    let scene_id = format!("scene_{}", i + 1);
                    let start = 3.0 + i as f64 * duration;
                    let end = start + duration;

                    scenes.push(openmedia_video::Scene {
                        id: scene_id.clone(),
                        start,
                        end,
                        elements: vec![
                            openmedia_video::SceneElement::Text {
                                content: title.clone(),
                                style: openmedia_video::TextStyle {
                                    font_family: "sans-serif".to_string(),
                                    font_size: 48.0,
                                    font_weight: 700,
                                    color: "#fbbf24".to_string(),
                                    text_align: "center".to_string(),
                                    line_height: None,
                                    letter_spacing: None,
                                },
                                position: openmedia_video::Position {
                                    x: openmedia_video::DimensionValue::Pixels((width / 2) as f64),
                                    y: openmedia_video::DimensionValue::Pixels(200.0),
                                },
                                anchor: openmedia_video::Anchor::Center,
                                timeline: None,
                            },
                            openmedia_video::SceneElement::Text {
                                content: point_text,
                                style: openmedia_video::TextStyle {
                                    font_family: "sans-serif".to_string(),
                                    font_size: 56.0,
                                    font_weight: 600,
                                    color: "#ffffff".to_string(),
                                    text_align: "center".to_string(),
                                    line_height: None,
                                    letter_spacing: None,
                                },
                                position: openmedia_video::Position {
                                    x: openmedia_video::DimensionValue::Pixels((width / 2) as f64),
                                    y: openmedia_video::DimensionValue::Pixels((height / 2) as f64),
                                },
                                anchor: openmedia_video::Anchor::Center,
                                timeline: Some(openmedia_video::ElementTimeline {
                                    keyframes: vec![
                                        openmedia_video::Keyframe {
                                            time: 0.0,
                                            opacity: Some(0.0),
                                            x: None, y: Some("100".to_string()),
                                            scale: None, scale_x: None, scale_y: None,
                                            rotation: None, easing: Some("ease_out".to_string()),
                                        },
                                        openmedia_video::Keyframe {
                                            time: 0.8,
                                            opacity: Some(1.0),
                                            x: None, y: Some("0".to_string()),
                                            scale: None, scale_x: None, scale_y: None,
                                            rotation: None, easing: None,
                                        }
                                    ]
                                }),
                            },
                        ],
                    });

                    let from = format!("scene_{}", i);
                    let to = scene_id;
                    transitions.push(openmedia_video::SceneTransition {
                        from,
                        to,
                        transition_type: openmedia_video::TransitionType::SlideUp,
                        duration: 0.5,
                        easing: None,
                    });
                }

                let total_duration = 3.0 + content_arr.len() as f64 * duration;

                openmedia_video::VideoScene {
                    width,
                    height,
                    fps,
                    duration: total_duration,
                    background: bg_color,
                    scenes,
                    transitions,
                    audio: None,
                }
            }
            "product_showcase" => {
                let name = req.parameters["product_name"].as_str().unwrap_or("Product").to_string();
                let image_src = req.parameters["product_image"].as_str()
                    .ok_or_else(|| "Missing parameters.product_image path".to_string())?.to_string();
                let features_arr = req.parameters["features"].as_array()
                    .ok_or_else(|| "Missing parameters.features array".to_string())?;
                let duration = req.parameters["scene_duration"].as_f64().unwrap_or(3.0);
                let bg_color = req.parameters["background_color"].as_str().unwrap_or("#111827").to_string();
                let width = req.parameters["width"].as_u64().unwrap_or(1920) as u32;
                let height = req.parameters["height"].as_u64().unwrap_or(1080) as u32;
                let fps = req.parameters["fps"].as_u64().unwrap_or(30) as u32;

                let mut scenes = Vec::new();
                let mut transitions = Vec::new();

                scenes.push(openmedia_video::Scene {
                    id: "scene_0".to_string(),
                    start: 0.0,
                    end: 3.0,
                    elements: vec![
                        openmedia_video::SceneElement::Text {
                            content: name.clone(),
                            style: openmedia_video::TextStyle {
                                font_family: "sans-serif".to_string(),
                                font_size: 64.0,
                                font_weight: 700,
                                color: "#3b82f6".to_string(),
                                text_align: "center".to_string(),
                                line_height: None,
                                letter_spacing: None,
                            },
                            position: openmedia_video::Position {
                                x: openmedia_video::DimensionValue::Pixels((width / 2) as f64),
                                y: openmedia_video::DimensionValue::Pixels(150.0),
                            },
                            anchor: openmedia_video::Anchor::Center,
                            timeline: None,
                        },
                        openmedia_video::SceneElement::Image {
                            src: image_src.clone(),
                            position: openmedia_video::Position {
                                x: openmedia_video::DimensionValue::Pixels((width / 2) as f64),
                                y: openmedia_video::DimensionValue::Pixels((height / 2 + 100) as f64),
                            },
                            size: openmedia_video::Size {
                                width: openmedia_video::DimensionValue::Pixels(600.0),
                                height: openmedia_video::DimensionValue::Pixels(450.0),
                            },
                            fit: openmedia_video::ObjectFit::Contain,
                            timeline: Some(openmedia_video::ElementTimeline {
                                keyframes: vec![
                                    openmedia_video::Keyframe {
                                        time: 0.0,
                                        opacity: Some(0.0),
                                        x: None, y: None,
                                        scale: Some(0.8), scale_x: None, scale_y: None,
                                        rotation: None, easing: Some("ease_out".to_string()),
                                    },
                                    openmedia_video::Keyframe {
                                        time: 1.0,
                                        opacity: Some(1.0),
                                        x: None, y: None,
                                        scale: Some(1.0), scale_x: None, scale_y: None,
                                        rotation: None, easing: None,
                                    }
                                ]
                            }),
                        }
                    ],
                });

                for (i, feature_val) in features_arr.iter().enumerate() {
                    let feature_text = feature_val.as_str().unwrap_or("").to_string();
                    let scene_id = format!("scene_{}", i + 1);
                    let start = 3.0 + i as f64 * duration;
                    let end = start + duration;

                    scenes.push(openmedia_video::Scene {
                        id: scene_id.clone(),
                        start,
                        end,
                        elements: vec![
                            openmedia_video::SceneElement::Text {
                                content: name.clone(),
                                style: openmedia_video::TextStyle {
                                    font_family: "sans-serif".to_string(),
                                    font_size: 40.0,
                                    font_weight: 700,
                                    color: "#3b82f6".to_string(),
                                    text_align: "left".to_string(),
                                    line_height: None,
                                    letter_spacing: None,
                                },
                                position: openmedia_video::Position {
                                    x: openmedia_video::DimensionValue::Pixels(150.0),
                                    y: openmedia_video::DimensionValue::Pixels(100.0),
                                },
                                anchor: openmedia_video::Anchor::TopLeft,
                                timeline: None,
                            },
                            openmedia_video::SceneElement::Image {
                                src: image_src.clone(),
                                position: openmedia_video::Position {
                                    x: openmedia_video::DimensionValue::Pixels(450.0),
                                    y: openmedia_video::DimensionValue::Pixels((height / 2) as f64),
                                },
                                size: openmedia_video::Size {
                                    width: openmedia_video::DimensionValue::Pixels(600.0),
                                    height: openmedia_video::DimensionValue::Pixels(450.0),
                                },
                                fit: openmedia_video::ObjectFit::Contain,
                                timeline: None,
                            },
                            openmedia_video::SceneElement::Text {
                                content: feature_text,
                                style: openmedia_video::TextStyle {
                                    font_family: "sans-serif".to_string(),
                                    font_size: 52.0,
                                    font_weight: 600,
                                    color: "#ffffff".to_string(),
                                    text_align: "left".to_string(),
                                    line_height: None,
                                    letter_spacing: None,
                                },
                                position: openmedia_video::Position {
                                    x: openmedia_video::DimensionValue::Pixels((width / 2 + 100) as f64),
                                    y: openmedia_video::DimensionValue::Pixels((height / 2) as f64),
                                },
                                anchor: openmedia_video::Anchor::CenterLeft,
                                timeline: Some(openmedia_video::ElementTimeline {
                                    keyframes: vec![
                                        openmedia_video::Keyframe {
                                            time: 0.0,
                                            opacity: Some(0.0),
                                            x: Some("-50".to_string()), y: None,
                                            scale: None, scale_x: None, scale_y: None,
                                            rotation: None, easing: Some("ease_out".to_string()),
                                        },
                                        openmedia_video::Keyframe {
                                            time: 0.8,
                                            opacity: Some(1.0),
                                            x: Some("0".to_string()), y: None,
                                            scale: None, scale_x: None, scale_y: None,
                                            rotation: None, easing: None,
                                        }
                                    ]
                                }),
                            },
                        ],
                    });

                    let from = format!("scene_{}", i);
                    let to = scene_id;
                    transitions.push(openmedia_video::SceneTransition {
                        from,
                        to,
                        transition_type: openmedia_video::TransitionType::Crossfade,
                        duration: 0.5,
                        easing: None,
                    });
                }

                let total_duration = 3.0 + features_arr.len() as f64 * duration;

                openmedia_video::VideoScene {
                    width,
                    height,
                    fps,
                    duration: total_duration,
                    background: bg_color,
                    scenes,
                    transitions,
                    audio: None,
                }
            }
  ```

- [ ] **Step 4: Run tests to verify they pass**
  Run: `cargo test --workspace`
  Expected: PASS

- [ ] **Step 5: Commit**
  ```bash
  git add crates/openmedia-mcp/src/lib.rs
  git commit -m "feat(mcp): support data_dashboard, social_media, and product_showcase templates in video_from_template"
  ```
