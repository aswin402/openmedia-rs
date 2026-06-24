# Custom Font Loading Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement dynamic custom font loading (local paths and HTTP URLs) and integration in SVG and HTML/CSS rendering pipelines.

**Architecture:** Update `VideoScene` struct to accept `custom_fonts`, add `reqwest` and `base64` dependencies to `openmedia-video`, resolve custom fonts (caching remote ones locally), populate `fontdb` for resvg, and inject base64 `@font-face` styles in Chromium HTML/CSS.

**Tech Stack:** Rust 1.82, reqwest, base64, resvg, openmedia-video, openmedia-mcp.

## Global Constraints
- Target Rust toolchain compatibility: 1.82
- All protocols are stdio JSON-RPC transport (telemetry to stderr only)

---

### Task 1: Update Cargo.toml and Custom Font Structures in openmedia-video

**Files:**
- Modify: `crates/openmedia-video/Cargo.toml`
- Modify: `crates/openmedia-video/src/lib.rs`

**Interfaces:**
- Consumes: None.
- Produces: `CustomFontSpec` struct, and update `VideoScene` to include `pub custom_fonts: Option<Vec<CustomFontSpec>>`.

- [ ] **Step 1: Add reqwest and base64 to openmedia-video Cargo.toml**
  Open `crates/openmedia-video/Cargo.toml` and add dependencies:
  ```toml
  reqwest = { version = "0.12", features = ["json"] }
  base64 = "0.22"
  ```

- [ ] **Step 2: Add CustomFontSpec struct and update VideoScene in crates/openmedia-video/src/lib.rs**
  Open `crates/openmedia-video/src/lib.rs` and add structure definitions around line 28 (below `VideoScene`):
  ```rust
  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct CustomFontSpec {
      pub family: String,
      pub src: String,
  }
  ```
  Now, update the `VideoScene` struct definition:
  ```rust
  // Target:
  pub struct VideoScene {
      // ... existing fields ...
      pub audio: Option<AudioConfig>,
  }

  // Replacement:
  pub struct VideoScene {
      // ... existing fields ...
      pub audio: Option<AudioConfig>,
      pub custom_fonts: Option<Vec<CustomFontSpec>>,
  }
  ```

- [ ] **Step 3: Run cargo check to verify code compiles**
  Run: `cargo check -p openmedia-video`
  Expected: PASS

- [ ] **Step 4: Commit**
  ```bash
  git add crates/openmedia-video/Cargo.toml crates/openmedia-video/src/lib.rs
  git commit -m "feat(video): add CustomFontSpec and update VideoScene struct"
  ```

---

### Task 2: Implement Font Resolution and Integration in Rendering Pipelines

**Files:**
- Modify: `crates/openmedia-video/src/lib.rs`

**Interfaces:**
- Consumes: `&[CustomFontSpec]` from `VideoScene`.
- Produces: `pub async fn resolve_custom_fonts(custom_fonts: &[CustomFontSpec]) -> std::collections::HashMap<String, Vec<u8>>` helper, integration with `resvg::usvg::Options`, and `@font-face` CSS injection in HTML builder.

- [ ] **Step 1: Write helper function to download and cache custom fonts**
  Add the `resolve_custom_fonts` function in `crates/openmedia-video/src/lib.rs` (above `render_video_scene` function):
  ```rust
  pub async fn resolve_custom_fonts(
      custom_fonts: &[CustomFontSpec],
  ) -> std::collections::HashMap<String, Vec<u8>> {
      let mut resolved = std::collections::HashMap::new();
      let client = reqwest::Client::new();
      
      let cache_dir = std::env::temp_dir().join("openmedia_fonts_cache");
      let _ = std::fs::create_dir_all(&cache_dir);

      for font in custom_fonts {
          let font_bytes = if font.src.starts_with("http://") || font.src.starts_with("https://") {
              use std::collections::hash_map::DefaultHasher;
              use std::hash::{Hash, Hasher};
              let mut hasher = DefaultHasher::new();
              font.src.hash(&mut hasher);
              let hash_val = hasher.finish();
              let cached_path = cache_dir.join(format!("{}.ttf", hash_val));

              if cached_path.exists() {
                  std::fs::read(&cached_path).ok()
              } else {
                  if let Ok(resp) = client.get(&font.src).send().await {
                      if let Ok(bytes) = resp.bytes().await {
                          let bytes_vec = bytes.to_vec();
                          let _ = std::fs::write(&cached_path, &bytes_vec);
                          Some(bytes_vec)
                      } else {
                          None
                      }
                  } else {
                      None
                  }
              }
          } else {
              std::fs::read(&font.src).ok()
          };

          if let Some(bytes) = font_bytes {
              resolved.insert(font.family.clone(), bytes);
          }
      }
      resolved
  }
  ```

- [ ] **Step 2: Add fontdb setup inside frame compilation path**
  In the CPU rendering function `render_video_scene` (or where SVG tree is constructed from string using `usvg::Tree::from_str` around lines 339-340):
  ```rust
  // Target:
  let svg_str = compile_scene_to_svg(scene, time, width, height)?;
  let opt = resvg::usvg::Options::default();
  let tree = resvg::usvg::Tree::from_str(&svg_str, &opt)

  // Replacement:
  let svg_str = compile_scene_to_svg(scene, time, width, height)?;
  let mut fontdb = resvg::usvg::fontdb::Database::new();
  fontdb.load_system_fonts();
  if let Some(ref fonts) = scene.custom_fonts {
      let resolved = resolve_custom_fonts(fonts).await;
      for (_, bytes) in resolved {
          fontdb.load_font_data(bytes);
      }
  }
  let mut opt = resvg::usvg::Options::default();
  opt.fontdb = std::sync::Arc::new(fontdb);
  let tree = resvg::usvg::Tree::from_str(&svg_str, &opt)
  ```

- [ ] **Step 3: Inject Base64 font-face definitions into Chromium HTML head**
  Update the HTML layout builder in `crates/openmedia-video/src/lib.rs` (around line 850):
  Resolve custom fonts and format dynamic CSS declarations:
  ```rust
  // Target:
  let bg_color = &scene.background;
  let mut html = format!(
      r#"<!DOCTYPE html>
  <html>
  <head>
  <style>
  ...

  // Replacement:
  let mut font_css = String::new();
  if let Some(ref fonts) = scene.custom_fonts {
      let resolved = resolve_custom_fonts(fonts).await;
      for (family, bytes) in resolved {
          use base64::Engine;
          let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
          font_css.push_str(&format!(
              r#"@font-face {{
                  font-family: '{}';
                  src: url('data:font/truetype;charset=utf-8;base64,{}') format('truetype');
              }}"#,
              family, b64
          ));
      }
  }

  let bg_color = &scene.background;
  let mut html = format!(
      r#"<!DOCTYPE html>
  <html>
  <head>
  <style>
    {}
    body {{
      margin: 0;
      padding: 0;
  ...
  "#,
  font_css,
  ...
  ```

- [ ] **Step 4: Run cargo check to verify compiles**
  Run: `cargo check -p openmedia-video`
  Expected: PASS

- [ ] **Step 5: Commit**
  ```bash
  git add crates/openmedia-video/src/lib.rs
  git commit -m "feat(video): support loading and injecting custom fonts in rendering paths"
  ```

---

### Task 3: Support Custom Fonts in templates and add integration tests

**Files:**
- Modify: `crates/openmedia-mcp/src/lib.rs`

**Interfaces:**
- Consumes: `"custom_fonts"` array parameter in `video_from_template` JSON parameters.
- Produces: Mapped `custom_fonts` field in the constructed `VideoScene` template response.

- [ ] **Step 1: Write integration test verifying font loading**
  Add `test_mcp_video_template_with_custom_font` inside `crates/openmedia-mcp/src/lib.rs` under `mod tests`:
  ```rust
  #[tokio::test]
  async fn test_mcp_video_template_with_custom_font() {
      let mut config = Config::default();
      let temp_dir = std::env::temp_dir();
      config.paths.output_dir = temp_dir.join("openmedia_test_template_fonts");
      let _ = std::fs::create_dir_all(&config.paths.output_dir);
      let server = OpenMediaServer::new(config).await.unwrap();

      let params = Parameters(VideoFromTemplateRequest {
          template_name: "slideshow".to_string(),
          parameters: serde_json::json!({
              "images": ["dummy.png"],
              "duration_per_image": 1.0,
              "custom_fonts": [
                  {
                      "family": "GoogleRoboto",
                      "src": "https://github.com/google/fonts/raw/main/ofl/roboto/Roboto-Regular.ttf"
                  }
              ],
              "width": 320,
              "height": 240,
              "fps": 5
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

- [ ] **Step 2: Parse custom_fonts inside video templates match statement**
  Add a parsing helper inside `crates/openmedia-mcp/src/lib.rs` (above `tests` module):
  ```rust
  fn parse_custom_fonts(parameters: &serde_json::Value) -> Option<Vec<openmedia_video::CustomFontSpec>> {
      if let Some(fonts_arr) = parameters.get("custom_fonts").and_then(|v| v.as_array()) {
          let mut specs = Vec::new();
          for font_val in fonts_arr {
              if let (Some(family), Some(src)) = (
                  font_val.get("family").and_then(|v| v.as_str()),
                  font_val.get("src").and_then(|v| v.as_str())
              ) {
                  specs.push(openmedia_video::CustomFontSpec {
                      family: family.to_string(),
                      src: src.to_string(),
                  });
              }
          }
          if !specs.is_empty() {
              return Some(specs);
          }
      }
      None
  }
  ```
  Now, update the `VideoScene` constructors in `crates/openmedia-mcp/src/lib.rs` to include `custom_fonts: parse_custom_fonts(&req.parameters)` for all template matching branches:
  - `slideshow`
  - `text_explainer`
  - `data_dashboard`
  - `social_media`
  - `product_showcase`
  - the catch-all `_` branch

- [ ] **Step 3: Run integration test**
  Run: `cargo test --package openmedia-mcp --lib -- tests::test_mcp_video_template_with_custom_font`
  Expected: PASS

- [ ] **Step 4: Run all tests in the workspace**
  Run: `cargo test`
  Expected: PASS

- [ ] **Step 5: Commit**
  ```bash
  git add crates/openmedia-mcp/src/lib.rs
  git commit -m "feat(mcp): support custom fonts parameter in video templates"
  ```
