use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct SetName {
    pub id: u64,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct SetNameDescirption {
    #[serde(rename = "SetName")]
    pub set_name: SetName,
}

pub fn create(id: u64, name: String) -> String {
    let set_name_description = SetNameDescirption {
        set_name: SetName { id, name },
    };
    serde_json::to_string(&set_name_description).unwrap()
}

pub fn parse(payload: &str) -> Result<SetNameDescirption, serde_json::Error> {
    serde_json::from_str(payload)
}
