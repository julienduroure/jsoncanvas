use serde::Serialize;
use serde::ser::{SerializeStruct, Serializer};
use serde::de::{self, Deserialize, Deserializer, Visitor, MapAccess};
use std::fmt;

#[derive(Debug)]
pub struct Edge {
    pub id: String,
    from_node: String,
    from_side: Option<Side>,
    from_end: Option<End>,
    to_node: String,
    to_side: Option<Side>,
    to_end: Option<End>,
    color: Option<crate::color::Color>,
    label: Option<String>,
}

impl Edge {
    pub fn new(id: String, from_node: String, from_side: Option<Side>, from_end: Option<End>, to_node: String, to_side: Option<Side>, to_end: Option<End>, color: Option<crate::color::Color>, label: Option<String>) -> Edge {
        Edge {
            id,
            from_node,
            from_side,
            from_end,
            to_node,
            to_side,
            to_end,
            color,
            label,
        }
    }

    pub fn set_color(&mut self, color: crate::color::Color) {
        self.color = Some(color);
    }

    pub fn remove_color(&mut self) {
        self.color = None;
    }

    pub fn set_label(&mut self, label: String) {
        self.label = Some(label);
    }

    pub fn remove_label(&mut self) {
        self.label = None;
    }

    pub fn set_from(&mut self, from_node: String, from_side: Option<Side>, from_end: Option<End>) {
        self.from_node = from_node;
        self.from_side = from_side;
        self.from_end = from_end;
    }

    pub fn set_to(&mut self, to_node: String, to_side: Option<Side>, to_end: Option<End>) {
        self.to_node = to_node;
        self.to_side = to_side;
        self.to_end = to_end;
    }

    pub fn remove_from(&mut self) {
        self.from_node = "".to_string();
        self.from_side = None;
        self.from_end = None;
    }

    pub fn remove_to(&mut self) {
        self.to_node = "".to_string();
        self.to_side = None;
        self.to_end = None;
    }

}

#[derive(Debug)]
pub enum Side {
    Top,
    Left,
    Right,
    Bottom,
}

#[derive(Debug)]
pub enum End {
    None,
    Arrow
}

impl Serialize for Side {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        {
            match self {
                Side::Top => serializer.serialize_str("top"),
                Side::Left => serializer.serialize_str("left"),
                Side::Right => serializer.serialize_str("right"),
                Side::Bottom => serializer.serialize_str("bottom"),
            }
        }

}

impl Serialize for End {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        {
            match self {
                End::None => serializer.serialize_str("none"),
                End::Arrow => serializer.serialize_str("arrow"),
            }
        }

}

impl Serialize for Edge {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        {
            let mut nb = 3;

            if self.color.is_some() { nb += 1; }
            if self.label.is_some() { nb += 1; }
            if self.from_side.is_some() { nb += 1; }
            if self.from_end.is_some() { nb += 1; }
            if self.to_side.is_some() { nb += 1; }
            if self.to_end.is_some() { nb += 1; }

            let mut state = serializer.serialize_struct("Edge", nb)?;
            state.serialize_field("id", &self.id)?;
            state.serialize_field("fromNode", &self.from_node)?;
            state.serialize_field("toNode", &self.to_node)?;

            if self.color.is_some() {
                state.serialize_field("color", &self.color)?;
            }

            if self.label.is_some() {
                state.serialize_field("label", &self.label)?;
            }

            if self.from_side.is_some() {
                state.serialize_field("fromSide", &self.from_side.as_ref().unwrap())?;
            }

            if self.from_end.is_some() {
                state.serialize_field("fromEnd", &self.from_end.as_ref().unwrap())?;
            }

            if self.to_side.is_some() {
                state.serialize_field("toSide", &self.to_side.as_ref().unwrap())?;
            }

            if self.to_end.is_some() {
                state.serialize_field("toEnd", &self.to_end.as_ref().unwrap())?;
            }


            state.end()
        }
}


impl<'de> Deserialize<'de> for Side {
    fn deserialize<D>(deserializer: D) -> Result<Side, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SideVisitor;

        impl<'de> Visitor<'de> for SideVisitor {
            type Value = Side;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing a side")
            }

            fn visit_str<E>(self, value: &str) -> Result<Side, E>
            where
                E: de::Error,
            {
                match value {
                    "top" => Ok(Side::Top),
                    "left" => Ok(Side::Left),
                    "right" => Ok(Side::Right),
                    "bottom" => Ok(Side::Bottom),
                    _ => Err(E::custom(format!("unknown side: {}", value))),
                }
            }
        }

        deserializer.deserialize_str(SideVisitor)
    }
}

impl <'de> Deserialize<'de> for End {
    fn deserialize<D>(deserializer: D) -> Result<End, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EndVisitor;

        impl<'de> Visitor<'de> for EndVisitor {
            type Value = End;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing an end")
            }

            fn visit_str<E>(self, value: &str) -> Result<End, E>
            where
                E: de::Error,
            {
                match value {
                    "none" => Ok(End::None),
                    "arrow" => Ok(End::Arrow),
                    _ => Err(E::custom(format!("unknown end: {}", value))),
                }
            }
        }

        deserializer.deserialize_str(EndVisitor)
    }
}

impl<'de> Deserialize<'de> for Edge {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EdgeVisitor;

        impl<'de> Visitor<'de> for EdgeVisitor {
            type Value = Edge;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Edge")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Edge, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut id: Option<String> = None;
                let mut from_node: Option<String> = None;
                let mut from_side: Option<Side> = None;
                let mut from_end: Option<End> = None;
                let mut to_node: Option<String> = None;
                let mut to_side: Option<Side> = None;
                let mut to_end: Option<End> = None;
                let mut color: Option<crate::color::Color> = None;
                let mut label: Option<String> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "id" => {
                            id = Some(map.next_value()?);
                        },
                        "fromNode" => {
                            from_node = Some(map.next_value()?);
                        },
                        "toNode" => {
                            to_node = Some(map.next_value()?);
                        },
                        "fromSide" => {
                            from_side = Some(map.next_value()?);
                        },
                        "fromEnd" => {
                            from_end = Some(map.next_value()?);
                        },
                        "toSide" => {
                            to_side = Some(map.next_value()?);
                        },
                        "toEnd" => {
                            to_end = Some(map.next_value()?);
                        },
                        "color" => {
                            color = Some(map.next_value()?);
                        },
                        "label" => {
                            label = Some(map.next_value()?);
                        },
                        _ => {
                            return Err(serde::de::Error::custom("invalid key"));
                        },
                    }
                }

                let id = id.ok_or_else(|| serde::de::Error::missing_field("id"))?;
                let from_node = from_node.ok_or_else(|| serde::de::Error::missing_field("fromNode"))?;
                let to_node = to_node.ok_or_else(|| serde::de::Error::missing_field("toNode"))?;

                Ok(Edge {
                    id,
                    from_node,
                    from_side,
                    from_end,
                    to_node,
                    to_side,
                    to_end,
                    color,
                    label,
                })

            }

        }

        deserializer.deserialize_map(EdgeVisitor)


    }
}
