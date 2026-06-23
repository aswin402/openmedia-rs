# OpenMedia-RS

<p align="center">
  <img src="assets/logo.svg" alt="OpenMedia-RS Logo" width="300px" />
</p>

> **A Rust-native Model Context Protocol (MCP) server suite that gives AI agents the power to generate, process, and refine images, videos, SVGs, and animations — entirely offline, on consumer hardware.**

Vibecoded by **Aswin** 🚀

---

## 🎨 Inspiration
The inspiration for OpenMedia-RS comes from:
1. **Remotion**: The revolutionary Node-based React-to-video framework. We wanted to build a Rust-pure, super lightweight, offline-first equivalent that compiles to a single binary.
2. **Native Offline AI Tools**: Eliminating Python virtual environments and complex system dependencies (like `diffusers`, `torch`, `npm`, `venv`) in favor of direct execution using efficient Rust-native ML backends.

---

## ⚡ What We Have Done (v0.0.9 SVG Canvas, Chart, and Icon Engine)
* **JSON-to-SVG Canvas Engine (`openmedia-svg`)**: Implemented deserialization schema rules for structured canvas shapes (Rects, Circles, Texts) to compile JSON arrays into standard SVG vector markup natively.
* **Custom Chart Engine (`create_chart`)**: Built vertical bars charting, smooth bezier curve plotting, and polar coordinates pie slice drawing (using arc paths) with configurable legend keys and titles.
* **Embedded Vector Icon Library (`create_icon`)**: Bundled 20 popular Feather/Lucide vector paths (e.g. home, user, settings, play, check) to scale and style on demand.
* **Mermaid Diagram Engine (`openmedia-svg`)**: Integrated native Mermaid diagram compilation using `mermaid-rs-renderer`, enabling offline flowchart, sequence diagram, and architecture graph generation natively in Rust without browser or network dependencies.
* **Mermaid MCP Tool (`diagram_generate_mermaid`)**: Registered the `diagram_generate_mermaid` tool to compile Mermaid string definitions, saving them to the output directory as SVG, PNG, JPEG, or WebP.
* **Model Auto-Download Experience (`openmedia-core`)**: Integrated registry streams to download CLIP text, CLIP vision, and LAION Aesthetic predictor models directly from the Hugging Face Hub, utilizing `reqwest` chunk-by-chunk streams.
* **Telemetry Progress Isolation**: Configured a thread-safe `StderrProgressReporter` to emit per-byte streaming progress metrics directly to `stderr`, preserving the integrity of standard output (`stdout`) for clean MCP JSON-RPC stdio transport communication.
* **Production Dockerization**: Set up a multi-stage production `Dockerfile` creating a lightweight Debian-slim container pre-configured with headless Chrome and FFmpeg runtime requirements.
* **CI/CD & GitHub Actions Release Automation**: Added `.github/workflows/release.yml` with a cross-compilation pipeline matrix building and publishing optimized assets for Linux (x86_64), macOS (x86_64, aarch64), and Windows (x86_64) on tag pushes.
* **Release Profile Optimizations**: Configured optimized release settings (`opt-level = 3`, LTO, codegen-units, panic abort, strip) inside the workspace [Cargo.toml](Cargo.toml) to minimize binary sizes and maximize speed.
* **Model Download MCP Tool**: Registered the `model_download` tool over the stdio interface, enabling AI agents to pull models on-demand.
* **Multi-Crate Workspace Architecture**: Created an 8-crate workspace spanning core engine, image, video, SVG, animate, process, quality improvement, and MCP server crates.
* **Dyn Compatible Trait Architecture**: Annotated [DiffusionPipeline](crates/openmedia-image/src/lib.rs) and [FrameRenderer](crates/openmedia-video/src/lib.rs) with `#[async_trait]` to resolve compiler object safety blockers.
* **JSON-RPC Stdio Loop**: Fully wired [OpenMediaServer](crates/openmedia-mcp/src/lib.rs) with the `rmcp` SDK macros (`#[tool_router(server_handler)]` and `#[tool]`), running completely over stdio transport.
* **Layout-to-Image Engines**:
  * **SVG Rasterizer (`rasterize_svg`)**: Powered by `resvg` + `tiny-skia` to convert SVG vector strings or files into PNG, JPEG, and WebP images on the CPU in $<20$ms.
  * **HTML/CSS Snapshotter (`html_to_image`)**: Integrates `chromiumoxide` to launch headless Chrome, render complex web templates, and capture screenshots.
* **SVG Animation Engine (`openmedia-animate`)**:
  * **SMIL XML Writer**: Generates `<animate>`, `<animateTransform>`, `<animateMotion>`, `<set>` elements with target `href` links and duration/delay triggers.
  * **CSS @keyframes Generator**: Handles keyframe percentages, target classes, iteration counts, fill modes, and animation shorthand.
  * **Path Morphing**: Parses, equalizes vertex counts using collapse logic, and interpolates between two path data strings.
  * **Sequencing Timeline**: Orchestrates sequential, parallel, and staggered animations by resolving absolute timings.
  * **Lottie Converter**: Imports Lottie JSON and translates shape layers/keyframes into animated SVGs.
* **Video Scene Composition Engine (`openmedia-video`)**:
  * **JSON-based Scene DSL**: Parses and validates multi-track visual element layers (Text, Image, Shape, SVG, Chart, Code, HTML) and layouts.
  * **Unified Compositor**: Layer-composites elements with alpha opacity/premultiplication correction on the CPU or handles complex layouts via headless Chromium rendering.
  * **Transitions Blender**: Implements frame-level crossfades, slides, and wipes between scene clips.
  * **Piped Video Encoder**: Encodes raw frame streams using an optimized FFmpeg pipe over stdin, outputting H.264/AAC MP4 files.
  * **Audio Track Mixer**: Dynamically mixes background narration and music tracks with configurable offsets, volumes, and fade timings.
* **Scoring & Self-Improvement System (`openmedia-improve`)**:
  * **CLIP Scorer**: Computes cosine similarity between image and text embeddings using `ort` (ONNX Runtime) session execution with Lanczos3 image scaling and BPE tokenization.
  * **Generation History Database**: Logs all tool outputs, request inputs, aesthetic scores, and generation parameters to a version-controlled SQLite database schema.
  * **Prompt Refiner**: Applies quality-boosting modifier tokens and default defect-reducing negative prompts based on quality score feedback.
  * **Iterative Refinement Loop**: Runs auto-refine feedback chains (generate → score → refine → rebuild) using fallback vector rendering.
* **24 MCP Tools Registered**: Integrated 6 animation tools, 8 video tools, 5 quality self-improvement tools, model download, Mermaid diagram generation, and the new SVG canvas/chart/icon builder tools.
* **Tested & Sandbox Verified**: Built robust unit and integration tests verifying MCP tool bindings, schema generation, image encoding, video compilation, transitions, audio mixing, history database inserts, prompt refinement, and registry model downloads. Tests pass cleanly with `cargo test --workspace`.
* **Mermaid Integration Verification**: Verification suite includes native Mermaid parser output tests and rasterized diagram image format tests.

---

## 🚀 What It Will Do (Features & Tools)

OpenMedia-RS exposes the following Model Context Protocol (MCP) tools directly to AI coding agents:

### 1. AI Image Generation (`openmedia-image`)
* **`generate_image`** (txt2img): Generates images using quantized models (like SDXL GGUF or FLUX Schnell).
* **`transform_image`** (img2img): Transforms an existing image guided by a text prompt and strength factor.
* **`inpaint_image`**: Fills white masked regions of an image guided by a text prompt.
* **`upscale_image`**: 2x or 4x super-resolution upscaling using Real-ESRGAN ONNX models.
* **`remove_background`**: Segment and isolate image foregrounds using U2-Net.

### 2. Video Composition (`openmedia-video`)
* **`html_to_image`** (Active 🟢): Renders HTML/CSS layout templates or files to PNG, JPEG, or WebP screenshots.
* **`video_create`** (Active 🟢): Renders frame-by-frame scenes defined using a JSON Scene DSL (HTML/CSS layout or SVG) and compiles them into H.264 videos using native FFmpeg pipeline hooks.
* **`video_preview`** (Active 🟢): Renders a preview frame at a specific timestamp.
* **`video_create_slideshow`** (Active 🟢): Compiles an image sequence with transitions (crossfade, slide, wipe) and mixes background audio.
* **`video_add_transition`** (Active 🟢): Adds scene transitions inside the DSL description.
* **`video_add_audio`** (Active 🟢): Fuses audio tracks into existing video containers or JSON descriptions.
* **`video_from_template`** (Active 🟢): Instantiates videos from prebuilt templates.
* **`video_extract_frames`** (Active 🟢): Extracts keyframe images from a video at specific time offsets.
* **`video_trim`** (Active 🟢): Trims a video file to a specific time range.

### 3. SVG Vector & Diagram Generation (`openmedia-svg`)
* **`rasterize_svg`** (Active 🟢): Converts SVG vector strings or files directly to PNG, JPEG, or WebP images.
* **`diagram_generate_mermaid`** (Active 🟢): Compiles flowcharts, sequence diagrams, and architecture graphs from Mermaid markdown text into SVG, PNG, JPEG, or WebP.
* **`create_svg`** (Active 🟢): Generate custom SVG layouts from a list of JSON-defined shapes and primitives.
* **`create_chart`** (Active 🟢): Generate customizable bar, line, and pie charts from raw JSON data.
* **`create_icon`** (Active 🟢): Retrieve styled vector interface icons from an embedded library.
* **`create_diagram`**: Renders auto-laid-out Flowcharts, UML sequence, architecture, and ER diagrams.

### 4. SVG Animation (`openmedia-animate`)
* **`animate_svg`** (Active 🟢): Apply animation presets (such as fade_in, spin, bounce, pulse, typewriter, draw_path) to SVG elements.
* **`animate_create_timeline`** (Active 🟢): Sequentially or concurrently coordinate animations of multiple elements.
* **`animate_morph_paths`** (Active 🟢): Interpolate morph frames between two path data strings.
* **`animate_generate_spinner`** (Active 🟢): Generate beautiful animated loading spinner SVGs.
* **`animate_from_lottie`** (Active 🟢): Import Lottie JSON and convert to an animated SVG.
* **`animate_to_lottie`** (Active 🟢): Export SVG to Lottie JSON.

### 5. Quality Evaluation & Self-Improvement (`openmedia-improve`)
* **`improve_score_image`** (Active 🟢): Score an image's alignment to a text prompt using CLIP and visual aesthetic predictor models.
* **`improve_refine_prompt`** (Active 🟢): Get prompt refinement modifications and recommendations based on quality parameters.
* **`improve_auto_refine`** (Active 🟢): Refine generated assets iteratively, evaluating intermediate output quality and logging historical chains.
* **`improve_feedback`** (Active 🟢): Log manual rating scores and artifact description comments on specific generations.
* **`improve_quality_report`** (Active 🟢): Fetch comprehensive quality database statistics and trends over time.

### 6. Model Management & Downloads (`openmedia-core`)
* **`model_download`** (Active 🟢): Download a specified model file (CLIP text/vision or Aesthetic predictor) from Hugging Face Hub with progress tracking directly to stderr.

---


## 💻 System Requirements

| Component | Minimum | Recommended |
| :--- | :--- | :--- |
| **Processor** | Dual-Core CPU (with AVX2 or ARM NEON support) | 4+ Core CPU (AVX-512 preferred) |
| **RAM** | 4 GB Total (fits under 1 GB for CLIP/Aesthetic scoring) | 8 GB+ Total |
| **ROM (Storage)** | < 1 GB (for optimized binary & base CLIP/Aesthetic models) | 10 GB+ (if downloading large diffusion models) |
| **Runtime** | Rust 1.82+ (Zero Python/Node.js required) | Rust 1.82+ |
| **Optional Extras**| FFmpeg (for video encoding/muxing), Chromium | FFmpeg, Vulkan SDK / CUDA Toolkit / Metal |

---

## ⚖️ Comparison to Other Tools

| Dimension | OpenMedia-RS | Python Diffusers Suite | Remotion (Node.js) |
| :--- | :--- | :--- | :--- |
| **Runtime Size** | **~60MB** (Stripped release binary) | 5GB+ (PyTorch, virtualenv, CUDA libraries) | 500MB+ (Node modules + package) |
| **Inference Path** | ONNX Runtime (`ort` for CLIP) & `wgpu` (compute) | PyTorch / Python Interpreter | N/A (Video rendering only) |
| **Hardware Fit** | **Ultra-low-spec friendly** (Runs on CPU with SIMD fallback or GPU; <1GB memory footprint) | High-spec required (8GB+ VRAM typical) | Moderate (CPU-bound rendering) |
| **MCP Integration**| **Native** (Built-in JSON-RPC stdio, 20+ tools) | Requires custom Python wrap scripts | Requires custom Node wrapper |
| **Scope** | Layout-to-Image + SVG Animation + Video Scene DSL + GPU Image Filters + Telemetry Scoring | AI Image Generation Only | Programmatic Video Composition Only |

---

## 🛡️ License
Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
