use get_params_derive::IntoJsonValue;
use serde::{Deserialize, Serialize};

/// Properties of a device.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug, IntoJsonValue, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Property {
    /// The power state of the device.
    /// on: smart LED is turned on / off: smart LED is turned off
    Power,
    /// Brightness percentage. Range 1 ~ 100
    Bright,
    /// Color temperature. Range 1700 ~ 6500(k)
    Ct,
    /// RGB Color of the device.
    /// Color. Range 1 ~ 16777215
    Rgb,
    /// Hue. Range 0 ~ 359
    Hue,
    /// Saturation. Range 0 ~ 100
    Sat,
    /// Color Mode of the device.
    /// 1: rgb mode / 2: color temperature mode / 3: hsv mode
    ColorMode,
    /// Color flow
    /// 0: no flow is running / 1:color flow is running
    Flowing,
    /// The remaining time of a sleep timer. Range 1 ~ 60 (minutes)
    DelayOff,
    /// Current flow parameters (only meaningful when [Property::Flowing] is 1)
    FlowParams,
    /// 1: Music mode is on / 0: Music mode is off
    MusicOn,
    /// The name of the device set by “set_name” command
    Name,
    /// Background light power status
    BgPower,
    /// Background light is flowing
    BgFlowing,
    /// Current flow parameters of background light
    BgFlowParams,
    /// Color temperature of background light
    BgCt,
    /// 1: rgb mode / 2: color temperature mode / 3: hsv mode
    BgLmode,
    /// Brightness percentage of background light
    BgBright,
    /// Color of background light
    BgRgb,
    /// Hue of background light
    BgHue,
    /// Saturation of background light
    BgSat,
    /// Brightness of night mode light
    NlBr,
    /// 0: daylight mode / 1: moonlight mode (ceiling light only)
    ActiveMode,
}
