# OpenMedia-RS — TODO

> **Last Updated**: 2026-06-22  
> **Legend**: 🔴 **P0** = MVP / must-have | 🟡 **P1** = important / should-have | 🟢 **P2** = nice-to-have / future  
> **Format**: `- [ ] PRIORITY | Task — context/details`

---

## 📦 Project Setup

- [ ] 🔴 **P0** | Initialize Cargo workspace with `[workspace]` in root `Cargo.toml` — all crates as members
- [ ] 🔴 **P0** | Create `crates/openmedia-core/` crate — shared types, config, errors, hardware detection
- [ ] 🔴 **P0** | Create `crates/openmedia-svg/` crate — SVG generation engine
- [ ] 🔴 **P0** | Create `crates/openmedia-svg-animate/` crate — SVG animation engine
- [ ] 🔴 **P0** | Create `crates/openmedia-image/` crate — image processing with wgpu
- [ ] 🔴 **P0** | Create `crates/openmedia-image-ai/` crate — AI image generation
- [ ] 🔴 **P0** | Create `crates/openmedia-video/` crate — video composition and encoding
- [ ] 🔴 **P0** | Create `crates/openmedia-improve/` crate — quality scoring and self-improvement
- [ ] 🔴 **P0** | Create `crates/openmedia-mcp/` crate — MCP server binary
- [ ] 🔴 **P0** | Create `.github/workflows/ci.yml` — lint, test, build on PR and push
- [x] 🔴 **P0** | Create `.github/workflows/release.yml` — cross-compile binaries on version tags
- [ ] 🔴 **P0** | Create `.gitignore` — target/, models/, *.png, *.mp4, IDE configs, .env
- [ ] 🟡 **P1** | Add `LICENSE` file — MIT or Apache-2.0 or dual license
- [ ] 🟡 **P1** | Create `README.md` — project overview, features, installation, usage
- [ ] 🟡 **P1** | Create `docs/` directory structure — architecture, API, guides
- [ ] 🟡 **P1** | Create `tests/integration/` directory — cross-crate integration tests
- [ ] 🟡 **P1** | Create `tests/golden/` directory — visual regression reference images
- [ ] 🟡 **P1** | Create `assets/templates/` directory — chart/diagram/video templates
- [ ] 🟢 **P2** | Create `CONTRIBUTING.md` — dev setup, PR process, code style guide
- [ ] 🟢 **P2** | Create `CHANGELOG.md` — conventional commits changelog
- [ ] 🟢 **P2** | Add `rustfmt.toml` — project-wide formatting config
- [ ] 🟢 **P2** | Add `clippy.toml` — project-wide lint configuration
- [ ] 🟢 **P2** | Add `deny.toml` — cargo-deny config for license and advisory checks
- [ ] 🟢 **P2** | Set up `pre-commit` hooks — fmt, clippy, test on commit
- [ ] 🟢 **P2** | Create `onpkg.json` — project manifest for AI agent context

---

## 🏗️ Core Infrastructure

### Configuration System
- [ ] 🔴 **P0** | Define `OpenMediaConfig` struct with all nested config sections — `SvgConfig`, `ImageConfig`, `AiConfig`, `VideoConfig`
- [ ] 🔴 **P0** | Implement config defaults — sane defaults for every field, works without config file
- [ ] 🔴 **P0** | Implement TOML config file loading — `~/.openmedia/config.toml` with `serde`
- [ ] 🔴 **P0** | Implement environment variable overlay — `OPENMEDIA_*` prefix for all settings
- [ ] 🔴 **P0** | Implement CLI argument parsing — `clap` with all config fields as arguments
- [ ] 🔴 **P0** | Implement layered config merge — defaults → file → env → CLI precedence
- [ ] 🟡 **P1** | Config validation — verify directories exist or are creatable, validate ranges
- [ ] 🟡 **P1** | Config hot-reload — watch config file for changes, apply without restart
- [ ] 🟡 **P1** | Config dump command — `--dump-config` flag to print resolved config

### Hardware Detection
- [ ] 🔴 **P0** | Define `HardwareProfile`, `GpuInfo`, `CpuInfo`, `MemoryInfo` structs
- [ ] 🔴 **P0** | Detect GPU via `wgpu::Instance::enumerate_adapters()` — vendor, model, VRAM
- [ ] 🔴 **P0** | Detect CUDA availability — check for `nvcc` or `nvml-wrapper`
- [ ] 🔴 **P0** | Detect Metal availability — macOS wgpu backend check
- [ ] 🔴 **P0** | Detect Vulkan availability — Linux/Windows wgpu backend check
- [ ] 🔴 **P0** | Detect CPU features — core count, AVX2, AVX-512, NEON via `std::arch`
- [ ] 🔴 **P0** | Detect available RAM — total and free via `sysinfo`
- [ ] 🔴 **P0** | Compute recommended backend — CUDA > Metal > Vulkan > CPU ranking
- [ ] 🟡 **P1** | Pretty-print hardware summary at startup — log with tracing
- [ ] 🟡 **P1** | DirectML detection on Windows — for ONNX Runtime
- [ ] 🟢 **P2** | OpenVINO detection — Intel GPU/NPU support check
- [ ] 🟢 **P2** | CoreML detection — macOS Neural Engine availability

### Model Registry
- [ ] 🔴 **P0** | Define `ModelInfo` struct — id, name, variant, format, size, sha256, url, path, status
- [ ] 🔴 **P0** | Define `ModelVariant` enum — Sd15, Sd21, SdxlBase, SdxlRefiner, SdxlTurbo, RealEsrgan, U2Net, Clip, AestheticPredictor
- [ ] 🔴 **P0** | Define `ModelFormat` enum — SafeTensors, Gguf, Onnx
- [ ] 🔴 **P0** | Define `ModelStatus` enum — NotDownloaded, Downloading, Ready, Corrupted, Loaded
- [ ] 🔴 **P0** | Define `ModelRegistry` trait — list, get, download, verify, delete
- [x] 🔴 **P0** | Implement `HuggingFaceRegistry` — download from HF Hub with auth token support
- [x] 🔴 **P0** | Implement streaming download with progress reporting — per-byte progress callbacks
- [ ] 🔴 **P0** | Implement SHA-256 verification after download — using `sha2` crate
- [x] 🔴 **P0** | Implement atomic file writes — download to `.tmp`, verify, rename
- [ ] 🟡 **P1** | Resume interrupted downloads — HTTP Range header support
- [ ] 🟡 **P1** | LRU cache eviction — when total model size exceeds configurable limit
- [ ] 🟡 **P1** | Concurrent download limiting — max 2 simultaneous downloads
- [ ] 🟡 **P1** | Custom model registration — user-provided model files
- [ ] 🟡 **P1** | Create `models/registry.json` — catalog of all known models with URLs and checksums
- [ ] 🟢 **P2** | Model conversion — ONNX ↔ SafeTensors ↔ GGUF (where possible)
- [ ] 🟢 **P2** | Model integrity monitoring — periodic checksum verification

### Types & Errors
- [ ] 🔴 **P0** | Define `OpenMediaError` top-level error enum — wraps all domain errors
- [ ] 🔴 **P0** | Define `SvgError` — InvalidShape, InvalidColor, TemplateMissing, OptimizationFailed, RasterizationFailed
- [ ] 🔴 **P0** | Define `ImageError` — FormatUnsupported, ShaderCompileFailed, GpuUnavailable, DecodeFailed, EncodeFailed
- [ ] 🔴 **P0** | Define `AiError` — ModelNotFound, ModelLoadFailed, InferenceFailed, OomError, BackendUnavailable, SchedulerError
- [ ] 🔴 **P0** | Define `VideoError` — SceneParseFailed, RenderFailed, EncodeFailed, AudioMixFailed, FfmpegNotFound
- [ ] 🔴 **P0** | Define `ImproveError` — ScoringFailed, DatabaseError, ClipModelFailed, PromptRefineFailed
- [ ] 🔴 **P0** | Implement `From` conversions — domain errors to `OpenMediaError`
- [ ] 🔴 **P0** | Implement `Serialize` for all errors — JSON with error codes for MCP
- [ ] 🔴 **P0** | Define `ErrorCode` enum — numeric codes for each error class
- [ ] 🔴 **P0** | Define `pub type Result<T>` alias — standard result type for all crates

### Progress Reporting
- [ ] 🔴 **P0** | Define `ProgressReporter` trait — `report(progress, message)`, `report_step(step, total, message)`
- [ ] 🔴 **P0** | Implement `McpProgressReporter` — sends MCP progress notifications
- [ ] 🟡 **P1** | Implement `NoopProgressReporter` — for tests and non-MCP contexts
- [ ] 🟡 **P1** | Implement `LogProgressReporter` — logs to tracing at info level
- [ ] 🟡 **P1** | Wire progress reporter into all long-running operations — generation, download, encoding

---

## 📐 SVG Generation

### SvgBuilder Core API
- [ ] 🔴 **P0** | Implement `SvgDocument` struct — viewBox, dimensions, xmlns declarations
- [ ] 🔴 **P0** | Implement `rect()` — x, y, width, height, rx, ry (rounded corners)
- [ ] 🔴 **P0** | Implement `circle()` — cx, cy, r
- [ ] 🔴 **P0** | Implement `ellipse()` — cx, cy, rx, ry
- [ ] 🔴 **P0** | Implement `line()` — x1, y1, x2, y2
- [ ] 🔴 **P0** | Implement `polyline()` — points array
- [ ] 🔴 **P0** | Implement `polygon()` — points array
- [ ] 🔴 **P0** | Implement `path()` — d attribute (path data string)
- [ ] 🔴 **P0** | Implement `PathBuilder` — move_to, line_to, curve_to, quad_to, arc_to, close
- [ ] 🔴 **P0** | Implement `text()` — content, position, font-family, font-size, font-weight, text-anchor
- [ ] 🔴 **P0** | Implement `group()` — `<g>` with nested elements and shared styles
- [ ] 🔴 **P0** | Implement `to_string()` — serialize to valid SVG XML
- [ ] 🟡 **P1** | Implement `use_element()` — reference `<defs>` elements with `<use>`
- [ ] 🟡 **P1** | Implement `image()` — embedded raster images (base64 data URI or href)
- [ ] 🟡 **P1** | Implement `to_pretty_string()` — indented XML for debugging
- [ ] 🟡 **P1** | Implement `foreignObject()` — embed HTML content in SVG
- [ ] 🟢 **P2** | Implement `symbol()` — reusable symbol definitions
- [ ] 🟢 **P2** | Implement `marker()` — arrowhead markers for lines/paths
- [ ] 🟢 **P2** | Implement `tspan()` — inline text styling within `<text>`

### Styles & Defs
- [ ] 🔴 **P0** | Implement `Style` builder — fill, stroke, opacity, font properties
- [ ] 🔴 **P0** | Implement fill properties — color, opacity, fill-rule (evenodd, nonzero)
- [ ] 🔴 **P0** | Implement stroke properties — color, width, opacity, linecap, linejoin, dasharray, dashoffset
- [ ] 🔴 **P0** | Implement `LinearGradient` — id, x1/y1/x2/y2, stops with offset/color/opacity
- [ ] 🔴 **P0** | Implement `RadialGradient` — id, cx/cy/r/fx/fy, stops, gradientTransform
- [ ] 🟡 **P1** | Implement `Pattern` — id, patternUnits, dimensions, child elements
- [ ] 🟡 **P1** | Implement transforms — translate, rotate, scale, skewX, skewY, matrix
- [ ] 🟡 **P1** | Implement `<clipPath>` — arbitrary shape clipping
- [ ] 🟡 **P1** | Implement `<mask>` — gradient-based transparency masks
- [ ] 🟡 **P1** | Implement `feGaussianBlur` filter — stdDeviation parameter
- [ ] 🟡 **P1** | Implement `feDropShadow` filter — dx, dy, stdDeviation, flood-color
- [ ] 🟡 **P1** | Implement `feColorMatrix` filter — saturate, hueRotate, luminanceToAlpha
- [ ] 🟡 **P1** | Implement filter chains — `<filter>` with `<feMerge>` compositing
- [ ] 🟢 **P2** | Implement `feComposite` filter — Porter-Duff operations
- [ ] 🟢 **P2** | Implement `feMorphology` filter — erode, dilate
- [ ] 🟢 **P2** | Implement `feTurbulence` filter — noise generation
- [ ] 🟢 **P2** | Implement `feDisplacementMap` filter — texture distortion
- [ ] 🟢 **P2** | Implement class-based styling — `<style>` element with CSS rules

### Icon Templates
- [ ] 🔴 **P0** | Define `IconTemplate` trait — `render(style, size, color, stroke_width) -> SvgDocument`
- [ ] 🔴 **P0** | Define `IconStyle` enum — Outline, Filled, Duotone
- [ ] 🔴 **P0** | Implement `IconRegistry` — lookup templates by name
- [ ] 🔴 **P0** | Implement Home/House icon — 3 styles (outline, filled, duotone)
- [ ] 🔴 **P0** | Implement User/Person icon — 3 styles
- [ ] 🔴 **P0** | Implement Settings/Gear icon — 3 styles
- [ ] 🔴 **P0** | Implement Search/Magnifying Glass icon — 3 styles
- [ ] 🔴 **P0** | Implement Mail/Envelope icon — 3 styles
- [ ] 🟡 **P1** | Implement Phone icon — 3 styles
- [ ] 🟡 **P1** | Implement Calendar icon — 3 styles
- [ ] 🟡 **P1** | Implement Clock icon — 3 styles
- [ ] 🟡 **P1** | Implement Camera icon — 3 styles
- [ ] 🟡 **P1** | Implement Heart icon — 3 styles
- [ ] 🟡 **P1** | Implement Star icon — 3 styles
- [ ] 🟡 **P1** | Implement Lock icon — 3 styles
- [ ] 🟡 **P1** | Implement Bell/Notification icon — 3 styles
- [ ] 🟡 **P1** | Implement Download icon — 3 styles
- [ ] 🟡 **P1** | Implement Upload icon — 3 styles
- [ ] 🟡 **P1** | Implement Share icon — 3 styles
- [ ] 🟡 **P1** | Implement Edit/Pencil icon — 3 styles
- [ ] 🟡 **P1** | Implement Delete/Trash icon — 3 styles
- [ ] 🟡 **P1** | Implement Check/Checkmark icon — 3 styles
- [ ] 🟡 **P1** | Implement Close/X icon — 3 styles
- [ ] 🟢 **P2** | Implement Menu/Hamburger icon — 3 styles
- [ ] 🟢 **P2** | Implement Arrow Left/Right icons — 3 styles
- [ ] 🟢 **P2** | Implement Plus/Add icon — 3 styles
- [ ] 🟢 **P2** | Implement Folder icon — 3 styles
- [ ] 🟢 **P2** | Create icon gallery HTML page — render all icons for preview

### Charts
- [ ] 🔴 **P0** | Define `ChartConfig` struct — width, height, margins, title, subtitle, colors, font
- [ ] 🔴 **P0** | Define `DataSeries` struct — name, values, color, marker
- [ ] 🔴 **P0** | Implement axis auto-scaling — `nice_numbers()` algorithm for human-readable tick marks
- [ ] 🔴 **P0** | Implement color palette system — 10+ accessible colors for multi-series
- [ ] 🔴 **P0** | Implement legend renderer — horizontal and vertical layout options
- [ ] 🔴 **P0** | Implement Bar Chart — vertical bars with category X-axis, value Y-axis
- [ ] 🔴 **P0** | Implement Bar Chart horizontal variant — horizontal bars
- [ ] 🟡 **P1** | Implement Bar Chart grouped variant — side-by-side bars for multi-series
- [ ] 🟡 **P1** | Implement Bar Chart stacked variant — stacked bars with total labels
- [ ] 🔴 **P0** | Implement Line Chart — single series with data point markers
- [ ] 🟡 **P1** | Implement Line Chart multi-series — multiple lines with legend
- [ ] 🟡 **P1** | Implement Line Chart smooth mode — cubic bezier interpolation
- [ ] 🟡 **P1** | Implement Line Chart area fill — gradient fill under line
- [ ] 🔴 **P0** | Implement Pie Chart — standard pie with labels and percentages
- [ ] 🟡 **P1** | Implement Pie Chart donut variant — configurable inner radius
- [ ] 🟡 **P1** | Implement Pie Chart exploded slice — pull out selected slice
- [ ] 🔴 **P0** | Implement Scatter Plot — X-Y data points with configurable markers
- [ ] 🟡 **P1** | Implement Scatter Plot bubble variant — point size encoding
- [ ] 🟡 **P1** | Implement Scatter Plot trend line — linear regression overlay
- [ ] 🟡 **P1** | Implement Radar Chart — multi-axis polygon with fill area
- [ ] 🟡 **P1** | Implement Gauge Chart — semicircle with needle and color zones
- [ ] 🟢 **P2** | Implement Heatmap — color-encoded grid cells
- [ ] 🟢 **P2** | Implement Treemap — hierarchical area-proportional rectangles
- [ ] 🟢 **P2** | Implement Waterfall Chart — running total visualization
- [ ] 🟢 **P2** | Implement chart gridlines — major/minor with configurable style
- [ ] 🟢 **P2** | Implement chart tooltip titles — `<title>` elements for hover text

### Diagrams
- [ ] 🔴 **P0** | Implement Flowchart — rectangle/diamond/rounded nodes with directed edges
- [ ] 🔴 **P0** | Implement Flowchart auto-layout — Sugiyama-style layered layout algorithm
- [ ] 🟡 **P1** | Implement Flowchart edge routing — orthogonal paths with bend minimization
- [ ] 🟡 **P1** | Implement Flowchart swimlanes — categorized parallel tracks
- [ ] 🔴 **P0** | Implement Architecture Diagram — component boxes with connection lines
- [ ] 🟡 **P1** | Implement Architecture Diagram nested groups — boundaries like Cloud/VPC/Subnet
- [ ] 🟡 **P1** | Implement Architecture Diagram connection styles — solid, dashed, dotted with labels
- [ ] 🟡 **P1** | Implement Sequence Diagram — actor lifelines with messages
- [ ] 🟡 **P1** | Implement Sequence Diagram activation bars — method call depth
- [ ] 🟡 **P1** | Implement Sequence Diagram fragments — alt/opt/loop boxes
- [ ] 🟡 **P1** | Implement Mindmap — central node with radial branching
- [ ] 🟢 **P2** | Implement Mindmap auto-layout — radial tree layout algorithm
- [ ] 🟢 **P2** | Implement ER Diagram — entities with relationships (1:1, 1:N, M:N)
- [ ] 🟢 **P2** | Implement Gantt Chart — timeline-based task visualization

### Optimizer & Rasterizer
- [ ] 🔴 **P0** | Remove unnecessary whitespace and default attributes from SVG output
- [ ] 🔴 **P0** | Shorten color values — `#ffffff` → `#fff`, named colors where shorter
- [ ] 🔴 **P0** | Round numeric values to configurable precision (default 2 decimals)
- [ ] 🟡 **P1** | Minify path data — remove redundant commands, shorten coordinates
- [ ] 🟡 **P1** | Collapse single-child groups — remove unnecessary `<g>` wrappers
- [ ] 🟡 **P1** | Merge identical `<defs>` — deduplicate gradients and filters
- [ ] 🟡 **P1** | Remove empty elements and unused defs
- [ ] 🟢 **P2** | Convert inline styles to attributes where shorter
- [ ] 🔴 **P0** | Integrate `resvg` + `usvg` for SVG-to-raster conversion
- [ ] 🔴 **P0** | Implement rasterize to PNG with alpha transparency
- [ ] 🔴 **P0** | Implement rasterize to JPEG with quality setting (1-100)
- [ ] 🟡 **P1** | Implement rasterize to WebP with quality setting
- [ ] 🟡 **P1** | Support custom DPI — 72 (screen), 150 (web), 300 (print)
- [ ] 🟡 **P1** | Support retina output — @2x, @3x modes
- [ ] 🟡 **P1** | Font loading via `fontdb` — system fonts + bundled fallback

---

## ✨ Animated SVG

### SMIL Animation
- [ ] 🔴 **P0** | Implement `<animate>` — attributeName, from, to, values, dur, begin, repeatCount, fill
- [ ] 🔴 **P0** | Implement `<animateTransform>` — translate, rotate, scale, skewX, skewY
- [ ] 🔴 **P0** | Implement `<animateMotion>` — path, keyPoints, keyTimes, rotate
- [ ] 🔴 **P0** | Implement `<set>` — instant attribute change at specified begin time
- [ ] 🟡 **P1** | Support `begin` timing expressions — offset, event-based, syncbase (`id.begin`, `id.end`)
- [ ] 🟡 **P1** | Support `calcMode` — discrete, linear, paced, spline with keySplines
- [ ] 🟡 **P1** | Support `accumulate` and `additive` — composing animations
- [ ] 🟢 **P2** | Support `restart` attribute — always, whenNotActive, never

### CSS Keyframes
- [ ] 🔴 **P0** | Implement `KeyframeAnimation` builder — name, duration, timing-function, delay, iterations
- [ ] 🔴 **P0** | Generate `@keyframes` CSS block with percentage-based steps
- [ ] 🔴 **P0** | Generate animation class CSS — shorthand and individual properties
- [ ] 🟡 **P1** | Support `animation-direction` — normal, reverse, alternate, alternate-reverse
- [ ] 🟡 **P1** | Support `animation-fill-mode` — none, forwards, backwards, both
- [ ] 🟡 **P1** | Support `animation-play-state` — running, paused
- [ ] 🟢 **P2** | Support `animation-composition` — replace, add, accumulate
- [ ] 🟢 **P2** | Support `animation-timeline` — scroll-driven animations (CSS spec)

### Easing Functions
- [ ] 🔴 **P0** | Implement linear easing
- [ ] 🔴 **P0** | Implement ease, ease-in, ease-out, ease-in-out
- [ ] 🔴 **P0** | Implement sine easings — ease-in-sine, ease-out-sine, ease-in-out-sine
- [ ] 🔴 **P0** | Implement quad easings — ease-in-quad, ease-out-quad, ease-in-out-quad
- [ ] 🔴 **P0** | Implement cubic easings — ease-in-cubic, ease-out-cubic, ease-in-out-cubic
- [ ] 🟡 **P1** | Implement quart easings — ease-in-quart, ease-out-quart, ease-in-out-quart
- [ ] 🟡 **P1** | Implement quint easings — ease-in-quint, ease-out-quint, ease-in-out-quint
- [ ] 🟡 **P1** | Implement expo easings — ease-in-expo, ease-out-expo, ease-in-out-expo
- [ ] 🟡 **P1** | Implement circ easings — ease-in-circ, ease-out-circ, ease-in-out-circ
- [ ] 🟡 **P1** | Implement back easings — ease-in-back, ease-out-back, ease-in-out-back
- [ ] 🟡 **P1** | Implement elastic easings — ease-in-elastic, ease-out-elastic, ease-in-out-elastic
- [ ] 🟡 **P1** | Implement bounce easings — ease-in-bounce, ease-out-bounce, ease-in-out-bounce
- [ ] 🔴 **P0** | Implement custom `cubic-bezier(x1, y1, x2, y2)` — Newton-Raphson solver
- [ ] 🟡 **P1** | Implement spring physics — `spring(mass, stiffness, damping)` simulation
- [ ] 🟡 **P1** | Implement `to_css()` — CSS-compatible easing string output
- [ ] 🟡 **P1** | Implement `to_smil_key_splines()` — SMIL spline format output
- [ ] 🟢 **P2** | Implement steps easing — `steps(n, jump-start|jump-end|jump-both|jump-none)`
- [ ] 🟢 **P2** | Generate lookup tables for complex easings — performance optimization

### Timeline & Sequencing
- [ ] 🔴 **P0** | Define `Timeline` struct — tracks, mode, stagger_delay, repeat, speed
- [ ] 🔴 **P0** | Define `AnimationTrack` — target element, animation, delay, duration
- [ ] 🔴 **P0** | Implement `TimelineMode::Sequential` — animations play one after another
- [ ] 🔴 **P0** | Implement `TimelineMode::Parallel` — all animations start simultaneously
- [ ] 🔴 **P0** | Implement `TimelineMode::Stagger` — incremental delay between animations
- [ ] 🔴 **P0** | Compute total timeline duration from tracks and mode
- [ ] 🟡 **P1** | Resolve timing — convert relative delays to absolute begin values
- [ ] 🟡 **P1** | Output timeline as SMIL — using syncbase timing references
- [ ] 🟡 **P1** | Output timeline as CSS — using animation-delay offsets
- [ ] 🟡 **P1** | Support nested timelines — timeline within a timeline for complex choreography
- [ ] 🟢 **P2** | Support labels/markers — named seek points in timeline
- [ ] 🟢 **P2** | Support speed control — global playback rate modifier

### Path Morphing
- [ ] 🔴 **P0** | Parse SVG path `d` attribute into structured segments
- [ ] 🔴 **P0** | Normalize all path segments to cubic bezier curves
- [ ] 🔴 **P0** | Implement `interpolate_paths(from, to, t)` — smooth interpolation
- [ ] 🟡 **P1** | Handle unequal segment counts — subdivide shorter path to match longer
- [ ] 🟡 **P1** | Generate SMIL `<animate attributeName="d">` for path morphing
- [ ] 🟡 **P1** | Generate CSS `@keyframes` with `d: path(...)` property
- [ ] 🟢 **P2** | Support multi-step morphing — A → B → C → A for looping
- [ ] 🟢 **P2** | Shape matching — optimal correspondence between segments for minimal distortion

### Animation Presets
- [ ] 🔴 **P0** | Implement `fade_in` preset — opacity 0 → 1
- [ ] 🔴 **P0** | Implement `fade_out` preset — opacity 1 → 0
- [ ] 🔴 **P0** | Implement `slide_in_left` preset — translateX(-100%) → 0
- [ ] 🔴 **P0** | Implement `slide_in_right` preset — translateX(100%) → 0
- [ ] 🔴 **P0** | Implement `slide_in_up` preset — translateY(100%) → 0
- [ ] 🔴 **P0** | Implement `slide_in_down` preset — translateY(-100%) → 0
- [ ] 🟡 **P1** | Implement `zoom_in` preset — scale(0) → scale(1) with fade
- [ ] 🟡 **P1** | Implement `zoom_out` preset — scale(1) → scale(0) with fade
- [ ] 🟡 **P1** | Implement `bounce` preset — translateY with elastic easing
- [ ] 🟡 **P1** | Implement `pulse` preset — scale(1) → scale(1.1) → scale(1) loop
- [ ] 🟡 **P1** | Implement `shake` preset — translateX oscillation
- [ ] 🟡 **P1** | Implement `spin` preset — rotate(0deg) → rotate(360deg) loop
- [ ] 🟡 **P1** | Implement `flip_x` preset — rotateX(0) → rotateX(360deg)
- [ ] 🟡 **P1** | Implement `flip_y` preset — rotateY(0) → rotateY(360deg)
- [ ] 🟡 **P1** | Implement `typewriter` preset — stroke-dashoffset text reveal
- [ ] 🟡 **P1** | Implement `draw_path` preset — stroke-dashoffset path drawing effect
- [ ] 🟢 **P2** | Implement `gradient_shift` preset — animate gradient stop positions
- [ ] 🟢 **P2** | Implement `morph_loop` preset — path morph A → B → A cycle
- [ ] 🟢 **P2** | Implement `rubber_band` preset — elastic stretch and snap back
- [ ] 🟢 **P2** | Implement `jello` preset — wobbling skew animation

### Lottie Support
- [ ] 🟡 **P1** | Parse Lottie JSON format — composition, layers, shapes, keyframes
- [ ] 🟡 **P1** | Convert Lottie shapes to SVG elements — rect, ellipse, path, fill, stroke
- [ ] 🟡 **P1** | Convert Lottie keyframes to SMIL/CSS animations
- [ ] 🟡 **P1** | Handle Lottie easing → cubic-bezier conversion
- [ ] 🟡 **P1** | Handle layer parenting — transform inheritance chain
- [ ] 🟡 **P1** | Handle trim paths — stroke-dasharray animation
- [ ] 🟢 **P2** | Handle masks and mattes — basic support
- [ ] 🟢 **P2** | Implement `svg_to_lottie()` — reverse conversion for interop
- [ ] 🟢 **P2** | Document supported/unsupported Lottie features

### Animated Spinners
- [ ] 🟡 **P1** | Implement circular spinner — rotating arc with gradient
- [ ] 🟡 **P1** | Implement dots spinner — pulsing dots in sequence
- [ ] 🟡 **P1** | Implement bars spinner — loading bars with stagger
- [ ] 🟢 **P2** | Implement ring spinner — expanding/contracting ring
- [ ] 🟢 **P2** | Implement skeleton loader — pulsing placeholder shapes

---

## 🖼️ AI Image Generation

### DiffusionPipeline Trait
- [ ] 🔴 **P0** | Define `DiffusionPipeline` trait — `generate()`, `supported_models()`, `estimated_vram()`
- [ ] 🔴 **P0** | Define `GenerationParams` struct — prompt, negative_prompt, width, height, steps, cfg_scale, seed, scheduler, num_images
- [ ] 🔴 **P0** | Define `GenerationResult` struct — image, seed, steps_used, generation_time, model_used

### Candle Backend (SafeTensors)
- [ ] 🔴 **P0** | Implement `CandlePipeline` struct — load and manage model components
- [ ] 🔴 **P0** | Load CLIP text encoder from SafeTensors — tokenize, encode, produce embeddings
- [ ] 🔴 **P0** | Load UNet from SafeTensors — support f32, f16, bf16 dtypes
- [ ] 🔴 **P0** | Load VAE decoder from SafeTensors — latent space to pixel space
- [ ] 🔴 **P0** | Implement text encoding pipeline — tokenize → CLIP → embeddings → unconditional
- [ ] 🔴 **P0** | Implement latent noise generation — Gaussian noise with specified seed
- [ ] 🔴 **P0** | Implement denoising loop — UNet inference × N steps with scheduler
- [ ] 🔴 **P0** | Implement VAE decode — latents → scaled → pixel image
- [ ] 🔴 **P0** | SD 1.5 support — 512x512, 77 token context, ~4GB VRAM
- [ ] 🔴 **P0** | CPU device support — slow but works everywhere
- [ ] 🔴 **P0** | CUDA device support — fast GPU inference on NVIDIA
- [ ] 🟡 **P1** | Metal device support — GPU inference on macOS Apple Silicon
- [ ] 🟡 **P1** | SDXL Base support — 1024x1024, dual text encoders, ~8GB VRAM
- [ ] 🟡 **P1** | SDXL Refiner support — optional second-pass refinement
- [ ] 🟡 **P1** | SDXL Turbo support — 1-4 step generation, no CFG needed, ~6GB VRAM
- [ ] 🟡 **P1** | SD 2.1 support — v-prediction model variant
- [ ] 🟡 **P1** | Implement VRAM management — unload/reload models on demand
- [ ] 🟡 **P1** | Implement dtype auto-selection — f16 on GPU, f32 on CPU
- [ ] 🟢 **P2** | Implement attention slicing — reduce VRAM usage for large images
- [ ] 🟢 **P2** | Implement VAE tiling — decode large latents in tiles

### diffusion_rs Backend (GGUF)
- [ ] 🟡 **P1** | Integrate `diffusion_rs` crate as optional dependency
- [ ] 🟡 **P1** | Implement `GgufPipeline` implementing `DiffusionPipeline`
- [ ] 🟡 **P1** | Support Q4_0 quantization — smallest, fastest, lowest quality
- [ ] 🟡 **P1** | Support Q4_K_M quantization — good balance of speed and quality
- [ ] 🟡 **P1** | Support Q5_K_M quantization — higher quality, slightly slower
- [ ] 🟡 **P1** | Support Q8_0 quantization — near-full quality, moderate speed
- [ ] 🟡 **P1** | Support CPU backend for GGUF models
- [ ] 🟡 **P1** | Support CUDA backend for GGUF models
- [ ] 🟡 **P1** | Support Vulkan backend for GGUF — Windows/Linux AMD/NVIDIA
- [ ] 🟡 **P1** | Support Metal backend for GGUF — macOS Apple Silicon
- [ ] 🟡 **P1** | Register GGUF models in model catalog — HF Hub URLs and checksums
- [ ] 🟢 **P2** | Benchmark GGUF vs SafeTensors — quality and speed comparison

### ONNX Runtime Backend
- [ ] 🟡 **P1** | Integrate `ort` crate as optional dependency
- [ ] 🟡 **P1** | Implement `OnnxPipeline` implementing `DiffusionPipeline`
- [ ] 🟡 **P1** | Load ONNX model files — separate text encoder, UNet, VAE
- [ ] 🟡 **P1** | DirectML execution provider — Windows GPU (AMD, Intel, NVIDIA)
- [ ] 🟡 **P1** | CUDA execution provider — NVIDIA GPU fallback
- [ ] 🟡 **P1** | CPU execution provider — universal fallback
- [ ] 🟢 **P2** | OpenVINO execution provider — Intel GPU/NPU
- [ ] 🟢 **P2** | CoreML execution provider — macOS Neural Engine
- [ ] 🟢 **P2** | Download Olive-optimized ONNX models from HF Hub
- [ ] 🟢 **P2** | Session memory management — load/unload to limit memory

### Schedulers
- [ ] 🔴 **P0** | Define `Scheduler` trait — `init()`, `step()`, `add_noise()`, `sigma_schedule()`
- [ ] 🔴 **P0** | Implement DDIM scheduler — deterministic, eta parameter, 20-50 steps
- [ ] 🔴 **P0** | Implement Euler scheduler — first-order ODE solver, 25-50 steps
- [ ] 🔴 **P0** | Implement Euler Ancestral scheduler — stochastic noise injection
- [ ] 🟡 **P1** | Implement DPM++ 2M scheduler — multi-step solver, 15-30 steps
- [ ] 🟡 **P1** | Implement DPM++ 2M Karras scheduler — Karras sigma schedule
- [ ] 🟡 **P1** | Implement LCM scheduler — 1-8 step generation for Turbo/LCM models
- [ ] 🟡 **P1** | Implement noise schedule computation — linear, cosine, Karras variants
- [ ] 🟡 **P1** | Implement timestep spacing — linspace, trailing, leading methods
- [ ] 🟢 **P2** | Implement PNDM scheduler — pseudo-numerical methods
- [ ] 🟢 **P2** | Implement UniPC scheduler — unified predictor-corrector

### img2img & Inpainting
- [ ] 🟡 **P1** | Implement VAE encoder — image → latent space encoding
- [ ] 🟡 **P1** | Implement img2img — encode input, add noise proportional to strength, denoise
- [ ] 🟡 **P1** | Support strength parameter (0.0-1.0) — controls similarity to input
- [ ] 🟡 **P1** | Implement inpainting — mask-aware denoising with latent blending
- [ ] 🟡 **P1** | Implement mask processing — load, resize to latent space, feather edges
- [ ] 🟡 **P1** | Support mask input formats — image file, base64, bounding box
- [ ] 🟢 **P2** | Support dedicated inpainting model — SD 1.5 Inpainting variant
- [ ] 🟢 **P2** | Implement outpainting — extend image beyond boundaries

### Upscaler (Real-ESRGAN)
- [ ] 🟡 **P1** | Load Real-ESRGAN model — ONNX or Candle
- [ ] 🟡 **P1** | Implement 2x upscaling — double resolution with quality enhancement
- [ ] 🟡 **P1** | Implement 4x upscaling — quadruple resolution with detail synthesis
- [ ] 🟡 **P1** | Implement tile-based processing — avoid VRAM overflow on large images
- [ ] 🟡 **P1** | Implement tile blending — overlap regions for seamless output
- [ ] 🟢 **P2** | Support face enhancement model variant — better face detail
- [ ] 🟢 **P2** | Support anime model variant — optimized for anime/illustration style

### Background Removal (U2-Net)
- [ ] 🟡 **P1** | Load U2-Net model — ONNX or Candle
- [ ] 🟡 **P1** | Implement preprocessing — resize to 320x320, normalize
- [ ] 🟡 **P1** | Implement inference — generate probability map
- [ ] 🟡 **P1** | Implement post-processing — threshold, refine edges, apply as alpha
- [ ] 🟡 **P1** | Output PNG with transparency — background removed, subject isolated
- [ ] 🟢 **P2** | Output separate mask image — for manual editing
- [ ] 🟢 **P2** | Foreground extraction — crop to subject bounding box
- [ ] 🟢 **P2** | Background replacement — composite subject onto new background

### Backend Auto-Selection
- [ ] 🔴 **P0** | Implement backend selection logic — CUDA > Metal > Vulkan > DirectML > CPU
- [ ] 🔴 **P0** | Check model format availability — SafeTensors, GGUF, ONNX
- [ ] 🔴 **P0** | Match backend to available hardware and model format
- [ ] 🟡 **P1** | Allow user override of backend selection via tool parameter
- [ ] 🟡 **P1** | Log backend selection reasoning — explain why backend was chosen
- [ ] 🟡 **P1** | VRAM estimation — warn if model likely to exceed available VRAM

### Generation Features
- [ ] 🔴 **P0** | Seed support — reproducible generation with specified seed
- [ ] 🔴 **P0** | Negative prompt support — steer generation away from unwanted content
- [ ] 🔴 **P0** | CFG scale control — guidance scale from 1.0 to 30.0
- [ ] 🔴 **P0** | Step count control — 1 to 100 steps
- [ ] 🔴 **P0** | Resolution control — width and height in multiples of 8
- [ ] 🔴 **P0** | Progress reporting — per-step progress with percentage and ETA
- [ ] 🟡 **P1** | Batch generation — generate N images from same prompt with different seeds
- [ ] 🟡 **P1** | Output format selection — PNG, JPEG, WebP with quality settings
- [ ] 🟢 **P2** | Prompt weighting — `(word:1.5)` syntax for emphasis
- [ ] 🟢 **P2** | CLIP skip — skip last N layers of CLIP for style control

---

## 🎬 Video Generation

### Scene DSL
- [ ] 🔴 **P0** | Define `Scene` struct — width, height, fps, background, tracks, audio
- [ ] 🔴 **P0** | Define `Track` struct — layer of clips with timing
- [ ] 🔴 **P0** | Define `Clip` variants — SvgClip, ImageClip, HtmlClip, TextClip, ColorClip
- [ ] 🔴 **P0** | Define `Transition` types — Fade, Slide, Zoom, Wipe, Dissolve
- [ ] 🔴 **P0** | Implement JSON parser for Scene DSL — with validation and helpful errors
- [ ] 🔴 **P0** | Implement duration calculation — total scene length from tracks/clips
- [ ] 🟡 **P1** | Define JSON schema for Scene DSL — for MCP tool input validation
- [ ] 🟡 **P1** | Implement scene builder API — programmatic scene construction
- [ ] 🟢 **P2** | Define `VideoClip` variant — embed existing video segments

### FrameRenderer Trait
- [ ] 🔴 **P0** | Define `FrameRenderer` trait — `render_frame(clip, time, width, height) -> RgbaImage`
- [ ] 🔴 **P0** | Define `ClipType` enum — for renderer capability matching
- [ ] 🔴 **P0** | Implement frame compositing — Porter-Duff alpha compositing for layers

### SVG Renderer
- [ ] 🔴 **P0** | Implement `SvgFrameRenderer` — rasterize SVG at specified dimensions
- [ ] 🔴 **P0** | Handle static SVGs — rasterize once, reuse frame for duration
- [ ] 🟡 **P1** | Handle animated SVGs — evaluate SMIL/CSS at time `t`, rasterize
- [ ] 🟡 **P1** | Handle viewBox scaling — fit SVG to target video resolution
- [ ] 🟡 **P1** | Implement animation state interpolation — resolve timing, evaluate values

### Native HTML Renderer
- [ ] 🟡 **P1** | Integrate `hyper_render` or Blitz layout engine
- [ ] 🟡 **P1** | Implement `HtmlFrameRenderer` — render HTML/CSS to pixels
- [ ] 🟡 **P1** | Support common HTML elements — div, span, p, h1-h6, img, table
- [ ] 🟡 **P1** | Support CSS flexbox layout — modern layout without browser
- [ ] 🟡 **P1** | Support CSS colors, fonts, borders, padding, margin
- [ ] 🟡 **P1** | Implement CSS animation interpolation at time `t`
- [ ] 🟢 **P2** | Support embedded images — base64 or file path
- [ ] 🟢 **P2** | Report capability matrix — which CSS features are supported

### Browser Renderer (Headless Chrome CDP)
- [ ] 🟡 **P1** | Integrate `chromiumoxide` or `headless_chrome` crate
- [ ] 🟡 **P1** | Implement `BrowserFrameRenderer` — full browser rendering fidelity
- [ ] 🟡 **P1** | Chrome lifecycle management — find binary, launch headless, cleanup
- [ ] 🟡 **P1** | Screenshot capture — `Page.captureScreenshot` via CDP
- [ ] 🟡 **P1** | Configure viewport to match video resolution
- [ ] 🟡 **P1** | Animation frame capture — pause animations, seek to time, screenshot
- [ ] 🟡 **P1** | Timeout handling — detect and recover from hung renders
- [ ] 🟢 **P2** | Connection pooling — reuse Chrome instances for batch rendering
- [ ] 🟢 **P2** | Chromium auto-download — provide download helper when not found

### Renderer Auto-Selection
- [ ] 🔴 **P0** | Implement content complexity analyzer — detect JS, complex CSS, iframes
- [ ] 🔴 **P0** | Implement renderer availability check — is Chrome installed?
- [ ] 🔴 **P0** | Implement `RendererSelector` with fallback chain — SVG → HTML → Browser
- [ ] 🟡 **P1** | Allow user override of renderer via tool parameter
- [ ] 🟡 **P1** | Log renderer selection with reasoning

### CSS Animation Parser & Interpolator
- [ ] 🟡 **P1** | Parse CSS `@keyframes` rules into structured data
- [ ] 🟡 **P1** | Parse `animation` shorthand properties
- [ ] 🟡 **P1** | Evaluate CSS animation property values at arbitrary time `t`
- [ ] 🟡 **P1** | Support CSS `transform` interpolation — decompose, interpolate, recompose
- [ ] 🟡 **P1** | Support CSS color interpolation — linear in sRGB space
- [ ] 🟢 **P2** | Support CSS `transition` interpolation
- [ ] 🟢 **P2** | Support CSS `calc()` in animation values

### Transitions
- [ ] 🔴 **P0** | Implement Fade transition — alpha cross-dissolve
- [ ] 🔴 **P0** | Implement Slide Left transition — incoming slides over outgoing
- [ ] 🔴 **P0** | Implement Slide Right transition — incoming slides over outgoing
- [ ] 🟡 **P1** | Implement Slide Up/Down transitions
- [ ] 🟡 **P1** | Implement Zoom In transition — outgoing zooms in while fading
- [ ] 🟡 **P1** | Implement Zoom Out transition — camera pulls back to reveal incoming
- [ ] 🟡 **P1** | Implement Wipe transition — directional reveal with configurable angle
- [ ] 🟡 **P1** | Implement Dissolve transition — pixelated cross-fade with block size
- [ ] 🟡 **P1** | Support easing functions on transitions — control progress curve
- [ ] 🟡 **P1** | Support configurable transition duration — default 0.5s
- [ ] 🟢 **P2** | Implement Iris transition — circular reveal from center
- [ ] 🟢 **P2** | Implement Page Curl transition — 3D page turn effect

### Video Encoding
- [ ] 🔴 **P0** | Detect FFmpeg binary — system PATH or bundled location
- [ ] 🔴 **P0** | Pipe raw RGBA frames to FFmpeg stdin — `tokio::process::Command`
- [ ] 🔴 **P0** | H.264 encoding — CRF quality (0-51), preset (ultrafast → veryslow)
- [ ] 🔴 **P0** | Output MP4 container with H.264
- [ ] 🟡 **P1** | VP9 encoding — CRF quality, row-mt multi-threading
- [ ] 🟡 **P1** | Output WebM container with VP9
- [ ] 🟡 **P1** | Configure pixel format, color space, bitrate
- [ ] 🟡 **P1** | Progress reporting — frame count, encoding speed (fps), ETA
- [ ] 🟡 **P1** | AV1 encoding via `rav1e` — native Rust, no FFmpeg needed
- [ ] 🟢 **P2** | AV1 quality settings — quantizer (0-255), speed (0-10)
- [ ] 🟢 **P2** | AV1 tile-based parallelism for multi-core encoding
- [ ] 🟢 **P2** | GIF output — animated GIF for short clips

### Audio
- [ ] 🟡 **P1** | Load audio files — MP3, WAV, OGG, FLAC
- [ ] 🟡 **P1** | Merge audio track with video — FFmpeg `-filter_complex amerge`
- [ ] 🟡 **P1** | Trim/pad audio to match video duration
- [ ] 🟡 **P1** | Volume normalization — consistent loudness
- [ ] 🟢 **P2** | Fade in/out at start/end — configurable duration
- [ ] 🟢 **P2** | Mix multiple audio tracks — background music + narration
- [ ] 🟢 **P2** | Audio ducking — lower music volume when narration plays

### Templates
- [ ] 🟡 **P1** | Slideshow template — images with transitions and background music
- [ ] 🟡 **P1** | Text Explainer template — animated bullet points appearing sequentially
- [ ] 🟡 **P1** | Data Dashboard template — animated charts appearing with data
- [ ] 🟡 **P1** | Social Media Clip template — 9:16 vertical with text overlay and CTA
- [ ] 🟡 **P1** | Product Showcase template — zoom on images with feature callouts
- [ ] 🟢 **P2** | Countdown Timer template — animated countdown with visual effects
- [ ] 🟢 **P2** | Before/After template — split-screen comparison with sliding reveal

### Utility Tools
- [ ] 🟡 **P1** | Video preview — generate sample frames without full video encoding
- [ ] 🟡 **P1** | Frame extraction — extract frames from existing video at specified timestamps
- [ ] 🟡 **P1** | Video trimming — cut video to specified time range
- [ ] 🟢 **P2** | Thumbnail generation — auto-select best frame for thumbnail
- [ ] 🟢 **P2** | Video metadata — read/write title, author, creation date

---

## 🎨 Image Processing

### wgpu Pipeline
- [ ] 🔴 **P0** | Initialize `wgpu::Instance` with Vulkan/Metal/DX12 backends
- [ ] 🔴 **P0** | Request adapter with power preference from config
- [ ] 🔴 **P0** | Create device and queue with compute pipeline support
- [ ] 🔴 **P0** | Implement `GpuContext` struct — manage device lifecycle
- [ ] 🔴 **P0** | Implement `ImageBuffer` — upload RGBA pixels to GPU storage buffer
- [ ] 🔴 **P0** | Implement `ImageBuffer::download()` — read back processed pixels
- [ ] 🔴 **P0** | Create compute pipeline builder — load WGSL → pipeline → bind groups
- [ ] 🟡 **P1** | Handle GPU device lost — re-initialize and retry
- [ ] 🟢 **P2** | Implement shader hot-reload — watch WGSL files for development

### WGSL Shaders
- [ ] 🔴 **P0** | Implement Gaussian Blur shader — separable H/V passes, configurable radius
- [ ] 🔴 **P0** | Implement Brightness/Contrast shader — linear adjustment with clamping
- [ ] 🔴 **P0** | Implement Grayscale shader — BT.709 luminance-weighted desaturation
- [ ] 🔴 **P0** | Implement Invert shader — per-channel inversion
- [ ] 🟡 **P1** | Implement Box Blur shader — fast blur for previews
- [ ] 🟡 **P1** | Implement Sharpen shader — unsharp mask (blur → subtract → add)
- [ ] 🟡 **P1** | Implement Hue/Saturation/Lightness shader — HSL color space adjustment
- [ ] 🟡 **P1** | Implement Color Temperature shader — Kelvin-to-RGB warm/cool shift
- [ ] 🟡 **P1** | Implement Sepia shader — color matrix transformation
- [ ] 🟡 **P1** | Implement Threshold shader — binary black/white by luminance
- [ ] 🟡 **P1** | Implement Posterize shader — reduce color levels per channel
- [ ] 🟡 **P1** | Implement Vignette shader — radial darkening from center
- [ ] 🟡 **P1** | Implement Edge Detection shader — Sobel operator
- [ ] 🟡 **P1** | Implement Emboss shader — directional convolution
- [ ] 🟡 **P1** | Implement Composite shader — Porter-Duff operations (over, in, out, atop, xor)
- [ ] 🟢 **P2** | Implement Noise shader — Perlin/simplex noise overlay
- [ ] 🟢 **P2** | Implement Chromatic Aberration shader — RGB channel offset
- [ ] 🟢 **P2** | Implement Pixelate shader — mosaic effect with configurable block size
- [ ] 🟢 **P2** | Implement Tilt-Shift shader — fake miniature effect with blur gradient

### CPU Fallback
- [ ] 🔴 **P0** | Implement CPU versions of all P0 shaders — using `image` crate
- [ ] 🟡 **P1** | Implement CPU versions of all P1 shaders
- [ ] 🟡 **P1** | Optimize with SIMD — AVX2/NEON for horizontal blur passes
- [ ] 🟡 **P1** | Parallelize with `rayon` — split image into horizontal strips
- [ ] 🔴 **P0** | Implement `ProcessingBackend` trait — `Gpu` and `Cpu` variants
- [ ] 🔴 **P0** | Auto-select backend based on `HardwareProfile`
- [ ] 🟡 **P1** | Allow manual backend override via config/tool parameter
- [ ] 🟡 **P1** | Correctness tests — compare CPU vs GPU output, max 1-bit difference

### Image Transforms
- [ ] 🔴 **P0** | Implement Resize — Lanczos3 algorithm (default)
- [ ] 🔴 **P0** | Implement Resize modes — fit, fill, stretch, contain (letterbox)
- [ ] 🟡 **P1** | Implement Resize algorithms — nearest-neighbor, bilinear, CatmullRom
- [ ] 🟡 **P1** | Support percentage-based dimensions — resize to 50%, 200%, etc.
- [ ] 🔴 **P0** | Implement Crop — absolute coordinates (x, y, w, h)
- [ ] 🟡 **P1** | Implement Crop relative modes — center crop, rule-of-thirds
- [ ] 🔴 **P0** | Implement Rotate 90°/180°/270° — lossless pixel rotation
- [ ] 🟡 **P1** | Implement Rotate arbitrary angle — with background fill color
- [ ] 🔴 **P0** | Implement Flip — horizontal, vertical, both

### Format Conversion & I/O
- [ ] 🔴 **P0** | Read/write PNG — with alpha channel support
- [ ] 🔴 **P0** | Read/write JPEG — quality setting 1-100
- [ ] 🔴 **P0** | Read/write WebP — lossy (quality) and lossless modes
- [ ] 🟡 **P1** | Read/write BMP — basic bitmap support
- [ ] 🟡 **P1** | Read/write TIFF — 8-bit and 16-bit support
- [ ] 🟡 **P1** | Read GIF — first frame extraction
- [ ] 🟡 **P1** | Read/write AVIF — `ravif` with quality and speed settings
- [ ] 🟡 **P1** | Format detection from magic bytes — don't rely on file extension
- [ ] 🟡 **P1** | EXIF handling — read with `kamadak-exif`, copy to output
- [ ] 🟢 **P2** | Read PSD — flattened composite only
- [ ] 🟢 **P2** | Read HDR — Radiance .hdr format

### Batch Processing & Chains
- [ ] 🟡 **P1** | Implement `FilterChain` — ordered list of operations
- [ ] 🟡 **P1** | Optimize chain — keep data on GPU between operations
- [ ] 🟡 **P1** | Implement batch processing — concurrent with `tokio::spawn`
- [ ] 🟡 **P1** | Configurable concurrency limit — default: CPU cores or 1 for GPU
- [ ] 🟡 **P1** | Progress reporting — per-image and per-operation
- [ ] 🟡 **P1** | Error handling — continue on individual failure, collect errors
- [ ] 🟢 **P2** | Glob pattern input — process all `*.jpg` in directory
- [ ] 🟢 **P2** | Template output naming — `{name}_processed.{ext}`

---

## 🧠 Self-Improvement

### CLIP Scorer
- [ ] 🟡 **P1** | Load CLIP image encoder — ViT-B/32 or ViT-L/14 via Candle/ONNX
- [ ] 🟡 **P1** | Load CLIP text encoder — matching architecture
- [ ] 🟡 **P1** | Implement image preprocessing — resize 224x224, CLIP-specific normalize
- [ ] 🟡 **P1** | Implement CLIP BPE tokenizer — text tokenization
- [ ] 🟡 **P1** | Encode image → embedding vector (512/768-dim)
- [ ] 🟡 **P1** | Encode text → embedding vector (512/768-dim)
- [ ] 🟡 **P1** | Compute cosine similarity — image-text relevance score
- [ ] 🟡 **P1** | Normalize score to 0.0-1.0 — based on empirical CLIP distribution
- [ ] 🟢 **P2** | Batch scoring — score multiple images against one prompt
- [ ] 🟢 **P2** | Comparison scoring — rank images by relevance to prompt

### Aesthetic Scorer
- [ ] 🟡 **P1** | Load LAION aesthetic predictor — linear probe on CLIP embeddings
- [ ] 🟡 **P1** | Reuse CLIP image encoder — avoid loading duplicate model
- [ ] 🟡 **P1** | Feed CLIP embedding into aesthetic MLP head
- [ ] 🟡 **P1** | Output aesthetic score 1.0-10.0
- [ ] 🟡 **P1** | Define `QualityReport` struct — clip_score, aesthetic_score, overall, suggestions
- [ ] 🟡 **P1** | Calibrate score ranges — <4 poor, 4-6 average, 6-8 good, 8+ excellent
- [ ] 🟢 **P2** | Generate improvement suggestions based on score analysis
- [ ] 🟢 **P2** | Technical quality metrics — sharpness, exposure, color distribution

### SQLite History
- [ ] 🟡 **P1** | Design SQLite schema — generations table with all fields
- [ ] 🟡 **P1** | Implement table creation and migrations — versioned schema
- [ ] 🟡 **P1** | Implement INSERT — auto-log every generation with parameters and scores
- [ ] 🟡 **P1** | Implement SELECT with filters — by tool, score range, date range, model
- [ ] 🟡 **P1** | Implement UPDATE — add feedback and notes to existing generation
- [ ] 🟡 **P1** | Implement DELETE — purge old entries, cascade to refinements
- [ ] 🟡 **P1** | Create indexes — prompt, score, timestamp for fast queries
- [ ] 🟡 **P1** | Implement aggregation queries — average scores per model, per prompt pattern
- [ ] 🟢 **P2** | History pruning — configurable max entries, auto-delete oldest
- [ ] 🟢 **P2** | Export to JSON — dump history for external analysis
- [ ] 🟢 **P2** | Import from JSON — restore history from backup

### Prompt Refiner
- [ ] 🟡 **P1** | Analyze prompt features — length, keyword presence, specificity level
- [ ] 🟡 **P1** | Build feature-score correlation from history data
- [ ] 🟡 **P1** | Token-level analysis — which words correlate with higher scores
- [ ] 🟡 **P1** | Style keyword database — track actual impact of "masterpiece", "4k", etc.
- [ ] 🟡 **P1** | Negative prompt analysis — effective negatives from history
- [ ] 🟡 **P1** | Implement `refine_prompt()` — return `Vec<PromptSuggestion>`
- [ ] 🟢 **P2** | A/B tracking — when suggestion is used, track if score improved
- [ ] 🟢 **P2** | Prompt template library — curated prompts for common use cases

### Feedback System
- [ ] 🟡 **P1** | Manual feedback API — rate generation as good/bad/neutral
- [ ] 🟡 **P1** | Store feedback in SQLite linked to generation ID
- [ ] 🟡 **P1** | Support user notes — free-text annotation
- [ ] 🟢 **P2** | Feedback aggregation — track satisfaction trends over time
- [ ] 🟢 **P2** | Feedback-weighted learning — good feedback boosts similar prompts

### Auto-Refine Loop
- [ ] 🟡 **P1** | Implement generate → score → refine → regenerate loop
- [ ] 🟡 **P1** | Configurable quality threshold — default 6.0 overall score
- [ ] 🟡 **P1** | Configurable max iterations — default 3
- [ ] 🟡 **P1** | Apply top prompt suggestion per iteration
- [ ] 🟡 **P1** | Keep best result across iterations — compare overall scores
- [ ] 🟡 **P1** | Store refinement chain — link refined to originals in SQLite
- [ ] 🟢 **P2** | Configurable refinement strategy — prompt-only, scheduler, resolution
- [ ] 🟢 **P2** | Analytics — track improvement rates and effective strategies

### Quality Reports
- [ ] 🟡 **P1** | Score distribution histogram — visual chart of score spread
- [ ] 🟡 **P1** | Score trends over time — line chart of quality improvement
- [ ] 🟡 **P1** | Best/worst generations — with parameters for comparison
- [ ] 🟡 **P1** | Prompt effectiveness analysis — which prompts score highest
- [ ] 🟡 **P1** | Model comparison — which model produces best average quality
- [ ] 🟢 **P2** | Scheduler comparison — quality vs speed tradeoffs
- [ ] 🟢 **P2** | Recommendations section — data-driven suggestions for improvement

---

## 🔗 MCP Server

### Server Setup
- [ ] 🔴 **P0** | Implement `OpenMediaServer` struct with `rmcp::ServerHandler`
- [ ] 🔴 **P0** | Implement stdio transport — primary connection method
- [ ] 🔴 **P0** | Implement `name()`, `version()`, `instructions()` metadata
- [ ] 🔴 **P0** | Implement `list_tools()` — return all tool schemas with JSON schemas
- [ ] 🔴 **P0** | Implement `call_tool()` — dispatch to handler functions
- [ ] 🔴 **P0** | Implement graceful shutdown — SIGINT/SIGTERM handling
- [ ] 🟡 **P1** | Implement startup banner — version, hardware, available features
- [ ] 🟡 **P1** | Implement tool input validation — descriptive error messages
- [ ] 🟡 **P1** | Implement concurrency control — limit parallel tool executions
- [ ] 🟢 **P2** | Implement SSE transport — alternative to stdio
- [ ] 🟢 **P2** | Implement tool timeout — cancel long-running operations

### SVG Tools (Phase 1)
- [ ] 🔴 **P0** | `svg_generate_icon` — generate icon by name, style, size, colors
- [x] 🔴 **P0** | `svg_generate_chart` — generate chart from data series and type
- [ ] 🔴 **P0** | `svg_generate_diagram` — generate diagram from structured spec
- [ ] 🔴 **P0** | `svg_create_custom` — create SVG from builder instruction set (JSON)
- [ ] 🔴 **P0** | `svg_optimize` — optimize existing SVG string for file size
- [ ] 🔴 **P0** | `svg_rasterize` — convert SVG to PNG/JPEG/WebP

### Animation Tools (Phase 2)
- [ ] 🔴 **P0** | `animate_svg` — apply animation presets to SVG elements
- [ ] 🔴 **P0** | `animate_create_timeline` — create sequenced animation timeline
- [ ] 🟡 **P1** | `animate_morph_paths` — morph between two SVG paths
- [ ] 🟡 **P1** | `animate_generate_spinner` — generate animated loading spinners
- [ ] 🟡 **P1** | `animate_from_lottie` — convert Lottie JSON to animated SVG
- [ ] 🟡 **P1** | `animate_to_lottie` — convert animated SVG to Lottie JSON

### Image Processing Tools (Phase 3)
- [ ] 🔴 **P0** | `image_apply_filter` — apply filter (blur, sharpen, color adjust, etc.)
- [ ] 🔴 **P0** | `image_resize` — resize with algorithm and mode selection
- [ ] 🔴 **P0** | `image_crop` — crop with absolute or relative coordinates
- [ ] 🔴 **P0** | `image_transform` — rotate, flip geometric transforms
- [ ] 🔴 **P0** | `image_convert` — convert between image formats
- [ ] 🟡 **P1** | `image_batch_process` — apply filter chain to multiple images

### AI Image Tools (Phase 4)
- [ ] 🔴 **P0** | `ai_generate_image` — text-to-image with full parameter control
- [ ] 🟡 **P1** | `ai_image_to_image` — transform existing image with text guidance
- [ ] 🟡 **P1** | `ai_inpaint` — fill masked region with AI-generated content
- [ ] 🟡 **P1** | `ai_upscale` — upscale image 2x or 4x with Real-ESRGAN
- [ ] 🟡 **P1** | `ai_remove_background` — remove background, output PNG with alpha
- [ ] 🟡 **P1** | `ai_list_models` — list models with download status and capabilities

### Video Tools (Phase 5)
- [ ] 🔴 **P0** | `video_create` — create video from Scene DSL
- [ ] 🔴 **P0** | `video_create_slideshow` — quick slideshow from images
- [ ] 🟡 **P1** | `video_add_transition` — add transition between clips
- [ ] 🟡 **P1** | `video_add_audio` — add audio track to video
- [ ] 🟡 **P1** | `video_from_template` — generate from template with parameters
- [ ] 🟡 **P1** | `video_preview` — generate preview frames only
- [ ] 🟡 **P1** | `video_extract_frames` — extract frames from existing video
- [ ] 🟡 **P1** | `video_trim` — trim video to time range

### Self-Improvement Tools (Phase 6)
- [ ] 🟡 **P1** | `improve_score_image` — score with CLIP + aesthetic metrics
- [ ] 🟡 **P1** | `improve_refine_prompt` — get prompt improvement suggestions
- [ ] 🟡 **P1** | `improve_auto_refine` — generate with automatic refinement loop
- [ ] 🟡 **P1** | `improve_feedback` — submit feedback on a generation
- [ ] 🟡 **P1** | `improve_quality_report` — generate quality analytics report

### MCP Resources
- [ ] 🔴 **P0** | `openmedia://capabilities` — hardware profile and available features
- [ ] 🔴 **P0** | `openmedia://models` — list of available/downloaded models
- [ ] 🟡 **P1** | `openmedia://history` — recent generation history
- [ ] 🟡 **P1** | `openmedia://config` — current resolved configuration

### Error Handling & Progress
- [ ] 🔴 **P0** | Implement structured MCP error responses — error code, message, details
- [ ] 🔴 **P0** | Implement input validation for every tool — type checking, range checking
- [ ] 🔴 **P0** | Implement progress notifications — per-step progress for long operations
- [ ] 🟡 **P1** | Implement cancellation support — abort long-running generations
- [ ] 🟡 **P1** | Implement rate limiting — prevent resource exhaustion from rapid calls

---

## 🧪 Testing

### Unit Tests
- [ ] 🔴 **P0** | `openmedia-core` unit tests — config, hardware, errors, types, progress
- [ ] 🔴 **P0** | `openmedia-svg` unit tests — builder, shapes, styles, gradients, filters
- [x] 🔴 **P0** | `openmedia-svg` chart unit tests — axis scaling, data rendering per chart type
- [ ] 🔴 **P0** | `openmedia-svg` diagram unit tests — layout algorithms, edge routing
- [ ] 🔴 **P0** | `openmedia-svg-animate` unit tests — easing functions, timeline, SMIL, CSS
- [ ] 🔴 **P0** | `openmedia-image` unit tests — shader output, transforms, format conversion
- [ ] 🟡 **P1** | `openmedia-image-ai` unit tests — scheduler math, model info, params validation
- [ ] 🟡 **P1** | `openmedia-video` unit tests — scene parsing, transitions, frame compositing
- [ ] 🟡 **P1** | `openmedia-improve` unit tests — scoring, queries, refinement logic
- [ ] 🟡 **P1** | `openmedia-mcp` unit tests — tool dispatch, input validation, error mapping

### Integration Tests
- [ ] 🔴 **P0** | MCP server start/stop test — server starts, `tools/list` returns, server stops
- [ ] 🔴 **P0** | SVG generation end-to-end — call tool, verify valid SVG output
- [ ] 🟡 **P1** | SVG → rasterize chain — generate SVG, rasterize to PNG, verify dimensions
- [ ] 🟡 **P1** | Animation timeline test — create timeline, verify SMIL output timing
- [ ] 🟡 **P1** | Image filter chain — apply multiple filters, verify output
- [ ] 🟡 **P1** | Lottie → SVG conversion — parse Lottie, convert, verify animation
- [ ] 🟡 **P1** | AI generation end-to-end — download test model, generate, verify image
- [ ] 🟡 **P1** | Video creation end-to-end — scene → frames → encode → verify playable
- [ ] 🟡 **P1** | Auto-refine loop — generate → score → refine → verify improvement

### Visual Regression Tests
- [ ] 🟡 **P1** | Icon golden images — each icon × each style at reference size
- [ ] 🟡 **P1** | Chart golden images — each chart type with reference data
- [ ] 🟡 **P1** | Diagram golden images — each diagram type with reference spec
- [ ] 🟡 **P1** | Animation frame captures — keyframes at t=0%, 50%, 100%
- [ ] 🟡 **P1** | Image filter comparisons — before/after for each shader
- [ ] 🟡 **P1** | Transition frame captures — transition at 0%, 25%, 50%, 75%, 100%
- [ ] 🟡 **P1** | Pixel difference threshold — max 1% variation across platforms
- [ ] 🟡 **P1** | Golden file update command — `OPENMEDIA_UPDATE_GOLDEN=1 cargo test`

### Benchmarks
- [ ] 🟡 **P1** | SVG generation throughput — icons, charts, diagrams per second
- [ ] 🟡 **P1** | SVG rasterization speed — at 720p, 1080p, 4K resolutions
- [ ] 🟡 **P1** | Image shader throughput — GPU vs CPU at 720p, 1080p, 4K
- [ ] 🟡 **P1** | AI generation speed — iterations/second per model per backend
- [ ] 🟡 **P1** | Video encoding speed — frames/second per codec
- [ ] 🟡 **P1** | CLIP scoring throughput — images/second
- [ ] 🟡 **P1** | Memory usage profiling — peak memory per operation type
- [ ] 🟢 **P2** | Criterion benchmark suite — statistical analysis with regression detection
- [ ] 🟢 **P2** | CI benchmark tracking — detect >5% regression, fail CI
- [ ] 🟢 **P2** | Benchmark hardware documentation — reproduce results

---

## 📚 Documentation

- [ ] 🔴 **P0** | API documentation — `cargo doc` with examples for all public types
- [ ] 🔴 **P0** | MCP Tool Reference — every tool with input schema, examples, expected output
- [ ] 🟡 **P1** | Architecture Guide — crate dependency graph, data flow diagrams
- [ ] 🟡 **P1** | Getting Started Guide — install, configure, first MCP tool call
- [ ] 🟡 **P1** | Model Guide — which models to download, hardware requirements, quality comparison
- [ ] 🟡 **P1** | Troubleshooting Guide — common errors with solutions
- [ ] 🟡 **P1** | Performance Tuning Guide — GPU selection, batch sizes, quality vs speed
- [ ] 🟢 **P2** | Contributing Guide — dev setup, PR process, code style
- [ ] 🟢 **P2** | CHANGELOG.md — all changes per version
- [ ] 🟢 **P2** | Scene DSL Reference — full JSON schema documentation with examples
- [ ] 🟢 **P2** | Icon Reference — visual gallery of all available icons
- [ ] 🟢 **P2** | Chart Reference — visual gallery with example data for each chart type
- [ ] 🟢 **P2** | Easing Reference — visual curves for all easing functions
- [ ] 🟢 **P2** | Shader Reference — before/after examples for each image filter
- [ ] 🟢 **P2** | Video Template Reference — preview thumbnails and parameter docs

---

## 🚀 Distribution

- [x] 🔴 **P0** | Cross-compile for Linux x86_64 — musl static linking
- [x] 🔴 **P0** | Cross-compile for macOS x86_64 — universal binary
- [x] 🔴 **P0** | Cross-compile for macOS aarch64 (Apple Silicon) — native ARM
- [x] 🔴 **P0** | Cross-compile for Windows x86_64 — MSVC static linking
- [ ] 🟡 **P1** | Cross-compile for Linux aarch64 — ARM servers and Raspberry Pi
- [x] 🟡 **P1** | Binary size optimization — strip debug symbols, LTO, UPX
- [x] 🟡 **P1** | GitHub Release automation — upload binaries on version tag
- [ ] 🟡 **P1** | Install script — `curl -fsSL ... | sh` for Linux/macOS
- [x] 🟡 **P1** | Docker image (CPU) — multi-stage build with minimal runtime
- [ ] 🟡 **P1** | Docker image (GPU) — NVIDIA CUDA base image
- [ ] 🟢 **P2** | Homebrew formula — macOS package manager
- [ ] 🟢 **P2** | AUR package — Arch Linux user repository
- [ ] 🟢 **P2** | Nix flake — reproducible builds for NixOS
- [ ] 🟢 **P2** | Docker Compose for development — with model volume mount
- [ ] 🟢 **P2** | crates.io publishing — library crates for Rust ecosystem
- [ ] 🟢 **P2** | Windows MSI installer — GUI installer for non-technical users

---

## 🔮 Future Ideas

- [ ] 🟢 **P2** | FLUX.1 model support — next-gen text-to-image model via diffusion_rs
- [ ] 🟢 **P2** | LoRA support — load and apply LoRA adapters for style fine-tuning
- [ ] 🟢 **P2** | ControlNet support — guided generation with edge/depth/pose maps
- [ ] 🟢 **P2** | IP-Adapter support — image-guided generation (style transfer)
- [ ] 🟢 **P2** | AI video generation — frame interpolation or model-based (Stable Video Diffusion)
- [ ] 🟢 **P2** | TTS integration — text-to-speech narration for videos
- [ ] 🟢 **P2** | 3D model rendering — basic 3D scene rendering for video frames
- [ ] 🟢 **P2** | Plugin system — third-party extensions for custom tools
- [ ] 🟢 **P2** | Web UI — browser-based interface for non-agent usage
- [ ] 🟢 **P2** | Real-time preview server — WebSocket-based live preview
- [ ] 🟢 **P2** | Prompt library — curated collection of effective prompts with examples
- [ ] 🟢 **P2** | Style presets — one-click art style application (Anime, Photorealistic, Oil Painting, etc.)
- [ ] 🟢 **P2** | Multi-language prompt support — auto-translate non-English prompts
- [ ] 🟢 **P2** | Batch video generation — generate multiple videos from template with data CSV
- [ ] 🟢 **P2** | Audio visualization — generate animated visualizers from audio input
- [ ] 🟢 **P2** | Screen recording integration — capture screen content as video input
- [ ] 🟢 **P2** | AI caption generation — auto-generate captions/subtitles for videos
- [ ] 🟢 **P2** | Watermark system — add configurable watermarks to generated media
- [ ] 🟢 **P2** | CDN integration — auto-upload generated assets to cloud storage
- [ ] 🟢 **P2** | Webhooks — notify external services on generation completion

---

## 📊 Progress Tracker

### Phase Completion

| Phase | Total Items | P0 | P1 | P2 | Done | Progress |
|-------|------------|----|----|-----|------|----------|
| 📦 Project Setup | 25 | 12 | 6 | 7 | 0 | ░░░░░░░░░░ 0% |
| 🏗️ Core Infrastructure | 47 | 23 | 18 | 6 | 0 | ░░░░░░░░░░ 0% |
| 📐 SVG Generation | 75 | 26 | 32 | 17 | 0 | ░░░░░░░░░░ 0% |
| ✨ Animated SVG | 62 | 18 | 29 | 15 | 0 | ░░░░░░░░░░ 0% |
| 🖼️ AI Image Generation | 72 | 20 | 35 | 17 | 0 | ░░░░░░░░░░ 0% |
| 🎬 Video Generation | 61 | 14 | 33 | 14 | 0 | ░░░░░░░░░░ 0% |
| 🎨 Image Processing | 52 | 17 | 24 | 11 | 0 | ░░░░░░░░░░ 0% |
| 🧠 Self-Improvement | 40 | 0 | 28 | 12 | 0 | ░░░░░░░░░░ 0% |
| 🔗 MCP Server | 49 | 22 | 22 | 5 | 0 | ░░░░░░░░░░ 0% |
| 🧪 Testing | 31 | 6 | 19 | 6 | 0 | ░░░░░░░░░░ 0% |
| 📚 Documentation | 15 | 2 | 5 | 8 | 0 | ░░░░░░░░░░ 0% |
| 🚀 Distribution | 16 | 4 | 7 | 5 | 0 | ░░░░░░░░░░ 0% |
| 🔮 Future Ideas | 20 | 0 | 0 | 20 | 0 | ░░░░░░░░░░ 0% |
| **TOTAL** | **565** | **164** | **258** | **143** | **0** | **░░░░░░░░░░ 0%** |

### Priority Breakdown

```
🔴 P0 (MVP):         164 items (29.0%) — Must complete for initial release
🟡 P1 (Important):   258 items (45.7%) — Should complete for full-featured release
🟢 P2 (Nice-to-have): 143 items (25.3%) — Future enhancements and polish
```

### Milestone Targets

| Milestone | Phase | P0 Items | Target Date |
|-----------|-------|----------|-------------|
| **M0: Foundation** | Phase 0 | 35 | Week 1 |
| **M1: SVG MVP** | Phases 1+2 | 44 | Week 3 |
| **M2: Image MVP** | Phase 3 | 17 | Week 5 |
| **M3: AI MVP** | Phase 4 | 20 | Week 9 |
| **M4: Video MVP** | Phase 5 | 14 | Week 12 |
| **M5: Full Release** | Phases 6+7 | 34 | Week 16 |

---

> **Update this file** as tasks are completed. Mark items with `- [x]` when done.  
> Run `grep -c '\- \[x\]' TODO.md` to count completed items.  
> Run `grep -c '\- \[ \]' TODO.md` to count remaining items.
