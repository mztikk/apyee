use get_params_derive::GetParams;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, GetParams, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Method {
    Toggle,
    SetPower(bool),
    #[serde(rename = "set_power")]
    SetPowerEffect(bool, Effect),
    #[serde(rename = "set_power")]
    SetPowerEffectDuration(bool, Effect, i32),
    SetRgb(i32),
    #[serde(rename = "set_rgb")]
    SetRgbEffect(i32, Effect),
    #[serde(rename = "set_rgb")]
    SetRgbEffectDuration(i32, Effect, i32),
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Effect {
    Sudden,
    Smooth,
}

impl From<Effect> for serde_json::Value {
    fn from(val: Effect) -> Self {
        match val {
            Effect::Sudden => "sudden".to_string().into(),
            Effect::Smooth => "smooth".to_string().into(),
        }
    }
}
