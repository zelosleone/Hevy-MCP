use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_with::serde_as;

use crate::hevy::{SetInput, SetType, WorkoutExerciseInput, WorkoutInput};

#[serde_as]
#[derive(Deserialize)]
pub(crate) struct WorkoutArgsCommon {
    pub title: String,
    #[serde(default)]
    pub is_private: bool,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub description: Option<String>,
    #[serde(default)]
    #[serde_as(as = "serde_with::DefaultOnNull")]
    pub exercises: Vec<WorkoutExerciseArg>,
}

impl WorkoutArgsCommon {
    pub(crate) fn into_input(self) -> WorkoutInput {
        WorkoutInput {
            title: self.title,
            description: self.description,
            is_private: self.is_private,
            start_time: self.start_time,
            end_time: self.end_time,
            exercises: self.exercises.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Deserialize)]
pub(crate) struct CreateWorkoutArgs {
    #[serde(flatten)]
    pub workout: WorkoutArgsCommon,
}

#[derive(Deserialize)]
pub(crate) struct UpdateWorkoutArgs {
    pub id: String,
    #[serde(flatten)]
    pub workout: WorkoutArgsCommon,
}

#[serde_as]
#[derive(Deserialize)]
pub(crate) struct WorkoutExerciseArg {
    pub exercise_template_id: String,
    pub superset_id: Option<u32>,
    pub notes: Option<String>,
    #[serde(default)]
    #[serde_as(as = "serde_with::DefaultOnNull")]
    pub sets: Vec<SetInputArgs>,
}

#[derive(Deserialize)]
pub(crate) struct SetInputArgs {
    pub set_type: Option<SetType>,
    pub weight_kg: Option<f64>,
    pub reps: Option<u32>,
    pub duration_seconds: Option<u32>,
    pub distance_meters: Option<f64>,
    pub rpe: Option<f64>,
    pub custom_metric: Option<f64>,
}

impl From<SetInputArgs> for SetInput {
    fn from(value: SetInputArgs) -> Self {
        Self {
            set_type: value.set_type,
            weight_kg: value.weight_kg,
            reps: value.reps,
            duration_seconds: value.duration_seconds,
            distance_meters: value.distance_meters,
            rpe: value.rpe,
            custom_metric: value.custom_metric,
        }
    }
}

impl From<WorkoutExerciseArg> for WorkoutExerciseInput {
    fn from(value: WorkoutExerciseArg) -> Self {
        Self {
            exercise_template_id: value.exercise_template_id,
            superset_id: value.superset_id,
            notes: value.notes,
            sets: value.sets.into_iter().map(Into::into).collect(),
        }
    }
}
