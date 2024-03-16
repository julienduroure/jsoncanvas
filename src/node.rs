use serde::{Deserialize, Serialize};
use url::Url;


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub id: String,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<crate::color::Color>,
    #[serde(flatten)]
    node_type: NodeType,
}

impl Node {
    pub fn new(id: String, x: i32, y: i32, width: i32, height: i32, color: Option<crate::color::Color>) -> Node {
        Node {
            id,
            x,
            y,
            width,
            height,
            color,
            node_type: NodeType::None,
        }
    }

    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    pub fn set_width(&mut self, width: i32) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: i32) {
        self.height = height;
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn set_size(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
    }

    pub fn set_color(&mut self, color: crate::color::Color) {
        self.color = Some(color);
    }

    pub fn remove_color(&mut self) {
        self.color = None;
    }

    pub fn set(&mut self, data: NodeType) {
        self.node_type = data;
    }

    pub fn get_text(&self) -> Option<String> {
        match &self.node_type {
            NodeType::Text(textnode) => Some(textnode.text.clone()),
            _ => None,
        }
    }

    pub fn get_file(&self) -> Option<String> {
        match &self.node_type {
            NodeType::File(filenode) => Some(filenode.file.clone()),
            _ => None,
        }
    }

    pub fn get_subpath(&self) -> Option<String> {
        match &self.node_type {
            NodeType::File(filenode) => {
                match &filenode.subpath {
                    Some(subpath) => Some(subpath.clone()),
                    None => None,
                }
            }
            _ => None,
        }
    }

    pub fn get_url(&self) -> Option<Url> {
        match &self.node_type {
            NodeType::Link(linknode) => Some(linknode.url.clone()),
            _ => None,
        }
    }

    pub fn get_label(&self) -> Option<String> {
        match &self.node_type {
            NodeType::Group(groupnode) => {
                match &groupnode.label {
                    Some(label) => Some(label.clone()),
                    None => None,
                }

            }
            _ => None,
        }
    }

    pub fn get_background_image(&self) -> Option<String> {
        match &self.node_type {
            NodeType::Group(groupnode) => {
                match &groupnode.background {
                    Some(background) => Some(background.image.clone()),
                    None => None,
                }
            }
            _ => None,
        }
    }

    pub fn get_background_style(&self) -> Option<&BackgroundStyle> {
        match &self.node_type {
            NodeType::Group(groupnode) => {
                match &groupnode.background {
                    Some(background) => {
                        match &background.background_style {
                            Some(style) => Some(style),
                            None => None,
                        }
                    }
                    None => None,
                }
            }
            _ => None,
        }
    }

}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextNode {
    text: String,
}

impl TextNode {
    pub fn new(text: String) -> TextNode {
        TextNode {
            text,
        }
    }

}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileNode {
    file: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    subpath: Option<String>,
}

impl FileNode {
    pub fn new(file: String, subpath: Option<String>) -> FileNode {
        FileNode {
            file,
            subpath,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkNode {
    url: Url,
}

impl LinkNode {
    pub fn new(url: Url) -> LinkNode {
        LinkNode {
            url,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupNode {
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    background: Option<BackGround>
}

impl GroupNode {
    pub fn new(label: Option<String>, background: Option<BackGround>) -> GroupNode {
        GroupNode {
            label,
            background,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackGround {
    image: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    background_style : Option<BackgroundStyle>
}

impl BackGround {
    pub fn new(image: String, background_style: Option<BackgroundStyle>) -> BackGround {
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NodeType {
    None,
    Text(TextNode),
    File(FileNode),
    Link(LinkNode),
    Group(GroupNode),
}
