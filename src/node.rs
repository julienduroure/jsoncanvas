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
            NodeType::Group(groupnode) => Some(groupnode.label.clone()),
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

#[derive(Debug)]
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
    label: String,
    background: Option<BackGround>
}

impl GroupNode {
    pub fn new(label: String, background: Option<BackGround>) -> GroupNode {
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

#[derive(Debug)]
pub enum NodeType {
    None,
    Text(TextNode),
    File(FileNode),
    Link(LinkNode),
    Group(GroupNode),
}

