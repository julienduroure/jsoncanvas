pub use hex_color::HexColor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PresetColor {
    Red = 1,
    Orange = 2,
    Yellow = 3,
    Green = 4,
    Cyan = 5,
    Purple = 6,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Color {
    Preset(PresetColor),
    Color(HexColor),
}

impl From<PresetColor> for Color {
    fn from(value: PresetColor) -> Self {
        Self::Preset(value)
    }
}

impl From<HexColor> for Color {
    fn from(value: HexColor) -> Self {
        Self::Color(value)
    }
}
