use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MeasurementTemperature {
    pub id: u128,
    pub temperature: f32,
}

#[derive(Serialize, Deserialize)]
pub struct MeasurementTemperatureDescirption {
    #[serde(rename = "MeasurementTemperature")]
    pub measurement_temperature: MeasurementTemperature,
}

pub fn create(id: u128, temperature: f32) -> String {
    let measurement_temperature_description = MeasurementTemperatureDescirption {
        measurement_temperature: MeasurementTemperature { id, temperature },
    };
    serde_json::to_string(&measurement_temperature_description).unwrap()
}

pub(crate) fn parse(payload: &str) -> Result<MeasurementTemperatureDescirption, serde_json::Error> {
    serde_json::from_str(payload)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_success() {
        // given
        let payload: &str = r#"{"MeasurementTemperature":{"id":1,"temperature":20.2}}"#;

        // when
        let result = parse(payload);

        // then
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.measurement_temperature.id, 1);
        assert_eq!(result.measurement_temperature.temperature, 20.2);
    }

    #[test]
    fn parse_fails_with_empty_string() {
        // given
        let payload: &str = "";

        // when
        let result = parse(payload);

        // then
        assert!(result.is_err());
    }

    #[test]
    fn parse_fails_with_invalid_json() {
        // given
        let payload: &str = r#"{"WrongMessage":{"id":1,"temperature":20.2}}"#;

        // when
        let result = parse(payload);

        // then
        assert!(result.is_err());
    }

    #[test]
    fn create_success() {
        // given
        let expected: &str = r#"{"MeasurementTemperature":{"id":1,"temperature":20.2}}"#;

        // when
        let result = create(1, 20.2);

        // then
        assert_eq!(expected, result.as_str());
    }
}
