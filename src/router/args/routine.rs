use serde::Deserialize;
use serde_with::serde_as;

use crate::hevy::{RepRange, RoutineExerciseInput, RoutineInput, RoutineSetInput, SetType};

#[serde_as]
#[derive(Deserialize)]
pub(crate) struct RoutineArgsCommon {
    pub title: String,
    pub folder_id: Option<String>,
    pub notes: Option<String>,
    #[serde(default)]
    #[serde_as(as = "serde_with::DefaultOnNull")]
    pub exercises: Vec<RoutineExerciseArg>,
}

impl RoutineArgsCommon {
    pub(crate) fn into_input(self) -> RoutineInput {
        RoutineInput {
            title: self.title,
            folder_id: self.folder_id,
            notes: self.notes,
            exercises: self.exercises.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Deserialize)]
pub(crate) struct CreateRoutineArgs {
    #[serde(flatten)]
    pub routine: RoutineArgsCommon,
}

#[serde_as]
#[derive(Deserialize)]
pub(crate) struct RoutineUpdateArgsCommon {
    pub title: String,
    pub notes: Option<String>,
    #[serde(default)]
    #[serde_as(as = "serde_with::DefaultOnNull")]
    pub exercises: Vec<RoutineExerciseArg>,
}

impl RoutineUpdateArgsCommon {
    pub(crate) fn into_input(self) -> RoutineInput {
        RoutineInput {
            title: self.title,
            folder_id: None,
            notes: self.notes,
            exercises: self.exercises.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Deserialize)]
pub(crate) struct UpdateRoutineArgs {
    pub id: String,
    #[serde(flatten)]
    pub routine: RoutineUpdateArgsCommon,
}

#[serde_as]
#[derive(Deserialize)]
pub(crate) struct RoutineExerciseArg {
    pub exercise_template_id: String,
    pub superset_id: Option<u32>,
    pub rest_seconds: Option<u32>,
    pub notes: Option<String>,
    #[serde(default)]
    #[serde_as(as = "serde_with::DefaultOnNull")]
    pub sets: Vec<RoutineSetInputArgs>,
}

#[derive(Deserialize)]
pub(crate) struct RoutineSetInputArgs {
    pub set_type: Option<SetType>,
    pub weight_kg: Option<f64>,
    pub reps: Option<u32>,
    pub rep_range: Option<RepRangeArgs>,
    pub duration_seconds: Option<u32>,
    pub distance_meters: Option<f64>,
    pub custom_metric: Option<f64>,
}

#[derive(Deserialize)]
pub(crate) struct RepRangeArgs {
    pub start: u32,
    pub end: u32,
}

impl From<RoutineSetInputArgs> for RoutineSetInput {
    fn from(value: RoutineSetInputArgs) -> Self {
        Self {
            set_type: value.set_type,
            weight_kg: value.weight_kg,
            reps: value.reps,
            rep_range: value.rep_range.map(|range| RepRange {
                start: range.start,
                end: range.end,
            }),
            duration_seconds: value.duration_seconds,
            distance_meters: value.distance_meters,
            custom_metric: value.custom_metric,
        }
    }
}

impl From<RoutineExerciseArg> for RoutineExerciseInput {
    fn from(value: RoutineExerciseArg) -> Self {
        Self {
            exercise_template_id: value.exercise_template_id,
            superset_id: value.superset_id,
            rest_seconds: value.rest_seconds,
            notes: value.notes,
            sets: value.sets.into_iter().map(Into::into).collect(),
        }
    }
}
