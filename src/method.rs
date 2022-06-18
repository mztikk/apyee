use crate::command::RawCommand;
use crate::property::Property;
use get_params_derive::{FromRawCommand, GetParams, IntoJsonValue};
use serde::{Deserialize, Serialize};

/// Methods to be called on a device.
#[derive(Serialize, Deserialize, Clone, GetParams, PartialEq, Eq, Debug, FromRawCommand)]
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
    SetPower(bool, Option<Effect>, Option<i32>),

    /// Set the RGB Color of the light.
    SetRgb(i32, Option<Effect>, Option<i32>),

    /// Set the HSV Color of the light.
    ///
    /// # Arguments
    /// * `hue` - The hue of the color. It should be expressed in decimal integer ranges from 0 to 359.
    /// * `sat` - The saturation of the color. It's range is 0 to 100.
    SetHsv(i32, i32, Option<Effect>, Option<i32>),

    /// Set the brightness of the light.
    ///
    /// # Arguments
    /// * `bright` - The brightness of the light. It's range is 1 to 100. The brightness is a percentage instead of a absolute value. 100 means maximum brightness while 1 means the minimum brightness.
    SetBright(i32, Option<Effect>, Option<i32>),

    /// This method is used to save current state of smart LED in persistent memory. So if user powers off and then powers on the smart LED again (hard power reset), the smart LED will show last saved state.
    SetDefault,

    /// [`Method::SetRgb`]
    BgSetRgb(i32, Option<Effect>, Option<i32>),

    /// [`Method::SetHsv`]
    BgSetHsv(i32, i32, Option<Effect>, Option<i32>),

    /// Saves current Background state; see [`Method::SetDefault`] for more info.
    BgSetDefault,

    /// [`Method::SetPower`]
    BgSetPower(bool, Option<Effect>, Option<i32>),

    /// [`Method::SetRgb`]
    BgSetBright(i32, Option<Effect>, Option<i32>),

    /// Set color temperature of the light.
    ///
    /// # Arguments
    /// * `ct_value` is the target color temperature. The type is integer and range is 1700 ~ 6500 (k).
    SetCtAbx(i32, Option<Effect>, Option<i32>),

    /// [`Method::SetCtAbx`]
    BgSetCtAbx(i32, Option<Effect>, Option<i32>),
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
