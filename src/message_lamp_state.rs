use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LampState {
    pub id: u64,
    pub state: bool,
}

#[derive(Serialize, Deserialize)]
pub struct LampStateDescirption {
    #[serde(rename = "LampState")]
    pub lamp_state: LampState,
}

pub fn create(id: u64, state: bool) -> String {
    let lamp_state_description = LampStateDescirption {
        lamp_state: LampState { id, state },
    };
    serde_json::to_string(&lamp_state_description).unwrap()
}

pub(crate) fn parse(payload: &str) -> Result<LampStateDescirption, serde_json::Error> {
    serde_json::from_str(payload)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_success() {
        // given
        let payload: &str = r#"{"LampState":{"id":1,"state":true}}"#;

        // when
        let result = parse(payload);

        // then
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.lamp_state.id, 1);
        assert_eq!(result.lamp_state.state, true);
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
        let payload: &str = r#"{"WrongMessage":{"id":1,"state":true}}"#;

        // when
        let result = parse(payload);

        // then
        assert!(result.is_err());
    }

    #[test]
    fn create_success() {
        // given
        let expected: &str = r#"{"LampState":{"id":1,"state":true}}"#;

        // when
        let result = create(1, true);

        // then
        assert_eq!(expected, result.as_str());
    }
}
