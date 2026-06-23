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

## ⚡ What We Have Done (v0.0.6 Self-Improvement System Completion)
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
* **19 MCP Tools Registered**: Integrated 6 animation tools, 8 video tools, and 5 quality self-improvement tools (`improve_score_image`, `improve_refine_prompt`, `improve_auto_refine`, `improve_feedback`, `improve_quality_report`) into the JSON-RPC Stdio router transport.
* **Tested & Sandbox Verified**: Built robust unit and integration tests verifying MCP tool bindings, schema generation, image encoding, video compilation, transitions, audio mixing, history database inserts, and prompt refinement. Tests pass cleanly with `cargo test --workspace`.

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
* **`create_svg`**: Fluent path, primitive, gradient, text, and filter definition to build raw optimized SVGs.
* **`create_chart`**: Generates bar, line, pie, scatter, radar, and gauge charts from raw JSON data.
* **`create_diagram`**: Renders auto-laid-out Flowcharts, UML sequence, architecture, and ER diagrams.
* **`create_icon`**: Accesses a built-in library of ~200 customizable icons.

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

---

## 💻 System Requirements

| Component | Minimum | Recommended |
| :--- | :--- | :--- |
| **Processor** | 4-Core CPU (with AVX2 support) | 8+ Core CPU (AVX-512 preferred) |
| **RAM** | 8 GB Total | 16 GB Total |
| **ROM (Storage)** | 10 GB (for binary & quantized models) | 50 GB (for multiple model variants) |
| **Runtime** | Rust 1.82+ (Zero Python/Node.js required) | Rust 1.82+ |
| **Optional Extras**| FFmpeg (for video containers), Chromium | FFmpeg, Vulkan SDK / CUDA Toolkit |

---

## ⚖️ Comparison to Other Tools

| Dimension | OpenMedia-RS | Python Diffusers Suite | Remotion (Node.js) |
| :--- | :--- | :--- | :--- |
| **Runtime Size** | ~100MB (Single binary) | Multi-gigabyte (pip/conda virtualenv) | Node modules + web browser package |
| **Inference Path** | Quantized GGUF / ONNX (wgpu & ORT) | PyTorch / Python Interpreter | N/A (Video rendering only) |
| **Hardware Fit** | Low-spec friendly (fits in 2-4GB RAM) | High-spec required (12GB+ VRAM typical)| CPU-bound rendering |
| **MCP Integration**| Built-in (native JSON-RPC over stdio) | Requires custom Python wrap scripts | Requires custom Node wrapper |
| **Scope** | Images + Video + SVG + Self-Improvement | Image Generation Only | Video Composition Only |

---

## 🛡️ License
Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
