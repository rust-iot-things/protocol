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
