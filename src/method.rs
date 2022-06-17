use crate::property::Property;
use get_params_derive::{GetParams, IntoJsonValue};
use serde::{Deserialize, Serialize};

/// Methods to be called on a device.
#[derive(Serialize, Deserialize, Clone, GetParams, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Method {
    /// Get the specified property value.
    GetProp(Property),
    /// Gets multiple property values.
    #[serde(rename = "get_prop")]
    GetProps(Vec<Property>),

    /// Toggle the power state of the device.
    Toggle,

    /// Set the power state of the device.
    SetPower(bool),
    /// Set the power state of the device with an [`Effect`].
    #[serde(rename = "set_power")]
    SetPowerEffect(bool, Effect),
    /// Set the power state of the device with an [`Effect`] and a duration.
    #[serde(rename = "set_power")]
    SetPowerEffectDuration(bool, Effect, i32),

    /// Set the RGB Color of the light.
    SetRgb(i32),
    /// Set the RGB Color of the light with an [`Effect`].
    #[serde(rename = "set_rgb")]
    SetRgbEffect(i32, Effect),
    /// Set the RGB Color of the light with an [`Effect`] and a duration.
    #[serde(rename = "set_rgb")]
    SetRgbEffectDuration(i32, Effect, i32),

    /// Set the HSV Color of the light.
    ///
    /// # Arguments
    /// * `hue` - The hue of the color. It should be expressed in decimal integer ranges from 0 to 359.
    /// * `sat` - The saturation of the color. It's range is 0 to 100.
    SetHsv(i32, i32),
    /// Set the HSV Color of the light with an [`Effect`].
    #[serde(rename = "set_hsv")]
    SetHsvEffect(i32, i32, Effect),
    /// Set the HSV Color of the light with an [`Effect`] and a duration.
    #[serde(rename = "set_hsv")]
    SetHsvEffectDuration(i32, i32, Effect, i32),

    /// Set the brightness of the light.
    ///
    /// # Arguments
    /// * `bright` - The brightness of the light. It's range is 1 to 100. The brightness is a percentage instead of a absolute value. 100 means maximum brightness while 1 means the minimum brightness.
    SetBright(i32),
    /// Set the brightness of the light with an [`Effect`].
    #[serde(rename = "set_bright")]
    SetBrightEffect(i32, Effect),
    /// Set the brightness of the light with an [`Effect`] and a duration.
    #[serde(rename = "set_bright")]
    SetBrightEffectDuration(i32, Effect, i32),

    /// This method is used to save current state of smart LED in persistent memory. So if user powers off and then powers on the smart LED again (hard power reset), the smart LED will show last saved state.
    SetDefault,

    /// [`Method::SetRgb`]
    BgSetRgb(i32),
    /// [`Method::SetRgbEffect`]
    #[serde(rename = "bg_set_rgb")]
    BgSetRgbEffect(i32, Effect),
    /// [`Method::SetRgbEffectDuration`]
    #[serde(rename = "bg_set_rgb")]
    BgSetRgbEffectDuration(i32, Effect, i32),

    /// [`Method::SetHsv`]
    BgSetHsv(i32, i32),
    /// [`Method::SetHsvEffect`]
    #[serde(rename = "bg_set_hsv")]
    BgSetHsvEffect(i32, i32, Effect),
    /// [`Method::SetHsvEffectDuration`]
    #[serde(rename = "bg_set_hsv")]
    BgSetHsvEffectDuration(i32, i32, Effect, i32),

    /// Saves current Background state; see [`Method::SetDefault`] for more info.
    BgSetDefault,

    /// [`Method::SetPower`]
    BgSetPower(bool),
    /// [`Method::SetPowerEffect`]
    #[serde(rename = "bg_set_power")]
    BgSetPowerEffect(bool, Effect),
    /// [`Method::SetPowerEffectDuration`]
    #[serde(rename = "bg_set_power")]
    BgSetPowerEffectDuration(bool, Effect, i32),

    /// [`Method::SetRgb`]
    BgSetBright(i32),
    /// [`Method::SetRgbEffect`]
    #[serde(rename = "bg_set_bright")]
    BgSetBrightEffect(i32, Effect),
    /// [`Method::SetRgbEffectDuration`]
    #[serde(rename = "bg_set_bright")]
    BgSetBrightEffectDuration(i32, Effect, i32),

    /// Set color temperature of the light.
    ///
    /// # Arguments
    /// * `ct_value` is the target color temperature. The type is integer and range is 1700 ~ 6500 (k).
    SetCtAbx(i32),
    /// Set color temperature of the light with an [`Effect`].
    #[serde(rename = "set_ct_abx")]
    SetCtAbxEffect(i32, Effect),
    /// Set color temperature of the light with an [`Effect`] and a duration.
    #[serde(rename = "set_ct_abx")]
    SetCtAbxEffectDuration(i32, Effect, i32),

    /// [`Method::SetCtAbx`]
    BgSetCtAbx(i32),
    /// [`Method::SetCtAbxEffect`]
    #[serde(rename = "bg_set_ct_abx")]
    BgSetCtAbxEffect(i32, Effect),
    /// [`Method::SetCtAbxEffectDuration`]
    #[serde(rename = "bg_set_ct_abx")]
    BgSetCtAbxEffectDuration(i32, Effect, i32),
}

/// The effect to use when setting a certain property.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug, IntoJsonValue)]
#[serde(rename_all = "snake_case")]
pub enum Effect {
    /// Values will be changed directly, with no duration or transition.
    Sudden,
    /// Values will be changed gradually, the total time of gradual change is specified by the duration.
    Smooth,
}
