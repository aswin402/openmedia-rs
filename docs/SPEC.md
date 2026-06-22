# OpenMedia-RS — Project Specification

> **A Rust-native MCP server suite that gives AI agents the power to generate, process, and refine images, videos, SVGs, and animations — entirely offline, on consumer hardware.**

---

## Table of Contents

1. [Project Overview](#1-project-overview)
2. [Goals & Non-Goals](#2-goals--non-goals)
3. [System Requirements](#3-system-requirements)
4. [Feature Specification](#4-feature-specification)
5. [MCP Protocol Specification](#5-mcp-protocol-specification)
6. [Architecture Constraints](#6-architecture-constraints)
7. [Performance Targets](#7-performance-targets)
8. [Security](#8-security)
9. [Distribution](#9-distribution)
10. [Versioning & Roadmap](#10-versioning--roadmap)
11. [Dependencies](#11-dependencies)

---

## 1. Project Overview

### 1.1 Name & Identity

| Field       | Value                                                        |
| ----------- | ------------------------------------------------------------ |
| Name        | **OpenMedia-RS**                                             |
| Tagline     | *"Every AI agent deserves a creative studio."*               |
| Repository  | `openmedia-rs`                                               |
| License     | MIT OR Apache-2.0                                            |
| Language    | Rust (2024 edition)                                          |
| Binary Name | `openmedia`                                                  |

### 1.2 Vision

AI coding agents (Claude, Gemini, GPT-based agents, custom LLM pipelines) routinely need to generate visual media — diagrams for documentation, thumbnails for content, videos for demos, charts for data analysis. Today, this requires spinning up Python services, managing virtual environments, downloading multi-gigabyte model files, and orchestrating fragile subprocess pipelines.

**OpenMedia-RS** eliminates this entire stack. It is a **single Rust binary** that exposes a comprehensive media generation toolkit over the **Model Context Protocol (MCP)**, enabling any MCP-compatible AI agent to:

- Generate images from text prompts using quantized Stable Diffusion and FLUX models
- Create videos from HTML/CSS scene descriptions with professional transitions
- Build SVG graphics, charts, and diagrams programmatically
- Create animated SVGs with SMIL, CSS keyframes, and path morphing
- Process and transform existing images with GPU-accelerated pipelines
- Self-improve its outputs using CLIP scoring, aesthetic evaluation, and prompt refinement

### 1.3 Problem Statement

| Problem                            | Impact                                                                     |
| ---------------------------------- | -------------------------------------------------------------------------- |
| Python dependency hell             | Agents waste tokens debugging `pip install` failures and venv conflicts    |
| GPU memory requirements            | Most image gen requires 12GB+ VRAM, excluding integrated GPU users         |
| No unified media API               | Agents must learn different tools for images, videos, SVGs, charts         |
| No offline capability              | Cloud APIs add latency, cost, and privacy concerns                         |
| No quality feedback loop           | Agents generate blindly — no way to evaluate or refine outputs             |
| MCP gap                            | No existing MCP server covers the full media generation spectrum           |

### 1.4 Target Users

OpenMedia-RS is **not** designed for human end-users directly. Its primary consumers are:

1. **MCP-compatible AI agents** — Claude Desktop, Cline, Continue, Cursor, custom agent frameworks
2. **Agent orchestration systems** — LangChain, CrewAI, AutoGen, custom pipelines that speak MCP
3. **Developer toolchains** — CI/CD pipelines needing programmatic media generation
4. **Self-hosted AI workstations** — Privacy-conscious setups that cannot use cloud APIs

The human user's role is limited to:
- Installing and configuring the binary
- Choosing which models to download
- Setting hardware preferences (CPU/GPU, memory limits)

All creative decisions are made by the AI agent through MCP tool calls.

---

## 2. Goals & Non-Goals

### 2.1 Goals

| #  | Goal                        | Rationale                                                                      |
| -- | --------------------------- | ------------------------------------------------------------------------------ |
| G1 | **Fully offline**           | No network calls after initial model download. Air-gapped environments OK.     |
| G2 | **Low-spec friendly**       | Must run on 8GB RAM + integrated GPU. Quality scales with hardware.            |
| G3 | **MCP-first**               | Every feature exposed as an MCP tool. No REST API, no CLI subcommands.         |
| G4 | **Single binary**           | `cargo install openmedia` produces one binary. No sidecar processes.           |
| G5 | **Self-improving**          | CLIP/aesthetic scoring lets agents evaluate and refine outputs autonomously.   |
| G6 | **Rust-pure**               | No Python, Node.js, or JVM dependencies. Optional FFmpeg/Chrome for extras.    |
| G7 | **Async-native**            | Built on tokio. Non-blocking I/O. Concurrent tool calls supported.             |
| G8 | **Quantized models**        | GGUF and ONNX quantized models to fit in 4-8GB RAM.                           |
| G9 | **Progressive quality**     | Auto-detect hardware and select best quality level within constraints.          |
| G10| **Comprehensive media**     | Images, videos, SVGs, animated SVGs, charts, diagrams — one server.            |

### 2.2 Non-Goals

| #   | Non-Goal                      | Reason                                                                      |
| --- | ----------------------------- | --------------------------------------------------------------------------- |
| NG1 | **GUI / Desktop app**         | Agents don't need GUIs. Terminal + MCP is the interface.                    |
| NG2 | **Web service / REST API**    | MCP stdio transport only. No HTTP server, no authentication complexity.     |
| NG3 | **Training / fine-tuning**    | Inference only. Training is a different problem domain.                     |
| NG4 | **Real-time streaming**       | Batch generation, not live video/audio streaming.                           |
| NG5 | **Mobile support**            | Desktop Linux/macOS/Windows only.                                           |
| NG6 | **Cloud deployment**          | Designed for local execution. Docker is for reproducibility, not scaling.   |
| NG7 | **Plugin system**             | Fixed feature set. Extensibility through MCP tool composition.              |
| NG8 | **Prompt engineering UI**     | Agents construct prompts programmatically. No prompt galleries.             |
| NG9 | **Model marketplace**         | Users download models manually or via documented URLs.                      |
| NG10| **Multi-user / auth**         | Single-user, single-agent (or multiplexed via MCP) only.                    |

---

## 3. System Requirements

### 3.1 Minimum Requirements

| Component        | Minimum                                     | Notes                                          |
| ---------------- | ------------------------------------------- | ---------------------------------------------- |
| **OS**           | Linux x86_64 (glibc 2.31+)                 | Ubuntu 20.04+, Fedora 33+, Arch              |
| **CPU**          | 4 cores, AVX2 support                       | Intel Haswell (2013) / AMD Excavator (2015)+   |
| **RAM**          | 8 GB total, 4 GB available                  | Quantized models fit in 2-4 GB                 |
| **GPU**          | Integrated (Intel UHD 630 / AMD Vega)       | wgpu for image processing only                 |
| **VRAM**         | 1 GB shared                                 | CPU-only diffusion if VRAM < 2 GB              |
| **Disk**         | 10 GB free                                  | ~2 GB binary + models, ~8 GB model storage     |
| **Rust**         | 1.82.0+                                     | 2024 edition features required                 |

### 3.2 Recommended Requirements

| Component        | Recommended                                 | Notes                                          |
| ---------------- | ------------------------------------------- | ---------------------------------------------- |
| **OS**           | Linux x86_64, macOS 14+ (ARM), Windows 11   | Full platform support                          |
| **CPU**          | 8+ cores, AVX-512 preferred                 | Parallel frame rendering                       |
| **RAM**          | 16 GB total, 8 GB available                 | Comfortable headroom for SDXL                  |
| **GPU**          | NVIDIA RTX 3060+ / AMD RX 6700+            | Vulkan 1.3 support for wgpu shaders            |
| **VRAM**         | 6 GB dedicated                              | SDXL FP16 fits comfortably                     |
| **Disk**         | 50 GB free                                  | Multiple model variants                        |

### 3.3 Optional Dependencies

| Dependency             | Version   | Required For                                      | Fallback                        |
| ---------------------- | --------- | ------------------------------------------------- | ------------------------------- |
| **FFmpeg**             | 6.0+      | H.264/VP9/AV1 video encoding, audio mixing        | Native uncompressed AVI output  |
| **Chrome / Chromium**  | 120+      | Tier-3 HTML rendering (complex CSS, JS, web fonts) | Tier-1/2 native rendering       |
| **Vulkan SDK**         | 1.3+      | GPU-accelerated image processing via wgpu          | CPU fallback with imageproc     |
| **CUDA Toolkit**       | 12.0+     | NVIDIA GPU acceleration for Candle backend         | CPU-only inference              |

### 3.4 Model Storage

Models are stored in a configurable directory (default: `~/.openmedia/models/`). The directory structure:

```
~/.openmedia/
├── models/
│   ├── diffusion/
│   │   ├── sd-1.5-q8_0.gguf          (~1.7 GB)
│   │   ├── sd-2.1-q8_0.gguf          (~2.0 GB)
│   │   ├── sdxl-base-q5_0.gguf       (~3.5 GB)
│   │   ├── sdxl-turbo-q8_0.gguf      (~3.8 GB)
│   │   ├── flux1-schnell-q4_0.gguf   (~4.2 GB)
│   │   └── flux1-dev-q4_0.gguf       (~6.8 GB)
│   ├── upscale/
│   │   ├── realesrgan-x4.onnx        (~64 MB)
│   │   └── realesrgan-x2.onnx        (~64 MB)
│   ├── segmentation/
│   │   └── u2net.onnx                 (~176 MB)
│   ├── clip/
│   │   ├── clip-vit-b32.onnx         (~350 MB)
│   │   └── aesthetic-predictor.onnx   (~25 MB)
│   └── vae/
│       ├── sd-vae-ft-mse.safetensors (~335 MB)
│       └── sdxl-vae.safetensors      (~335 MB)
├── config.toml
├── history.db
└── output/
    ├── images/
    ├── videos/
    └── svgs/
```

---

## 4. Feature Specification

### 4.1 AI Image Generation

#### 4.1.1 Capabilities

| Capability              | Description                                                                 |
| ----------------------- | --------------------------------------------------------------------------- |
| **txt2img**             | Generate images from text prompts                                           |
| **img2img**             | Transform existing images guided by text prompts                            |
| **inpaint**             | Fill masked regions of images guided by text prompts                         |
| **upscale**             | Super-resolution upscaling (2x, 4x) via Real-ESRGAN                        |
| **background_remove**   | Remove image backgrounds using U2-Net segmentation                          |

#### 4.1.2 Supported Models

| Model              | Quantization | VRAM (est.) | Quality    | Speed      | Resolution     |
| ------------------- | ------------ | ----------- | ---------- | ---------- | -------------- |
| SD 1.5              | Q8_0 GGUF   | 2 GB        | Good       | Fast       | 512×512        |
| SD 2.1              | Q8_0 GGUF   | 2.5 GB      | Better     | Fast       | 768×768        |
| SDXL Base           | Q5_0 GGUF   | 4 GB        | Excellent  | Medium     | 1024×1024      |
| SDXL Turbo          | Q8_0 GGUF   | 4 GB        | Very Good  | Very Fast  | 512×512        |
| FLUX.1 Schnell      | Q4_0 GGUF   | 5 GB        | Excellent  | Fast       | 512–1024       |
| FLUX.1 Dev          | Q4_0 GGUF   | 8 GB        | Best       | Slow       | 512–1024       |

#### 4.1.3 Inference Backends

| Backend           | Crate           | Hardware          | Models Supported         | Priority |
| ----------------- | --------------- | ----------------- | ------------------------ | -------- |
| **Candle**        | `candle-core`   | CPU, CUDA, Metal  | SD 1.5, 2.1, SDXL       | Primary  |
| **diffusion_rs**  | `diffusion_rs`  | CPU (GGUF)        | SD 1.5, 2.1, SDXL, FLUX | Primary  |
| **ORT (ONNX)**    | `ort`           | CPU, CUDA, DML    | All (ONNX exports)       | Fallback |

Backend selection logic:
1. If CUDA available and model has Candle support → use Candle (CUDA)
2. If Metal available (macOS) and model has Candle support → use Candle (Metal)
3. If GGUF model available → use diffusion_rs
4. If ONNX model available → use ORT
5. Fallback: diffusion_rs on CPU with Q4_0 quantization

#### 4.1.4 txt2img Parameters

| Parameter            | Type       | Required | Default        | Range / Constraints                       | Description                              |
| -------------------- | ---------- | -------- | -------------- | ----------------------------------------- | ---------------------------------------- |
| `prompt`             | `string`   | ✅       | —              | 1–2000 chars                              | Text description of desired image        |
| `negative_prompt`    | `string`   | ❌       | `""`           | 0–2000 chars                              | What to avoid in the image               |
| `model`              | `string`   | ❌       | `"auto"`       | Model ID or `"auto"`                      | Model to use; auto selects by hardware   |
| `width`              | `integer`  | ❌       | `512`          | 256–2048, must be divisible by 8          | Output width in pixels                   |
| `height`             | `integer`  | ❌       | `512`          | 256–2048, must be divisible by 8          | Output height in pixels                  |
| `steps`              | `integer`  | ❌       | `20`           | 1–150                                     | Number of denoising steps                |
| `cfg_scale`          | `float`    | ❌       | `7.5`          | 1.0–30.0                                  | Classifier-free guidance scale           |
| `seed`               | `integer`  | ❌       | random         | 0–2^64                                    | RNG seed for reproducibility             |
| `scheduler`          | `string`   | ❌       | `"dpm++"`      | ddim, dpm++, euler, euler_a, lcm          | Noise scheduler algorithm                |
| `batch_size`         | `integer`  | ❌       | `1`            | 1–4                                       | Number of images to generate             |
| `output_format`      | `string`   | ❌       | `"png"`        | png, jpeg, webp                           | Output image format                      |
| `output_quality`     | `integer`  | ❌       | `95`           | 1–100                                     | JPEG/WebP compression quality            |
| `clip_skip`          | `integer`  | ❌       | `1`            | 1–4                                       | CLIP encoder layers to skip              |
| `auto_refine`        | `boolean`  | ❌       | `false`        | —                                         | Enable self-improvement refinement loop  |
| `max_refine_rounds`  | `integer`  | ❌       | `3`            | 1–10                                      | Max refinement iterations if auto_refine |

#### 4.1.5 img2img Parameters

Inherits all `txt2img` parameters plus:

| Parameter            | Type       | Required | Default        | Range / Constraints                       | Description                              |
| -------------------- | ---------- | -------- | -------------- | ----------------------------------------- | ---------------------------------------- |
| `input_image`        | `string`   | ✅       | —              | Valid file path or base64                  | Source image to transform                |
| `strength`           | `float`    | ❌       | `0.75`         | 0.0–1.0                                   | How much to transform (0=none, 1=full)   |

#### 4.1.6 Inpaint Parameters

Inherits all `txt2img` parameters plus:

| Parameter            | Type       | Required | Default        | Range / Constraints                       | Description                              |
| -------------------- | ---------- | -------- | -------------- | ----------------------------------------- | ---------------------------------------- |
| `input_image`        | `string`   | ✅       | —              | Valid file path or base64                  | Source image with region to fill          |
| `mask_image`         | `string`   | ✅       | —              | Valid file path or base64                  | Mask (white = fill, black = keep)        |
| `mask_blur`          | `integer`  | ❌       | `4`            | 0–64                                      | Gaussian blur radius on mask edges       |
| `inpaint_full`       | `boolean`  | ❌       | `true`         | —                                         | Process full image vs. masked region only|

#### 4.1.7 Upscale Parameters

| Parameter            | Type       | Required | Default        | Range / Constraints                       | Description                              |
| -------------------- | ---------- | -------- | -------------- | ----------------------------------------- | ---------------------------------------- |
| `input_image`        | `string`   | ✅       | —              | Valid file path or base64                  | Image to upscale                         |
| `scale`              | `integer`  | ❌       | `4`            | 2, 4                                      | Upscaling factor                         |
| `model`              | `string`   | ❌       | `"realesrgan"` | realesrgan                                | Upscaling model                          |
| `tile_size`          | `integer`  | ❌       | `512`          | 128–1024                                  | Tile size for memory-efficient processing|
| `output_format`      | `string`   | ❌       | `"png"`        | png, jpeg, webp                           | Output format                            |

#### 4.1.8 Background Removal Parameters

| Parameter            | Type       | Required | Default        | Range / Constraints                       | Description                              |
| -------------------- | ---------- | -------- | -------------- | ----------------------------------------- | ---------------------------------------- |
| `input_image`        | `string`   | ✅       | —              | Valid file path or base64                  | Image for background removal             |
| `threshold`          | `float`    | ❌       | `0.5`          | 0.0–1.0                                   | Segmentation threshold                   |
| `feather`            | `integer`  | ❌       | `2`            | 0–20                                      | Edge feathering in pixels                |
| `output_format`      | `string`   | ❌       | `"png"`        | png, webp                                 | Must support alpha channel               |
| `background_color`   | `string`   | ❌       | `null`         | Hex color or null for transparent         | Replacement background color             |

---

### 4.2 Video Generation

#### 4.2.1 Architecture: 3-Tier HTML Rendering

Video generation is based on rendering HTML+CSS scene descriptions frame-by-frame. Three rendering tiers provide a quality/dependency tradeoff:

| Tier   | Renderer               | Engine                  | CSS Support  | JS Support | Web Fonts | Dependencies      |
| ------ | ---------------------- | ----------------------- | ------------ | ---------- | --------- | ----------------- |
| Tier 1 | **SvgRenderer**        | resvg + tiny-skia       | None         | No         | No        | None (pure Rust)  |
| Tier 2 | **NativeRenderer**     | hyper_render (custom)   | Subset       | No         | Limited   | None (pure Rust)  |
| Tier 3 | **BrowserRenderer**    | Headless Chrome (CDP)   | Full         | Yes        | Yes       | Chrome/Chromium   |

Renderer selection logic:
1. Parse scene elements. If any element requires JS or full CSS → Tier 3
2. If elements use supported CSS subset (flexbox, grid, transforms, animations) → Tier 2
3. If elements are SVG-expressible (shapes, text, images) → Tier 1
4. Agent can override with explicit `renderer` parameter

#### 4.2.2 Scene JSON DSL

Videos are defined as a JSON scene description with the following structure:

```json
{
  "width": 1920,
  "height": 1080,
  "fps": 30,
  "duration": 10.0,
  "background": "#1a1a2e",
  "scenes": [
    {
      "id": "intro",
      "start": 0.0,
      "end": 5.0,
      "elements": [
        {
          "type": "text",
          "content": "Hello World",
          "style": {
            "font_family": "Inter",
            "font_size": 72,
            "color": "#ffffff",
            "text_align": "center"
          },
          "position": { "x": "50%", "y": "50%" },
          "anchor": "center",
          "timeline": {
            "keyframes": [
              { "time": 0.0, "opacity": 0.0, "scale": 0.8 },
              { "time": 0.5, "opacity": 1.0, "scale": 1.0, "easing": "ease_out_cubic" },
              { "time": 4.5, "opacity": 1.0 },
              { "time": 5.0, "opacity": 0.0 }
            ]
          }
        },
        {
          "type": "image",
          "src": "/path/to/logo.png",
          "position": { "x": "50%", "y": "30%" },
          "size": { "width": 200, "height": 200 },
          "timeline": {
            "keyframes": [
              { "time": 0.5, "opacity": 0.0, "y": "-20px" },
              { "time": 1.5, "opacity": 1.0, "y": "0px", "easing": "ease_out_bounce" }
            ]
          }
        },
        {
          "type": "shape",
          "shape": "rounded_rect",
          "size": { "width": "80%", "height": 4 },
          "position": { "x": "50%", "y": "65%" },
          "style": {
            "fill": "linear-gradient(90deg, #e94560, #0f3460)",
            "border_radius": 2
          },
          "timeline": {
            "keyframes": [
              { "time": 1.0, "scale_x": 0.0 },
              { "time": 2.0, "scale_x": 1.0, "easing": "ease_out_expo" }
            ]
          }
        }
      ]
    }
  ],
  "transitions": [
    {
      "from": "intro",
      "to": "content",
      "type": "crossfade",
      "duration": 0.5
    }
  ],
  "audio": {
    "tracks": [
      {
        "src": "/path/to/music.mp3",
        "start": 0.0,
        "volume": 0.8,
        "fade_in": 1.0,
        "fade_out": 2.0
      }
    ]
  }
}
```

#### 4.2.3 Scene Element Types

| Element Type    | Properties                                                    | Tier 1 | Tier 2 | Tier 3 |
| --------------- | ------------------------------------------------------------- | ------ | ------ | ------ |
| `text`          | content, font, size, color, align, line_height, letter_spacing| ✅     | ✅     | ✅     |
| `image`         | src (file/base64), size, fit (cover/contain/fill)             | ✅     | ✅     | ✅     |
| `shape`         | rect, rounded_rect, circle, ellipse, polygon, line            | ✅     | ✅     | ✅     |
| `svg`           | inline SVG or file reference                                  | ✅     | ✅     | ✅     |
| `group`         | container for nested elements, transforms                     | ✅     | ✅     | ✅     |
| `html`          | raw HTML string (rendered as-is)                              | ❌     | Partial| ✅     |
| `code`          | syntax-highlighted code block                                 | ❌     | ✅     | ✅     |
| `chart`         | embedded chart (delegates to openmedia-svg)                   | ✅     | ✅     | ✅     |
| `video_embed`   | embedded video/gif overlay                                    | ❌     | ❌     | ✅     |

#### 4.2.4 Transition Types

| Transition       | Parameters                        | Description                                   |
| ---------------- | --------------------------------- | --------------------------------------------- |
| `crossfade`      | duration                          | Alpha blend between scenes                    |
| `slide_left`     | duration, easing                  | New scene slides in from right                |
| `slide_right`    | duration, easing                  | New scene slides in from left                 |
| `slide_up`       | duration, easing                  | New scene slides in from bottom               |
| `slide_down`     | duration, easing                  | New scene slides in from top                  |
| `zoom_in`        | duration, easing, focus_point     | Zoom into focus point, reveal new scene       |
| `zoom_out`       | duration, easing                  | Zoom out from current scene                   |
| `wipe_left`      | duration, easing                  | Wipe reveal from right to left                |
| `wipe_right`     | duration, easing                  | Wipe reveal from left to right                |
| `wipe_down`      | duration, easing                  | Wipe reveal from top to bottom                |
| `wipe_up`        | duration, easing                  | Wipe reveal from bottom to top                |
| `dissolve`       | duration, grain_size              | Pixelated dissolve                            |
| `iris_in`        | duration, center                  | Circular iris open                            |
| `iris_out`       | duration, center                  | Circular iris close                           |
| `none`           | —                                 | Hard cut                                      |

#### 4.2.5 Image Slideshow Mode

A simplified mode for creating videos from a sequence of images:

| Parameter            | Type       | Required | Default     | Description                              |
| -------------------- | ---------- | -------- | ----------- | ---------------------------------------- |
| `images`             | `string[]` | ✅       | —           | Ordered list of image file paths         |
| `duration_per_image` | `float`    | ❌       | `3.0`       | Seconds each image is displayed          |
| `transition`         | `string`   | ❌       | `crossfade` | Transition between images                |
| `transition_duration`| `float`    | ❌       | `0.5`       | Transition duration in seconds           |
| `ken_burns`          | `boolean`  | ❌       | `false`     | Enable slow zoom/pan effect              |
| `audio`              | `string`   | ❌       | `null`      | Background audio file path               |
| `output_resolution`  | `string`   | ❌       | `1080p`     | 720p, 1080p, 1440p, 4k                  |

#### 4.2.6 Video Encoding

| Codec    | Container | Quality Presets           | Hardware Accel | FFmpeg Required |
| -------- | --------- | ------------------------- | -------------- | --------------- |
| H.264    | MP4       | fast, balanced, quality   | NVENC, QSV     | ✅              |
| VP9      | WebM      | fast, balanced, quality   | —              | ✅              |
| AV1      | MP4/WebM  | fast, balanced, quality   | —              | ✅              |
| Raw AVI  | AVI       | uncompressed              | —              | ❌              |

#### 4.2.7 Resolution Presets

| Preset   | Resolution  | Typical Use                |
| -------- | ----------- | -------------------------- |
| `480p`   | 854×480     | Quick previews             |
| `720p`   | 1280×720    | Standard web               |
| `1080p`  | 1920×1080   | Full HD (default)          |
| `1440p`  | 2560×1440   | QHD                        |
| `4k`     | 3840×2160   | Ultra HD                   |

---

### 4.3 SVG Generation

#### 4.3.1 Builder API

A fluent, chainable API for constructing SVGs programmatically:

- **Primitives**: `rect`, `circle`, `ellipse`, `line`, `polyline`, `polygon`, `path`
- **Text**: `text` with font family, size, weight, anchor, decoration, transforms
- **Containers**: `group` (g), `defs`, `symbol`, `use`
- **Gradients**: `linear_gradient`, `radial_gradient` with arbitrary stops
- **Filters**: `blur`, `drop_shadow`, `glow`, `color_matrix`
- **Clipping**: `clip_path`, `mask`
- **Transforms**: `translate`, `rotate`, `scale`, `skew_x`, `skew_y`, `matrix`
- **Metadata**: `title`, `desc`, `viewBox`, `preserveAspectRatio`

#### 4.3.2 Icon Generation

Pre-built icon set with ~200 common icons (arrows, checkmarks, social media, file types, etc.) rendered as optimized SVG paths. Icons support:

- Custom size (16–512px)
- Custom color / gradient fill
- Custom stroke width and color
- Rounded / square variants
- Badge overlays (notification dots, counts)

#### 4.3.3 Chart Types

| Chart Type      | Data Format                                    | Customizations                                           |
| --------------- | ---------------------------------------------- | -------------------------------------------------------- |
| **Bar**         | `{labels: [], datasets: [{data: [], color}]}`  | Horizontal/vertical, stacked, grouped, rounded corners   |
| **Line**        | `{labels: [], datasets: [{data: [], color}]}`  | Curved/straight, filled area, data points, multi-axis    |
| **Pie**         | `{segments: [{value, label, color}]}`          | Donut variant, exploded slices, labels, percentages      |
| **Scatter**     | `{points: [{x, y, size?, color?}]}`            | Bubble variant, trend lines, quadrants                   |
| **Radar**       | `{axes: [], datasets: [{data: [], color}]}`    | Filled/outline, custom axis labels, multiple datasets    |
| **Heatmap**     | `{rows: [], cols: [], values: [[]]}`           | Color scales, cell labels, clustering                    |
| **Treemap**     | `{nodes: [{name, value, children?}]}`          | Squarified layout, nested labels, color by depth         |
| **Gauge**       | `{value, min, max, thresholds: []}`            | Arc gauge, linear gauge, custom ticks                    |

All charts support:
- Custom dimensions, padding, margins
- Title, subtitle, legend (position, style)
- Axis labels, tick marks, grid lines
- Responsive viewBox scaling
- Dark / light theme presets
- Animation-ready class annotations

#### 4.3.4 Diagram Types

| Diagram Type     | Input Format                                       | Output                                          |
| ---------------- | -------------------------------------------------- | ----------------------------------------------- |
| **Flowchart**    | Nodes + edges with labels and conditions           | Auto-laid-out directed graph                     |
| **Sequence**     | Actors + messages with timing                      | UML-style sequence diagram                       |
| **Architecture** | Components + connections with protocols             | System architecture boxes and arrows             |
| **ER Diagram**   | Entities + attributes + relationships              | Entity-relationship diagram                      |
| **Tree**         | Hierarchical node structure                        | Top-down or left-right tree layout               |
| **Mind Map**     | Central topic + branches                           | Radial mind map layout                           |
| **Gantt**        | Tasks + durations + dependencies                   | Gantt chart with timeline                        |
| **Timeline**     | Events + dates                                     | Horizontal or vertical timeline                  |
| **Network**      | Nodes + edges with weights                         | Force-directed graph layout                      |

#### 4.3.5 SVG Optimization

Post-processing pipeline:
1. Remove unnecessary whitespace and comments
2. Collapse redundant groups
3. Simplify path data (reduce decimal precision)
4. Remove unused defs/gradients
5. Minify attribute values
6. Optional: convert text to paths for font independence
7. Output size reduction target: 40-60%

#### 4.3.6 SVG Rasterization

Convert SVGs to raster formats using resvg:

| Parameter        | Type      | Default  | Description                              |
| ---------------- | --------- | -------- | ---------------------------------------- |
| `width`          | integer   | SVG's    | Output width (maintains aspect ratio)    |
| `height`         | integer   | SVG's    | Output height                            |
| `dpi`            | integer   | 96       | Dots per inch for rendering              |
| `background`     | string    | none     | Background color (transparent default)   |
| `format`         | string    | png      | png, jpeg, webp                          |

---

### 4.4 Animated SVG Generation

#### 4.4.1 SMIL Animations

Supported SMIL animation elements:
- `<animate>` — attribute animation (fill, opacity, transform values)
- `<animateTransform>` — translate, rotate, scale, skewX, skewY
- `<animateMotion>` — motion along a path with `<mpath>`
- `<set>` — discrete attribute setting at a time point

#### 4.4.2 CSS Keyframe Animations

Full `@keyframes` support with:
- Named keyframe sequences
- Percentage-based timing (0%, 25%, 50%, 100%)
- `animation-*` properties: duration, timing-function, delay, iteration-count, direction, fill-mode
- Multiple animations per element
- Custom CSS properties (variables) for dynamic theming

#### 4.4.3 Path Morphing

Morph between two SVG paths with matched point counts:
- Automatic point interpolation for paths with different segment counts
- Support for cubic bezier, quadratic bezier, arc, and line segments
- Configurable interpolation method: linear, cubic, spring
- Path normalization (clockwise winding, start point alignment)

#### 4.4.4 Animation Presets

| Preset            | Type        | Description                                      | Parameters           |
| ----------------- | ----------- | ------------------------------------------------ | -------------------- |
| `fade_in`         | SMIL        | Opacity 0→1                                      | duration, delay      |
| `fade_out`        | SMIL        | Opacity 1→0                                      | duration, delay      |
| `slide_in_left`   | SMIL        | Translate from left edge                         | duration, distance   |
| `slide_in_right`  | SMIL        | Translate from right edge                        | duration, distance   |
| `slide_in_up`     | SMIL        | Translate from bottom edge                       | duration, distance   |
| `slide_in_down`   | SMIL        | Translate from top edge                          | duration, distance   |
| `bounce`          | CSS         | Bouncing vertical motion                         | duration, height     |
| `pulse`           | CSS         | Scale oscillation                                | duration, intensity  |
| `spin`            | CSS         | 360° rotation                                    | duration, direction  |
| `shake`           | CSS         | Horizontal oscillation                           | duration, intensity  |
| `wobble`          | CSS         | Rotational oscillation                           | duration, angle      |
| `typewriter`      | CSS+JS      | Character-by-character text reveal               | speed, cursor        |
| `draw_path`       | SMIL        | Stroke-dashoffset path drawing                   | duration, direction  |
| `morph`           | SMIL        | Path morphing between shapes                     | duration, easing     |
| `gradient_shift`  | CSS         | Animated gradient color cycling                  | duration, colors     |
| `parallax_scroll` | CSS         | Multi-layer parallax on scroll (data-speed)      | layers, speed_range  |
| `stagger`         | CSS         | Sequential delay across child elements           | base_delay, offset   |

#### 4.4.5 Easing Functions

| Easing              | CSS Equivalent                     | Mathematical Definition             |
| ------------------- | ---------------------------------- | ----------------------------------- |
| `linear`            | `linear`                           | `t`                                 |
| `ease_in_quad`      | `cubic-bezier(0.55,0.085,0.68,0.53)` | `t²`                             |
| `ease_out_quad`     | `cubic-bezier(0.25,0.46,0.45,0.94)` | `t(2-t)`                          |
| `ease_in_out_quad`  | `cubic-bezier(0.455,0.03,0.515,0.955)` | piecewise quadratic             |
| `ease_in_cubic`     | —                                  | `t³`                                |
| `ease_out_cubic`    | —                                  | `(t-1)³ + 1`                       |
| `ease_in_out_cubic` | —                                  | piecewise cubic                     |
| `ease_in_expo`      | —                                  | `2^(10(t-1))`                       |
| `ease_out_expo`     | —                                  | `-2^(-10t) + 1`                     |
| `ease_in_out_expo`  | —                                  | piecewise exponential               |
| `ease_out_bounce`   | —                                  | piecewise polynomial bounce         |
| `ease_in_back`      | —                                  | `t² × ((s+1)t - s)` with s=1.70158 |
| `ease_out_back`     | —                                  | overshoot and settle                |
| `ease_in_elastic`   | —                                  | damped sine wave (in)               |
| `ease_out_elastic`  | —                                  | damped sine wave (out)              |
| `spring`            | —                                  | physics-based spring simulation     |
| `custom_bezier`     | `cubic-bezier(a,b,c,d)`           | user-defined cubic bezier           |

#### 4.4.6 Lottie Conversion

Convert animated SVGs to Lottie JSON format for use in web and mobile applications:
- Map SMIL/CSS animations to Lottie keyframes
- Support shape layers, solid layers, and image layers
- Export as `.json` (Lottie) or `.lottie` (compressed)
- Subset of features supported (no filters, no clip-path animations)

---

### 4.5 Image Processing

#### 4.5.1 GPU Pipeline (wgpu + WGSL)

All image processing operations run through a GPU compute pipeline when available:

- **Backend**: wgpu with Vulkan (Linux), Metal (macOS), DX12 (Windows)
- **Shaders**: WGSL compute shaders for each operation
- **Batch processing**: Multiple operations fused into a single GPU dispatch
- **Memory management**: Texture atlas for intermediate results, automatic fallback on OOM

#### 4.5.2 CPU Fallback

When GPU is unavailable or operation exceeds VRAM:
- `image` crate for format I/O (decode/encode)
- `imageproc` crate for geometric and color operations
- Rayon for parallel pixel processing on multi-core CPUs

#### 4.5.3 Operations

| Operation           | GPU Shader    | CPU Fallback  | Parameters                                           |
| ------------------- | ------------- | ------------- | ---------------------------------------------------- |
| **Gaussian Blur**   | `blur.wgsl`   | imageproc     | radius (0.1–100.0), sigma (auto or custom)           |
| **Box Blur**        | `blur.wgsl`   | manual        | radius (1–50)                                        |
| **Sharpen**         | `sharpen.wgsl`| imageproc     | amount (0.0–5.0), radius (0.5–10.0), threshold (0–255)|
| **Unsharp Mask**    | `sharpen.wgsl`| imageproc     | amount, radius, threshold                            |
| **Brightness**      | `color.wgsl`  | image         | value (-100 to 100)                                  |
| **Contrast**        | `color.wgsl`  | image         | value (-100 to 100)                                  |
| **Saturation**      | `color.wgsl`  | manual        | value (-100 to 100)                                  |
| **Hue Rotate**      | `color.wgsl`  | manual        | degrees (0–360)                                      |
| **Grayscale**       | `color.wgsl`  | image         | —                                                    |
| **Sepia**           | `color.wgsl`  | manual        | intensity (0.0–1.0)                                  |
| **Invert**          | `color.wgsl`  | image         | —                                                    |
| **Threshold**       | `color.wgsl`  | imageproc     | value (0–255)                                        |
| **Color Matrix**    | `color.wgsl`  | manual        | 5×4 matrix                                           |
| **Composite**       | `blend.wgsl`  | manual        | blend_mode, opacity                                  |
| **Resize**          | `resize.wgsl` | image         | width, height, method (nearest/bilinear/lanczos3)    |
| **Crop**            | memory copy   | image         | x, y, width, height                                  |
| **Rotate**          | `transform.wgsl` | imageproc  | angle (any), interpolation                           |
| **Flip**            | `transform.wgsl` | image      | horizontal, vertical                                 |
| **Pad**             | memory copy   | manual        | top, right, bottom, left, color                      |
| **Format Convert**  | N/A           | image         | target format, quality                               |

#### 4.5.4 Blend Modes

| Mode            | Formula                                          |
| --------------- | ------------------------------------------------ |
| `normal`        | `src × alpha + dst × (1 - alpha)`                |
| `multiply`      | `src × dst`                                      |
| `screen`        | `1 - (1 - src)(1 - dst)`                         |
| `overlay`       | conditional multiply/screen                      |
| `darken`        | `min(src, dst)`                                  |
| `lighten`       | `max(src, dst)`                                  |
| `color_dodge`   | `dst / (1 - src)`                                |
| `color_burn`    | `1 - (1 - dst) / src`                            |
| `hard_light`    | conditional multiply/screen (swapped)            |
| `soft_light`    | Pegtop formula                                   |
| `difference`    | `|src - dst|`                                    |
| `exclusion`     | `src + dst - 2 × src × dst`                      |

---

### 4.6 Self-Improvement System

#### 4.6.1 CLIP Scoring

Evaluate text-image alignment using CLIP ViT-B/32:

- Input: generated image + original prompt
- Output: cosine similarity score (0.0–1.0)
- Threshold: scores below 0.25 trigger auto-refine suggestion
- Model: ONNX-quantized CLIP (~350 MB)

#### 4.6.2 Aesthetic Scoring

Predict human aesthetic preference:

- Input: generated image
- Output: aesthetic score (1.0–10.0, higher = more aesthetic)
- Model: Linear probe on CLIP features (~25 MB)
- Threshold: scores below 4.5 trigger refinement suggestion

#### 4.6.3 Generation History (SQLite)

All generations are logged to a local SQLite database:

```sql
CREATE TABLE generations (
    id              TEXT PRIMARY KEY,      -- UUID v7
    created_at      TEXT NOT NULL,         -- ISO 8601
    tool_name       TEXT NOT NULL,         -- MCP tool that created this
    request_params  TEXT NOT NULL,         -- JSON of input parameters
    output_path     TEXT NOT NULL,         -- Path to generated file
    output_format   TEXT NOT NULL,         -- png, mp4, svg, etc.
    output_size     INTEGER NOT NULL,      -- File size in bytes
    width           INTEGER,              -- Image/video width
    height          INTEGER,              -- Image/video height
    duration        REAL,                 -- Video/animation duration
    model_used      TEXT,                 -- Model identifier
    backend_used    TEXT,                 -- candle, diffusion_rs, ort
    generation_time REAL NOT NULL,        -- Wall-clock seconds
    clip_score      REAL,                -- CLIP alignment score
    aesthetic_score REAL,                -- Aesthetic quality score
    refined_from    TEXT,                 -- Parent generation ID
    refinement_round INTEGER DEFAULT 0,  -- Refinement iteration
    metadata        TEXT                  -- Additional JSON metadata
);

CREATE INDEX idx_generations_tool ON generations(tool_name);
CREATE INDEX idx_generations_created ON generations(created_at);
CREATE INDEX idx_generations_refined ON generations(refined_from);
```

#### 4.6.4 Prompt Refinement

When `auto_refine` is enabled and quality scores are below threshold:

1. Generate initial image with original prompt
2. Score with CLIP and aesthetic models
3. If scores below threshold:
   a. Analyze prompt structure
   b. Add quality-boosting suffixes (e.g., "highly detailed, professional, 8k")
   c. Adjust negative prompt with common defect terms
   d. Increase step count by 25%
   e. Regenerate with refined parameters
4. Compare scores of original vs. refined
5. Return the higher-scoring result
6. Log both generations with `refined_from` linking

#### 4.6.5 Feedback API

Agents can provide explicit feedback on generations:

| Parameter      | Type    | Description                                          |
| -------------- | ------- | ---------------------------------------------------- |
| `generation_id`| string  | UUID of the generation to rate                       |
| `rating`       | float   | Agent's rating (0.0–1.0)                             |
| `feedback`     | string  | Free-text feedback description                       |
| `keep`         | boolean | Whether to keep the output file                      |

Feedback is stored and used to adjust future refinement thresholds.

---

## 5. MCP Protocol Specification

### 5.1 Transport

| Property          | Value                                    |
| ----------------- | ---------------------------------------- |
| Transport         | stdio (stdin/stdout)                     |
| Protocol Version  | MCP 2025-03-26                           |
| Encoding          | JSON-RPC 2.0                             |
| Server Name       | `openmedia`                              |
| Server Version    | `0.1.0`                                  |

### 5.2 Capabilities

```json
{
  "capabilities": {
    "tools": { "listChanged": true },
    "resources": { "subscribe": true, "listChanged": true },
    "prompts": { "listChanged": false }
  }
}
```

### 5.3 Tool Registry

#### 5.3.1 Image Generation Tools

**`generate_image`** — Generate an image from a text prompt (txt2img)

| Parameter          | Type     | Required | Default   | Description                              |
| ------------------ | -------- | -------- | --------- | ---------------------------------------- |
| `prompt`           | string   | ✅       | —         | Text description of desired image        |
| `negative_prompt`  | string   | ❌       | `""`      | What to avoid                            |
| `model`            | string   | ❌       | `"auto"`  | Model ID                                 |
| `width`            | integer  | ❌       | `512`     | Width (256–2048, div by 8)               |
| `height`           | integer  | ❌       | `512`     | Height (256–2048, div by 8)              |
| `steps`            | integer  | ❌       | `20`      | Denoising steps (1–150)                  |
| `cfg_scale`        | number   | ❌       | `7.5`     | Guidance scale (1.0–30.0)                |
| `seed`             | integer  | ❌       | random    | RNG seed                                 |
| `scheduler`        | string   | ❌       | `"dpm++"` | Scheduler algorithm                      |
| `batch_size`       | integer  | ❌       | `1`       | Batch count (1–4)                        |
| `output_format`    | string   | ❌       | `"png"`   | png/jpeg/webp                            |
| `output_quality`   | integer  | ❌       | `95`      | Compression quality (1–100)              |
| `clip_skip`        | integer  | ❌       | `1`       | CLIP layers to skip (1–4)                |
| `auto_refine`      | boolean  | ❌       | `false`   | Self-improvement loop                    |
| `max_refine_rounds`| integer  | ❌       | `3`       | Max refinement iterations (1–10)         |

Returns: `{ path: string, width: integer, height: integer, seed: integer, clip_score?: number, aesthetic_score?: number, generation_id: string }`

---

**`transform_image`** — Transform an existing image guided by a text prompt (img2img)

| Parameter          | Type     | Required | Default   | Description                              |
| ------------------ | -------- | -------- | --------- | ---------------------------------------- |
| `input_image`      | string   | ✅       | —         | File path or base64 of source image      |
| `prompt`           | string   | ✅       | —         | Transformation description               |
| `negative_prompt`  | string   | ❌       | `""`      | What to avoid                            |
| `strength`         | number   | ❌       | `0.75`    | Transform strength (0.0–1.0)             |
| `model`            | string   | ❌       | `"auto"`  | Model ID                                 |
| `steps`            | integer  | ❌       | `20`      | Denoising steps                          |
| `cfg_scale`        | number   | ❌       | `7.5`     | Guidance scale                           |
| `seed`             | integer  | ❌       | random    | RNG seed                                 |
| `scheduler`        | string   | ❌       | `"dpm++"` | Scheduler                                |
| `output_format`    | string   | ❌       | `"png"`   | Output format                            |

Returns: `{ path: string, width: integer, height: integer, seed: integer, generation_id: string }`

---

**`inpaint_image`** — Fill masked regions of an image

| Parameter          | Type     | Required | Default   | Description                              |
| ------------------ | -------- | -------- | --------- | ---------------------------------------- |
| `input_image`      | string   | ✅       | —         | Source image                             |
| `mask_image`       | string   | ✅       | —         | Mask (white=fill, black=keep)            |
| `prompt`           | string   | ✅       | —         | What to fill with                        |
| `negative_prompt`  | string   | ❌       | `""`      | What to avoid                            |
| `mask_blur`        | integer  | ❌       | `4`       | Mask edge blur (0–64)                    |
| `inpaint_full`     | boolean  | ❌       | `true`    | Process full image                       |
| `model`            | string   | ❌       | `"auto"`  | Model ID                                 |
| `steps`            | integer  | ❌       | `20`      | Denoising steps                          |
| `cfg_scale`        | number   | ❌       | `7.5`     | Guidance scale                           |
| `seed`             | integer  | ❌       | random    | RNG seed                                 |
| `output_format`    | string   | ❌       | `"png"`   | Output format                            |

Returns: `{ path: string, width: integer, height: integer, seed: integer, generation_id: string }`

---

**`upscale_image`** — Super-resolution upscaling

| Parameter          | Type     | Required | Default        | Description                          |
| ------------------ | -------- | -------- | -------------- | ------------------------------------ |
| `input_image`      | string   | ✅       | —              | Image to upscale                     |
| `scale`            | integer  | ❌       | `4`            | Factor: 2 or 4                       |
| `tile_size`        | integer  | ❌       | `512`          | Processing tile size (128–1024)      |
| `output_format`    | string   | ❌       | `"png"`        | Output format                        |

Returns: `{ path: string, width: integer, height: integer, scale: integer, generation_id: string }`

---

**`remove_background`** — Remove image background

| Parameter          | Type     | Required | Default   | Description                              |
| ------------------ | -------- | -------- | --------- | ---------------------------------------- |
| `input_image`      | string   | ✅       | —         | Input image                              |
| `threshold`        | number   | ❌       | `0.5`     | Segmentation threshold (0.0–1.0)         |
| `feather`          | integer  | ❌       | `2`       | Edge feathering (0–20px)                 |
| `background_color` | string   | ❌       | null      | Replacement color (hex) or transparent   |
| `output_format`    | string   | ❌       | `"png"`   | Must support alpha if transparent        |

Returns: `{ path: string, width: integer, height: integer, generation_id: string }`

---

#### 5.3.2 Video Generation Tools

**`create_video`** — Create a video from a scene description

| Parameter          | Type     | Required | Default    | Description                             |
| ------------------ | -------- | -------- | ---------- | --------------------------------------- |
| `scene`            | object   | ✅       | —          | Scene JSON DSL (see §4.2.2)            |
| `codec`            | string   | ❌       | `"h264"`   | h264, vp9, av1, raw                     |
| `quality`          | string   | ❌       | `"balanced"`| fast, balanced, quality                 |
| `output_path`      | string   | ❌       | auto       | Custom output path                      |
| `renderer`         | string   | ❌       | `"auto"`   | svg, native, browser, auto              |

Returns: `{ path: string, width: integer, height: integer, duration: number, fps: integer, codec: string, file_size: integer, generation_id: string }`

---

**`create_slideshow`** — Create a video from image sequence

| Parameter            | Type     | Required | Default       | Description                         |
| -------------------- | -------- | -------- | ------------- | ----------------------------------- |
| `images`             | string[] | ✅       | —             | Image file paths                    |
| `duration_per_image` | number   | ❌       | `3.0`         | Seconds per image                   |
| `transition`         | string   | ❌       | `"crossfade"` | Transition type                     |
| `transition_duration`| number   | ❌       | `0.5`         | Transition seconds                  |
| `ken_burns`          | boolean  | ❌       | `false`       | Zoom/pan effect                     |
| `audio`              | string   | ❌       | null          | Background audio path               |
| `resolution`         | string   | ❌       | `"1080p"`     | 720p/1080p/1440p/4k                 |
| `codec`              | string   | ❌       | `"h264"`      | Video codec                         |

Returns: `{ path: string, width: integer, height: integer, duration: number, fps: integer, generation_id: string }`

---

**`add_audio_to_video`** — Mix audio into an existing video

| Parameter          | Type     | Required | Default   | Description                              |
| ------------------ | -------- | -------- | --------- | ---------------------------------------- |
| `video`            | string   | ✅       | —         | Input video path                         |
| `audio`            | string   | ✅       | —         | Audio file path (mp3, wav, ogg, flac)    |
| `start_time`       | number   | ❌       | `0.0`     | Audio start offset in video              |
| `volume`           | number   | ❌       | `1.0`     | Volume multiplier (0.0–2.0)              |
| `fade_in`          | number   | ❌       | `0.0`     | Fade-in duration seconds                 |
| `fade_out`         | number   | ❌       | `0.0`     | Fade-out duration seconds                |
| `replace`          | boolean  | ❌       | `false`   | Replace existing audio vs. mix           |

Returns: `{ path: string, duration: number, generation_id: string }`

---

#### 5.3.3 SVG Generation Tools

**`create_svg`** — Build an SVG using the builder API

| Parameter          | Type     | Required | Default   | Description                              |
| ------------------ | -------- | -------- | --------- | ---------------------------------------- |
| `elements`         | object[] | ✅       | —         | Array of SVG element descriptors         |
| `width`            | integer  | ❌       | `800`     | SVG width                                |
| `height`           | integer  | ❌       | `600`     | SVG height                               |
| `background`       | string   | ❌       | null      | Background color                         |
| `viewbox`          | string   | ❌       | auto      | Custom viewBox                           |
| `optimize`         | boolean  | ❌       | `true`    | Run SVG optimizer                        |

Returns: `{ path: string, width: integer, height: integer, file_size: integer, generation_id: string }`

---

**`create_chart`** — Generate a data visualization chart

| Parameter          | Type     | Required | Default   | Description                              |
| ------------------ | -------- | -------- | --------- | ---------------------------------------- |
| `chart_type`       | string   | ✅       | —         | bar, line, pie, scatter, radar, etc.     |
| `data`             | object   | ✅       | —         | Chart data (format varies by type)       |
| `title`            | string   | ❌       | null      | Chart title                              |
| `subtitle`         | string   | ❌       | null      | Chart subtitle                           |
| `width`            | integer  | ❌       | `800`     | Chart width                              |
| `height`           | integer  | ❌       | `600`     | Chart height                             |
| `theme`            | string   | ❌       | `"dark"`  | dark, light, custom                      |
| `legend`           | boolean  | ❌       | `true`    | Show legend                              |
| `legend_position`  | string   | ❌       | `"bottom"`| top, bottom, left, right                 |
| `grid`             | boolean  | ❌       | `true`    | Show grid lines                          |
| `animate`          | boolean  | ❌       | `false`   | Add draw-in animation                    |

Returns: `{ path: string, width: integer, height: integer, generation_id: string }`

---

**`create_diagram`** — Generate a technical diagram

| Parameter          | Type     | Required | Default   | Description                              |
| ------------------ | -------- | -------- | --------- | ---------------------------------------- |
| `diagram_type`     | string   | ✅       | —         | flowchart, sequence, architecture, etc.  |
| `data`             | object   | ✅       | —         | Diagram data (format varies by type)     |
| `title`            | string   | ❌       | null      | Diagram title                            |
| `width`            | integer  | ❌       | `1200`    | Diagram width                            |
| `height`           | integer  | ❌       | auto      | Height (auto-calculated if omitted)      |
| `theme`            | string   | ❌       | `"dark"`  | Visual theme                             |
| `direction`        | string   | ❌       | `"TB"`    | TB, BT, LR, RL (layout direction)       |

Returns: `{ path: string, width: integer, height: integer, generation_id: string }`

---

**`create_icon`** — Generate an icon from the built-in icon set

| Parameter          | Type     | Required | Default   | Description                              |
| ------------------ | -------- | -------- | --------- | ---------------------------------------- |
| `name`             | string   | ✅       | —         | Icon name (e.g., "check", "arrow_right") |
| `size`             | integer  | ❌       | `24`      | Size in pixels (16–512)                  |
| `color`            | string   | ❌       | `"#fff"`  | Fill color (hex)                         |
| `stroke_width`     | number   | ❌       | `2.0`     | Stroke width                             |
| `stroke_color`     | string   | ❌       | null      | Stroke color                             |
| `variant`          | string   | ❌       | `"rounded"`| rounded, square, outline, filled        |

Returns: `{ path: string, svg_content: string, generation_id: string }`

---

**`rasterize_svg`** — Convert SVG to raster image

| Parameter          | Type     | Required | Default   | Description                              |
| ------------------ | -------- | -------- | --------- | ---------------------------------------- |
| `input_svg`        | string   | ✅       | —         | SVG file path or inline SVG string       |
| `width`            | integer  | ❌       | auto      | Output width                             |
| `height`           | integer  | ❌       | auto      | Output height                            |
| `dpi`              | integer  | ❌       | `96`      | Rendering DPI                            |
| `background`       | string   | ❌       | null      | Background color                         |
| `format`           | string   | ❌       | `"png"`   | png, jpeg, webp                          |

Returns: `{ path: string, width: integer, height: integer, generation_id: string }`

---

**`optimize_svg`** — Optimize an existing SVG for size

| Parameter          | Type     | Required | Default   | Description                              |
| ------------------ | -------- | -------- | --------- | ---------------------------------------- |
| `input_svg`        | string   | ✅       | —         | SVG file path or inline string           |
| `precision`        | integer  | ❌       | `2`       | Decimal precision for coordinates (0–8)  |
| `remove_metadata`  | boolean  | ❌       | `true`    | Remove editor metadata                   |
| `convert_to_paths` | boolean  | ❌       | `false`   | Convert text to paths                    |

Returns: `{ path: string, original_size: integer, optimized_size: integer, reduction_pct: number }`

---

#### 5.3.4 Animation Tools

**`create_animation`** — Create an animated SVG

| Parameter          | Type     | Required | Default   | Description                              |
| ------------------ | -------- | -------- | --------- | ---------------------------------------- |
| `elements`         | object[] | ✅       | —         | SVG elements with animation properties   |
| `width`            | integer  | ❌       | `800`     | Canvas width                             |
| `height`           | integer  | ❌       | `600`     | Canvas height                            |
| `duration`         | number   | ❌       | `2.0`     | Total animation duration                 |
| `loop`             | boolean  | ❌       | `true`    | Loop animation                           |
| `background`       | string   | ❌       | null      | Background color                         |
| `animation_type`   | string   | ❌       | `"smil"`  | smil, css, combined                      |

Returns: `{ path: string, width: integer, height: integer, duration: number, generation_id: string }`

---

**`apply_preset`** — Apply an animation preset to an SVG element

| Parameter          | Type     | Required | Default   | Description                              |
| ------------------ | -------- | -------- | --------- | ---------------------------------------- |
| `input_svg`        | string   | ✅       | —         | SVG file to animate                      |
| `element_selector` | string   | ✅       | —         | CSS selector for target element(s)       |
| `preset`           | string   | ✅       | —         | Preset name (see §4.4.4)                |
| `duration`         | number   | ❌       | `1.0`     | Animation duration                       |
| `delay`            | number   | ❌       | `0.0`     | Animation delay                          |
| `easing`           | string   | ❌       | `"ease_out_cubic"` | Easing function              |

Returns: `{ path: string, duration: number, generation_id: string }`

---

**`morph_svg`** — Morph between two SVG paths

| Parameter          | Type     | Required | Default   | Description                              |
| ------------------ | -------- | -------- | --------- | ---------------------------------------- |
| `from_path`        | string   | ✅       | —         | Starting SVG path data (d attribute)     |
| `to_path`          | string   | ✅       | —         | Ending SVG path data                     |
| `duration`         | number   | ❌       | `1.0`     | Morph duration                           |
| `easing`           | string   | ❌       | `"ease_in_out_cubic"` | Easing function          |
| `steps`            | integer  | ❌       | `60`      | Interpolation steps                      |
| `output_type`      | string   | ❌       | `"smil"`  | smil, css, frames                        |

Returns: `{ path: string, duration: number, frame_count: integer, generation_id: string }`

---

**`convert_to_lottie`** — Convert animated SVG to Lottie JSON

| Parameter          | Type     | Required | Default   | Description                              |
| ------------------ | -------- | -------- | --------- | ---------------------------------------- |
| `input_svg`        | string   | ✅       | —         | Animated SVG file path                   |
| `fps`              | integer  | ❌       | `30`      | Lottie framerate                         |
| `compress`         | boolean  | ❌       | `false`   | Output .lottie (compressed)              |

Returns: `{ path: string, fps: integer, duration: number, file_size: integer, generation_id: string }`

---

#### 5.3.5 Image Processing Tools

**`process_image`** — Apply processing operations to an image

| Parameter          | Type     | Required | Default   | Description                              |
| ------------------ | -------- | -------- | --------- | ---------------------------------------- |
| `input_image`      | string   | ✅       | —         | Source image path                        |
| `operations`       | object[] | ✅       | —         | Ordered list of operations               |
| `output_format`    | string   | ❌       | `"png"`   | Output format                            |
| `output_quality`   | integer  | ❌       | `95`      | Compression quality                      |
| `use_gpu`          | boolean  | ❌       | `true`    | Attempt GPU acceleration                 |

Each operation in `operations` is:
```json
{
  "op": "blur|sharpen|brightness|contrast|saturation|...",
  "params": { /* operation-specific */ }
}
```

Returns: `{ path: string, width: integer, height: integer, operations_applied: integer, gpu_used: boolean, generation_id: string }`

---

**`composite_images`** — Composite multiple images together

| Parameter          | Type     | Required | Default   | Description                              |
| ------------------ | -------- | -------- | --------- | ---------------------------------------- |
| `base_image`       | string   | ✅       | —         | Base layer image                         |
| `layers`           | object[] | ✅       | —         | Overlay layers                           |
| `output_format`    | string   | ❌       | `"png"`   | Output format                            |

Each layer:
```json
{
  "image": "path_or_base64",
  "x": 0, "y": 0,
  "width": null, "height": null,
  "opacity": 1.0,
  "blend_mode": "normal"
}
```

Returns: `{ path: string, width: integer, height: integer, layers_composited: integer, generation_id: string }`

---

**`convert_image`** — Convert between image formats

| Parameter          | Type     | Required | Default   | Description                              |
| ------------------ | -------- | -------- | --------- | ---------------------------------------- |
| `input_image`      | string   | ✅       | —         | Source image                             |
| `output_format`    | string   | ✅       | —         | Target format (png/jpeg/webp/gif/bmp/tiff)|
| `quality`          | integer  | ❌       | `95`      | Compression quality                      |
| `resize_width`     | integer  | ❌       | null      | Optional resize width                    |
| `resize_height`    | integer  | ❌       | null      | Optional resize height                   |
| `resize_method`    | string   | ❌       | `"lanczos3"` | Resize filter                         |

Returns: `{ path: string, width: integer, height: integer, original_format: string, file_size: integer, generation_id: string }`

---

#### 5.3.6 Self-Improvement Tools

**`score_image`** — Evaluate image quality with CLIP and aesthetic scoring

| Parameter          | Type     | Required | Default   | Description                              |
| ------------------ | -------- | -------- | --------- | ---------------------------------------- |
| `input_image`      | string   | ✅       | —         | Image to evaluate                        |
| `prompt`           | string   | ❌       | null      | Original prompt (for CLIP scoring)       |

Returns: `{ clip_score: number|null, aesthetic_score: number, generation_id: string }`

---

**`refine_image`** — Refine a previously generated image

| Parameter          | Type     | Required | Default   | Description                              |
| ------------------ | -------- | -------- | --------- | ---------------------------------------- |
| `generation_id`    | string   | ✅       | —         | ID of generation to refine               |
| `feedback`         | string   | ❌       | null      | What to improve                          |
| `max_rounds`       | integer  | ❌       | `3`       | Max refinement attempts                  |

Returns: `{ path: string, original_scores: object, refined_scores: object, rounds_used: integer, improved: boolean, generation_id: string }`

---

**`get_generation_history`** — Query generation history

| Parameter          | Type     | Required | Default   | Description                              |
| ------------------ | -------- | -------- | --------- | ---------------------------------------- |
| `tool_name`        | string   | ❌       | null      | Filter by tool                           |
| `limit`            | integer  | ❌       | `20`      | Max results (1–100)                      |
| `offset`           | integer  | ❌       | `0`       | Pagination offset                        |
| `sort_by`          | string   | ❌       | `"created_at"` | Sort field                          |
| `sort_order`       | string   | ❌       | `"desc"`  | asc or desc                              |
| `min_clip_score`   | number   | ❌       | null      | Minimum CLIP score filter                |
| `min_aesthetic`    | number   | ❌       | null      | Minimum aesthetic score filter           |

Returns: `{ generations: object[], total: integer, offset: integer, limit: integer }`

---

**`provide_feedback`** — Provide feedback on a generation

| Parameter          | Type     | Required | Default   | Description                              |
| ------------------ | -------- | -------- | --------- | ---------------------------------------- |
| `generation_id`    | string   | ✅       | —         | Generation to rate                       |
| `rating`           | number   | ✅       | —         | Rating (0.0–1.0)                         |
| `feedback`         | string   | ❌       | null      | Free-text feedback                       |
| `keep`             | boolean  | ❌       | `true`    | Keep or delete the output file           |

Returns: `{ acknowledged: boolean }`

---

#### 5.3.7 Utility Tools

**`list_models`** — List available models

| Parameter          | Type     | Required | Default   | Description                              |
| ------------------ | -------- | -------- | --------- | ---------------------------------------- |
| `category`         | string   | ❌       | null      | Filter: diffusion, upscale, clip, etc.   |

Returns: `{ models: [{ id, name, category, size_bytes, quantization, status }] }`

---

**`get_system_info`** — Get hardware and capability information

No parameters.

Returns: `{ cpu: object, gpu: object, ram: object, vram: object, available_backends: string[], ffmpeg_available: boolean, chrome_available: boolean }`

---

**`get_server_config`** — Get current server configuration

No parameters.

Returns: `{ config: object }` (full config.toml as JSON)

---

**`cleanup_outputs`** — Clean up generated output files

| Parameter          | Type     | Required | Default   | Description                              |
| ------------------ | -------- | -------- | --------- | ---------------------------------------- |
| `older_than_days`  | integer  | ❌       | `7`       | Delete files older than N days           |
| `keep_rated`       | boolean  | ❌       | `true`    | Keep files with positive feedback        |
| `dry_run`          | boolean  | ❌       | `true`    | Preview without deleting                 |

Returns: `{ files_found: integer, files_deleted: integer, space_freed_bytes: integer }`

---

### 5.4 Resources

| URI Pattern                          | Description                                     |
| ------------------------------------ | ----------------------------------------------- |
| `openmedia://models`                 | List of all available models with status         |
| `openmedia://models/{id}`            | Details for a specific model                     |
| `openmedia://config`                 | Current server configuration                     |
| `openmedia://history`                | Recent generation history (last 50)              |
| `openmedia://history/{id}`           | Specific generation details                      |
| `openmedia://system`                 | Hardware and capability information              |
| `openmedia://output/{category}`      | List output files (images/videos/svgs)           |

### 5.5 Error Codes

| Code   | Name                        | Description                                                |
| ------ | --------------------------- | ---------------------------------------------------------- |
| -32700 | ParseError                  | Invalid JSON in request                                    |
| -32600 | InvalidRequest              | Request does not conform to MCP schema                     |
| -32601 | MethodNotFound              | Unknown tool name                                          |
| -32602 | InvalidParams               | Parameter validation failed                                |
| -32603 | InternalError               | Unexpected server error                                    |
| 1001   | ModelNotFound               | Requested model not available/downloaded                    |
| 1002   | ModelLoadFailed             | Model failed to load (OOM, corrupt file)                   |
| 1003   | InferenceError              | Error during diffusion inference                           |
| 1004   | BackendUnavailable          | No suitable inference backend available                    |
| 2001   | RenderingError              | Frame rendering failed                                     |
| 2002   | EncodingError               | Video encoding failed (FFmpeg error)                       |
| 2003   | FfmpegNotFound              | FFmpeg required but not installed                          |
| 2004   | ChromeNotFound              | Chrome required for Tier-3 rendering but not found         |
| 3001   | InvalidSvgInput             | Malformed SVG input                                        |
| 3002   | ChartDataError              | Invalid or insufficient chart data                         |
| 3003   | DiagramLayoutError          | Auto-layout algorithm failed                               |
| 4001   | GpuError                    | GPU pipeline initialization or execution error             |
| 4002   | ImageDecodeError            | Failed to decode input image                               |
| 4003   | ImageEncodeError            | Failed to encode output image                              |
| 5001   | ScoringError                | CLIP/aesthetic model inference failed                      |
| 5002   | HistoryDbError              | SQLite database operation failed                           |
| 6001   | OutputPathError             | Cannot write to output path (permissions, disk full)       |
| 6002   | InputFileNotFound           | Referenced input file does not exist                       |
| 6003   | FileTooLarge                | Input file exceeds size limit                              |

### 5.6 Progress Reporting

Long-running operations emit MCP progress notifications:

```json
{
  "jsonrpc": "2.0",
  "method": "notifications/progress",
  "params": {
    "progressToken": "gen_abc123",
    "progress": 45,
    "total": 100,
    "message": "Denoising step 9/20"
  }
}
```

Progress-reporting operations:
- Image generation: per denoising step
- Video creation: per rendered frame
- Upscaling: per processed tile
- Batch operations: per item in batch
- Refinement: per refinement round

---

## 6. Architecture Constraints

### 6.1 Workspace Structure

The project is organized as a Cargo workspace with 8 crates:

```
openmedia-rs/
├── Cargo.toml               # Workspace manifest
├── Cargo.lock
├── config.toml               # Default configuration
├── crates/
│   ├── openmedia-core/       # Shared types, config, errors, hardware detection
│   ├── openmedia-image/      # AI image generation (txt2img, img2img, upscale)
│   ├── openmedia-video/      # Video rendering and encoding
│   ├── openmedia-svg/        # SVG builder, charts, diagrams
│   ├── openmedia-animate/    # SVG animation (SMIL, CSS, morphing)
│   ├── openmedia-process/    # Image processing (GPU/CPU pipeline)
│   ├── openmedia-improve/    # Self-improvement (CLIP, aesthetic, history)
│   └── openmedia-mcp/        # MCP server, tool registration, main binary
├── shaders/
│   ├── blur.wgsl
│   ├── sharpen.wgsl
│   ├── color.wgsl
│   ├── blend.wgsl
│   ├── resize.wgsl
│   └── transform.wgsl
├── tests/
│   ├── integration/
│   └── fixtures/
└── docs/
    ├── SPEC.md
    └── CORE.md
```

### 6.2 Dependency Rules

1. **No Python / Node.js**: All functionality in pure Rust. No `pyo3`, no `napi-rs`.
2. **No runtime downloads**: Models must be pre-downloaded. No HTTP calls during operation.
3. **Single binary**: `openmedia-mcp` produces the only binary. All crates are `lib` type except `openmedia-mcp`.
4. **Async everywhere**: All public APIs are async. Runtime is `tokio` (multi-threaded).
5. **Error propagation**: All errors use `OpenMediaError` enum. No `unwrap()` in library code.
6. **No unsafe code**: Except in GPU interop (wgpu) and FFI boundaries (FFmpeg/Chrome CDP). All unsafe blocks documented.

### 6.3 Crate Dependency Graph

```
openmedia-mcp
├── openmedia-image
│   ├── openmedia-core
│   └── openmedia-improve (optional, for auto_refine)
├── openmedia-video
│   ├── openmedia-core
│   └── openmedia-svg (for chart/SVG scene elements)
├── openmedia-svg
│   └── openmedia-core
├── openmedia-animate
│   ├── openmedia-core
│   └── openmedia-svg
├── openmedia-process
│   └── openmedia-core
├── openmedia-improve
│   └── openmedia-core
└── openmedia-core
```

### 6.4 Model Storage Paths

Configuration priority for model storage:
1. `OPENMEDIA_MODEL_DIR` environment variable
2. `model_dir` in `config.toml`
3. Default: `~/.openmedia/models/`

---

## 7. Performance Targets

### 7.1 Image Generation

| Operation                    | Hardware                  | Target Time     | Notes                           |
| ---------------------------- | ------------------------- | --------------- | ------------------------------- |
| txt2img SD 1.5 512×512 20st  | i7-12700 CPU              | < 30 seconds    | Q8_0 GGUF, diffusion_rs        |
| txt2img SD 1.5 512×512 20st  | RTX 3060 GPU              | < 8 seconds     | FP16, Candle CUDA               |
| txt2img SDXL 1024×1024 20st  | i7-12700 CPU              | < 120 seconds   | Q5_0 GGUF                       |
| txt2img SDXL 1024×1024 20st  | RTX 3060 GPU              | < 20 seconds    | FP16, Candle CUDA               |
| txt2img SDXL Turbo 512×512 4st | i7-12700 CPU            | < 15 seconds    | Q8_0, 4-step inference          |
| txt2img FLUX.1 Schnell 4st   | RTX 3060 GPU              | < 12 seconds    | Q4_0, 4-step                    |
| Upscale 512→2048 4x          | RTX 3060 GPU              | < 5 seconds     | ONNX RealESRGAN                 |
| Upscale 512→2048 4x          | i7-12700 CPU              | < 30 seconds    | Tiled processing                |
| Background removal           | CPU                       | < 10 seconds    | U2Net ONNX                      |

### 7.2 Video Generation

| Operation                    | Hardware                  | Target Time     | Notes                           |
| ---------------------------- | ------------------------- | --------------- | ------------------------------- |
| 10s 1080p 30fps Tier-1       | Any                       | < 15 seconds    | SVG renderer, parallel frames   |
| 10s 1080p 30fps Tier-2       | Any                       | < 30 seconds    | Native renderer                 |
| 10s 1080p 30fps Tier-3       | + Chrome                  | < 60 seconds    | Browser renderer                |
| 10s 4K 30fps Tier-1          | 16GB RAM                  | < 45 seconds    | High memory for large frames    |
| H.264 encoding 1080p         | + FFmpeg                  | < 5 seconds     | Hardware encoding if available   |
| Slideshow 10 images 1080p    | + FFmpeg                  | < 20 seconds    | With crossfade transitions       |

### 7.3 SVG & Processing

| Operation                    | Target Time               | Notes                           |
| ---------------------------- | ------------------------- | ------------------------------- |
| SVG chart generation         | < 200ms                   | Any chart type                  |
| SVG diagram (20 nodes)       | < 500ms                   | Including auto-layout           |
| SVG optimization             | < 100ms                   | Per SVG file                    |
| SVG rasterization 1024px     | < 300ms                   | resvg                           |
| Image blur (1080p)           | < 50ms GPU / < 500ms CPU  |                                 |
| Image composite (5 layers)   | < 100ms GPU / < 1s CPU    |                                 |
| CLIP scoring                 | < 2 seconds               | ONNX, CPU                       |
| Aesthetic scoring            | < 500ms                   | Linear probe                    |

### 7.4 Memory Targets

| Operation                    | Peak RAM       | Peak VRAM      |
| ---------------------------- | -------------- | -------------- |
| SD 1.5 inference (Q8_0)     | 3 GB           | — (CPU)        |
| SDXL inference (Q5_0)       | 5 GB           | — (CPU)        |
| SDXL inference (FP16)       | 2 GB           | 6 GB           |
| Video rendering 1080p       | 1 GB           | —              |
| Video rendering 4K          | 3 GB           | —              |
| Image processing pipeline   | 500 MB         | 256 MB         |
| Idle (server running)       | 50 MB          | 0              |

---

## 8. Security

### 8.1 Model Integrity

- All model files verified against SHA-256 checksums on first load
- Checksums stored in `~/.openmedia/models/checksums.sha256`
- Failed checksum → refuse to load, return `ModelLoadFailed` error
- No automatic model downloads. User must manually download and verify.

### 8.2 I/O Sandboxing

- Output files written ONLY to configured output directory (`~/.openmedia/output/` default)
- Input file reads restricted to:
  - Configured model directory
  - Configured output directory
  - Paths explicitly passed as tool parameters
- No directory traversal (`../`) allowed in tool parameters
- File paths canonicalized before any I/O operation

### 8.3 No Telemetry

- Zero network calls during operation (after initial model download)
- No analytics, crash reporting, or usage tracking
- No phone-home behavior
- All data stays on disk in user-controlled directories

### 8.4 Resource Limits

| Resource            | Default Limit    | Configurable |
| ------------------- | ---------------- | ------------ |
| Max image dimension | 4096×4096        | ✅           |
| Max video duration  | 600 seconds      | ✅           |
| Max video resolution| 3840×2160 (4K)   | ✅           |
| Max batch size      | 4                | ✅           |
| Max concurrent ops  | 2                | ✅           |
| Max output file size| 2 GB             | ✅           |
| SQLite DB max size  | 1 GB             | ✅           |

---

## 9. Distribution

### 9.1 Source Installation

```bash
# Install from crates.io
cargo install openmedia

# Install from source
git clone https://github.com/user/openmedia-rs.git
cd openmedia-rs
cargo install --path crates/openmedia-mcp
```

### 9.2 Pre-built Binaries

| Platform              | Architecture | Binary Name                     |
| --------------------- | ------------ | ------------------------------- |
| Linux                 | x86_64       | `openmedia-linux-x86_64`        |
| Linux                 | aarch64      | `openmedia-linux-aarch64`       |
| macOS                 | x86_64       | `openmedia-darwin-x86_64`       |
| macOS                 | aarch64      | `openmedia-darwin-aarch64`      |
| Windows               | x86_64       | `openmedia-windows-x86_64.exe`  |

### 9.3 Docker

```dockerfile
FROM rust:1.82-slim AS builder
WORKDIR /build
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ffmpeg && rm -rf /var/lib/apt/lists/*
COPY --from=builder /build/target/release/openmedia /usr/local/bin/
VOLUME /models
VOLUME /output
ENV OPENMEDIA_MODEL_DIR=/models
ENV OPENMEDIA_OUTPUT_DIR=/output
ENTRYPOINT ["openmedia"]
```

### 9.4 MCP Client Configurations

**Claude Desktop (`claude_desktop_config.json`):**
```json
{
  "mcpServers": {
    "openmedia": {
      "command": "openmedia",
      "args": [],
      "env": {
        "OPENMEDIA_MODEL_DIR": "/home/user/.openmedia/models",
        "OPENMEDIA_OUTPUT_DIR": "/home/user/.openmedia/output"
      }
    }
  }
}
```

**Cline (`.cline/mcp_settings.json`):**
```json
{
  "mcpServers": {
    "openmedia": {
      "command": "openmedia",
      "args": ["--config", "/home/user/.openmedia/config.toml"],
      "disabled": false
    }
  }
}
```

**Continue (`config.yaml`):**
```yaml
mcpServers:
  - name: openmedia
    command: openmedia
    args:
      - --config
      - /home/user/.openmedia/config.toml
```

---

## 10. Versioning & Roadmap

### 10.1 Semantic Versioning

The project follows [SemVer 2.0.0](https://semver.org/):
- **MAJOR**: Breaking MCP tool schema changes
- **MINOR**: New tools, new parameters, new models
- **PATCH**: Bug fixes, performance improvements

### 10.2 Milestone Roadmap

#### v0.1.0 — Foundation (Target: Week 4)
- [ ] Workspace setup with all 8 crates
- [ ] `openmedia-core`: Config, errors, hardware detection
- [ ] `openmedia-image`: txt2img with SD 1.5 via diffusion_rs
- [ ] `openmedia-mcp`: Basic MCP server with `generate_image` tool
- [ ] `openmedia-svg`: Basic SVG builder (rect, circle, text, path)
- [ ] Integration tests for txt2img pipeline

#### v0.2.0 — Image Expansion (Target: Week 8)
- [ ] img2img and inpaint support
- [ ] SDXL and SDXL Turbo models
- [ ] Candle backend for CUDA acceleration
- [ ] ORT backend for ONNX models
- [ ] `upscale_image` tool (Real-ESRGAN)
- [ ] `remove_background` tool (U2-Net)
- [ ] Multiple schedulers (DDIM, DPM++, Euler, Euler-A, LCM)

#### v0.3.0 — SVG & Charts (Target: Week 11)
- [ ] Full SVG builder API with gradients, filters, masks
- [ ] All chart types (bar, line, pie, scatter, radar, heatmap, treemap, gauge)
- [ ] Diagram generation (flowchart, sequence, architecture, ER)
- [ ] SVG optimization pipeline
- [ ] SVG rasterization via resvg
- [ ] `create_chart`, `create_diagram`, `create_icon` tools

#### v0.4.0 — Animation (Target: Week 14)
- [ ] SMIL animation support
- [ ] CSS @keyframes generation
- [ ] Path morphing engine
- [ ] All 15+ animation presets
- [ ] Easing function library
- [ ] `create_animation`, `apply_preset`, `morph_svg` tools

#### v0.5.0 — Video (Target: Week 18)
- [ ] Scene JSON DSL parser
- [ ] Tier-1 SVG renderer (resvg)
- [ ] Tier-2 native HTML renderer (hyper_render)
- [ ] FFmpeg integration for H.264/VP9/AV1 encoding
- [ ] Transition system (crossfade, slide, wipe, zoom, iris)
- [ ] Image slideshow mode
- [ ] Audio mixing
- [ ] `create_video`, `create_slideshow`, `add_audio_to_video` tools

#### v0.6.0 — Image Processing (Target: Week 21)
- [ ] wgpu GPU pipeline with WGSL shaders
- [ ] All image processing operations (blur, sharpen, color, composite, etc.)
- [ ] CPU fallback with imageproc
- [ ] Blend modes
- [ ] `process_image`, `composite_images`, `convert_image` tools

#### v0.7.0 — Self-Improvement (Target: Week 24)
- [ ] CLIP scoring (ONNX ViT-B/32)
- [ ] Aesthetic scoring
- [ ] SQLite generation history
- [ ] Prompt refinement engine
- [ ] Feedback API
- [ ] Auto-refine loop for image generation
- [ ] `score_image`, `refine_image`, `get_generation_history`, `provide_feedback` tools

#### v0.8.0 — FLUX & Advanced (Target: Week 27)
- [ ] FLUX.1 Schnell model support
- [ ] FLUX.1 Dev model support
- [ ] Tier-3 browser renderer (Chrome CDP)
- [ ] Lottie conversion
- [ ] `convert_to_lottie` tool

#### v0.9.0 — Polish & Testing (Target: Week 30)
- [ ] Comprehensive integration test suite
- [ ] Performance optimization pass
- [ ] Memory leak detection and fixes
- [ ] Documentation (README, examples, tutorials)
- [ ] CI/CD pipeline (GitHub Actions)

#### v1.0.0 — Stable Release (Target: Week 32)
- [ ] API freeze (no breaking changes without major version bump)
- [ ] All tools documented with examples
- [ ] Pre-built binaries for all platforms
- [ ] Docker image published
- [ ] crates.io publication
- [ ] MCP client configuration guides

---

## 11. Dependencies

### 11.1 Core Dependencies

| Crate              | Version     | Purpose                                    | Used By              |
| ------------------- | ----------- | ------------------------------------------ | -------------------- |
| `tokio`            | `1.42`      | Async runtime                              | All crates           |
| `serde`            | `1.0`       | Serialization/deserialization              | All crates           |
| `serde_json`       | `1.0`       | JSON handling                              | All crates           |
| `thiserror`        | `2.0`       | Error derive macros                        | All crates           |
| `tracing`          | `0.1`       | Structured logging                         | All crates           |
| `tracing-subscriber`| `0.3`      | Log output formatting                     | openmedia-mcp        |
| `uuid`             | `1.11`      | UUID v7 generation for IDs                 | openmedia-core       |
| `chrono`           | `0.4`       | Date/time handling                         | openmedia-core       |
| `toml`             | `0.8`       | Config file parsing                        | openmedia-core       |
| `directories`      | `5.0`       | Platform-specific directory paths          | openmedia-core       |

### 11.2 MCP & Protocol

| Crate              | Version     | Purpose                                    | Used By              |
| ------------------- | ----------- | ------------------------------------------ | -------------------- |
| `rmcp`             | `0.1`       | Rust MCP SDK (server, tools, transport)    | openmedia-mcp        |

### 11.3 Image Generation

| Crate              | Version     | Purpose                                    | Used By              |
| ------------------- | ----------- | ------------------------------------------ | -------------------- |
| `candle-core`      | `0.8`       | Tensor computation (CPU/CUDA/Metal)        | openmedia-image      |
| `candle-nn`        | `0.8`       | Neural network layers                      | openmedia-image      |
| `candle-transformers`| `0.8`     | Transformer architectures (UNet, CLIP)     | openmedia-image      |
| `diffusion_rs`     | `0.5`       | GGUF diffusion model inference             | openmedia-image      |
| `ort`              | `2.0`       | ONNX Runtime bindings                      | openmedia-image, improve |
| `image`            | `0.25`      | Image format I/O                           | Multiple crates      |
| `imageproc`        | `0.25`      | Image processing (CPU fallback)            | openmedia-process    |

### 11.4 Video & Rendering

| Crate              | Version     | Purpose                                    | Used By              |
| ------------------- | ----------- | ------------------------------------------ | -------------------- |
| `resvg`            | `0.44`      | SVG rendering engine                       | openmedia-video, svg |
| `tiny-skia`        | `0.11`      | 2D rendering for compositing               | openmedia-video      |
| `chromiumoxide`    | `0.7`       | Chrome DevTools Protocol client            | openmedia-video      |
| `symphonia`        | `0.5`       | Audio decoding                             | openmedia-video      |

### 11.5 SVG & Animation

| Crate              | Version     | Purpose                                    | Used By              |
| ------------------- | ----------- | ------------------------------------------ | -------------------- |
| `svg`              | `0.17`      | SVG DOM construction                       | openmedia-svg        |
| `lyon`             | `1.0`       | Path tessellation and manipulation         | openmedia-animate    |
| `kurbo`            | `0.11`      | 2D geometry (bezier curves, paths)         | openmedia-animate    |

### 11.6 GPU & Processing

| Crate              | Version     | Purpose                                    | Used By              |
| ------------------- | ----------- | ------------------------------------------ | -------------------- |
| `wgpu`             | `23.0`      | GPU compute (Vulkan/Metal/DX12)            | openmedia-process    |
| `bytemuck`         | `1.19`      | Safe casting for GPU buffers               | openmedia-process    |
| `rayon`            | `1.10`      | CPU parallelism                            | Multiple crates      |

### 11.7 Storage & Data

| Crate              | Version     | Purpose                                    | Used By              |
| ------------------- | ----------- | ------------------------------------------ | -------------------- |
| `rusqlite`         | `0.32`      | SQLite database (generation history)       | openmedia-improve    |
| `sha2`             | `0.10`      | SHA-256 checksums for model verification   | openmedia-core       |
| `base64`           | `0.22`      | Base64 encoding/decoding for image data    | openmedia-core       |

### 11.8 Development Dependencies

| Crate              | Version     | Purpose                                    |
| ------------------- | ----------- | ------------------------------------------ |
| `criterion`        | `0.5`       | Benchmarking framework                     |
| `tempfile`         | `3.14`      | Temporary files for tests                  |
| `insta`            | `1.41`      | Snapshot testing                           |
| `tokio-test`       | `0.4`       | Async test utilities                       |
| `pretty_assertions`| `1.4`       | Better assertion diffs                     |

---

*This specification is a living document. Last updated: 2026-06-22.*
*All features are subject to change during development.*
*Contributions welcome — see CONTRIBUTING.md for guidelines.*
