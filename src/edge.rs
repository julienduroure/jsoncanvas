use serde::Serialize;
use serde::ser::{SerializeStruct, Serializer};

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

pub enum Side {
    Top,
    Left,
    Right,
    Bottom,
}

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
