# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [v0.0.15] - 2026-06-25

### Added
- **Video Template CRUD Operations**: Implemented `template_create`, `template_read`, `template_update`, and `template_delete` MCP tools, enabling clients/agents to manage custom template definitions.
- **Dynamic Custom Template Interpolation**: Extended `video_from_template` to support custom templates by loading saved definitions from `assets/templates/`, interpolating parameter placeholders using string substitution, and rendering the compiled scenes.
- **Integration Tests**: Added a complete CRUD workflow integration test `test_mcp_template_crud_workflow` to verify template creation, listing, reading, updating, rendering, and deletion.

## [v0.0.14] - 2026-06-25

### Added
- **Native Vector Chart Rendering Support**: Implemented proper data parsing and chart generation in `generate_chart` (in `openmedia-svg`), resolving a previous placeholder stub. Chart elements can now parse JSON data correctly and delegate to internal `create_chart` layouts.
- **MCP Video Creation Example**: Added a runnable example (`mcp_video_example.rs`) demonstrating how to use the `video_create` MCP tool via the `OpenMediaServer` client to render shapes, texts, keyframe-based animations, and data charts offline without Chromium-based frame failures.
- **Headless Browser Diagnostic Tool**: Added `test_chrome.rs` utility to debug browser launching and event stream handlers.

## [v0.0.13] - 2026-06-25

### Added
- **Custom Transition Easing & Duration Overrides**: Extended the transition engine to support customizable easing overrides (`linear`, `ease_in`, `ease_out`, `ease_in_out`, and case-insensitive matching). Allowed overriding default durations and easing in templates via JSON parameters (`transition_type`, `transition_duration`, `transition_easing`).
- **Dynamic Font Loading & SVG Integration**: Added support for loading local `.ttf`/`.otf` files and downloading remote fonts. Configured `resvg`'s `fontdb` database dynamically in the frame rendering pipeline.
- **High-Performance Font Caching**: Designed thread-safe caches for system font databases, HTML base64 CSS declarations, and transient download failures (with 60s TTL) to maximize video rendering frame rates.
- **Atomic Disk Writing**: Implemented UUID-based temporary file writing followed by atomic file renaming (`std::fs::rename`) to guarantee filesystem cache integrity.
- **Offline-Safe Tests**: Refactored video template integration tests to dynamically generate local mock font files, resolving external HTTP dependencies.

## [v0.0.12] - 2026-06-24

### Added
- **Audio Overlays in Video Templates**: Enabled mixing of multi-track audio configs or background music in the `video_from_template` tool.
- **Audio Config Parsing**: Added robust parsing for `"background_music"` (simple string path) and `"audio_tracks"` (complex structures containing volume, offset/start, and fade-in/fade-out parameters) directly from template parameters.
- **FFmpeg Error Diagnosis**: Enhanced FFmpeg audio mixing subprocess execution to capture stdout/stderr and explicitly return error status codes instead of failing silently.
- **Verification Tests**: Added integration test `test_mcp_video_template_with_audio` generating dynamic WAV files to verify audio mixing compilation logic.

## [v0.0.11] - 2026-06-23

### Added
- **Pre-Designed Video Templates**: Implemented `data_dashboard`, `social_media`, and `product_showcase` template matches and scene structures in the `video_from_template` tool.
- **Dynamic Layout Assembly**: Added support for statistical title cards, custom charts embedding, vertical 9:16 layout formats, animations, and transitions.
- **Integration Tests**: Added `test_mcp_video_template_data_dashboard` to verify correct template compilation and output path validation.

## [v0.0.10] - 2026-06-23

### Added
- **Refined Lightweight Architecture**: Re-scoped the project to focus on a strictly lightweight vector rendering, diagram compilation, and layout animation engine (< 50MB RAM/ROM footprint). Bypassed heavy local AI diffusion models in favor of online LLM/multimodal agents acting as the visual code designers.
- **Custom Mermaid Styling & Theme Overrides**: Extended `diagram_generate_mermaid` tool and the underlying `render_mermaid` engine to support theme presets (`default`, `dark`, `forest`, `neutral`), custom layout spacing adjustments, and partial field overrides via JSON.
- **System Resource Guidelines**: Documented exact CPU, RAM, and ROM constraints for each media module to ensure optimal local performance.

## [v0.0.9] - 2026-06-23

### Added
- **JSON-to-SVG Canvas Engine**: Created `schema.rs` and implemented `build_svg_from_json` parsing JSON arrays into styled shapes (Rects, Circles, Texts) natively.
- **Math-based SVG Chart Generator**: Built vertical bars charting, smooth bezier curve plotting, and polar coordinates pie slice drawing (using arc paths) with percentage legends.
- **Embedded Vector Icon Library**: Bundled 20 popular Feather/Lucide vector paths (e.g., home, user, settings) to scale and style on demand.
- **New MCP Tools**: Registered `create_svg`, `create_chart`, and `create_icon` JSON-RPC tool schemas on the MCP server.
- **Integrated Validation Tests**: Added comprehensive integration test suites verifying correct SVG compilation and element attributes.

## [v0.0.8] - 2026-06-23

### Added
- **Mermaid Diagram Engine**: Integrated native Mermaid parser and renderer (`mermaid-rs-renderer`) into `openmedia-svg` to compile Mermaid text blocks to SVGs natively offline in Rust.
- **Mermaid Generation MCP Tool**: Registered the `diagram_generate_mermaid` JSON-RPC tool on the MCP server to render Mermaid diagrams to SVG, PNG, JPEG, or WebP.
- **Integrated Verification Suites**: Added comprehensive tests verifying native Mermaid rendering and image rasterization.

## [v0.0.7] - 2026-06-23

### Added
- **Model Auto-Download Experience**: Integrated automatic model downloader inside the model registry (`openmedia-core`), streaming CLIP text, CLIP vision, and LAION Aesthetic predictor models directly from the Hugging Face Hub via `reqwest` chunk-by-chunk streams.
- **Model Download MCP Tool**: Registered `model_download` JSON-RPC tool on the MCP server (`openmedia-mcp`) allowing agents to request downloads dynamically with an input validation schema.
- **Stderr Progress Reporting**: Built a dedicated thread-safe `StderrProgressReporter` to emit per-byte streaming download progress metrics directly to `stderr` without corrupting standard output streams used for MCP stdio communications.
- **Multi-Stage Dockerfile**: Set up a production-ready, multi-stage `Dockerfile` creating a lightweight Debian-slim container pre-configured with Chrome headless and FFmpeg requirements.
- **GitHub Actions Release Workflow**: Created `.github/workflows/release.yml` automatically building and packaging binaries for Linux (x86_64), macOS (x86_64, aarch64), and Windows (x86_64) on tag pushes.
- **Release Profile Optimizations**: Configured optimized release profile (`opt-level = 3`, Link-Time Optimization `lto = true`, `codegen-units = 1`, `panic = "abort"`, and `strip = true`) to minimize binaries size and speed up execution.
- **Documentation Updates**: Updated `README.md` system requirements and comparison tables to reflect the actual, highly-optimized footprint and scope of OpenMedia-RS.

### Fixed
- Replaced `native-tls` default feature flags in `reqwest` dependency with static `rustls-tls` backend to resolve system-level OpenSSL compilation and linkage issues.
- Fixed unused import compiler warnings in the model registry crate.
- Fixed a test assertion in the `model_download` suite to align with display formatting of model registry errors.

## [v0.0.6] - 2026-06-23

### Added
- **Scoring Engine (`openmedia-improve`)**: Implemented CLIP text-image alignment scoring using ONNX Runtime (`ort`) and ndarray preprocessing (Lanczos3 resize, mean/std normalization, NCHW flat float representation, cosine similarity calculation). Implemented LAION aesthetic scorer linear probe inference placeholder. Built fallback/mock scorer mechanisms when model files are not yet loaded.
- **Generation History System**: Set up a SQLite database system to record generation records (inputs, outputs, parameters, scores, timestamps) automatically, supporting complex queries, stats collection, and user feedback submission.
- **Prompt Refinement System**: Implemented iterative prompt refiner that appends quality suffixes ("highly detailed", "professional", etc.) and default negative parameters ("blurry", "low quality"), adjusting steps and CFG guidance dynamically based on score history.
- **5 Self-Improvement MCP Tools**: Exposed `improve_score_image`, `improve_refine_prompt`, `improve_auto_refine` (implementing a complete SVG rendering and rasterizing refinement loop with database record chaining), `improve_feedback`, and `improve_quality_report` JSON-RPC tools.
- **Integrated Verification Suites**: Added comprehensive tests in `mcp_improve_tests.rs` verifying scoring, refinement suggestions, auto-refinement loops, feedback, and database statistics.

### Fixed
- Fixed compilation and borrow checker issues with `ort` session builders by wrapping session objects in thread-safe `Mutex` wrappers.
- Handled type compatibility of output maps from ONNX inference by directly extracting output tensors from the result iterators.
- Corrected raw string literal terminations in SVG template generation by switching to `r##` delimiters to safely allow `#` characters.

## [v0.0.5] - 2026-06-23

### Added
- **Video Scene Composition Engine (`openmedia-video`)**: Implemented a JSON-based Video Scene DSL parser and compiler. Built CPU SVG rasterizing frame rendering with `tiny-skia`/`resvg`, and headless browser rendering via `chromiumoxide`. Implemented pixel-level frame transitions (Crossfade, Slide Left/Right/Up/Down, Wipe Left/Right) for SVG frames and CSS transitions for HTML pages. Wired up a silent H.264 video encoder utilizing FFmpeg piped MJPEG stdin stream, and a multi-track audio mixer/muxer (with delay and volume blending scripts).
- **MCP Video Tools**: Registered 8 new video tools on the JSON-RPC interface: `video_create`, `video_preview`, `video_create_slideshow`, `video_add_transition`, `video_add_audio`, `video_from_template`, `video_extract_frames`, `video_trim`.
- **Integrated Verification Suites**: Added comprehensive integration tests in `mcp_video_tests.rs` covering scene composition, preview frame extraction, slideshow generation, template rendering, frame extraction, video trimming, transitions, and audio track additions.

### Fixed
- Fixed color type conversion from RGBA8 to RGB8 before encoding to JPEG (since JPEG doesn't support transparency) to resolve FFmpeg MJPEG encoding failures.
- Corrected `chromiumoxide` Page viewport resizing using raw CDP Emulation commands (`SetDeviceMetricsOverrideParams`).
- Resolved `chromiumoxide` Browser close ownership borrowing issues by removing `Arc` wrapper and using `mut self` ownership.
- Eliminated all unused imports and mutability clippy warnings in `openmedia-mcp` and `openmedia-video` crates to ensure a clean, warning-free build.

## [v0.0.4] - 2026-06-23

### Added
- **GPU Compute Processing Core (`openmedia-process`)**: Configured a `wgpu` v23.0 context pipeline utilizing WGSL compute shaders with 16-byte uniform alignment. Added a little-endian pixel buffer upload/download framework, and implemented the `invert.wgsl` compute shader to perform GPU-accelerated image inversion.
- **CPU Image Processing Fallback (`openmedia-process`)**: Implemented multi-threaded CPU parallel filters using `rayon` and `imageproc` covering grayscale, invert, brightness, contrast, saturation, hue rotation, sepia, threshold, box blur (utilizing RGBA channel splitting), and composite filters.
- **Geometric Transforms & Formats support**: Added aspect-ratio-aware resizing (Nearest, Bilinear, Lanczos3), cropping, rotations, and flips. Mapped encoders for PNG, JPEG, WebP, and AVIF outputs with custom quality configurations.
- **Sequential Filter Chains & Concurrent Batch Processor**: Implemented the `FilterChain` pipeline to execute sequential operations with GPU priority and CPU fallback. Added `batch_process_files` to concurrently process images matched by glob pattern using tokio threads.
- **MCP Image Processing Tools**: Exposed 6 new tools over the JSON-RPC interface: `image_apply_filter`, `image_resize`, `image_crop`, `image_transform`, `image_convert`, and `image_batch_process`.
- **Integrated Verification Suites**: Added CPU tests, GPU tests, transform tests, batch tests, and end-to-end server integration tests verifying correct image outputs.

### Fixed
- Fixed all compiler and clippy warnings workspace-wide, including unused imports, redundant closures, manual div_ceil division implementations, and unnecessary mutability assertions.

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
