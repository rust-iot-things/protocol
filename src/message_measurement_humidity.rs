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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_success() {
        // given
        let payload: &str = r#"{"MeasurmentHumidity":{"id":1,"humidity":2}}"#;

        // when
        let result = parse(payload);

        // then
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.measurement_humidity.id, 1);
        assert_eq!(result.measurement_humidity.humidity, 2);
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
        let payload: &str = r#"{"WrongMessage":{"id":1,"humidity":2}}"#;

        // when
        let result = parse(payload);

        // then
        assert!(result.is_err());
    }

    #[test]
    fn create_success() {
        // given
        let expected: &str = r#"{"MeasurmentHumidity":{"id":1,"humidity":2}}"#;

        // when
        let result = create(1, 2);

        // then
        assert_eq!(expected, result.as_str());
    }
}
