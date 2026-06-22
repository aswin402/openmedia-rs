use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessOperation {
    GaussianBlur { radius: f32, sigma: Option<f32> },
    BoxBlur { radius: u32 },
    Sharpen { amount: f32, radius: f32, threshold: u8 },
    UnsharpMask { amount: f32, radius: f32, threshold: u8 },
    Brightness { value: i32 },
    Contrast { value: i32 },
    Saturation { value: i32 },
    HueRotate { degrees: f32 },
    Grayscale,
    Sepia { intensity: f32 },
    Invert,
    Threshold { value: u8 },
    ColorMatrix { matrix: [[f32; 5]; 4] },
    Resize { width: u32, height: u32, method: ResizeMethod },
    Crop { x: u32, y: u32, width: u32, height: u32 },
    Rotate { angle: f64, expand: bool },
    FlipHorizontal,
    FlipVertical,
    Pad { top: u32, right: u32, bottom: u32, left: u32, color: [u8; 4] },
    Composite { overlay: String, x: i32, y: i32, blend_mode: BlendMode, opacity: f32 },
}

/// Blend modes for compositing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
}

/// Image resize method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResizeMethod {
    Nearest,
    Bilinear,
    Lanczos3,
}

impl BlendMode {
    /// Blend two pixel values (0.0–1.0)
    pub fn blend(&self, src: f32, dst: f32) -> f32 {
        match self {
            Self::Normal => src,
            Self::Multiply => src * dst,
            Self::Screen => 1.0 - (1.0 - src) * (1.0 - dst),
            Self::Overlay => {
                if dst < 0.5 { 2.0 * src * dst }
                else { 1.0 - 2.0 * (1.0 - src) * (1.0 - dst) }
            }
            Self::Darken => src.min(dst),
            Self::Lighten => src.max(dst),
            Self::ColorDodge => {
                if src >= 1.0 { 1.0 } else { (dst / (1.0 - src)).min(1.0) }
            }
            Self::ColorBurn => {
                if src <= 0.0 { 0.0 } else { 1.0 - ((1.0 - dst) / src).min(1.0) }
            }
            Self::HardLight => {
                if src < 0.5 { 2.0 * src * dst }
                else { 1.0 - 2.0 * (1.0 - src) * (1.0 - dst) }
            }
            Self::SoftLight => {
                if src < 0.5 {
                    dst - (1.0 - 2.0 * src) * dst * (1.0 - dst)
                } else {
                    let d = if dst <= 0.25 {
                        ((16.0 * dst - 12.0) * dst + 4.0) * dst
                    } else {
                        dst.sqrt()
                    };
                    dst + (2.0 * src - 1.0) * (d - dst)
                }
            }
            Self::Difference => (src - dst).abs(),
            Self::Exclusion => src + dst - 2.0 * src * dst,
        }
    }
}

pub struct DummyGpuPipeline;

impl DummyGpuPipeline {
    pub fn new() -> Self {
        Self
    }

    pub fn is_available(&self) -> bool {
        false
    }
}

pub mod cpu;
pub use cpu::apply_cpu_operation;

pub mod gpu;
pub use gpu::apply_gpu_operation;

pub mod transforms;
pub use transforms::{resize_image, crop_image};

pub mod io;
pub use io::write_image_with_format;
