use crate::color::Color;
use crate::NodeId;
use crate::PixelCoordinate;
use crate::PixelDimension;
use ambassador::{delegatable_trait, Delegate};
use serde::{Deserialize, Serialize};

mod file;
mod group;
mod link;
mod text;

pub use file::FileNode;
pub use group::{Background, BackgroundStyle, GroupNode};
pub use link::LinkNode;
pub use text::TextNode;

#[derive(Debug, Serialize, Deserialize)]
pub struct GenericNode {
    pub id: NodeId,
    x: PixelCoordinate,
    y: PixelCoordinate,
    width: PixelDimension,
    height: PixelDimension,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<crate::color::Color>,
}

impl GenericNode {
    pub fn new(
        id: NodeId,
        x: PixelCoordinate,
        y: PixelCoordinate,
        width: PixelDimension,
        height: PixelDimension,
        color: Option<Color>,
    ) -> Self {
        Self {
            id,
            x,
            y,
            width,
            height,
            color,
        }
    }
}

#[delegatable_trait]
pub trait GenericNodeInfo {
    fn id(&self) -> &NodeId;
    fn get_x(&self) -> PixelCoordinate;
    fn get_y(&self) -> PixelCoordinate;
    fn get_width(&self) -> PixelDimension;
    fn get_height(&self) -> PixelDimension;
    fn color(&self) -> &Option<Color>;
}

pub use ambassador_impl_GenericNodeInfo;

impl GenericNodeInfo for GenericNode {
    fn id(&self) -> &NodeId {
        &self.id
    }

    fn get_x(&self) -> PixelCoordinate {
        self.x
    }

    fn get_y(&self) -> PixelCoordinate {
        self.y
    }

    fn get_width(&self) -> PixelDimension {
        self.width
    }

    fn get_height(&self) -> PixelDimension {
        self.height
    }

    fn color(&self) -> &Option<Color> {
        &self.color
    }
}

#[derive(Debug, Delegate, Serialize, Deserialize)]
#[delegate(GenericNodeInfo)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Node {
    Text(TextNode),
    File(FileNode),
    Link(LinkNode),
    Group(GroupNode),
}

impl From<GroupNode> for Node {
    fn from(node: GroupNode) -> Self {
        Node::Group(node)
    }
}

impl From<TextNode> for Node {
    fn from(node: TextNode) -> Self {
        Node::Text(node)
    }
}

impl From<FileNode> for Node {
    fn from(node: FileNode) -> Self {
        Node::File(node)
    }
}

impl From<LinkNode> for Node {
    fn from(node: LinkNode) -> Self {
        Node::Link(node)
    }
}
