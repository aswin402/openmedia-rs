# Custom Transitions Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement custom transition duration and easing overrides for video templates.

**Architecture:** Create an `apply_transition_easing` helper function in `openmedia-video`, apply it to transition rendering paths, and parse optional transition parameter overrides (`transition_duration`, `transition_easing`, `transition_type`) inside `openmedia-mcp` templates.

**Tech Stack:** Rust 1.82, openmedia-video, openmedia-mcp.

## Global Constraints
- Target Rust toolchain compatibility: 1.82
- All protocols are stdio JSON-RPC transport (telemetry to stderr only)

---

### Task 1: Add apply_transition_easing Helper and Update Rendering Progress Paths

**Files:**
- Modify: `crates/openmedia-video/src/lib.rs`

**Interfaces:**
- Consumes: Raw linear progress value (`f64`) and optional easing configuration (`&Option<String>`).
- Produces: `pub fn apply_transition_easing(progress: f64, easing: &Option<String>) -> f64` helper function.

- [ ] **Step 1: Implement the easing helper and update tests in openmedia-video**
  Add the `apply_transition_easing` helper function in `crates/openmedia-video/src/lib.rs` (above the `blend_frames` definition at line 1164):
  ```rust
  pub fn apply_transition_easing(progress: f64, easing: &Option<String>) -> f64 {
      if let Some(ref eas) = easing {
          match eas.to_lowercase().as_str() {
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
          }
      } else {
          progress
      }
  }
  ```

- [ ] **Step 2: Update CPU transition rasterization loop**
  Update the linear progress calculation inside the `active_trans` block in `crates/openmedia-video/src/lib.rs` (around line 323):
  ```rust
  // Target:
  let progress = (time - trans_start) / trans.duration;
  
  // Replacement:
  let progress = apply_transition_easing((time - trans_start) / trans.duration, &trans.easing);
  ```

- [ ] **Step 3: Update HTML/Chromium transition rendering loop**
  Update the transition progress calculation in the HTML output generator in `crates/openmedia-video/src/lib.rs` (around line 842):
  ```rust
  // Target:
  transition_progress = (time - trans_start) / trans.duration;
  
  // Replacement:
  transition_progress = apply_transition_easing((time - trans_start) / trans.duration, &trans.easing);
  ```

- [ ] **Step 4: Add unit tests for transition easing**
  Add unit tests inside a new `tests` module or within `crates/openmedia-video/src/lib.rs` to verify correct output values for transition easing:
  ```rust
  #[cfg(test)]
  mod tests {
      use super::*;
  
      #[test]
      fn test_transition_easing_calculations() {
          let easing_none = None;
          let easing_in = Some("ease_in".to_string());
          let easing_out = Some("ease_out".to_string());
          let easing_in_out = Some("ease_in_out".to_string());
  
          assert_eq!(apply_transition_easing(0.5, &easing_none), 0.5);
          assert_eq!(apply_transition_easing(0.5, &easing_in), 0.25);
          assert_eq!(apply_transition_easing(0.5, &easing_out), 0.75);
          assert_eq!(apply_transition_easing(0.25, &easing_in_out), 0.125);
          assert_eq!(apply_transition_easing(0.75, &easing_in_out), 0.875);
      }
  }
  ```

- [ ] **Step 5: Run tests to verify implementation**
  Run: `cargo test --package openmedia-video`
  Expected: PASS

- [ ] **Step 6: Commit**
  ```bash
  git add crates/openmedia-video/src/lib.rs
  git commit -m "feat(video): add support for transition easing in rendering paths"
  ```

---

### Task 2: Implement Template Transition Overrides in openmedia-mcp

**Files:**
- Modify: `crates/openmedia-mcp/src/lib.rs`

**Interfaces:**
- Consumes: `transition_duration`, `transition_easing`, and `transition_type` from `req.parameters`.
- Produces: Configured `SceneTransition` structures generated during templates composition matching.

- [ ] **Step 1: Write integration test verifying transition overrides**
  Add a test inside `crates/openmedia-mcp/src/lib.rs` (under `mod tests`) to check custom template transition duration and easing parsing:
  ```rust
  #[tokio::test]
  async fn test_mcp_video_template_with_transition_overrides() {
      let mut config = Config::default();
      let temp_dir = std::env::temp_dir();
      config.paths.output_dir = temp_dir.join("openmedia_test_template_transitions");
      let _ = std::fs::create_dir_all(&config.paths.output_dir);
      let server = OpenMediaServer::new(config).await.unwrap();

      let params = Parameters(VideoFromTemplateRequest {
          template_name: "product_showcase".to_string(),
          parameters: serde_json::json!({
              "features": ["Feature A"],
              "background_color": "#121212",
              "transition_duration": 1.5,
              "transition_easing": "ease_in_out",
              "transition_type": "slide_left",
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

- [ ] **Step 2: Update templates to support transition parameter overrides**
  Parse transition overrides in `crates/openmedia-mcp/src/lib.rs`:
  
  For **`data_dashboard`** (around lines 2095-2103):
  ```rust
  // Target:
  transitions.push(openmedia_video::SceneTransition {
      from,
      to,
      transition_type: openmedia_video::TransitionType::SlideLeft,
      duration: 0.5,
      easing: None,
  });

  // Replacement:
  let custom_trans_type = req.parameters.get("transition_type")
      .and_then(|v| v.as_str())
      .map(|s| match s.to_lowercase().as_str() {
          "crossfade" => openmedia_video::TransitionType::Crossfade,
          "slide_left" | "slideleft" => openmedia_video::TransitionType::SlideLeft,
          "slide_right" | "slideright" => openmedia_video::TransitionType::SlideRight,
          "slide_up" | "slideup" => openmedia_video::TransitionType::SlideUp,
          "slide_down" | "slidedown" => openmedia_video::TransitionType::SlideDown,
          _ => openmedia_video::TransitionType::SlideLeft,
      })
      .unwrap_or(openmedia_video::TransitionType::SlideLeft);

  let custom_duration = req.parameters.get("transition_duration")
      .and_then(|v| v.as_f64())
      .unwrap_or(0.5);

  let custom_easing = req.parameters.get("transition_easing")
      .and_then(|v| v.as_str())
      .map(|s| s.to_string());

  transitions.push(openmedia_video::SceneTransition {
      from,
      to,
      transition_type: custom_trans_type,
      duration: custom_duration,
      easing: custom_easing,
  });
  ```

  For **`social_media`** (around lines 2240-2248):
  Apply the exact same helper parsing logic, defaulting `transition_type` to `SlideUp`.

  For **`product_showcase`** (around lines 2415-2423):
  Apply the exact same helper parsing logic, defaulting `transition_type` to `Crossfade`.

  For **`slideshow`** (around line 1917):
  If the `transition_type` parameter is present in `slideshow` parameters, add a loop to build and push `SceneTransition`s between adjacent image slides.
  ```rust
  let mut transitions = Vec::new();
  if let Some(trans_type_str) = req.parameters.get("transition_type").and_then(|v| v.as_str()) {
      let custom_trans_type = match trans_type_str.to_lowercase().as_str() {
          "crossfade" => openmedia_video::TransitionType::Crossfade,
          "slide_left" | "slideleft" => openmedia_video::TransitionType::SlideLeft,
          "slide_right" | "slideright" => openmedia_video::TransitionType::SlideRight,
          "slide_up" | "slideup" => openmedia_video::TransitionType::SlideUp,
          "slide_down" | "slidedown" => openmedia_video::TransitionType::SlideDown,
          _ => openmedia_video::TransitionType::Crossfade,
      };

      let custom_duration = req.parameters.get("transition_duration")
          .and_then(|v| v.as_f64())
          .unwrap_or(0.5);

      let custom_easing = req.parameters.get("transition_easing")
          .and_then(|v| v.as_str())
          .map(|s| s.to_string());

      for i in 0..(images.len() - 1) {
          transitions.push(openmedia_video::SceneTransition {
              from: format!("scene_{}", i),
              to: format!("scene_{}", i + 1),
              transition_type: custom_trans_type.clone(),
              duration: custom_duration,
              easing: custom_easing.clone(),
          });
      }
  }
  ```

- [ ] **Step 3: Run integration tests to verify transition overrides**
  Run: `cargo test --package openmedia-mcp --lib -- tests::test_mcp_video_template_with_transition_overrides`
  Expected: PASS

- [ ] **Step 4: Run workspace tests**
  Run: `cargo test`
  Expected: PASS

- [ ] **Step 5: Commit**
  ```bash
  git add crates/openmedia-mcp/src/lib.rs
  git commit -m "feat(mcp): support transition parameter overrides in video templates"
  ```
