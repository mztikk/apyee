#![deny(missing_docs)]
#![deny(rustdoc::missing_doc_code_examples)]

//! Yeelight API
//! This library provides a Rust API for the Yeelight device.
//! It is based on the official Yeelight API documentation.
//!
//! # Examples
//! ```
//! use apyee::device::Device;
//! use apyee::method::Method;
//! async {
//!     // Create a new Device with the IP address of the device and the default port.
//!     // creating the Device will also connect to it and start listening for responses.
//!     let mut device = Device::new("192.168.100.5").await.unwrap();
//!
//!     // Send a command through a convenience method and toggle its power state.
//!     device.toggle().await.unwrap();
//!
//!     // Set its RGB Color to red.
//!     device.set_rgb(255, 0, 0).await.unwrap();
//!
//!     // Send any possible command to the device.
//!     device.execute_method(Method::SetBright(50, None, None)).await.unwrap();
//! ```
//! };

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
            command::Command::new(0, Method::SetPower(true, Some(Effect::Smooth), Some(500)));
        let json = serde_json::to_string(&command).unwrap();
        println!("{}", json);
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
