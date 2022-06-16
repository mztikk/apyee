pub mod command;
pub mod device;
pub mod method;

#[cfg(test)]
mod tests {
    use crate::{
        command::{self, CommandResponse, CommandResult},
        device::Device,
        method::{Effect, Method},
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
    fn test_response_parsing() {
        let data = "{\"id\":1, \"result\":[\"ok\"]}";
        let response: CommandResponse = serde_json::from_str(data).unwrap();
        assert_eq!(response.id, 1);
        assert_eq!(response.result.len(), 1);
        assert_eq!(response.result[0], CommandResult::Ok);
    }
}
