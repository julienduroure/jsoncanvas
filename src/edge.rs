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
