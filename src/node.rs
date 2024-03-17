use serde::{Deserialize, Serialize};
use ambassador::{delegatable_trait, Delegate};
use std::path::PathBuf;
use url::Url;
use crate::color::Color;
use crate::PixelCoordinate;
use crate::PixelDimension;



#[derive(Debug, Serialize, Deserialize)]
pub struct GenericNode {
    pub id: String,
    x: PixelCoordinate,
    y: PixelCoordinate,
    width: PixelDimension,
    height: PixelDimension,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<crate::color::Color>,
}

impl GenericNode {
    pub fn new(
        id: String,
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
    fn id(&self) -> &String;
    fn get_x(self) -> PixelCoordinate;
    fn get_y(self) -> PixelCoordinate;
    fn get_width(self) -> PixelDimension;
    fn get_height(self) -> PixelDimension;
    fn color(&self) -> &Option<Color>;
}

impl GenericNodeInfo for GenericNode {
    fn id(&self) -> &String {
        &self.id
    }

    fn get_x(self) -> PixelCoordinate {
        self.x
    }

    fn get_y(self) -> PixelCoordinate {
        self.y
    }

    fn get_width(self) -> PixelDimension {
        self.width
    }

    fn get_height(self) -> PixelDimension {
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

#[derive(Debug, Delegate, Serialize, Deserialize)]
#[delegate(GenericNodeInfo, target = "generic")]
pub struct TextNode {
    #[serde(flatten)]
    generic: GenericNode,
    text: String,
}

impl TextNode {
    pub fn new(
        id: String,
        x: PixelCoordinate,
        y: PixelCoordinate,
        width: PixelDimension,
        height: PixelDimension,
        color: Option<Color>,
        text: String,
    ) -> Self {
        Self {
            generic: GenericNode::new(id, x, y, width, height, color),
            text,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}


#[derive(Debug, Delegate, Serialize, Deserialize)]
#[delegate(GenericNodeInfo, target = "generic")]
pub struct FileNode {
    #[serde(flatten)]
    generic: GenericNode,
    file: PathBuf,
    #[serde(skip_serializing_if = "Option::is_none")]
    subpath: Option<String>,
}

impl FileNode {
    pub fn new(
        id: String,
        x: PixelCoordinate,
        y: PixelCoordinate,
        width: PixelDimension,
        height: PixelDimension,
        color: Option<Color>,
        file: PathBuf,
        subpath: Option<String>,
    ) -> Self {
        Self {
            generic: GenericNode::new(id, x, y, width, height, color),
            file,
            subpath,
        }
    }

    pub fn file(&self) -> &PathBuf {
        &self.file
    }

    pub fn subpath(&self) -> Option<&String> {
        self.subpath.as_ref()
    }
}


#[derive(Debug, Delegate, Serialize, Deserialize)]
#[delegate(GenericNodeInfo, target = "generic")]
pub struct LinkNode {
    #[serde(flatten)]
    generic: GenericNode,
    url: Url,
}

impl LinkNode {
    pub fn new(
        id: String,
        x: PixelCoordinate,
        y: PixelCoordinate,
        width: PixelDimension,
        height: PixelDimension,
        color: Option<Color>,
        url: Url,
    ) -> Self {
        Self {
            generic: GenericNode::new(id, x, y, width, height, color),
            url,
        }
    }

    pub fn url(&self) -> &Url {
        &self.url
    }
}


#[derive(Debug, Delegate, Serialize, Deserialize)]
#[delegate(GenericNodeInfo, target = "generic")]
pub struct GroupNode {
    #[serde(flatten)]
    generic: GenericNode,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    background: Option<BackGround>,
}

impl GroupNode {
    pub fn new(
        id: String,
        x: PixelCoordinate,
        y: PixelCoordinate,
        width: PixelDimension,
        height: PixelDimension,
        color: Option<Color>,
        label: Option<String>,
        background: Option<BackGround>,
    ) -> Self {
        Self {
            generic: GenericNode::new(id, x, y, width, height, color),
            label,
            background,
        }
    }

    pub fn label(&self) -> Option<&String> {
        self.label.as_ref()
    }

    pub fn background(&self) -> Option<&BackGround> {
        self.background.as_ref()
    }
}



#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackGround {
    image: PathBuf,
    #[serde(skip_serializing_if = "Option::is_none")]
    background_style : Option<BackgroundStyle>
}

impl BackGround {
    pub fn new(image: PathBuf, background_style: Option<BackgroundStyle>) -> BackGround {
        BackGround {
            image,
            background_style,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BackgroundStyle {
    Cover,
    Ratio,
    Repeat,
}


