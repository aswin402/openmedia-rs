# Custom Font Loading Design Spec

**Date**: 2026-06-24  
**Status**: Draft  

---

## 1. Goal
Provide the ability to load dynamic fonts (local `.ttf`/`.otf` files or remote font binary URLs) on the fly during scene and video compilation. This ensures custom brand typography is correctly rendered in both the HTML/CSS (Chromium-based) and SVG (resvg-based) rendering pipelines.

---

## 2. Core Font Loading Engine

### Font Specifications schema
The video Scene DSL and template parameters will support a `custom_fonts` array:

```json
"custom_fonts": [
  {
    "family": "BrandFont",
    "src": "assets/fonts/BrandFont.ttf"
  },
  {
    "family": "GoogleRoboto",
    "src": "https://github.com/google/fonts/raw/main/ofl/roboto/Roboto-Regular.ttf"
  }
]
```

### Font Resolution & Downloading
Before rendering frames, we resolve each specified font:
1. **Local Files**: Verify the path exists and read the file into memory bytes.
2. **Remote URLs**: If the source starts with `http://` or `https://`, check a local cache directory (e.g. `assets/fonts/cache/<hash_of_url>.ttf`). If not cached, download the binary font via `reqwest` and save it to the cache directory.
3. Keep an in-memory map of `family_name -> Vec<u8>` for all successfully loaded fonts.

---

## 3. Pipeline Integration

### 1. SVG/CPU Path (resvg & fontdb)
By default, `resvg` uses `usvg::Options` with an `Arc<fontdb::Database>`.
Instead of creating `usvg::Options::default()` per frame, we will build a custom `usvg::Options` with a pre-configured font database:
```rust
let mut fontdb = resvg::usvg::fontdb::Database::new();
fontdb.load_system_fonts();

// For each resolved custom font:
fontdb.load_font_data(font_bytes);
```

### 2. HTML/CSS Path (Chromium)
To render custom fonts in Chromium without triggering file-origin errors or network sandboxing blockages during animation playback, we inject base64-encoded `@font-face` style declarations in the HTML `<head>`:

```html
<style>
  @font-face {
    font-family: 'BrandFont';
    src: url('data:font/truetype;charset=utf-8;base64,BASE64_DATA...') format('truetype');
  }
  @font-face {
    font-family: 'GoogleRoboto';
    src: url('data:font/truetype;charset=utf-8;base64,BASE64_DATA...') format('truetype');
  }
</style>
```

---

## 4. Video Template Updates

The `video_from_template` tool parameters JSON (`req.parameters`) will support `"custom_fonts"`. We will parse it and pass it to the generated `VideoScene` struct.

### VideoScene Struct Modifications
We will add `custom_fonts` to `VideoScene` in `crates/openmedia-video/src/lib.rs`:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomFontSpec {
    pub family: String,
    pub src: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoScene {
    // ... existing fields ...
    pub custom_fonts: Option<Vec<CustomFontSpec>>,
}
```

---

## 5. Testing Strategy
We will add an integration test:
1. Specify a local mock/test `.ttf` file.
2. Build a video scene using this custom font.
3. Verify that the font is successfully registered with `fontdb` and correctly injected into the generated HTML output.
