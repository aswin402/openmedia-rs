use serde::{Deserialize, Serialize};
use openmedia_core::{Result, OpenMediaError, QualityScore};
use std::sync::Mutex;
use rusqlite::Connection;

pub struct ClipScorer;

impl ClipScorer {
    pub async fn load(_model_dir: &std::path::Path) -> Result<Self> {
        Ok(Self)
    }

    pub async fn score(
        &self,
        _image_path: &std::path::Path,
        _prompt: &str,
    ) -> Result<f32> {
        Ok(0.8)
    }

    pub async fn score_aesthetic(&self, _image_path: &std::path::Path) -> Result<f32> {
        Ok(7.5)
    }
}

pub struct AestheticScorer;

impl AestheticScorer {
    pub async fn load(_model_path: &std::path::Path) -> Result<Self> {
        Ok(Self)
    }

    pub async fn score(&self, _image_path: &std::path::Path) -> Result<f32> {
        Ok(7.5)
    }
}

pub struct GenerationHistory {
    #[allow(dead_code)]
    conn: Mutex<Connection>,
}

impl GenerationHistory {
    pub fn open(db_path: &std::path::Path) -> Result<Self> {
        let conn = Connection::open(db_path)
            .map_err(|e| OpenMediaError::DatabaseError(e.to_string()))?;

        conn.execute_batch(include_str!("../sql/schema.sql"))
            .map_err(|e| OpenMediaError::DatabaseError(e.to_string()))?;

        Ok(Self { conn: Mutex::new(conn) })
    }

    pub fn record(&self, _entry: &GenerationRecord) -> Result<()> {
        Ok(())
    }

    pub fn get(&self, _id: &str) -> Result<Option<GenerationRecord>> {
        Ok(None)
    }

    pub fn query(&self, _filter: &HistoryFilter) -> Result<Vec<GenerationRecord>> {
        Ok(vec![])
    }

    pub fn record_feedback(&self, _feedback: &Feedback) -> Result<()> {
        Ok(())
    }

    pub fn stats(&self) -> Result<HistoryStats> {
        Ok(HistoryStats {
            total_generations: 0,
            total_size_bytes: 0,
            avg_clip_score: None,
            avg_aesthetic_score: None,
            db_size_bytes: 0,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationRecord {
    pub id: String,
    pub created_at: String,
    pub tool_name: String,
    pub request_params: serde_json::Value,
    pub output_path: String,
    pub output_format: String,
    pub output_size: u64,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub duration: Option<f64>,
    pub model_used: Option<String>,
    pub backend_used: Option<String>,
    pub generation_time: f64,
    pub clip_score: Option<f32>,
    pub aesthetic_score: Option<f32>,
    pub refined_from: Option<String>,
    pub refinement_round: u32,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct HistoryFilter {
    pub tool_name: Option<String>,
    pub limit: u32,
    pub offset: u32,
    pub sort_by: String,
    pub sort_order: String,
    pub min_clip_score: Option<f32>,
    pub min_aesthetic: Option<f32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HistoryStats {
    pub total_generations: u64,
    pub total_size_bytes: u64,
    pub avg_clip_score: Option<f32>,
    pub avg_aesthetic_score: Option<f32>,
    pub db_size_bytes: u64,
}

pub struct PromptRefiner {
    pub quality_suffixes: Vec<String>,
    pub negative_defaults: Vec<String>,
}

impl PromptRefiner {
    pub fn new() -> Self {
        Self {
            quality_suffixes: vec![
                "highly detailed".into(),
                "professional".into(),
                "sharp focus".into(),
                "studio lighting".into(),
                "8k uhd".into(),
                "masterpiece".into(),
            ],
            negative_defaults: vec![
                "blurry".into(),
                "low quality".into(),
                "distorted".into(),
                "deformed".into(),
                "disfigured".into(),
                "bad anatomy".into(),
                "watermark".into(),
                "text".into(),
                "signature".into(),
            ],
        }
    }

    pub fn refine(
        &self,
        original_prompt: &str,
        original_negative: &str,
        _scores: &QualityScore,
        _round: u32,
    ) -> RefinedPrompt {
        RefinedPrompt {
            prompt: format!("{}, highly detailed", original_prompt),
            negative_prompt: format!("{}, blurry", original_negative),
            suggested_steps: 30,
            suggested_cfg_scale: 7.5,
            changes: vec!["Added quality keywords".into()],
        }
    }
}

#[derive(Debug, Clone)]
pub struct RefinedPrompt {
    pub prompt: String,
    pub negative_prompt: String,
    pub suggested_steps: u32,
    pub suggested_cfg_scale: f32,
    pub changes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feedback {
    pub generation_id: String,
    pub rating: f32,
    pub feedback: Option<String>,
    pub keep: bool,
    pub created_at: String,
}
