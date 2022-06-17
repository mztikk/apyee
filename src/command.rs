use crate::method::Method;
use serde::{Deserialize, Serialize, Serializer};

fn variant_name_only<S>(method: &Method, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let name = serde_variant::to_variant_name(method).unwrap();

    serializer.serialize_str(name)
}

/// A command to be sent to a device, containing a unique ID which is echoed back by the response.
/// The command is serialized to JSON and sent to the device.
///
/// [`Command`]s are created using the [`Command::new`] function.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Command {
    /// The unique ID of the command.
    pub id: usize,
    /// The method to be called on the device.
    #[serde(serialize_with = "variant_name_only")]
    pub method: Method,
    /// The parameters to be passed for the method.
    pub params: Vec<serde_json::Value>,
}

impl Command {
    /// Creates a new command with a unique ID and a [`Method`].
    pub fn new(id: usize, method: Method) -> Self {
        Self {
            id,
            params: method.get_params(),
            method,
        }
    }
}

/// A response from a device, containing the echoed ID of the Command, a result and optional Error.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct CommandResponse {
    /// The unique, echoed ID of the command.
    pub id: usize,
    /// The result of the command.
    pub result: Vec<CommandResult>,
    /// The error of the command, if any.
    pub error: Option<CommandResponseError>,
}

/// The result of a [`Command`].
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum CommandResult {
    /// The command was successful ("ok").
    Ok,
}

/// The error of a [`Command`], containing a error code and a description.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct CommandResponseError {
    /// The error code.
    pub code: i32,
    /// The error description.
    pub message: String,
}
