use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MeasurmentHumidity {
    pub id: u64,
    pub humidity: u8,
}

#[derive(Serialize, Deserialize)]
pub struct MeasurmentHumidityDescirption {
    #[serde(rename = "MeasurmentHumidity")]
    pub measurement_humidity: MeasurmentHumidity,
}

pub fn create(id: u64, humidity: u8) -> String {
    let measurement_humidity_description = MeasurmentHumidityDescirption {
        measurement_humidity: MeasurmentHumidity { id, humidity },
    };
    serde_json::to_string(&measurement_humidity_description).unwrap()
}

pub(crate) fn parse(payload: &str) -> Result<MeasurmentHumidityDescirption, serde_json::Error> {
    serde_json::from_str(payload)
}
