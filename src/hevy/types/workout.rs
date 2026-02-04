use chrono::{DateTime, Utc};
use serde::Deserializer;
use serde::de;
use serde::{Deserialize, Serialize};

use crate::hevy::types::common::{SetType, deserialize_option_string_id, deserialize_string_id};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workout {
    #[serde(deserialize_with = "deserialize_string_id")]
    pub id: String,
    pub title: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "deserialize_option_string_id"
    )]
    pub routine_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub is_private: bool,
    pub start_time: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<DateTime<Utc>>,
    #[serde(default)]
    pub exercises: Vec<WorkoutExercise>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutsResponse {
    pub workouts: Vec<Workout>,
}

#[derive(Debug, Clone)]
pub(crate) struct WorkoutResponse(pub Workout);

impl WorkoutResponse {
    pub fn into_workout(self) -> Workout {
        self.0
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum WorkoutOrList {
    One(Workout),
    Many(Vec<Workout>),
}

impl<'de> Deserialize<'de> for WorkoutResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum WorkoutResponseBody {
            Wrapped { workout: WorkoutOrList },
            Direct(WorkoutOrList),
        }

        let value = WorkoutResponseBody::deserialize(deserializer)?;
        let workout = match value {
            WorkoutResponseBody::Wrapped { workout } => workout,
            WorkoutResponseBody::Direct(workout) => workout,
        };

        let resolved = match workout {
            WorkoutOrList::One(value) => value,
            WorkoutOrList::Many(mut value) => value
                .pop()
                .ok_or_else(|| de::Error::custom("Empty workout list"))?,
        };

        Ok(WorkoutResponse(resolved))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutExercise {
    pub index: u32,
    #[serde(deserialize_with = "deserialize_string_id")]
    pub exercise_template_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(alias = "supersets_id")]
    pub superset_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(default)]
    pub sets: Vec<ExerciseSet>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseSet {
    pub index: u32,
    #[serde(rename = "type")]
    pub set_type: SetType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_kg: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reps: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_meters: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rpe: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_metric: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutInput {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub is_private: bool,
    pub start_time: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<DateTime<Utc>>,
    #[serde(default)]
    pub exercises: Vec<WorkoutExerciseInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutExerciseInput {
    pub exercise_template_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub superset_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(default)]
    pub sets: Vec<SetInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetInput {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub set_type: Option<SetType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_kg: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reps: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_meters: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rpe: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_metric: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutCountResponse {
    pub workout_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutEventsResponse {
    pub page: u32,
    pub page_count: u32,
    pub events: Vec<WorkoutEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "deserialize_option_string_id"
    )]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workout: Option<Workout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
}
