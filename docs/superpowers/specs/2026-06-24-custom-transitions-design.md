# Custom Transition Speeds and Easing Overrides Design Spec

**Date**: 2026-06-24  
**Status**: Draft  

---

## 1. Goal
Provide AI agents and users with the ability to customize transition speeds (duration) and easing functions in template-based videos generated via the `video_from_template` MCP tool, as well as core transition rendering support for easing functions in the video composition engine.

---

## 2. Core Transitions Engine Enhancements

### Transition Easing Application
We will implement an `apply_transition_easing` helper in `crates/openmedia-video/src/lib.rs` to compute the eased progress of a transition:

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

This helper will be applied in the two main transition rendering paths:
1. **CPU/Rasterization Path** (`crates/openmedia-video/src/lib.rs`):
   ```rust
   let raw_progress = (time - trans_start) / trans.duration;
   let progress = apply_transition_easing(raw_progress, &trans.easing);
   ```
2. **HTML/Chromium Path** (`crates/openmedia-video/src/lib.rs`):
   ```rust
   let raw_progress = (time - trans_start) / trans.duration;
   let transition_progress = apply_transition_easing(raw_progress, &trans.easing);
   ```

---

## 3. Video Template Customization Parameters

The `video_from_template` parameters JSON (`req.parameters`) will support the following properties:
* **`transition_duration`** (float/number): Duration of transitions in seconds (e.g. `0.8`). If omitted, defaults to the template's standard duration (usually `0.5`).
* **`transition_easing`** (string): Easing function to use (e.g., `"ease_in"`, `"ease_out"`, `"ease_in_out"`). If omitted, defaults to `None` (linear).
* **`transition_type`** (string): Allows overriding transition type for templates (e.g., `"crossfade"`, `"slide_left"`, `"slide_up"`).

### Updated Matching Templates in `openmedia-mcp`
We will parse these values from `req.parameters` and populate the transitions accordingly:

1. **`data_dashboard`**:
   * Uses `transition_type` override or defaults to `SlideLeft`.
   * Uses `transition_duration` override or defaults to `0.5`.
   * Uses `transition_easing` override.
2. **`social_media`**:
   * Uses `transition_type` override or defaults to `SlideUp`.
   * Uses `transition_duration` override or defaults to `0.5`.
   * Uses `transition_easing` override.
3. **`product_showcase`**:
   * Uses `transition_type` override or defaults to `Crossfade`.
   * Uses `transition_duration` override or defaults to `0.5`.
   * Uses `transition_easing` override.
4. **`slideshow`**:
   * If a `transition_type` parameter is provided (e.g. `"crossfade"`), we will generate `SceneTransition`s between adjacent image slides using the `transition_duration` and `transition_easing` settings.

---

## 4. Testing Strategy
We will add unit/integration tests to verify that:
1. Easing functions are correctly applied to the transition progress.
2. The `video_from_template` tool parses the transition parameters and generates transition specs with custom durations, easing types, and styles.
