use serde::Deserializer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum StringOrNumber {
    String(String),
    U64(u64),
    I64(i64),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum NumberOrString {
    U64(u64),
    I64(i64),
    String(String),
}

pub fn deserialize_string_id<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value = StringOrNumber::deserialize(deserializer)?;
    Ok(match value {
        StringOrNumber::String(value) => value,
        StringOrNumber::U64(value) => value.to_string(),
        StringOrNumber::I64(value) => value.to_string(),
    })
}

pub fn deserialize_option_string_id<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<StringOrNumber>::deserialize(deserializer)?;
    Ok(value.map(|value| match value {
        StringOrNumber::String(value) => value,
        StringOrNumber::U64(value) => value.to_string(),
        StringOrNumber::I64(value) => value.to_string(),
    }))
}

pub fn deserialize_option_u32<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<NumberOrString>::deserialize(deserializer)?;
    Ok(value.and_then(|value| match value {
        NumberOrString::U64(value) => Some(value as u32),
        NumberOrString::I64(value) => {
            if value < 0 {
                None
            } else {
                Some(value as u32)
            }
        }
        NumberOrString::String(value) => value.parse::<u32>().ok(),
    }))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub page: u32,
    pub page_count: u32,
    #[serde(flatten)]
    pub data: T,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExerciseType {
    WeightReps,
    Reps,
    Duration,
    WeightDuration,
    DistanceDuration,
    Weight,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MuscleGroup {
    Abdominals,
    Abductors,
    Adductors,
    Biceps,
    Calves,
    Cardio,
    Chest,
    Forearms,
    FullBody,
    Glutes,
    Hamstrings,
    Lats,
    LowerBack,
    Neck,
    Quadriceps,
    Shoulders,
    Traps,
    Triceps,
    UpperBack,
    Other,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Equipment {
    Barbell,
    Dumbbell,
    Machine,
    Cable,
    Bodyweight,
    Band,
    Kettlebell,
    TrapBar,
    SmithMachine,
    Other,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SetType {
    Normal,
    Warmup,
    Dropset,
    Failure,
    #[serde(other)]
    Unknown,
}
