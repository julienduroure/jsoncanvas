use serde::{Deserialize, Serialize};
use serde::ser::{SerializeStruct, Serializer};
use serde::de::{Visitor, Deserializer, MapAccess};

use std::fmt;

#[derive(Debug)]
pub struct Node {
    pub id: String,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    color: Option<crate::color::Color>,
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

    pub fn get_url(&self) -> Option<String> {
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

#[derive(Debug)]
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

#[derive(Debug, Deserialize)]
pub struct FileNode {
    file: String,
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

#[derive(Debug)]
pub struct LinkNode {
    url: String,
}

impl LinkNode {
    pub fn new(url: String) -> LinkNode {
        LinkNode {
            url,
        }
    }
}

#[derive(Debug)]
pub struct GroupNode {
    label: Option<String>,
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

#[derive(Debug)]
pub struct BackGround {
    image: String,
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

#[derive(Debug)]
pub enum BackgroundStyle {
    Cover,
    Ratio,
    Repeat,
}

impl Serialize for BackgroundStyle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        {
            match self {
                BackgroundStyle::Cover => serializer.serialize_str("cover"),
                BackgroundStyle::Ratio => serializer.serialize_str("ratio"),
                BackgroundStyle::Repeat => serializer.serialize_str("repeat"),
            }
        }
}

impl Serialize for BackGround {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        {
            if self.background_style.is_some() {
                let mut state = serializer.serialize_struct("BackGround", 2)?;
                state.serialize_field("background", &self.image)?;
                state.serialize_field("backgroundStyle", &self.background_style)?;
                state.end()
            } else {
                let mut state = serializer.serialize_struct("BackGround", 1)?;
                state.serialize_field("background", &self.image)?;
                state.end()
            }
        }
}

impl BackGround {
    fn serialize_fields<S>(&self, state: &mut S) -> Result<(), S::Error>
    where
        S: SerializeStruct,
    {
        match &self.background_style {
            Some(_style) => {
                state.serialize_field("background", &self.image)?;
                state.serialize_field("backgroundStyle", &self.background_style)?;
            },
            None => {
                state.serialize_field("background", &self.image)?;
            }
        }
        Ok(())
    }

}


#[derive(Debug)]
pub enum NodeType {
    None,
    Text(TextNode),
    File(FileNode),
    Link(LinkNode),
    Group(GroupNode),
}

impl Serialize for NodeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        {
            match self {
                NodeType::Text(textnode) => {
                    let mut state = serializer.serialize_struct("TextNode", 1)?;
                    state.serialize_field("text", &textnode.text)?;
                    state.end()
                },
                NodeType::File(filenode) => {
                    if filenode.subpath.is_some() {
                        let mut state = serializer.serialize_struct("FileNode", 2)?;
                        state.serialize_field("file", &filenode.file)?;
                        state.serialize_field("subpath", &filenode.subpath)?;
                        state.end()
                    } else {
                        let mut state = serializer.serialize_struct("FileNode", 1)?;
                        state.serialize_field("file", &filenode.file)?;
                        state.end()
                    }
                },
                NodeType::Link(linknode) => {
                    let mut state = serializer.serialize_struct("LinkNode", 1)?;
                    state.serialize_field("url", &linknode.url)?;
                    state.end()
                },
                NodeType::Group(groupnode) => {
                    if groupnode.background.is_some() && groupnode.background.as_ref().unwrap().background_style.is_some() {
                            let mut state = serializer.serialize_struct("GroupNode", 3)?;
                            state.serialize_field("label", &groupnode.label)?;
                            groupnode.background.as_ref().unwrap().serialize_fields(&mut state)?;
                            state.end()
                    } else if groupnode.background.is_some() && !groupnode.background.as_ref().unwrap().background_style.is_some() {
                        let mut state = serializer.serialize_struct("GroupNode", 2)?;
                            state.serialize_field("label", &groupnode.label)?;
                            groupnode.background.as_ref().unwrap().serialize_fields(&mut state)?;
                            state.end()

                    } else {
                        if groupnode.label.is_some() {
                            let mut state = serializer.serialize_struct("GroupNode", 1)?;
                            state.serialize_field("label", &groupnode.label)?;
                            state.end()
                        } else {
                            let state = serializer.serialize_struct("GroupNode", 0)?;
                            state.end()
                        }
                    }
                },
                _ => {
                    let state = serializer.serialize_struct("None", 0)?;
                    state.end()
                }
            }
        }

}

impl NodeType {
    fn serialize_fields<S>(&self, state: &mut S) -> Result<(), S::Error>
    where
        S: SerializeStruct,
    {
        match self {
            NodeType::Text(textnode) => {
                state.serialize_field("text", &textnode.text)?;
                state.serialize_field("type", "text")?;
            },
            NodeType::File(filenode) => {
                if filenode.subpath.is_some() {
                    state.serialize_field("file", &filenode.file)?;
                    state.serialize_field("subpath", &filenode.subpath)?;
                } else {
                    state.serialize_field("file", &filenode.file)?;
                }
                state.serialize_field("type", "file")?;
            },
            NodeType::Link(linknode) => {
                state.serialize_field("url", &linknode.url)?;
                state.serialize_field("type", "link")?;
            },
            NodeType::Group(group) => {
                if group.label.is_some() {
                    state.serialize_field("label", &group.label)?;
                }
                if group.background.is_some() {
                    state.serialize_field("background", &group.background.as_ref().unwrap().image)?;
                }
                if group.background.is_some() && group.background.as_ref().unwrap().background_style.is_some() {
                    state.serialize_field("backgroundStyle", &group.background.as_ref().unwrap().background_style.as_ref().unwrap())?;
                }
                state.serialize_field("type", "group")?;
            }
            _ => {
                // This should never happen
            }
        }
        Ok(())
    }
}

impl Serialize for Node {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        {
            let nb = if self.color.is_some() { 7 } else { 6 };
            let mut state = serializer.serialize_struct("Node", nb)?;
            state.serialize_field("id", &self.id)?;
            state.serialize_field("x", &self.x)?;
            state.serialize_field("y", &self.y)?;
            state.serialize_field("width", &self.width)?;
            state.serialize_field("height", &self.height)?;
            if self.color.is_some() {
                state.serialize_field("color", &self.color)?;
            }
            self.node_type.serialize_fields(&mut state)?;
            state.end()
        }
}

impl<'de> Deserialize<'de> for Node {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NodeVisitor;

        impl<'de> Visitor<'de> for NodeVisitor {
            type Value = Node;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Node")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Node, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut node_type: Option<NodeType> = None;
                let mut id: Option<String> = None;
                let mut x: Option<i32> = None;
                let mut y: Option<i32> = None;
                let mut width: Option<i32> = None;
                let mut height: Option<i32> = None;
                let mut color: Option<crate::color::Color> = None;

                let mut text: Option<String> = None;
                let mut file: Option<String> = None;
                let mut subpath: Option<String> = None;
                let mut url: Option<String> = None;
                let mut label: Option<String> = None;
                let mut background: Option<String> = None;
                let mut style: Option<String> = None;

                enum Type {
                    Text,
                    File,
                    Link,
                    Group,
                }

                let mut type_node: Option<Type> = None;


                while let Some(key) = map.next_key()? {
                    match key {
                        "type" => {
                            let type_value: String = map.next_value()?;
                            match type_value.as_str() {
                                "text" => {
                                    type_node = Some(Type::Text);
                                }
                                "file" => {
                                    type_node = Some(Type::File);
                                }
                                "link" => {
                                    type_node = Some(Type::Link);
                                }
                                "group" => {
                                    type_node = Some(Type::Group);
                                }
                                _ => return Err(serde::de::Error::custom("invalid type")),
                            }
                        },
                        "id" => {
                            id = Some(map.next_value()?);
                        },
                        "x" => {
                            x = Some(map.next_value()?);
                        },
                        "y" => {
                            y = Some(map.next_value()?);
                        },
                        "width" => {
                            width = Some(map.next_value()?);
                        },
                        "height" => {
                            height = Some(map.next_value()?);
                        },
                        "color" => {
                            color = Some(map.next_value()?);
                        },
                        "text" => {
                            text = Some(map.next_value()?);
                        },
                        "file" => {
                            file = Some(map.next_value()?);
                        },
                        "subpath" => {
                            subpath = Some(map.next_value()?);
                        },
                        "url" => {
                            url = Some(map.next_value()?);
                        },
                        "label" => {
                            label = Some(map.next_value()?);
                        },
                        "background" => {
                            background =  Some(map.next_value()?);
                        },
                        "backgroundStyle" => {
                            style = Some(map.next_value()?);
                        },
                        _ => return Err(serde::de::Error::custom("invalid key")),
                    }
                }

                let id = id.ok_or_else(|| serde::de::Error::missing_field("id"))?;
                let x = x.ok_or_else(|| serde::de::Error::missing_field("x"))?;
                let y = y.ok_or_else(|| serde::de::Error::missing_field("y"))?;
                let width = width.ok_or_else(|| serde::de::Error::missing_field("width"))?;
                let height = height.ok_or_else(|| serde::de::Error::missing_field("height"))?;

                match type_node {
                    Some(Type::Text) => {
                        let text = text.ok_or_else(|| serde::de::Error::missing_field("text"))?;
                        node_type = Some(NodeType::Text(TextNode::new(text)));
                    },
                    Some(Type::File) => {
                        let file = file.ok_or_else(|| serde::de::Error::missing_field("file"))?;
                        node_type = Some(NodeType::File(FileNode::new(file, subpath)));
                    },
                    Some(Type::Link) => {
                        let url = url.ok_or_else(|| serde::de::Error::missing_field("url"))?;
                        node_type = Some(NodeType::Link(LinkNode::new(url)));
                    },
                    Some(Type::Group) => {
                        if background.is_some() && style.is_some() {
                            match style.unwrap().as_str() {
                                "cover" => {
                                    node_type = Some(NodeType::Group(GroupNode::new(label, Some(BackGround::new(background.unwrap(), Some(BackgroundStyle::Cover))))));
                                },
                                "ratio" => {
                                    node_type = Some(NodeType::Group(GroupNode::new(label, Some(BackGround::new(background.unwrap(), Some(BackgroundStyle::Ratio))))));
                                },
                                "repeat" => {
                                    node_type = Some(NodeType::Group(GroupNode::new(label, Some(BackGround::new(background.unwrap(), Some(BackgroundStyle::Repeat))))));
                                },
                                _ => return Err(serde::de::Error::custom("invalid background style")),
                            }
                        } else if background.is_some() && !style.is_some() {
                            node_type = Some(NodeType::Group(GroupNode::new(label, Some(BackGround::new(background.unwrap(), None)))));
                        } else if label.is_some() {
                            node_type = Some(NodeType::Group(GroupNode::new(label, None)));
                        } else {
                            node_type = Some(NodeType::Group(GroupNode::new(None, None)));
                        }
                    },
                    _ => {
                        return Err(serde::de::Error::custom("invalid type"));
                    }
                }

                Ok(Node {
                    id,
                    x,
                    y,
                    width,
                    height,
                    color,
                    node_type: node_type.unwrap(),
                })
            }
        }

        deserializer.deserialize_map(NodeVisitor)
    }
}
