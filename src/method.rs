use crate::property::Property;
use get_params_derive::{GetParams, IntoJsonValue};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, GetParams, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Method {
    GetProp(Property),
    #[serde(rename = "get_prop")]
    GetProps(Vec<Property>),
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

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug, IntoJsonValue)]
#[serde(rename_all = "snake_case")]
pub enum Effect {
    Sudden,
    Smooth,
}
