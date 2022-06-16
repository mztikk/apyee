use get_params_derive::IntoJsonValue;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug, IntoJsonValue)]
#[serde(rename_all = "snake_case")]
pub enum Property {
    Power,
    Bright,
    Ct,
    Rgb,
    Hue,
    Sat,
    ColorMode,
    Flowing,
    DelayOff,
    FlowParams,
    MusicOn,
    Name,
    BgPower,
    BgFlowing,
    BgFlowParams,
    BgCt,
    BgLmode,
    BgBright,
    BgRgb,
    BgHue,
    BgSat,
    BlBr,
    ActiveMode,
}
