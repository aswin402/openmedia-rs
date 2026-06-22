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

## ⚡ What We Have Done (v0.0.1 Phase 0 Completion)
* **Multi-Crate Workspace Architecture**: Created an 8-crate workspace spanning core engine, image, video, SVG, animate, process, quality improvement, and MCP server crates.
* **Dyn Compatible Trait Architecture**: Annotated [DiffusionPipeline](crates/openmedia-image/src/lib.rs) and [FrameRenderer](crates/openmedia-video/src/lib.rs) with `#[async_trait]` to resolve compiler object safety blockers.
* **JSON-RPC Stdio Loop**: Fully wired [OpenMediaServer](crates/openmedia-mcp/src/lib.rs) with the `rmcp` SDK macros (`#[tool_router(server_handler)]` and `#[tool]`), running completely over stdio transport.
* **Tested & Sandbox Verified**: Built robust unit tests for configuration parsing and the `ping` tool using sandboxed directories. Tests pass successfully with `cargo test --workspace`.

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
* **`create_video`**: Renders frame-by-frame scenes defined using a JSON Scene DSL (HTML/CSS layout) and compiles them into H.264/VP9/AV1 videos using native FFmpeg pipeline hooks.
* **`create_slideshow`**: Compiles an image sequence with transitions (crossfade, slide, zoom, Ken Burns) and mixes background audio.
* **`add_audio_to_video`**: Fuses audio tracks into existing video containers.

### 3. SVG vector & Diagram Generation (`openmedia-svg`)
* **`create_svg`**: Fluent path, primitive, gradient, text, and filter definition to build raw optimized SVGs.
* **`create_chart`**: Generates bar, line, pie, scatter, radar, and gauge charts from raw JSON data.
* **`create_diagram`**: Renders auto-laid-out Flowcharts, UML sequence, architecture, and ER diagrams.
* **`create_icon`**: Accesses a built-in library of ~200 customizable icons.

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
