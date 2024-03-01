use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Model {
    pub category: String,
    pub created_at: String,
    pub duration_sec: u64,
    pub rating: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelFC {
    pub category: String,
    pub duration_sec: u64,
    pub rating: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

impl Model {
    pub fn new(model_fc: ModelFC) -> Self {
        Self {
            category: model_fc.category,
            created_at: Utc::now().to_string(),
            duration_sec: model_fc.duration_sec,
            comment: model_fc.comment,
            rating: model_fc.rating,
        }
    }
}
