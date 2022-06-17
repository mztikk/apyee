#![deny(missing_docs)]
#![deny(rustdoc::missing_doc_code_examples)]

//! Yeelight API

/// Commands and their responses which are sent and received from the [`crate::device::Device`].
pub mod command;
/// The [`crate::device::Device`] itself, used to interact with the Yeelight device.
pub mod device;
/// The [`crate::method::Method`]s which are called on the Yeelight device.
pub mod method;
/// The [`crate::property::Property`]s which are queried from the Yeelight device.
pub mod property;

#[cfg(test)]
mod tests {
    use crate::{
        command::{self, CommandResponse, CommandResult},
        device::Device,
        method::{Effect, Method},
        property::Property,
    };

    #[test]
    fn it_works() {
        let command =
            command::Command::new(0, Method::SetPowerEffectDuration(true, Effect::Smooth, 500));
        let json = serde_json::to_string(&command).unwrap();
        println!("{}", json);
    }

    #[test]
    fn command_serialization() {
        let command = command::Command::new(
            0,
            Method::SetRgbEffectDuration(Device::get_rgb_color(255, 0, 0), Effect::Smooth, 500),
        );
        let json = serde_json::to_string(&command).unwrap();
        assert_eq!(
            json,
            r#"{"id":0,"method":"set_rgb","params":[16711680,"smooth",500]}"#
        );
    }

    #[test]
    fn command_get_prop_serialization() {
        let command = command::Command::new(
            0,
            Method::GetProps(vec![Property::Power, Property::Rgb, Property::BgRgb]),
        );
        let json = serde_json::to_string(&command).unwrap();
        assert_eq!(
            json,
            r#"{"id":0,"method":"get_prop","params":["power","rgb","bg_rgb"]}"#
        );
    }

    #[test]
    fn test_response_parsing() {
        let data = "{\"id\":1, \"result\":[\"ok\"]}";
        let response: CommandResponse = serde_json::from_str(data).unwrap();
        assert_eq!(response.id, 1);
        assert_eq!(response.result.len(), 1);
        assert_eq!(response.result[0], CommandResult::Ok);
    }
}
