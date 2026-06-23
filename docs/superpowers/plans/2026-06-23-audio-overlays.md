# Audio Overlays Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement the background music and multi-track audio overlays mapping inside template-based video generation.

**Architecture:** Create a `parse_audio_config` helper inside `crates/openmedia-mcp/src/lib.rs` that reads parameters from the JSON request and populates the `openmedia_video::AudioConfig` structure, then pass this config to all matched video scene rendering arms.

**Tech Stack:** Rust 1.82, openmedia-video, openmedia-mcp.

## Global Constraints
- Target Rust toolchain compatibility: 1.82
- All protocols are stdio JSON-RPC transport (telemetry to stderr only)

---

### Task 1: Implement Audio Config Parser and Update Template Matches
**Files:**
- Modify: `crates/openmedia-mcp/src/lib.rs`
- Test: `crates/openmedia-mcp/src/lib.rs` (under tests module)

**Interfaces:**
- Consumes: JSON template request parameters (`req.parameters`).
- Produces: `parse_audio_config(parameters: &serde_json::Value) -> Option<openmedia_video::AudioConfig>` helper function.

- [ ] **Step 1: Write the failing test**
  Add `test_mcp_video_template_with_audio` to verify that passing audio parameters generates a video successfully.
  ```rust
  #[tokio::test]
  async fn test_mcp_video_template_with_audio() {
      let mut config = Config::default();
      let temp_dir = std::env::temp_dir();
      config.paths.output_dir = temp_dir.join("openmedia_test_template_audio");
      let _ = std::fs::create_dir_all(&config.paths.output_dir);
      let server = OpenMediaServer::new(config).await.unwrap();

      let params = Parameters(VideoFromTemplateRequest {
          template_name: "slideshow".to_string(),
          parameters: serde_json::json!({
              "images": [],
              "duration_per_image": 1.0,
              "background_music": "assets/test_audio.mp3",
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

- [ ] **Step 2: Run tests to verify the test fails or runs placeholder**
  Run: `cargo test --package openmedia-mcp --lib -- tests::test_mcp_video_template_with_audio`
  Expected: Passes but does not mix audio in the output spec, or fails if we assert target `audio` details.

- [ ] **Step 3: Implement parse_audio_config helper and update match branches**
  Add the `parse_audio_config` helper function in `crates/openmedia-mcp/src/lib.rs` around line 2690 (right above the `tests` module):
  ```rust
  fn parse_audio_config(parameters: &serde_json::Value) -> Option<openmedia_video::AudioConfig> {
      if let Some(tracks_arr) = parameters.get("audio_tracks").and_then(|v| v.as_array()) {
          let mut tracks = Vec::new();
          for track_val in tracks_arr {
              if let Some(src) = track_val.get("src").and_then(|v| v.as_str()) {
                  let start = track_val.get("start").and_then(|v| v.as_f64()).unwrap_or(0.0);
                  let volume = track_val.get("volume").and_then(|v| v.as_f64()).map(|v| v as f32).unwrap_or(1.0);
                  let fade_in = track_val.get("fade_in").and_then(|v| v.as_f64());
                  let fade_out = track_val.get("fade_out").and_then(|v| v.as_f64());
                  
                  tracks.push(openmedia_video::AudioTrack {
                      src: src.to_string(),
                      start,
                      volume,
                      fade_in,
                      fade_out,
                  });
              }
          }
          if !tracks.is_empty() {
              return Some(openmedia_video::AudioConfig { tracks });
          }
      } else if let Some(bg_music) = parameters.get("background_music").and_then(|v| v.as_str()) {
          return Some(openmedia_video::AudioConfig {
              tracks: vec![openmedia_video::AudioTrack {
                  src: bg_music.to_string(),
                  start: 0.0,
                  volume: 0.5,
                  fade_in: None,
                  fade_out: None,
              }],
          });
      }
      None
  }
  ```
  Now, replace every occurrence of `audio: None` with `audio: parse_audio_config(&req.parameters)` in the `video_from_template` match statement. This includes `slideshow`, `text_explainer`, `data_dashboard`, `social_media`, `product_showcase`, and the catch-all `_` arms.

- [ ] **Step 4: Run tests to verify they pass**
  Run: `cargo test --workspace`
  Expected: PASS

- [ ] **Step 5: Commit**
  ```bash
  git add crates/openmedia-mcp/src/lib.rs
  git commit -m "feat(mcp): support background_music and audio_tracks options in video templates"
  ```
