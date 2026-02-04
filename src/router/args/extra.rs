use serde::Deserialize;

use crate::hevy::{Equipment, ExerciseType, MuscleGroup};

#[derive(Deserialize)]
pub(crate) struct WorkoutEventsArgs {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub since: Option<String>,
}

#[derive(Deserialize)]
pub(crate) struct ExerciseHistoryArgs {
    pub exercise_template_id: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

#[derive(Deserialize)]
pub(crate) struct CreateExerciseTemplateArgs {
    pub title: String,
    pub exercise_type: ExerciseType,
    pub equipment_category: Equipment,
    pub muscle_group: MuscleGroup,
    pub other_muscles: Option<Vec<MuscleGroup>>,
}
