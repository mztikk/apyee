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

    SetHsv(i32, i32),
    #[serde(rename = "set_hsv")]
    SetHsvEffect(i32, i32, Effect),
    #[serde(rename = "set_hsv")]
    SetHsvEffectDuration(i32, i32, Effect, i32),

    SetBright(i32),
    #[serde(rename = "set_bright")]
    SetBrightEffect(i32, Effect),
    #[serde(rename = "set_bright")]
    SetBrightEffectDuration(i32, Effect, i32),

    SetDefault,

    BgSetRgb(i32),
    #[serde(rename = "bg_set_rgb")]
    BgSetRgbEffect(i32, Effect),
    #[serde(rename = "bg_set_rgb")]
    BgSetRgbEffectDuration(i32, Effect, i32),

    BgSetHsv(i32, i32),
    #[serde(rename = "bg_set_hsv")]
    BgSetHsvEffect(i32, i32, Effect),
    #[serde(rename = "bg_set_hsv")]
    BgSetHsvEffectDuration(i32, i32, Effect, i32),

    BgSetDefault,

    BgSetPower(bool),
    #[serde(rename = "bg_set_power")]
    BgSetPowerEffect(bool, Effect),
    #[serde(rename = "bg_set_power")]
    BgSetPowerEffectDuration(bool, Effect, i32),

    BgSetBright(i32),
    #[serde(rename = "bg_set_bright")]
    BgSetBrightEffect(i32, Effect),
    #[serde(rename = "bg_set_bright")]
    BgSetBrightEffectDuration(i32, Effect, i32),

    SetCtAbx(i32),
    #[serde(rename = "set_ct_abx")]
    SetCtAbxEffect(i32, Effect),
    #[serde(rename = "set_ct_abx")]
    SetCtAbxEffectDuration(i32, Effect, i32),

    BgSetCtAbx(i32),
    #[serde(rename = "bg_set_ct_abx")]
    BgSetCtAbxEffect(i32, Effect),
    #[serde(rename = "bg_set_ct_abx")]
    BgSetCtAbxEffectDuration(i32, Effect, i32),
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug, IntoJsonValue)]
#[serde(rename_all = "snake_case")]
pub enum Effect {
    Sudden,
    Smooth,
}
