# Mermaid Extensions Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Extend the native Mermaid rendering engine and MCP tool in OpenMedia-RS to support custom themes (presets like dark, forest, neutral), layout styling variables, and partial JSON theme overrides.

**Architecture:** Extend `openmedia-svg::render_mermaid` to accept customizable `Theme` and `LayoutConfig` objects from `mermaid-rs-renderer`. Build helper functions to resolve theme overrides from JSON input and apply layout configurations like spacing and aspect ratios.

**Tech Stack:** Rust, mermaid-rs-renderer, serde_json.

## Global Constraints
- Target Rust toolchain compatibility: 1.82
- All protocols are stdio JSON-RPC transport (telemetry to stderr only)

---

### Task 1: Extend SVG Mermaid Engine
**Files:**
- Modify: `crates/openmedia-svg/src/lib.rs`
- Test: `crates/openmedia-svg/src/lib.rs` (under tests module)

**Interfaces:**
- Consumes: `mermaid-rs-renderer` core API.
- Produces: `pub fn render_mermaid(code: &str, theme: Option<mermaid_rs_renderer::Theme>, layout: Option<mermaid_rs_renderer::LayoutConfig>) -> std::result::Result<String, String>`

- [ ] **Step 1: Write the failing tests**
  Add unit tests testing custom theme application and layout adjustments to the `tests` module in `crates/openmedia-svg/src/lib.rs`.
  ```rust
  #[test]
  fn test_render_mermaid_with_custom_theme() {
      let code = "flowchart TD\n  A --> B";
      let mut theme = mermaid_rs_renderer::Theme::modern();
      theme.primary_color = "#ff0000".to_string(); // Bright Red
      let result = render_mermaid(code, Some(theme), None);
      assert!(result.is_ok());
      let svg = result.unwrap();
      assert!(svg.contains("#ff0000"));
  }

  #[test]
  fn test_render_mermaid_with_layout_spacing() {
      let code = "flowchart LR\n  A --> B";
      let mut layout = mermaid_rs_renderer::LayoutConfig::default();
      layout.node_spacing = 150.0;
      let result = render_mermaid(code, None, Some(layout));
      assert!(result.is_ok());
  }
  ```

- [ ] **Step 2: Run tests to verify they fail**
  Run: `cargo test -p openmedia-svg`
  Expected: FAIL (compilation errors due to signature mismatch)

- [ ] **Step 3: Modify signature and implementation**
  Update the `render_mermaid` function to accept `theme` and `layout` options, using `mermaid_rs_renderer::render_with_options` under the hood.
  ```rust
  pub fn render_mermaid(
      code: &str,
      theme: Option<mermaid_rs_renderer::Theme>,
      layout: Option<mermaid_rs_renderer::LayoutConfig>,
  ) -> std::result::Result<String, String> {
      let options = mermaid_rs_renderer::RenderOptions {
          theme: theme.unwrap_or_else(mermaid_rs_renderer::Theme::modern),
          layout: layout.unwrap_or_default(),
      };
      mermaid_rs_renderer::render_with_options(code, options)
          .map_err(|e| format!("Failed to render Mermaid diagram: {:?}", e))
  }
  ```

- [ ] **Step 4: Run tests to verify they pass**
  Run: `cargo test -p openmedia-svg`
  Expected: PASS

- [ ] **Step 5: Commit**
  ```bash
  git add crates/openmedia-svg/src/lib.rs
  git commit -m "feat(svg): support custom theme and layout configs in render_mermaid"
  ```

---

### Task 2: Implement Theme Presets and Override Logic
**Files:**
- Modify: `crates/openmedia-mcp/src/lib.rs`
- Test: `crates/openmedia-mcp/src/lib.rs` (under tests module)

**Interfaces:**
- Consumes: `GenerateMermaidRequest` parameters and `mermaid_rs_renderer::Theme` struct.
- Produces: `override_theme_fields(theme: &mut Theme, overrides: &serde_json::Value)` helper function and theme presets parser.

- [ ] **Step 1: Write the failing tests**
  Add unit tests verifying theme presets mapping and field override logic in `crates/openmedia-mcp/src/lib.rs`.
  ```rust
  #[test]
  fn test_theme_preset_override() {
      let mut theme = mermaid_rs_renderer::Theme::modern();
      let overrides = serde_json::json!({
          "primary_color": "#00ff00",
          "font_size": 20.0
      });
      override_theme_fields(&mut theme, &overrides);
      assert_eq!(theme.primary_color, "#00ff00");
      assert_eq!(theme.font_size, 20.0);
  }
  ```

- [ ] **Step 2: Run tests to verify they fail**
  Run: `cargo test -p openmedia-mcp`
  Expected: FAIL (compilation errors due to missing helper function)

- [ ] **Step 3: Implement helper function and preset resolution**
  Add the `override_theme_fields` helper function and preset builder inside `crates/openmedia-mcp/src/lib.rs`.
  ```rust
  fn override_theme_fields(theme: &mut mermaid_rs_renderer::Theme, overrides: &serde_json::Value) {
      if let Some(map) = overrides.as_object() {
          for (key, val) in map {
              if let Some(val_str) = val.as_str() {
                  match key.as_str() {
                      "font_family" => theme.font_family = val_str.to_string(),
                      "primary_color" => theme.primary_color = val_str.to_string(),
                      "primary_text_color" => theme.primary_text_color = val_str.to_string(),
                      "primary_border_color" => theme.primary_border_color = val_str.to_string(),
                      "line_color" => theme.line_color = val_str.to_string(),
                      "secondary_color" => theme.secondary_color = val_str.to_string(),
                      "tertiary_color" => theme.tertiary_color = val_str.to_string(),
                      "edge_label_background" => theme.edge_label_background = val_str.to_string(),
                      "cluster_background" => theme.cluster_background = val_str.to_string(),
                      "cluster_border" => theme.cluster_border = val_str.to_string(),
                      "background" => theme.background = val_str.to_string(),
                      "sequence_actor_fill" => theme.sequence_actor_fill = val_str.to_string(),
                      "sequence_actor_border" => theme.sequence_actor_border = val_str.to_string(),
                      "sequence_actor_line" => theme.sequence_actor_line = val_str.to_string(),
                      "sequence_note_fill" => theme.sequence_note_fill = val_str.to_string(),
                      "sequence_note_border" => theme.sequence_note_border = val_str.to_string(),
                      "sequence_activation_fill" => theme.sequence_activation_fill = val_str.to_string(),
                      "sequence_activation_border" => theme.sequence_activation_border = val_str.to_string(),
                      "text_color" => theme.text_color = val_str.to_string(),
                      _ => {}
                  }
              } else if let Some(val_f64) = val.as_f64() {
                  if key == "font_size" {
                      theme.font_size = val_f64 as f32;
                  }
              }
          }
      }
  }

  fn resolve_theme_preset(preset: &str) -> mermaid_rs_renderer::Theme {
      match preset.to_lowercase().as_str() {
          "default" | "classic" => mermaid_rs_renderer::Theme::mermaid_default(),
          "dark" => {
              let mut theme = mermaid_rs_renderer::Theme::modern();
              theme.background = "#0f172a".to_string();
              theme.primary_color = "#1e293b".to_string();
              theme.primary_text_color = "#f8fafc".to_string();
              theme.primary_border_color = "#475569".to_string();
              theme.line_color = "#94a3b8".to_string();
              theme.secondary_color = "#334155".to_string();
              theme.tertiary_color = "#0f172a".to_string();
              theme.text_color = "#f8fafc".to_string();
              theme.edge_label_background = "#1e293b".to_string();
              theme.cluster_background = "#1e293b".to_string();
              theme.cluster_border = "#334155".to_string();
              theme
          }
          "forest" => {
              let mut theme = mermaid_rs_renderer::Theme::modern();
              theme.primary_color = "#f0fdf4".to_string();
              theme.primary_text_color = "#166534".to_string();
              theme.primary_border_color = "#86efac".to_string();
              theme.line_color = "#15803d".to_string();
              theme.secondary_color = "#dcfce7".to_string();
              theme.tertiary_color = "#ffffff".to_string();
              theme.text_color = "#166534".to_string();
              theme.edge_label_background = "#ffffff".to_string();
              theme.cluster_background = "#f0fdf4".to_string();
              theme.cluster_border = "#bbf7d0".to_string();
              theme
          }
          "neutral" => {
              let mut theme = mermaid_rs_renderer::Theme::modern();
              theme.primary_color = "#f9fafb".to_string();
              theme.primary_text_color = "#111827".to_string();
              theme.primary_border_color = "#e5e7eb".to_string();
              theme.line_color = "#4b5563".to_string();
              theme.secondary_color = "#f3f4f6".to_string();
              theme.tertiary_color = "#ffffff".to_string();
              theme.text_color = "#111827".to_string();
              theme.edge_label_background = "#ffffff".to_string();
              theme.cluster_background = "#f9fafb".to_string();
              theme.cluster_border = "#d1d5db".to_string();
              theme
          }
          _ => mermaid_rs_renderer::Theme::modern(),
      }
  }
  ```

- [ ] **Step 4: Run tests to verify they pass**
  Run: `cargo test -p openmedia-mcp`
  Expected: PASS

- [ ] **Step 5: Commit**
  ```bash
  git add crates/openmedia-mcp/src/lib.rs
  git commit -m "feat(mcp): implement theme presets parser and JSON override helper"
  ```

---

### Task 3: Integrate Custom Styles and Presets into MCP Server
**Files:**
- Modify: `crates/openmedia-mcp/src/lib.rs`
- Test: `crates/openmedia-mcp/src/lib.rs` (under tests module)

**Interfaces:**
- Consumes: Extended parameters in `GenerateMermaidRequest`.
- Produces: Extended `diagram_generate_mermaid` tool execution logic.

- [ ] **Step 1: Write the failing integration test**
  Add an integration test `test_mcp_diagram_generate_mermaid_styling` verifying presets and overrides work end-to-end.
  ```rust
  #[tokio::test]
  async fn test_mcp_diagram_generate_mermaid_styling() {
      let mut config = Config::default();
      let temp_dir = std::env::temp_dir();
      config.paths.output_dir = temp_dir.join("openmedia_test_mermaid_styling");
      let _ = std::fs::create_dir_all(&config.paths.output_dir);
      let server = OpenMediaServer::new(config).await.unwrap();

      let code = "flowchart LR\n  A --> B".to_string();
      let params = Parameters(GenerateMermaidRequest {
          code,
          theme: Some("forest".to_string()),
          custom_theme: Some(serde_json::json!({
              "primary_color": "#aabbcc"
          })),
          width: None,
          height: None,
          background_color: None,
          output_format: Some("svg".to_string()),
          node_spacing: Some(120.0),
          rank_spacing: Some(140.0),
          preferred_aspect_ratio: Some(1.77),
      });

      let result = server.diagram_generate_mermaid(params).await;
      assert!(result.is_ok());
      let val = result.unwrap().0;
      let output: openmedia_core::ImageOutput = serde_json::from_value(val).unwrap();
      let content = std::fs::read_to_string(&output.path).unwrap();
      assert!(content.contains("#aabbcc")); // custom theme override was applied
      let _ = std::fs::remove_file(output.path);
  }
  ```

- [ ] **Step 2: Run tests to verify they fail**
  Run: `cargo test -p openmedia-mcp`
  Expected: FAIL (compilation errors on request struct parameters)

- [ ] **Step 3: Update GenerateMermaidRequest and diagram_generate_mermaid**
  Update the request structure to add layout parameters and custom theme fields, and adjust the method logic to parse and inject them into `render_mermaid`.
  ```rust
  // Modify GenerateMermaidRequest
  pub struct GenerateMermaidRequest {
      pub code: String,
      pub theme: Option<String>,
      pub custom_theme: Option<serde_json::Value>,
      pub width: Option<u32>,
      pub height: Option<u32>,
      pub background_color: Option<String>,
      pub output_format: Option<String>,
      pub node_spacing: Option<f32>,
      pub rank_spacing: Option<f32>,
      pub preferred_aspect_ratio: Option<f32>,
  }

  // Update diagram_generate_mermaid method body:
  // Build layout config
  let mut layout_config = mermaid_rs_renderer::LayoutConfig::default();
  if let Some(spacing) = req.node_spacing {
      layout_config.node_spacing = spacing;
  }
  if let Some(spacing) = req.rank_spacing {
      layout_config.rank_spacing = spacing;
  }
  if let Some(ratio) = req.preferred_aspect_ratio {
      layout_config.preferred_aspect_ratio = Some(ratio);
  }

  // Resolve theme
  let mut final_theme = if let Some(ref preset) = req.theme {
      resolve_theme_preset(preset)
  } else {
      mermaid_rs_renderer::Theme::modern()
  };

  if let Some(ref overrides) = req.custom_theme {
      override_theme_fields(&mut final_theme, overrides);
  }

  // Call render_mermaid with resolved options
  let svg_content = openmedia_svg::render_mermaid(&req.code, Some(final_theme), Some(layout_config))
      .map_err(|e| format!("Failed to render Mermaid diagram: {}", e))?;
  ```

- [ ] **Step 4: Run tests to verify they pass**
  Run: `cargo test --workspace`
  Expected: PASS

- [ ] **Step 5: Commit**
  ```bash
  git add crates/openmedia-mcp/src/lib.rs
  git commit -m "feat(mcp): integrate theme custom styles and presets in diagram_generate_mermaid"
  ```
