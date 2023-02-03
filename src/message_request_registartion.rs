use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct RequestRegistration {
    pub id: u128,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct RequestRegistrationDescirption {
    #[serde(rename = "RequestRegistration")]
    pub request_requistration: RequestRegistration,
}

pub fn create(id: u128) -> String {
    let request_registration_description = RequestRegistrationDescirption {
        request_requistration: RequestRegistration { id },
    };
    serde_json::to_string(&request_registration_description).unwrap()
}

pub(crate) fn parse(payload: &str) -> Result<RequestRegistrationDescirption, serde_json::Error> {
    serde_json::from_str(payload)
}

mod test {
    use super::*;

    #[test]
    fn parse_success() {
        // given
        let payload: &str = r#"{"RequestRegistration": {"id": 1}}"#;

        // when
        let result = parse(payload);

        // then
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.request_requistration.id, 1);
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
        let payload: &str = r#"{"WrongMessage": {"id": 1}}"#;

        // when
        let result = parse(payload);

        // then
        assert!(result.is_err());
    }

    #[test]
    fn create_success() {
        // given
        let expected: &str = r#"{"RequestRegistration":{"id":1}}"#;

        // when
        let result = create(1);

        // then
        assert_eq!(expected, result.as_str());
    }
}
