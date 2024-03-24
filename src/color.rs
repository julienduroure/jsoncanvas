pub use hex_color::HexColor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum PresetColor {
    #[serde(rename = "1")]
    Red = 1,
    #[serde(rename = "2")]
    Orange = 2,
    #[serde(rename = "3")]
    Yellow = 3,
    #[serde(rename = "4")]
    Green = 4,
    #[serde(rename = "5")]
    Cyan = 5,
    #[serde(rename = "6")]
    Purple = 6,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preset_deser() {
        let preset: PresetColor = serde_json::from_str("\"1\"").unwrap();
        assert_eq!(preset, PresetColor::Red);
    }

    #[test]
    fn color_preset_deser() {
        let color: Color = serde_json::from_str("\"2\"").unwrap();
        assert_eq!(color, Color::Preset(PresetColor::Orange));
    }

    #[test]
    fn color_rgb_deser() {
        let color: Color = serde_json::from_str("\"#FF0000\"").unwrap();
        assert_eq!(color, Color::Color(HexColor::rgb(255, 0, 0)))
    }

    #[test]
    fn color_ser() {
        assert_eq!(
            serde_json::to_string(&Color::Preset(PresetColor::Yellow)).unwrap(),
            "\"3\""
        );
        assert_eq!(
            serde_json::to_string(&Color::Color(HexColor::rgb(255, 0, 0))).unwrap(),
            "\"#FF0000\""
        );
    }
}
