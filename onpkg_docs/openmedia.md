---
name: openmedia
description: Guidelines and best practices for creating stunning vector graphics (SVGs), data charts, flowcharts, animations, and videos using the openmedia-rs tools and MCP server. Helps the agent achieve visually pleasing, professional-grade output with perfect colors, typography, keyframes, transitions, and audio configs.
---

This skill guides the design, structure, and execution of high-quality media generated via the `openmedia-rs` library and MCP server. It ensures that output vector graphics, charts, diagrams, and videos avoid generic "AI-generated" looks and instead utilize modern, high-quality design systems, motion, and typography.

---

## 🎨 1. Design Aesthetics & Visual "Good Taste"

When generating layouts, colors, or templates, follow these rules to ensure the output feels premium:

### Color Systems
- **Avoid Defaults**: Never use pure red (`#ff0000`), pure blue (`#0000ff`), or default saturated colors.
- **Tailored Palettes**: Commit to harmonious HSL/HEX palettes. Recommended:
  - **Sleek Dark Mode**: Deep blues/purples (`#1a1a2e`, `#162447`, `#252b48`) paired with vibrant cyan or teal accents (`#00adb5`).
  - **Glassmorphism**: Translucent fills (`rgba(255,255,255,0.08)`) with semi-transparent borders (`rgba(255,255,255,0.15)`) over rich gradient backgrounds.
  - **Minimalist Clean**: Off-whites (`#f8fafc`, `#f1f5f9`) paired with slate/indigo text and accents (`#1e293b`, `#4f46e5`).
- **Contrast**: Maintain an accessible contrast ratio between text elements and backgrounds.

### Typography
- **Font Families**: By default, use `sans-serif` or load high-quality custom web fonts (e.g., `Roboto`, `Inter`, `Montserrat`) using the custom fonts parameter.
- **Font Weights**: Pair bold headers (font weight `700` or `800`) with readable body text (font weight `400`).

---

## 📐 2. SVG & Icon Generation (`create_svg`, `create_icon`)

When creating raw SVGs or rendering icons:
- **Clean Structure**: Define clear dimensions (`width`, `height`) and wrap items in logical coordinate spaces.
- **Embedded Icons**: Use built-in Lucide/Feather icon templates by requesting them by name (e.g., `home`, `user`, `settings`, `check`, `arrow-right`).
- **Scalable Coordinates**: Always use percentage-based dimensions or calculate responsive scale offsets for custom path shapes.

---

## 📊 3. Data Chart Visualizations (`create_chart`)

When using the chart generator:
- **Select the Right Type**:
  - `bar`: For category comparisons.
  - `line`: For trends over time.
  - `pie`: For part-to-whole relationships (prefer less than 7 slices).
- **Themes**: Match the chart theme (`dark` or `light`) to the background of the parent scene.
- **Layout Padding**: Maintain margin spacing (at least 40px–60px) around chart axes to prevent labels and values from being clipped.
- **Data Series Format**: Provide a list of objects containing `label` (string) and `value` (float). Ensure the array is never empty.

---

## 🎬 4. Video Scenes & Animations (`video_create`, `video_from_template`)

### Native Elements & Offlines Rendering
- **Use Native Elements First**: When running offline or to ensure maximum performance, utilize native `shape`, `text`, and `chart` elements. Avoid `html` and `code` cards unless a headless browser environment (Chrome/Chromium) is fully configured and active.
- **Deterministic Keyframes**: Always define explicit keyframe timelines to animate properties like:
  - `opacity`: Fade in (`0.0` to `1.0`) or fade out.
  - `scale` or `scale_x`/`scale_y`: Grow/shrink elements smoothly.
  - `rotation`: Rotate shapes or text.
  - `x`/`y` offsets: Move elements dynamically across the screen.

### Transitions & Easing
- **Smooth Easing**: Specify easing overrides in keyframes or transitions. Avoid abrupt cuts.
  - `ease_in_out`: For organic, natural movement.
  - `ease_out`: For elements entering the screen (decelerates at the end).
  - `ease_in`: For elements exiting the screen (accelerates at the end).
- **Transition Overrides**: Use parameter overrides (`transition_type`, `transition_duration`, `transition_easing`) in templates to fine-tune transitions between scenes.

### Audio Mixing
- **Multi-Track Audio**: Mux background music and voiceover overlays simultaneously.
- **Volume & Fading**: Set volume levels (`volume` range `0.0` to `1.0`) to ensure background music does not drown out speech. Use `fade_in` and `fade_out` configurations to smooth audio boundaries.

---

## 🔄 5. Self-Improvement & Prompt Refinement (`improve_*`)

If visual generation alignment is low:
- **Iterative Scoring**: Run the scoring MCP method to calculate CLIP text-image cosine similarities and aesthetic rating predictions.
- **Prompt Refinement Suffixes**: Automatically expand simple prompts into highly descriptive instructions detailing lighting (e.g., "studio lighting", "soft glow"), style ("vector graphic", "clean rendering"), and details, while utilizing negative parameters to filter out blurred artifacts.
