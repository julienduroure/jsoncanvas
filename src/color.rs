pub use hex_color::HexColor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Color {
    Preset(String),
    Color(HexColor),
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Color, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Ok(Color::from(value))
    }
}

impl From<String> for Color {
    fn from(value: String) -> Self {
        if value.starts_with('#') {
            Self::Color(HexColor::parse(&value).unwrap())
        } else {
            Self::Preset(value)
        }
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
        let preset: Color = Color::Preset(serde_json::from_str("\"red\"").unwrap());
        assert_eq!(preset, Color::Preset("red".to_string()));
    }

    #[test]
    fn color_preset_deser() {
        let color: Color = serde_json::from_str("\"2\"").unwrap();
        assert_eq!(color, Color::Preset("2".to_string()));
    }

    #[test]
    fn color_rgb_deser() {
        let color: Color = serde_json::from_str("\"#FF0000\"").unwrap();
        assert_eq!(color, Color::Color(HexColor::rgb(255, 0, 0)))
    }

    #[test]
    fn color_ser() {
        assert_eq!(
            serde_json::to_string(&Color::Preset("yellow".to_string())).unwrap(),
            "\"yellow\""
        );
        assert_eq!(
            serde_json::to_string(&Color::Color(HexColor::rgb(255, 0, 0))).unwrap(),
            "\"#FF0000\""
        );
    }
}
