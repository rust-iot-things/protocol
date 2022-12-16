use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MeasurementTemperature {
    pub id: u64,
    pub temperature: f32,
}

#[derive(Serialize, Deserialize)]
pub struct MeasurementTemperatureDescirption {
    #[serde(rename = "MeasurementTemperature")]
    pub measurement_temperature: MeasurementTemperature,
}

pub fn create(id: u64, temperature: f32) -> String {
    let measurement_temperature_description = MeasurementTemperatureDescirption {
        measurement_temperature: MeasurementTemperature { id, temperature },
    };
    serde_json::to_string(&measurement_temperature_description).unwrap()
}

pub(crate) fn parse(payload: &str) -> Result<MeasurementTemperatureDescirption, serde_json::Error> {
    serde_json::from_str(payload)
}
