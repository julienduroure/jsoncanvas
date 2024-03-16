use serde::{Serialize, Deserialize};
pub use hex_color::HexColor;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PresetColor {
    Red,
    Orange,
    Yellow,
    Green,
    Cyan,
    Purple,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Color {
    Preset(PresetColor),
    Color(HexColor),
}
