use chrono::{DateTime, Utc};
use serde::Deserializer;
use serde::de;
use serde::{Deserialize, Serialize};

use crate::hevy::types::common::{
    SetType, deserialize_option_string_id, deserialize_option_u32, deserialize_string_id,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Routine {
    #[serde(deserialize_with = "deserialize_string_id")]
    pub id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "deserialize_option_string_id"
    )]
    pub folder_id: Option<String>,
    #[serde(default)]
    pub exercises: Vec<RoutineExercise>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutinesResponse {
    pub routines: Vec<Routine>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct RoutineResponse {
    #[serde(deserialize_with = "deserialize_routine")]
    pub routine: Routine,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum RoutineOrList {
    One(Routine),
    Many(Vec<Routine>),
}

fn deserialize_routine<'de, D>(deserializer: D) -> Result<Routine, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum RoutineResponseBody {
        Wrapped { routine: RoutineOrList },
        Direct(RoutineOrList),
    }

    let value = RoutineResponseBody::deserialize(deserializer)?;
    let routine = match value {
        RoutineResponseBody::Wrapped { routine } => routine,
        RoutineResponseBody::Direct(routine) => routine,
    };

    match routine {
        RoutineOrList::One(value) => Ok(value),
        RoutineOrList::Many(mut value) => value
            .pop()
            .ok_or_else(|| de::Error::custom("Empty routine list")),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutineExercise {
    pub index: u32,
    #[serde(deserialize_with = "deserialize_string_id")]
    pub exercise_template_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(alias = "supersets_id")]
    pub superset_id: Option<u32>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "deserialize_option_u32"
    )]
    pub rest_seconds: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(default)]
    pub sets: Vec<RoutineSet>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutineSet {
    pub index: u32,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub set_type: Option<SetType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_kg: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reps: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rep_range: Option<RepRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_meters: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_metric: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutineInput {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
    #[serde(default)]
    pub exercises: Vec<RoutineExerciseInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutineExerciseInput {
    pub exercise_template_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub superset_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rest_seconds: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(default)]
    pub sets: Vec<RoutineSetInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutineSetInput {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub set_type: Option<SetType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_kg: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reps: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rep_range: Option<RepRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_meters: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_metric: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepRange {
    pub start: u32,
    pub end: u32,
}
