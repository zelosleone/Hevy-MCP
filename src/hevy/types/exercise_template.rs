use serde::Deserializer;
use serde::{Deserialize, Serialize};

use crate::hevy::types::common::{Equipment, ExerciseType, MuscleGroup, deserialize_string_id};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseTemplate {
    #[serde(deserialize_with = "deserialize_string_id")]
    pub id: String,
    pub title: String,
    #[serde(rename = "type")]
    pub exercise_type: ExerciseType,
    pub primary_muscle_group: MuscleGroup,
    #[serde(default)]
    pub secondary_muscle_groups: Vec<MuscleGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equipment: Option<Equipment>,
    pub is_custom: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseTemplatesResponse {
    pub exercise_templates: Vec<ExerciseTemplate>,
}

#[derive(Debug, Clone)]
pub(crate) struct ExerciseTemplateResponse(pub ExerciseTemplate);

impl ExerciseTemplateResponse {
    pub fn into_exercise_template(self) -> ExerciseTemplate {
        self.0
    }
}

impl<'de> Deserialize<'de> for ExerciseTemplateResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum ExerciseTemplateResponseBody {
            Wrapped { exercise_template: ExerciseTemplate },
            Direct(ExerciseTemplate),
        }

        let value = ExerciseTemplateResponseBody::deserialize(deserializer)?;
        let resolved = match value {
            ExerciseTemplateResponseBody::Wrapped { exercise_template } => exercise_template,
            ExerciseTemplateResponseBody::Direct(exercise_template) => exercise_template,
        };

        Ok(ExerciseTemplateResponse(resolved))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateExerciseTemplateInput {
    pub title: String,
    pub exercise_type: ExerciseType,
    pub equipment_category: Equipment,
    pub muscle_group: MuscleGroup,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_muscles: Option<Vec<MuscleGroup>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateExerciseTemplateResponse {
    #[serde(deserialize_with = "deserialize_string_id")]
    pub id: String,
}
