use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyMeasurement {
    pub date: String,
    #[serde(flatten)]
    pub values: BodyMeasurementValues,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BodyMeasurementValues {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_kg: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lean_mass_kg: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fat_percent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neck_cm: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shoulder_cm: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chest_cm: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_bicep_cm: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_bicep_cm: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_forearm_cm: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_forearm_cm: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abdomen: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waist: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hips: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_thigh: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_thigh: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_calf: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_calf: Option<f64>,
}

impl BodyMeasurementValues {
    pub fn labelled(&self) -> Vec<(&'static str, f64, &'static str)> {
        [
            ("Weight", self.weight_kg, "kg"),
            ("Lean mass", self.lean_mass_kg, "kg"),
            ("Body fat", self.fat_percent, "%"),
            ("Neck", self.neck_cm, "cm"),
            ("Shoulder", self.shoulder_cm, "cm"),
            ("Chest", self.chest_cm, "cm"),
            ("Left bicep", self.left_bicep_cm, "cm"),
            ("Right bicep", self.right_bicep_cm, "cm"),
            ("Left forearm", self.left_forearm_cm, "cm"),
            ("Right forearm", self.right_forearm_cm, "cm"),
            ("Abdomen", self.abdomen, "cm"),
            ("Waist", self.waist, "cm"),
            ("Hips", self.hips, "cm"),
            ("Left thigh", self.left_thigh, "cm"),
            ("Right thigh", self.right_thigh, "cm"),
            ("Left calf", self.left_calf, "cm"),
            ("Right calf", self.right_calf, "cm"),
        ]
        .into_iter()
        .filter_map(|(label, value, unit)| value.map(|value| (label, value, unit)))
        .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyMeasurementsResponse {
    pub body_measurements: Vec<BodyMeasurement>,
}
