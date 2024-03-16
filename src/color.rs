use serde::{Serialize, Serializer};
use serde::de::{self, Visitor, Deserialize, Deserializer};
use std::fmt;
pub use hex_color::HexColor;

#[derive(Debug)]
pub enum PresetColor {
    Red,
    Orange,
    Yellow,
    Green,
    Cyan,
    Purple,
}

impl Serialize for PresetColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match self {
            PresetColor::Red => "red",
            PresetColor::Orange => "orange",
            PresetColor::Yellow => "yellow",
            PresetColor::Green => "green",
            PresetColor::Cyan => "cyan",
            PresetColor::Purple => "purple",
        })
    }
}

impl<'de> Deserialize<'de> for PresetColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de> {
        struct PresetColorVisitor;

        impl<'de> Visitor<'de> for PresetColorVisitor {
            type Value = PresetColor;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing a color")
            }

            fn visit_str<E>(self, value: &str) -> Result<PresetColor, E>
                where
                    E: de::Error,
            {
                match value {
                    "red" => Ok(PresetColor::Red),
                    "orange" => Ok(PresetColor::Orange),
                    "yellow" => Ok(PresetColor::Yellow),
                    "green" => Ok(PresetColor::Green),
                    "cyan" => Ok(PresetColor::Cyan),
                    "purple" => Ok(PresetColor::Purple),
                    _ => Err(E::custom(format!("unknown color: {}", value))),
                }
            }
        }

        deserializer.deserialize_str(PresetColorVisitor)

    }
}

#[derive(Debug)]
pub enum Color {
    Preset(PresetColor),
    Color(HexColor),
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Color::Preset(color) => color.serialize(serializer),
            Color::Color(color) => color.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "red" => Ok(Color::Preset(PresetColor::Red)),
            "orange" => Ok(Color::Preset(PresetColor::Orange)),
            "yellow" => Ok(Color::Preset(PresetColor::Yellow)),
            "green" => Ok(Color::Preset(PresetColor::Green)),
            "cyan" => Ok(Color::Preset(PresetColor::Cyan)),
            "purple" => Ok(Color::Preset(PresetColor::Purple)),
            _ => {
                Ok(Color::Color(HexColor::parse(&value).unwrap()))
            },
        }
    }
}
