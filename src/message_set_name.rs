use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct SetName {
    pub id: u128,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct SetNameDescirption {
    #[serde(rename = "SetName")]
    pub set_name: SetName,
}

pub fn create(id: u128, name: String) -> String {
    let set_name_description = SetNameDescirption {
        set_name: SetName { id, name },
    };
    serde_json::to_string(&set_name_description).unwrap()
}

pub fn parse(payload: &str) -> Result<SetNameDescirption, serde_json::Error> {
    serde_json::from_str(payload)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_success() {
        // given
        let payload: &str = r#"{"SetName":{"id":1,"name":"new_thing"}}"#;

        // when
        let result = parse(payload);

        // then
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.set_name.id, 1);
        assert_eq!(result.set_name.name, "new_thing");
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
        let payload: &str = r#"{"WrongMessage":{"id":1,"name":"new_thing"}}"#;

        // when
        let result = parse(payload);

        // then
        assert!(result.is_err());
    }

    #[test]
    fn create_success() {
        // given
        let expected: &str = r#"{"SetName":{"id":1,"name":"new_thing"}}"#;

        // when
        let result = create(1, "new_thing".into());

        // then
        assert_eq!(expected, result.as_str());
    }
}
