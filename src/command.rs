use crate::method::Method;
use serde::{Deserialize, Serialize, Serializer};

fn variant_name_only<S>(method: &Method, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let name = serde_variant::to_variant_name(method).unwrap();

    serializer.serialize_str(name)
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Command {
    pub(crate) id: usize,
    #[serde(serialize_with = "variant_name_only")]
    pub(crate) method: Method,
    pub(crate) params: Vec<serde_json::Value>,
}

impl Command {
    pub fn new(id: usize, method: Method) -> Self {
        Self {
            id,
            params: method.get_params(),
            method,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct CommandResponse {
    pub id: usize,
    pub result: Vec<CommandResult>,
    pub error: Option<CommandResponseError>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum CommandResult {
    Ok,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct CommandResponseError {
    pub code: i32,
    pub message: String,
}
