use serde::Deserialize;

use crate::hevy::BodyMeasurementValues;

#[derive(Deserialize)]
pub(crate) struct DateArgs {
    pub date: String,
}

#[derive(Deserialize)]
pub(crate) struct BodyMeasurementArgs {
    pub date: String,
    #[serde(flatten)]
    pub values: BodyMeasurementValues,
}
