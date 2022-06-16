use crate::method::Method;
use serde::{Deserialize, Serialize, Serializer};

fn variant_name_only<S>(method: &Method, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let name = serde_variant::to_variant_name(method).unwrap();

    serializer.serialize_str(name)
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct CommandResult {
    pub id: usize,
    pub error: Option<CommandErrorResult>,
}

#[derive(Serialize, Deserialize)]
pub struct CommandErrorResult {
    pub code: i32,
    pub message: String,
}
