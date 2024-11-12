// #![deny(missing_docs)]
// #![deny(rustdoc::missing_doc_code_examples)]

//! Yeelight API
//! This library provides a Rust API for the Yeelight device.
//! It is based on the [official Yeelight API documentation](https://home.yeelight.de/site/templates/downloads/yeelight_inter-operation-spec.pdf).
//!
//! # Examples
//! ```no_run
//! use apyee::device::Device;
//! use apyee::method::Method;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a new Device with the IP address of the device and the default port.
//!     // creating the Device will also connect to it and start listening for responses.
//!     let mut device = Device::new("192.168.100.5").await?;
//!
//!     // Send a command through a convenience method and toggle its power state.
//!     device.toggle().await?;
//!
//!     // Set its RGB Color to red.
//!     device.set_rgb(255, 0, 0).await?;
//!
//!     // Send any possible command to the device.
//!     device.execute_method(Method::SetBright(50, None, None)).await?;
//!
//!     Ok(())
//! }
//! ```

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
        command::{self, Command, CommandResponse, CommandResult, RawCommand},
        device::Device,
        method::{Effect, Method, MethodParseError},
        property::Property,
    };

    #[test]
    fn it_works() {
        let command =
            command::Command::new(0, Method::SetPower(true, Some(Effect::Smooth), Some(500)));
        let json = serde_json::to_string(&command).unwrap();
        println!("{}", json);
    }

    #[test]
    fn method_deserialization() {
        let json = r#"{"set_rgb":[16711680,"smooth",500]}"#;
        let method = serde_json::from_str::<Method>(json);
        assert!(method.is_ok());
        let method = method.unwrap();
        assert_eq!(
            method,
            Method::SetRgb(
                Device::get_rgb_color(255, 0, 0),
                Some(Effect::Smooth),
                Some(500),
            )
        );
    }

    #[test]
    fn command_deserialization() {
        let json = r#"{"id":0,"method":"set_rgb","params":[16711680,"smooth",500]}"#;
        let command = serde_json::from_str::<Command>(json).unwrap();
        assert_eq!(
            command,
            command::Command::new(
                0,
                Method::SetRgb(
                    Device::get_rgb_color(255, 0, 0),
                    Some(Effect::Smooth),
                    Some(500),
                ),
            )
        );
    }

    #[test]
    fn method_hsv_from_raw_command() {
        let raw_command = RawCommand {
            id: 0,
            method: String::from("set_hsv"),
            params: vec![],
        };
        let method = Method::try_from(&raw_command);
        assert!(method.is_err());
        let method = method.err().unwrap();
        assert!(match method {
            MethodParseError::MissingFieldValue {
                field_index,
                field_name: _,
                method_name: _,
            } => field_index == 0,
            MethodParseError::Json(_) => false,
        });

        let raw_command = RawCommand {
            id: 0,
            method: String::from("set_hsv"),
            params: vec![serde_json::to_value(100).unwrap()],
        };
        let method = Method::try_from(&raw_command);
        assert!(method.is_err());
        let method = method.err().unwrap();
        assert!(match method {
            MethodParseError::MissingFieldValue {
                field_index,
                field_name: _,
                method_name: _,
            } => field_index == 1,
            MethodParseError::Json(_) => false,
        });
    }

    #[test]
    fn command_hsv_deserialization() {
        let json = r#"{"id":0,"method":"set_hsv","params":[]}"#;
        let command = serde_json::from_str::<Command>(json);
        assert!(command.is_err());

        let json = r#"{"id":0,"method":"set_hsv","params":[100]}"#;
        let command = serde_json::from_str::<Command>(json);
        assert!(command.is_err());

        let json = r#"{"id":0,"method":"set_hsv","params":[100,50]}"#;
        let command = serde_json::from_str::<Command>(json).unwrap();
        assert_eq!(
            command,
            command::Command::new(0, Method::SetHsv(100, 50, None, None,),)
        );

        let json = r#"{"id":0,"method":"set_hsv","params":[100,50, "smooth"]}"#;
        let command = serde_json::from_str::<Command>(json).unwrap();
        assert_eq!(
            command,
            command::Command::new(0, Method::SetHsv(100, 50, Some(Effect::Smooth), None))
        );

        let json = r#"{"id":0,"method":"set_hsv","params":[100,50, "smooth", 5]}"#;
        let command = serde_json::from_str::<Command>(json).unwrap();
        assert_eq!(
            command,
            command::Command::new(0, Method::SetHsv(100, 50, Some(Effect::Smooth), Some(5)))
        );
    }

    #[test]
    fn command_serialization() {
        let command = command::Command::new(
            0,
            Method::SetRgb(
                Device::get_rgb_color(255, 0, 0),
                Some(Effect::Smooth),
                Some(500),
            ),
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
