# Advanced Transitions and Chart Types Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add Area, Scatter, and Radar chart support to openmedia-svg, and Blur, Glitch, and Radial Wipe transition blending to openmedia-video.

**Architecture:** Custom math rendering for polar/radar grid coordinates and polygon close-paths in openmedia-svg. Separable box blur, RGB split glitch displacement, and clock wipe polar boundary checks on raw pixel buffers in openmedia-video.

**Tech Stack:** Rust 1.82, image, resvg, tiny-skia, serde, serde_json.

## Global Constraints
- Target Rust toolchain compatibility: 1.82
- All protocols are stdio JSON-RPC transport (telemetry to stderr only)

---

### Task 1: Extend ChartType and TransitionType Enums

**Files:**
- Modify: `crates/openmedia-svg/src/lib.rs:500-520` (add Area, Scatter, Radar to ChartType)
- Modify: `crates/openmedia-video/src/lib.rs:223-238` (add Blur, Glitch, RadialWipe to TransitionType)
- Test: `crates/openmedia-svg/src/lib.rs` (verify deserialization of new variants)

**Interfaces:**
- Consumes: None
- Produces: `ChartType` and `TransitionType` variants

- [ ] **Step 1: Write the failing test**
  Add the following test at the end of `crates/openmedia-svg/src/lib.rs` tests module:
  ```rust
  #[test]
  fn test_new_chart_and_transition_variants_parsing() {
      let area_json = "\"area\"";
      let area: ChartType = serde_json::from_str(area_json).unwrap();
      assert!(matches!(area, ChartType::Area));

      let scatter_json = "\"scatter\"";
      let scatter: ChartType = serde_json::from_str(scatter_json).unwrap();
      assert!(matches!(scatter, ChartType::Scatter));

      let radar_json = "\"radar\"";
      let radar: ChartType = serde_json::from_str(radar_json).unwrap();
      assert!(matches!(radar, ChartType::Radar));
  }
  ```
- [ ] **Step 2: Run test to verify it fails**
  Run: `cargo test --package openmedia-svg --lib -- tests::test_new_chart_and_transition_variants_parsing`
  Expected: Compile error due to missing variants `Area`, `Scatter`, `Radar` in `ChartType`.
- [ ] **Step 3: Write minimal implementation**
  Add variants to `ChartType` in `crates/openmedia-svg/src/lib.rs`:
  ```rust
  #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
  #[serde(rename_all = "snake_case")]
  pub enum ChartType {
      Bar,
      Line,
      Pie,
      Area,
      Scatter,
      Radar,
  }
  ```
  Add variants to `TransitionType` in `crates/openmedia-video/src/lib.rs`:
  ```rust
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
      Blur,
      Glitch,
      RadialWipe,
  }
  ```
  Also update `crates/openmedia-svg/src/lib.rs:616` to map new `ChartType` values to strings:
  ```rust
  let chart_type_str = match config.chart_type {
      ChartType::Bar => "bar",
      ChartType::Line => "line",
      ChartType::Pie => "pie",
      ChartType::Area => "area",
      ChartType::Scatter => "scatter",
      ChartType::Radar => "radar",
  };
  ```
- [ ] **Step 4: Run test to verify it passes**
  Run: `cargo test --package openmedia-svg --lib -- tests::test_new_chart_and_transition_variants_parsing`
  Expected: PASS
- [ ] **Step 5: Commit**
  ```bash
  git add crates/openmedia-svg/src/lib.rs crates/openmedia-video/src/lib.rs
  git commit -m "feat: add Area, Scatter, Radar to ChartType and Blur, Glitch, RadialWipe to TransitionType"
  ```

---

### Task 2: Implement Area and Scatter Chart Types

**Files:**
- Modify: `crates/openmedia-svg/src/chart.rs` (add match arms for area and scatter in `create_chart`)
- Test: `crates/openmedia-svg/tests/chart_tests.rs` (test area and scatter output SVG structure)

**Interfaces:**
- Consumes: `ChartType::Area` and `ChartType::Scatter`
- Produces: Valid SVG strings containing `<polygon>` and `<circle>` tags

- [ ] **Step 1: Write the failing test**
  Add the following tests in `tests/chart_tests.rs`:
  ```rust
  #[test]
  fn test_area_chart_generation() {
      let data = vec![
          ChartPoint { label: "Jan".to_string(), value: 10.0 },
          ChartPoint { label: "Feb".to_string(), value: 20.0 },
      ];
      let svg = create_chart("area", Some("Area Test"), &data, 800, 600).unwrap();
      assert!(svg.contains("<polygon"));
  }

  #[test]
  fn test_scatter_chart_generation() {
      let data = vec![
          ChartPoint { label: "Jan".to_string(), value: 10.0 },
          ChartPoint { label: "Feb".to_string(), value: 20.0 },
      ];
      let svg = create_chart("scatter", Some("Scatter Test"), &data, 800, 600).unwrap();
      assert!(svg.contains("<circle cx="));
  }
  ```
- [ ] **Step 2: Run test to verify it fails**
  Run: `cargo test --test chart_tests`
  Expected: FAIL with "Unsupported chart type" panic/error.
- [ ] **Step 3: Write minimal implementation**
  In `crates/openmedia-svg/src/chart.rs`, add match arms:
  ```rust
          "area" => {
              let margin_left = 60.0;
              let margin_right = 40.0;
              let margin_top = 70.0;
              let margin_bottom = 60.0;

              let plot_width = width as f64 - margin_left - margin_right;
              let plot_height = height as f64 - margin_top - margin_bottom;

              let max_val = data.iter().map(|p| p.value).fold(0.0f64, f64::max);
              let max_val = if max_val <= 0.0 { 1.0 } else { max_val };

              // Grid lines
              let grid_count = 5;
              for i in 0..=grid_count {
                  let ratio = i as f64 / grid_count as f64;
                  let y_val = margin_top + plot_height * (1.0 - ratio);
                  let label_val = ratio * max_val;
                  builder.path(&format!("M {} {} L {} {}", margin_left, y_val, width as f64 - margin_right, y_val))
                      .stroke("#333355").stroke_width(1.0).fill("none").finish();
                  builder.text(10.0, y_val + 4.0, &format!("{:.1}", label_val))
                      .fill("#94a3b8").font_size(10.0).font_family("sans-serif").finish();
              }

              let n = data.len();
              let y_base = margin_top + plot_height;
              
              // 1. Build area polygon path
              let mut poly_points = format!("M {} {}", margin_left, y_base);
              for i in 0..n {
                  let p = &data[i];
                  let x = if n > 1 {
                      margin_left + (i as f64 / (n - 1) as f64) * plot_width
                  } else {
                      margin_left + plot_width / 2.0
                  };
                  let y = margin_top + plot_height - (p.value / max_val) * plot_height;
                  poly_points.push_str(&format!(" L {} {}", x, y));
              }
              let last_x = if n > 1 { margin_left + plot_width } else { margin_left + plot_width / 2.0 };
              poly_points.push_str(&format!(" L {} {} Z", last_x, y_base));

              builder.path(&poly_points)
                  .fill("#3b82f6")
                  .opacity(0.3)
                  .finish();

              // 2. Draw line and markers
              let mut line_path = String::new();
              for i in 0..n {
                  let p = &data[i];
                  let x = if n > 1 {
                      margin_left + (i as f64 / (n - 1) as f64) * plot_width
                  } else {
                      margin_left + plot_width / 2.0
                  };
                  let y = margin_top + plot_height - (p.value / max_val) * plot_height;

                  if i == 0 {
                      line_path.push_str(&format!("M {} {}", x, y));
                  } else {
                      line_path.push_str(&format!(" L {} {}", x, y));
                  }

                  builder.circle(x, y, 4.0)
                      .fill("#ffffff").stroke("#3b82f6").stroke_width(2.0).finish();

                  // Labels
                  let label_x = x - (p.label.len() as f64 * 3.0);
                  builder.text(label_x, y_base + 20.0, &p.label)
                      .fill("#94a3b8").font_size(10.0).font_family("sans-serif").finish();
              }
              builder.path(&line_path)
                  .stroke("#3b82f6").stroke_width(3.0).fill("none").finish();
          }
          "scatter" => {
              let margin_left = 60.0;
              let margin_right = 40.0;
              let margin_top = 70.0;
              let margin_bottom = 60.0;

              let plot_width = width as f64 - margin_left - margin_right;
              let plot_height = height as f64 - margin_top - margin_bottom;

              let max_val = data.iter().map(|p| p.value).fold(0.0f64, f64::max);
              let max_val = if max_val <= 0.0 { 1.0 } else { max_val };

              // Grid lines
              let grid_count = 5;
              for i in 0..=grid_count {
                  let ratio = i as f64 / grid_count as f64;
                  let y_val = margin_top + plot_height * (1.0 - ratio);
                  let label_val = ratio * max_val;
                  builder.path(&format!("M {} {} L {} {}", margin_left, y_val, width as f64 - margin_right, y_val))
                      .stroke("#333355").stroke_width(1.0).fill("none").finish();
                  builder.text(10.0, y_val + 4.0, &format!("{:.1}", label_val))
                      .fill("#94a3b8").font_size(10.0).font_family("sans-serif").finish();
              }

              let n = data.len();
              let y_base = margin_top + plot_height;

              for i in 0..n {
                  let p = &data[i];
                  let x = if n > 1 {
                      margin_left + (i as f64 / (n - 1) as f64) * plot_width
                  } else {
                      margin_left + plot_width / 2.0
                  };
                  let y = margin_top + plot_height - (p.value / max_val) * plot_height;

                  let color = palette[i % palette.len()];
                  builder.circle(x, y, 6.0)
                      .fill(color).stroke("#ffffff").stroke_width(1.5).finish();

                  // Value label above dot
                  builder.text(x - 10.0, y - 10.0, &format!("{:.1}", p.value))
                      .fill("#ffffff").font_size(9.0).font_family("sans-serif").finish();

                  // X-axis labels
                  let label_x = x - (p.label.len() as f64 * 3.0);
                  builder.text(label_x, y_base + 20.0, &p.label)
                      .fill("#94a3b8").font_size(10.0).font_family("sans-serif").finish();
              }
          }
  ```
- [ ] **Step 4: Run test to verify it passes**
  Run: `cargo test --test chart_tests`
  Expected: PASS
- [ ] **Step 5: Commit**
  ```bash
  git add crates/openmedia-svg/src/chart.rs tests/chart_tests.rs
  git commit -m "feat: implement area and scatter SVG chart generators"
  ```

---

### Task 3: Implement Radar Chart Type

**Files:**
- Modify: `crates/openmedia-svg/src/chart.rs` (add match arm for radar in `create_chart`)
- Test: `crates/openmedia-svg/tests/chart_tests.rs` (test radar spokes and layout SVG structure)

**Interfaces:**
- Consumes: `ChartType::Radar`
- Produces: Valid SVG strings containing `<polygon>` grids and data points coordinates

- [ ] **Step 1: Write the failing test**
  Add the following test in `tests/chart_tests.rs`:
  ```rust
  #[test]
  fn test_radar_chart_generation() {
      let data = vec![
          ChartPoint { label: "A".to_string(), value: 80.0 },
          ChartPoint { label: "B".to_string(), value: 60.0 },
          ChartPoint { label: "C".to_string(), value: 90.0 },
      ];
      let svg = create_chart("radar", Some("Radar Test"), &data, 800, 600).unwrap();
      assert!(svg.contains("polygon"));
  }
  ```
- [ ] **Step 2: Run test to verify it fails**
  Run: `cargo test --test chart_tests`
  Expected: FAIL with "Unsupported chart type: radar"
- [ ] **Step 3: Write minimal implementation**
  In `crates/openmedia-svg/src/chart.rs`, add match arm:
  ```rust
          "radar" => {
              let cx = width as f64 / 2.0;
              let cy = height as f64 / 2.0 + 20.0;
              let r = (width.min(height) as f64 * 0.7) / 2.0;

              let max_val = data.iter().map(|p| p.value).fold(0.0f64, f64::max);
              let max_val = if max_val <= 0.0 { 1.0 } else { max_val };
              let n = data.len();

              // Draw concentric grids (5 rings)
              let ring_count = 5;
              for i in 1..=ring_count {
                  let ratio = i as f64 / ring_count as f64;
                  let ring_r = r * ratio;
                  let mut grid_path = String::new();
                  
                  for j in 0..n {
                      let angle = j as f64 * (2.0 * std::f64::consts::PI / n as f64) - std::f64::consts::FRAC_PI_2;
                      let x = cx + ring_r * angle.cos();
                      let y = cy + ring_r * angle.sin();
                      if j == 0 {
                          grid_path.push_str(&format!("M {} {}", x, y));
                      } else {
                          grid_path.push_str(&format!(" L {} {}", x, y));
                      }
                  }
                  grid_path.push_str(" Z");
                  builder.path(&grid_path)
                      .stroke("#333355").stroke_width(1.0).fill("none").finish();
              }

              // Draw spokes (lines from center to outer ring vertices)
              let mut data_path = String::new();
              for j in 0..n {
                  let angle = j as f64 * (2.0 * std::f64::consts::PI / n as f64) - std::f64::consts::FRAC_PI_2;
                  let outer_x = cx + r * angle.cos();
                  let outer_y = cy + r * angle.sin();
                  
                  builder.path(&format!("M {} {} L {} {}", cx, cy, outer_x, outer_y))
                      .stroke("#333355").stroke_width(1.0).fill("none").finish();

                  // Labels
                  let label_dist = r + 20.0;
                  let label_x = cx + label_dist * angle.cos() - (data[j].label.len() as f64 * 3.0);
                  let label_y = cy + label_dist * angle.sin() + 4.0;
                  builder.text(label_x, label_y, &data[j].label)
                      .fill("#94a3b8").font_size(11.0).font_family("sans-serif").finish();

                  // Data point coord
                  let val_r = r * (data[j].value / max_val);
                  let val_x = cx + val_r * angle.cos();
                  let val_y = cy + val_r * angle.sin();
                  
                  if j == 0 {
                      data_path.push_str(&format!("M {} {}", val_x, val_y));
                  } else {
                      data_path.push_str(&format!(" L {} {}", val_x, val_y));
                  }
              }
              data_path.push_str(" Z");

              // Draw data polygon area
              builder.path(&data_path)
                  .fill("#3b82f6").opacity(0.35).finish();
              builder.path(&data_path)
                  .stroke("#3b82f6").stroke_width(2.5).fill("none").finish();

              // Draw circles on data vertices
              for j in 0..n {
                  let angle = j as f64 * (2.0 * std::f64::consts::PI / n as f64) - std::f64::consts::FRAC_PI_2;
                  let val_r = r * (data[j].value / max_val);
                  let val_x = cx + val_r * angle.cos();
                  let val_y = cy + val_r * angle.sin();
                  
                  builder.circle(val_x, val_y, 4.0)
                      .fill("#ffffff").stroke("#3b82f6").stroke_width(2.0).finish();
              }
          }
  ```
- [ ] **Step 4: Run test to verify it passes**
  Run: `cargo test --test chart_tests`
  Expected: PASS
- [ ] **Step 5: Commit**
  ```bash
  git add crates/openmedia-svg/src/chart.rs
  git commit -m "feat: implement radar (spiderweb) SVG chart renderer"
  ```

---

### Task 4: Implement Blur, Glitch, and Radial Wipe transitions

**Files:**
- Modify: `crates/openmedia-video/src/lib.rs` (implement box blur, radial checks, and glitch horizontal splitting in `blend_frames`)
- Test: `crates/openmedia-video/src/lib.rs` (write unit tests for the transitions blending output checks)

**Interfaces:**
- Consumes: `TransitionType::Blur`, `TransitionType::Glitch`, `TransitionType::RadialWipe`
- Produces: Blended `image::RgbaImage` frames

- [ ] **Step 1: Write the failing test**
  Add the following test at the end of the `tests` module in `crates/openmedia-video/src/lib.rs`:
  ```rust
  #[test]
  fn test_advanced_transitions_blend() {
      let from = image::RgbaImage::from_pixel(100, 100, image::Rgba([255, 0, 0, 255]));
      let to = image::RgbaImage::from_pixel(100, 100, image::Rgba([0, 0, 255, 255]));

      let blended_blur = blend_frames(&from, &to, 0.5, &TransitionType::Blur);
      assert_eq!(blended_blur.width(), 100);

      let blended_glitch = blend_frames(&from, &to, 0.5, &TransitionType::Glitch);
      assert_eq!(blended_glitch.width(), 100);

      let blended_radial = blend_frames(&from, &to, 0.5, &TransitionType::RadialWipe);
      assert_eq!(blended_radial.width(), 100);
  }
  ```
- [ ] **Step 2: Run test to verify it fails**
  Run: `cargo test --package openmedia-video --lib -- tests::test_advanced_transitions_blend`
  Expected: Compile errors due to missing variants or default falling back to normal crossfade without rendering.
- [ ] **Step 3: Write minimal implementation**
  Add a `box_blur` helper function inside `crates/openmedia-video/src/lib.rs`:
  ```rust
  fn box_blur(img: &image::RgbaImage, radius: usize) -> image::RgbaImage {
      let w = img.width();
      let h = img.height();
      let mut temp = image::RgbaImage::new(w, h);
      let mut out = image::RgbaImage::new(w, h);
      
      // Pass 1: Horizontal
      for y in 0..h {
          for x in 0..w {
              let mut r_sum = 0u32;
              let mut g_sum = 0u32;
              let mut b_sum = 0u32;
              let mut a_sum = 0u32;
              let mut count = 0u32;
              
              let start = if x as usize >= radius { x as usize - radius } else { 0 };
              let end = std::cmp::min(x as usize + radius, w as usize - 1);
              
              for k in start..=end {
                  let p = img.get_pixel(k as u32, y);
                  r_sum += p[0] as u32;
                  g_sum += p[1] as u32;
                  b_sum += p[2] as u32;
                  a_sum += p[3] as u32;
                  count += 1;
              }
              temp.put_pixel(x, y, image::Rgba([
                  (r_sum / count) as u8,
                  (g_sum / count) as u8,
                  (b_sum / count) as u8,
                  (a_sum / count) as u8,
              ]));
          }
      }
      
      // Pass 2: Vertical
      for x in 0..w {
          for y in 0..h {
              let mut r_sum = 0u32;
              let mut g_sum = 0u32;
              let mut b_sum = 0u32;
              let mut a_sum = 0u32;
              let mut count = 0u32;
              
              let start = if y as usize >= radius { y as usize - radius } else { 0 };
              let end = std::cmp::min(y as usize + radius, h as usize - 1);
              
              for k in start..=end {
                  let p = temp.get_pixel(x, k as u32);
                  r_sum += p[0] as u32;
                  g_sum += p[1] as u32;
                  b_sum += p[2] as u32;
                  a_sum += p[3] as u32;
                  count += 1;
              }
              out.put_pixel(x, y, image::Rgba([
                  (r_sum / count) as u8,
                  (g_sum / count) as u8,
                  (b_sum / count) as u8,
                  (a_sum / count) as u8,
              ]));
          }
      }
      out
  }
  ```
  Now, add match arms to `blend_frames` in `crates/openmedia-video/src/lib.rs`:
  ```rust
          TransitionType::Blur => {
              let intensity = 1.0 - (progress - 0.5).abs() * 2.0;
              let radius = (intensity * 10.0).round() as usize;
              if radius > 0 {
                  let blurred_from = box_blur(from, radius);
                  let blurred_to = box_blur(to, radius);
                  for y in 0..h {
                      for x in 0..w {
                          let p1 = blurred_from.get_pixel(x, y);
                          let p2 = blurred_to.get_pixel(x, y);
                          let r = (p1[0] as f64 * (1.0 - progress) + p2[0] as f64 * progress) as u8;
                          let g = (p1[1] as f64 * (1.0 - progress) + p2[1] as f64 * progress) as u8;
                          let b = (p1[2] as f64 * (1.0 - progress) + p2[2] as f64 * progress) as u8;
                          let a = (p1[3] as f64 * (1.0 - progress) + p2[3] as f64 * progress) as u8;
                          out.put_pixel(x, y, image::Rgba([r, g, b, a]));
                      }
                  }
              } else {
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
          TransitionType::Glitch => {
              let intensity = 1.0 - (progress - 0.5).abs() * 2.0;
              let disp_max = (intensity * 15.0) as i32;
              
              for y in 0..h {
                  // scanline row displacement
                  let mut seed = y as u32 + (progress * 1000.0) as u32;
                  seed ^= seed << 13;
                  seed ^= seed >> 17;
                  seed ^= seed << 5;
                  
                  let offset_x = if seed % 100 < (intensity * 40.0) as u32 {
                      ((seed % 31) as i32 - 15) * disp_max / 15
                  } else {
                      0
                  };

                  for x in 0..w {
                      let get_chan = |img: &image::RgbaImage, channel: usize, dx: i32| -> u8 {
                          let target_x = std::cmp::max(0, std::cmp::min(w as i32 - 1, x as i32 + dx)) as u32;
                          img.get_pixel(target_x, y)[channel]
                      };

                      // Read split channels from source
                      let r1 = get_chan(from, 0, offset_x - 3);
                      let g1 = get_chan(from, 1, offset_x);
                      let b1 = get_chan(from, 2, offset_x + 3);
                      let a1 = get_chan(from, 3, offset_x);

                      // Read split channels from target
                      let r2 = get_chan(to, 0, offset_x - 3);
                      let g2 = get_chan(to, 1, offset_x);
                      let b2 = get_chan(to, 2, offset_x + 3);
                      let a2 = get_chan(to, 3, offset_x);

                      // Blend channels
                      let mut r = (r1 as f64 * (1.0 - progress) + r2 as f64 * progress) as i32;
                      let mut g = (g1 as f64 * (1.0 - progress) + g2 as f64 * progress) as i32;
                      let mut b = (b1 as f64 * (1.0 - progress) + b2 as f64 * progress) as i32;
                      let a = (a1 as f64 * (1.0 - progress) + a2 as f64 * progress) as u8;

                      // Add a touch of noise/static
                      if intensity > 0.1 && (seed % 97) == 0 {
                          let noise = ((seed % 51) as i32 - 25) * (intensity * 2.0) as i32;
                          r = std::cmp::max(0, std::cmp::min(255, r + noise));
                          g = std::cmp::max(0, std::cmp::min(255, g + noise));
                          b = std::cmp::max(0, std::cmp::min(255, b + noise));
                      }

                      out.put_pixel(x, y, image::Rgba([r as u8, g as u8, b as u8, a]));
                  }
              }
          }
          TransitionType::RadialWipe => {
              let cx = w as f64 / 2.0;
              let cy = h as f64 / 2.0;
              for y in 0..h {
                  for x in 0..w {
                      let dx = x as f64 - cx;
                      let dy = y as f64 - cy;
                      let angle = dy.atan2(dx) + std::f64::consts::PI; // [0, 2*PI]
                      let angle_ratio = angle / (2.0 * std::f64::consts::PI);
                      if angle_ratio < progress {
                          out.put_pixel(x, y, *to.get_pixel(x, y));
                      } else {
                          out.put_pixel(x, y, *from.get_pixel(x, y));
                      }
                  }
              }
          }
  ```
- [ ] **Step 4: Run test to verify it passes**
  Run: `cargo test --package openmedia-video --lib -- tests::test_advanced_transitions_blend`
  Expected: PASS
- [ ] **Step 5: Commit**
  ```bash
  git add crates/openmedia-video/src/lib.rs
  git commit -m "feat: implement Blur, Glitch, and RadialWipe transition frame blending"
  ```

---

### Task 5: Integrate Enums and Presets in MCP Crate

**Files:**
- Modify: `crates/openmedia-mcp/src/lib.rs` (case-insensitive string parsers for presets)
- Test: `crates/openmedia-mcp/src/lib.rs` (verify workflow using new presets)

**Interfaces:**
- Consumes: MCP transition parameters
- Produces: Parsed `TransitionType` matching the exact string options

- [ ] **Step 1: Write the failing test**
  Add the following test case inside the tests block of `crates/openmedia-mcp/src/lib.rs`:
  ```rust
  #[test]
  fn test_mcp_transition_presets_parsing() {
      assert_eq!(parse_transition_type("blur"), openmedia_video::TransitionType::Blur);
      assert_eq!(parse_transition_type("GLITCH"), openmedia_video::TransitionType::Glitch);
      assert_eq!(parse_transition_type("radial_wipe"), openmedia_video::TransitionType::RadialWipe);
  }
  ```
- [ ] **Step 2: Run test to verify it fails**
  Run: `cargo test --package openmedia-mcp --lib -- tests::test_mcp_transition_presets_parsing`
  Expected: Compile/run failure due to unmapped string presets.
- [ ] **Step 3: Write minimal implementation**
  Find `parse_transition_type` (or where transition type strings are mapped) inside `crates/openmedia-mcp/src/lib.rs` and update it to parse `blur`, `glitch`, and `radial_wipe` correctly.
  Let's check if the parse helper is named `parse_transition_type`.
  Wait, let's look at `crates/openmedia-mcp/src/lib.rs` using grep.
  We can search for `TransitionType` conversions or matches in `crates/openmedia-mcp/src/lib.rs`.
  Let's see: `parse_transition_params` helper exists. Let's make sure it handles these.
  ```rust
  fn parse_transition_params(parameters: &serde_json::Value, default_type: openmedia_video::TransitionType) -> (openmedia_video::TransitionType, f64, Option<String>) {
      // ...
  }
  ```
  We should modify this helper to map the strings properly.
  ```rust
  let trans_type = match type_str.to_lowercase().as_str() {
      "none" => openmedia_video::TransitionType::None,
      "crossfade" => openmedia_video::TransitionType::Crossfade,
      "slide_left" | "slideleft" | "slide-left" => openmedia_video::TransitionType::SlideLeft,
      "slide_right" | "slideright" | "slide-right" => openmedia_video::TransitionType::SlideRight,
      "slide_up" | "slideup" | "slide-up" => openmedia_video::TransitionType::SlideUp,
      "slide_down" | "slidedown" | "slide-down" => openmedia_video::TransitionType::SlideDown,
      "zoom_in" | "zoomin" | "zoom-in" => openmedia_video::TransitionType::ZoomIn,
      "zoom_out" | "zoomout" | "zoom-out" => openmedia_video::TransitionType::ZoomOut,
      "wipe_left" | "wipeleft" | "wipe-left" => openmedia_video::TransitionType::WipeLeft,
      "wipe_right" | "wiperight" | "wipe-right" => openmedia_video::TransitionType::WipeRight,
      "blur" => openmedia_video::TransitionType::Blur,
      "glitch" => openmedia_video::TransitionType::Glitch,
      "radial_wipe" | "radialwipe" | "radial-wipe" => openmedia_video::TransitionType::RadialWipe,
      _ => default_type,
  };
  ```
- [ ] **Step 4: Run test to verify it passes**
  Run: `cargo test --package openmedia-mcp --lib -- tests::test_mcp_transition_presets_parsing`
  Expected: PASS
- [ ] **Step 5: Commit**
  ```bash
  git add crates/openmedia-mcp/src/lib.rs
  git commit -m "feat(mcp): map Blur, Glitch, RadialWipe in transition string parameter parser"
  ```
