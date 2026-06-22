# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [v0.0.3] - 2026-06-22

### Added
- **SVG Animation Engine (`openmedia-animate`)**: Implemented complete SMIL XML tags serialization (`<animate>`, `<animateTransform>`, `<animateMotion>`, `<set>`), CSS `@keyframes` styling block builders, sequential/parallel/staggered timelines coordinate resolvers, dynamic path coordinate morphing with vertex equalizing, and Lottie layers parsing.
- **MCP Animation Tools**: Exposed 6 new animation tools over the stdio interface: `animate_svg`, `animate_create_timeline`, `animate_morph_paths`, `animate_generate_spinner`, `animate_from_lottie`, and `animate_to_lottie`.
- **Integrated Verification Suites**: Added comprehensive unit tests and integration tests covering animation presets, timeline offsets, morph frames interpolation, and JSON schema derivations.

### Fixed
- Fixed compiler borrowing errors (E0382) and suppressed unused variables/redundant initializations in `openmedia-mcp` to ensure a warnings-free compile across the workspace.

## [v0.0.2] - 2026-06-22

### Added
- **SVG Rasterization Engine (`rasterize_svg`)**: Powered by `resvg` and `tiny-skia` with aspect-ratio-aware dimensions matching, custom scale transform calculations, and alpha channel demultiplexing supporting PNG, JPEG, and WebP exports.
- **HTML/CSS Screenshot Engine (`html_to_image`)**: Integrates `chromiumoxide` to drive headless Chrome over a native Tokio runtime, rendering HTML page contents and capturing high-performance screenshots.
- **MCP Tool Registration**: Bound both tools in `OpenMediaServer` with `schemars::JsonSchema` inputs and `rmcp::handler::server::wrapper::Json<serde_json::Value>` outputs for self-documenting JSON schemas.
- **Animated SVG Logo**: Created a glowing animated SVG logo (`assets/logo.svg`) with a transparent background using inline CSS keyframes for rotation, pulsing, and wave animation.
- **Workspace Integration Tests**: Added verification suites for SVG raster scaling and headless browser navigation.

## [v0.0.1] - 2026-06-22

### Project Vision
**OpenMedia-RS** is an offline-first Model Context Protocol (MCP) server workspace written entirely in native Rust. Our vision is to empower AI coding assistants with a lightweight, multi-crate visual media studio capable of rendering images, videos, SVGs, and animations directly on consumer laptops, with self-correcting prompt feedback and local compute backend fallback options.

### Added
- **Multi-Crate Workspace Layout**: Structured the project workspace into 8 modular crates:
  - `openmedia-core`: Common configurations, hardware diagnostics, model registry catalog, error handlers, and progress callbacks.
  - `openmedia-image`: Quantized Stable Diffusion/FLUX inference pipelines and upscaling filters.
  - `openmedia-video`: HTML/CSS and SVG video renderers, Ken Burns frame composition, and audio mixer wrappers.
  - `openmedia-svg`: Core SVG node builders, data chart renderers, and icon catalog.
  - `openmedia-animate`: CSS keyframes, SMIL properties, and path interpolation.
  - `openmedia-process`: GPU compute pipelines (wgpu/WGSL) and CPU imageproc fallbacks.
  - `openmedia-improve`: Generation history logging (SQLite schema) and CLIP/aesthetic score evaluations.
  - `openmedia-mcp`: The main stdio transport server runner.
- **Dyn Compatible Trait Dispatch**: Re-architected `DiffusionPipeline` and `FrameRenderer` trait definitions utilizing `async-trait` to resolve compiler object-safety errors.
- **Stdio Transport Loop**: Integrated `rmcp` macros (`#[tool_router]` and `#[tool_handler]`) to bind the `ping` tool and run JSON-RPC message passing over stdin/stdout.
- **Diagnostic Logging**: Structured formatting that isolates telemetry prints exclusively to `stderr`, leaving `stdout` intact for protocol compliance.
- **Configuration & Integration Tests**: Implemented unit tests for system directories, fallback rules, environment overrides, and server validation.

### Changed
- Downgraded workspace rust-version from edition `2024` to `2021` to support local compile environments using the Rust 1.82 toolchain.
- Fixed dependency syntax package names (e.g., `diffusion_rs` to `diffusion-rs` v`0.1.20`).
- Configured ONNX Runtime (`ort`) features to disable `tls-native` and enable `tls-rustls` to resolve platform OpenSSL linkage errors.
