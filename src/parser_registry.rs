use crate::{
    message_request_registartion::{self, RequestRegistration, RequestRegistrationDescirption},
    message_set_name::{self, SetName, SetNameDescirption},
};

pub enum RegistryType {
    SetNameType(message_set_name::SetNameDescirption),
    RequestRegistration(message_request_registartion::RequestRegistrationDescirption),
    None,
}

pub fn parse(payload: &str) -> RegistryType {
    if let Ok(it) = message_set_name::parse(payload) {
        return RegistryType::SetNameType(SetNameDescirption {
            set_name: SetName {
                id: it.set_name.id,
                name: it.set_name.name,
            },
        });
    }

    if let Ok(it) = message_request_registartion::parse(payload) {
        return RegistryType::RequestRegistration(RequestRegistrationDescirption {
            request_requistration: RequestRegistration {
                id: it.request_requistration.id,
            },
        });
    }
    RegistryType::None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_set_name() {
        // given
        let payload: &str = r#"{"SetName":{"id":1,"name":"new_thing"}}"#;

        // when
        let result: RegistryType = parse(payload);

        // then
        let expected = RegistryType::SetNameType(SetNameDescirption {
            set_name: SetName {
                id: 1,
                name: "new_thing".into(),
            },
        });
        // assert_eq!(expected, result);
    }

    // #[test]
    // fn parse_fails_with_empty_string() {
    //     // given
    //     let payload: &str = "";

    //     // when
    //     let result = parse(payload);

    //     // then
    //     assert!(result.is_err());
    // }

    // #[test]
    // fn parse_fails_with_invalid_json() {
    //     // given
    //     let payload: &str = r#"{"WrongMessage":{"id":1,"name":"new_thing"}}"#;

    //     // when
    //     let result = parse(payload);

    //     // then
    //     assert!(result.is_err());
    // }

    // #[test]
    // fn create_success() {
    //     // given
    //     let expected: &str = r#"{"SetName":{"id":1,"name":"new_thing"}}"#;

    //     // when
    //     let result = create(1, "new_thing".into());

    //     // then
    //     assert_eq!(expected, result.as_str());
    // }
}
