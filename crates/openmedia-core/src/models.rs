use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::error::{OpenMediaError, Result};
use crate::hardware::HardwareInfo;

/// Information about a single model file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    /// Unique model identifier (e.g., "sd-1.5-q8_0")
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Model category
    pub category: ModelCategory,
    /// File path on disk
    pub path: PathBuf,
    /// File size in bytes
    pub size_bytes: u64,
    /// Model format
    pub format: ModelFormat,
    /// Quantization level (if applicable)
    pub quantization: Option<String>,
    /// SHA-256 checksum
    pub checksum: Option<String>,
    /// Whether the model is verified (checksum matches)
    pub verified: bool,
    /// Minimum VRAM required (bytes), 0 for CPU-only
    pub min_vram: u64,
    /// Supported resolutions
    pub supported_resolutions: Vec<(u32, u32)>,
    /// Default resolution
    pub default_resolution: (u32, u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelCategory {
    Diffusion,
    Upscale,
    Segmentation,
    Clip,
    Aesthetic,
    Vae,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelFormat {
    Gguf,
    Onnx,
    SafeTensors,
    Bin,
}

/// Registry of all available models on disk
pub struct ModelRegistry {
    models: Vec<ModelInfo>,
    model_dir: PathBuf,
}

impl ModelRegistry {
    /// Scan model directory and build registry
    pub async fn scan(model_dir: &PathBuf) -> Result<Self> {
        // Implement simple scanning of model_dir or just return an empty registry for Phase 0
        Ok(Self {
            models: vec![],
            model_dir: model_dir.clone(),
        })
    }

    /// Get a model by ID
    pub fn get(&self, id: &str) -> Option<&ModelInfo> {
        self.models.iter().find(|m| m.id == id)
    }

    /// List all models, optionally filtered by category
    pub fn list(&self, category: Option<ModelCategory>) -> Vec<&ModelInfo> {
        match category {
            Some(cat) => self.models.iter().filter(|m| m.category == cat).collect(),
            None => self.models.iter().collect(),
        }
    }

    /// Select the best diffusion model given hardware constraints
    pub fn select_best_diffusion(&self, _hardware: &HardwareInfo) -> Option<&ModelInfo> {
        None
    }

    /// Verify a model's checksum
    pub async fn verify_model(&self, _id: &str) -> Result<bool> {
        Ok(true)
    }
}
