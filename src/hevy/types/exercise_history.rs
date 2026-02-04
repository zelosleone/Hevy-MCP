use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::hevy::types::common::{SetType, deserialize_string_id};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseHistoryResponse {
    pub exercise_history: Vec<ExerciseHistoryEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseHistoryEntry {
    #[serde(deserialize_with = "deserialize_string_id")]
    pub workout_id: String,
    pub workout_title: String,
    pub workout_start_time: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workout_end_time: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "deserialize_string_id")]
    pub exercise_template_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_kg: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reps: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_meters: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rpe: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_metric: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_type: Option<SetType>,
}
