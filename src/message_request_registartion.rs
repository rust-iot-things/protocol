use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RequestRegistration {
    pub id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct RequestRegistrationDescirption {
    #[serde(rename = "RequestRegistration")]
    pub request_requistration: RequestRegistration,
}

pub fn create(id: u64) -> String {
    let request_registration_description = RequestRegistrationDescirption {
        request_requistration: RequestRegistration { id },
    };
    serde_json::to_string(&request_registration_description).unwrap()
}

pub(crate) fn parse(payload: &str) -> Result<RequestRegistrationDescirption, serde_json::Error> {
    serde_json::from_str(payload)
}
