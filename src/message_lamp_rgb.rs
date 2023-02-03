use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LampRGB {
    pub id: String,
    pub r: u32,
    pub g: u32,
    pub b: u32,
}

#[derive(Serialize, Deserialize)]
pub struct LampRGBDescirption {
    #[serde(rename = "LampRGB")]
    pub lamp_rgb: LampRGB,
}

pub fn create(id: String, r: u32, g: u32, b: u32) -> String {
    let lamp_rgb_description = LampRGBDescirption {
        lamp_rgb: LampRGB { id, r, g, b },
    };
    serde_json::to_string(&lamp_rgb_description).unwrap()
}

pub(crate) fn parse(payload: &str) -> Result<LampRGBDescirption, serde_json::Error> {
    serde_json::from_str(payload)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_success() {
        // given
        let payload: &str = r#"{"LampRGB":{"id":"1","r":1,"g":2,"b":3}}"#;

        // when
        let result = parse(payload);

        // then
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.lamp_rgb.id, "1");
        assert_eq!(result.lamp_rgb.r, 1);
        assert_eq!(result.lamp_rgb.g, 2);
        assert_eq!(result.lamp_rgb.b, 3);
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
        let payload: &str = r#"{"WrongMessage":{"id":"1","r":1,"g":2,"b":3}}"#;

        // when
        let result = parse(payload);

        // then
        assert!(result.is_err());
    }

    #[test]
    fn create_success() {
        // given
        let expected: &str = r#"{"LampRGB":{"id":"1","r":1,"g":2,"b":3}}"#;

        // when
        let result = create("1".into(), 1, 2, 3);

        // then
        assert_eq!(expected, result.as_str());
    }
}
