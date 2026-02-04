use chrono::{DateTime, Utc};
use serde::Deserializer;
use serde::{Deserialize, Serialize};

use crate::hevy::types::common::deserialize_string_id;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutineFolder {
    #[serde(deserialize_with = "deserialize_string_id")]
    pub id: String,
    pub title: String,
    pub index: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutineFoldersResponse {
    pub routine_folders: Vec<RoutineFolder>,
}

#[derive(Debug, Clone)]
pub(crate) struct RoutineFolderResponse(pub RoutineFolder);

impl RoutineFolderResponse {
    pub fn into_routine_folder(self) -> RoutineFolder {
        self.0
    }
}

impl<'de> Deserialize<'de> for RoutineFolderResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum RoutineFolderResponseBody {
            Wrapped { routine_folder: RoutineFolder },
            Direct(RoutineFolder),
        }

        let value = RoutineFolderResponseBody::deserialize(deserializer)?;
        let resolved = match value {
            RoutineFolderResponseBody::Wrapped { routine_folder } => routine_folder,
            RoutineFolderResponseBody::Direct(routine_folder) => routine_folder,
        };

        Ok(RoutineFolderResponse(resolved))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutineFolderInput {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<u32>,
}
