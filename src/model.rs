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
    pub fn new(model_fc: ModelFC) -> std::result::Result<Self, anyhow::Error> {
        if (model_fc.rating < 1) || (model_fc.rating > 10) {
            return Err(anyhow::Error::msg("Rating must be between 1 and 10"));
        }
        Ok(Self {
            category: model_fc.category,
            created_at: Utc::now().to_string(),
            duration_sec: model_fc.duration_sec,
            comment: model_fc.comment,
            rating: model_fc.rating,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let model_fc = ModelFC {
            category: "category".to_string(),
            duration_sec: 10,
            rating: 5,
            comment: Some("comment".to_string()),
        };
        let model = Model::new(model_fc).unwrap();
        assert_eq!(model.category, "category");
        assert_eq!(model.duration_sec, 10);
        assert_eq!(model.rating, 5);
        assert_eq!(model.comment, Some("comment".to_string()));
    }

    #[test]
    fn test_new_invalid_rating() {
        let model_fc = ModelFC {
            category: "category".to_string(),
            duration_sec: 10,
            rating: 11,
            comment: Some("comment".to_string()),
        };
        let model = Model::new(model_fc);
        assert!(model.is_err());
    }
}
