pub mod command;
pub mod device;
pub mod method;

#[cfg(test)]
mod tests {
    use crate::{
        command,
        method::{Effect, Method},
    };

    #[test]
    fn it_works() {
        let command =
            command::Command::new(0, Method::SetPowerEffectDuration(true, Effect::Smooth, 500));
        let json = serde_json::to_string(&command).unwrap();
        println!("{}", json);
    }
}
