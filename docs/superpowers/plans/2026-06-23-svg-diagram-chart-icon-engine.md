# SVG Creation, Chart, and Icon Engine Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement the `create_svg`, `create_chart`, and `create_icon` Model Context Protocol (MCP) tools inside OpenMedia-RS, allowing agents to generate modern vector illustrations, clean dashboard charts, and interface icons natively offline in Rust.

**Architecture:** Build direct SVG builders, mathematical coordinate plotters for bar/line/pie charts, and an embedded library of ~100 essential Feather/Lucide icon paths directly within the `openmedia-svg` crate.

**Tech Stack:** Rust, openmedia-svg, openmedia-mcp.

## Global Constraints
- Target Rust toolchain compatibility: 1.82
- All protocols are stdio JSON-RPC transport (telemetry to stderr only)

---

### Task 1: JSON-to-SVG Schema and Fluent Builder Integration
**Files:**
- Create: `crates/openmedia-svg/src/schema.rs`
- Modify: `crates/openmedia-svg/src/lib.rs`
- Test: `crates/openmedia-svg/tests/svg_tests.rs`

**Interfaces:**
- Consumes: `SvgBuilder` API.
- Produces: `impl SvgBuilder` serialization/deserialization schemas and `create_svg` tool endpoint.

- [ ] **Step 1: Write the failing tests**
  Create an integration test verifying that a list of JSON-defined elements compiles into valid SVG markup.
  ```rust
  #[test]
  fn test_json_svg_generation() {
      let elements_json = serde_json::json!([
          {"type": "rect", "x": 10.0, "y": 10.0, "width": 100.0, "height": 50.0, "fill": "blue"},
          {"type": "circle", "cx": 50.0, "cy": 50.0, "r": 30.0, "fill": "red"}
      ]);
      let svg = openmedia_svg::build_svg_from_json(800, 600, &elements_json).unwrap();
      assert!(svg.contains("rect x=\"10\""));
      assert!(svg.contains("fill=\"blue\""));
      assert!(svg.contains("circle cx=\"50\""));
  }
  ```

- [ ] **Step 2: Run tests to verify they fail**
  Run: `cargo test --test svg_tests`
  Expected: FAIL (compilation errors due to missing schema file and build function)

- [ ] **Step 3: Implement JSON-to-SVG Builder**
  Add the JSON deserializer schema mapping in `crates/openmedia-svg/src/schema.rs` and the `build_svg_from_json` helper.
  ```rust
  // schema.rs
  use serde::{Deserialize, Serialize};

  #[derive(Debug, Clone, Serialize, Deserialize)]
  #[serde(tag = "type", rename_all = "lowercase")]
  pub enum JsonElement {
      Rect { x: f64, y: f64, width: f64, height: f64, rx: Option<f64>, ry: Option<f64>, fill: Option<String>, stroke: Option<String> },
      Circle { cx: f64, cy: f64, r: f64, fill: Option<String>, stroke: Option<String> },
      Text { x: f64, y: f64, content: String, fill: Option<String>, font_size: Option<f64>, font_family: Option<String> },
  }
  ```
  Expose `build_svg_from_json` in `lib.rs`.

- [ ] **Step 4: Run tests to verify they pass**
  Run: `cargo test --test svg_tests`
  Expected: PASS

- [ ] **Step 5: Commit**
  ```bash
  git add crates/openmedia-svg/src/
  git commit -m "feat(svg): add schema structure and JSON-to-SVG builder"
  ```

---

### Task 2: Math-based SVG Chart Generator (Bar, Line, Pie)
**Files:**
- Create: `crates/openmedia-svg/src/chart.rs`
- Modify: `crates/openmedia-svg/src/lib.rs`
- Test: `crates/openmedia-svg/tests/chart_tests.rs`

**Interfaces:**
- Consumes: SvgBuilder API.
- Produces: `pub fn create_chart(chart_type: &str, title: Option<&str>, data: &[ChartPoint], width: u32, height: u32) -> Result<String>`

- [ ] **Step 1: Write the failing tests**
  Create tests verifying chart output contains axes, labels, and geometry.
  ```rust
  #[test]
  fn test_bar_chart_generation() {
      let data = vec![
          openmedia_svg::ChartPoint { label: "A".to_string(), value: 10.0 },
          openmedia_svg::ChartPoint { label: "B".to_string(), value: 20.0 }
      ];
      let svg = openmedia_svg::create_chart("bar", Some("My Bar Chart"), &data, 800, 600).unwrap();
      assert!(svg.contains("My Bar Chart"));
      assert!(svg.contains("rect"));
  }
  ```

- [ ] **Step 2: Run tests to verify they fail**
  Run: `cargo test --test chart_tests`
  Expected: FAIL (compilation errors due to missing file and signature)

- [ ] **Step 3: Implement math-based SVG plotting**
  Implement drawing axes, vertical rectangles (for bar), smooth bezier line paths (for line), and polar angle slices (for pie) using standard trigonometry.
  ```rust
  // Pie Slice path helper:
  // Math: CX + R * cos(angle)
  ```

- [ ] **Step 4: Run tests to verify they pass**
  Run: `cargo test --test chart_tests`
  Expected: PASS

- [ ] **Step 5: Commit**
  ```bash
  git add crates/openmedia-svg/src/chart.rs
  git commit -m "feat(svg): implement custom high-performance SVG chart generator"
  ```

---

### Task 3: Embedded Icon Library (~100 Essential Vector Icons)
**Files:**
- Create: `crates/openmedia-svg/src/icons.rs`
- Modify: `crates/openmedia-svg/src/lib.rs`
- Test: `crates/openmedia-svg/tests/icon_tests.rs`

**Interfaces:**
- Consumes: Hardcoded static slice maps.
- Produces: `pub fn get_icon_svg(name: &str, size: u32, color: &str, stroke_width: f32) -> Option<String>`

- [ ] **Step 1: Write the failing tests**
  Write tests verifying that requesting an icon (like "home" or "settings") returns valid SVG markup with custom size and stroke.
  ```rust
  #[test]
  fn test_icon_retrieval() {
      let svg = openmedia_svg::get_icon_svg("home", 32, "#ff0000", 2.5).unwrap();
      assert!(svg.contains("viewBox=\"0 0 24 24\""));
      assert!(svg.contains("stroke=\"#ff0000\""));
      assert!(svg.contains("stroke-width=\"2.5\""));
  }
  ```

- [ ] **Step 2: Run tests to verify they fail**
  Run: `cargo test --test icon_tests`
  Expected: FAIL (compilation error due to missing module)

- [ ] **Step 3: Embed SVG icon path definitions**
  Create `crates/openmedia-svg/src/icons.rs` containing a hardcoded static hash map or match statement with ~100 common Lucide/Feather icon paths (e.g. home, user, play, search, settings, arrow-right).
  ```rust
  pub fn get_icon_path(name: &str) -> Option<&'static str> {
      match name {
          "home" => Some("M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z M9 22V12h6v10"),
          "settings" => Some("M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.1a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6z"),
          _ => None
      }
  }
  ```

- [ ] **Step 4: Run tests to verify they pass**
  Run: `cargo test --test icon_tests`
  Expected: PASS

- [ ] **Step 5: Commit**
  ```bash
  git add crates/openmedia-svg/src/icons.rs
  git commit -m "feat(svg): embed essential vector icon paths library"
  ```

---

### Task 4: Expose MCP SVG tools (`create_svg`, `create_chart`, `create_icon`)
**Files:**
- Modify: `crates/openmedia-mcp/src/lib.rs`
- Test: `crates/openmedia-mcp/src/lib.rs` (under tests module)

**Interfaces:**
- Consumes: Svg, chart, and icon builders inside `openmedia-svg`.
- Produces: MCP JSON-RPC tool endpoints mapping directly to the vectors output paths.

- [ ] **Step 1: Write the failing tests**
  Write integration tests in `crates/openmedia-mcp/src/lib.rs` verifying that the three new MCP tools return correct `ImageOutput` structures when called with valid parameters.
  ```rust
  #[tokio::test]
  async fn test_mcp_create_chart() {
      // Setup server and run chart create tool
  }
  ```

- [ ] **Step 2: Run tests to verify they fail**
  Run: `cargo test -p openmedia-mcp`
  Expected: FAIL (compilation errors due to missing tool methods and types)

- [ ] **Step 3: Define Request Structs and Implement Tool Methods**
  Implement request parameters (`CreateSvgRequest`, `CreateChartRequest`, `CreateIconRequest`) and wire them up in `impl OpenMediaServer`. Save the output `.svg` vectors into the output directory and return metadata.
  ```rust
  // MCP tools registration
  ```

- [ ] **Step 4: Run tests to verify they pass**
  Run: `cargo test --workspace`
  Expected: PASS

- [ ] **Step 5: Commit**
  ```bash
  git add crates/openmedia-mcp/src/lib.rs
  git commit -m "feat(mcp): register create_svg, create_chart, and create_icon MCP tools"
  ```
