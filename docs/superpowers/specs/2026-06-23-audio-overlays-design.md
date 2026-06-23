# Audio Overlays Design Spec

**Date**: 2026-06-23  
**Status**: Approved  

---

## 1. Goal
Provide AI agents with the ability to inject background music or custom multi-track audio overlays into template-based videos generated via the `video_from_template` MCP tool.

---

## 2. Architecture & Data Flow

### Interface Parameters
The `video_from_template` tool parameters JSON (`req.parameters`) will support two alternative formats:

1. **Background Music Shorthand (`background_music`)**:
   A single string referencing a local audio file path or URL.
   ```json
   "background_music": "/path/to/soundtrack.mp3"
   ```
   *Behavior:* Maps to a single audio track starting at `0.0` seconds with a default volume of `0.5`.

2. **Custom Audio Tracks Array (`audio_tracks`)**:
   An array of structures mapping directly to `openmedia_video::AudioTrack`.
   ```json
   "audio_tracks": [
     {
       "src": "/path/to/voiceover.wav",
       "start": 0.0,
       "volume": 1.0,
       "fade_in": 0.5,
       "fade_out": 1.0
     },
     {
       "src": "/path/to/bg_music.mp3",
       "start": 0.0,
       "volume": 0.3
     }
   ]
   ```

### Shared Parsing Helper
We will implement `parse_audio_config` in `crates/openmedia-mcp/src/lib.rs` to extract and parse the above parameters dynamically:

```rust
fn parse_audio_config(parameters: &serde_json::Value) -> Option<openmedia_video::AudioConfig> {
    if let Some(tracks_arr) = parameters.get("audio_tracks").and_then(|v| v.as_array()) {
        let mut tracks = Vec::new();
        for track_val in tracks_arr {
            if let Some(src) = track_val.get("src").and_then(|v| v.as_str()) {
                let start = track_val.get("start").and_then(|v| v.as_f64()).unwrap_or(0.0);
                let volume = track_val.get("volume").and_then(|v| v.as_f64()).map(|v| v as f32).unwrap_or(1.0);
                let fade_in = track_val.get("fade_in").and_then(|v| v.as_f64());
                let fade_out = track_val.get("fade_out").and_then(|v| v.as_f64());
                
                tracks.push(openmedia_video::AudioTrack {
                    src: src.to_string(),
                    start,
                    volume,
                    fade_in,
                    fade_out,
                });
            }
        }
        if !tracks.is_empty() {
            return Some(openmedia_video::AudioConfig { tracks });
        }
    } else if let Some(bg_music) = parameters.get("background_music").and_then(|v| v.as_str()) {
        return Some(openmedia_video::AudioConfig {
            tracks: vec![openmedia_video::AudioTrack {
                src: bg_music.to_string(),
                start: 0.0,
                volume: 0.5,
                fade_in: None,
                fade_out: None,
            }],
        });
    }
    None
}
```

This helper will be invoked inside all match arms in the templates match expression to populate the `audio` field of the `VideoScene`.

---

## 3. Testing Strategy
We will add `test_mcp_video_template_with_audio` to verify that `video_from_template` handles the parsing correctly and that the compilation successfully produces a video spec containing the audio configuration.
