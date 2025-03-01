use serde::{Deserialize, Serialize};

use crate::{color::Color, EdgeId, NodeId};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edge {
    pub id: EdgeId,
    pub from_node: NodeId,
    #[serde(skip_serializing_if = "Option::is_none")]
    from_side: Option<Side>,
    #[serde(skip_serializing_if = "Option::is_none")]
    from_end: Option<End>,
    pub to_node: NodeId,
    #[serde(skip_serializing_if = "Option::is_none")]
    to_side: Option<Side>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to_end: Option<End>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<crate::color::Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
}

pub type Terminus = (NodeId, Option<Side>, Option<End>);

impl Edge {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: EdgeId,
        from_node: NodeId,
        from_side: Option<Side>,
        from_end: Option<End>,
        to_node: NodeId,
        to_side: Option<Side>,
        to_end: Option<End>,
        color: Option<crate::color::Color>,
        label: Option<String>,
    ) -> Edge {
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

    pub fn id(&self) -> &EdgeId {
        &self.id
    }

    pub fn from_node(&self) -> &NodeId {
        &self.from_node
    }

    pub fn from_side(&self) -> Option<&Side> {
        self.from_side.as_ref()
    }

    pub fn from_end(&self) -> Option<&End> {
        self.from_end.as_ref()
    }

    pub fn to_node(&self) -> &NodeId {
        &self.to_node
    }

    pub fn to_side(&self) -> Option<&Side> {
        self.to_side.as_ref()
    }

    pub fn to_end(&self) -> Option<&End> {
        self.to_end.as_ref()
    }

    pub fn color(&self) -> Option<&Color> {
        self.color.as_ref()
    }

    pub fn label(&self) -> Option<&String> {
        self.label.as_ref()
    }

    pub fn set_color(&mut self, color: Color) -> &mut Self {
        self.color = Some(color);
        self
    }

    pub fn remove_color(&mut self) -> Option<Color> {
        std::mem::take(&mut self.color)
    }

    pub fn set_label(&mut self, label: String) -> &mut Self {
        self.label = Some(label);
        self
    }

    pub fn remove_label(&mut self) -> Option<String> {
        std::mem::take(&mut self.label)
    }

    pub fn set_from(&mut self, node: NodeId, side: Option<Side>, end: Option<End>) -> Terminus {
        (
            std::mem::replace(&mut self.from_node, node),
            std::mem::replace(&mut self.from_side, side),
            std::mem::replace(&mut self.from_end, end),
        )
    }

    pub fn set_to(&mut self, node: NodeId, side: Option<Side>, end: Option<End>) -> Terminus {
        (
            std::mem::replace(&mut self.to_node, node),
            std::mem::replace(&mut self.to_side, side),
            std::mem::replace(&mut self.to_end, end),
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Side {
    Top,
    Left,
    Right,
    Bottom,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum End {
    None,
    Arrow,
}
