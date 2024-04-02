//! # jsoncanvas
//!
//! `jsoncanvas` is a library for creating and manipulating JSON objects representing a canvas.
//!
//! Specification source: <https://jsoncanvas.org/>
//!
//! ## Example
//!
//! ```
//! use jsoncanvas::JsonCanvas;
//! let s: String = "{\"nodes\":[{\"id\":\"id7\",\"x\":0,\"y\":0,\"width\":100,\"height\":100,\"background\":\"path/to/image.png\",\"type\":\"group\"},{\"id\":\"id5\",\"x\":0,\"y\":0,\"width\":100,\"height\":100,\"color\":\"#ff0000\",\"label\":\"Label\",\"type\":\"group\"},{\"id\":\"id2\",\"x\":0,\"y\":0,\"width\":100,\"height\":100,\"color\":\"1\",\"file\":\"dir/to/path/file.png\",\"type\":\"file\"},{\"id\":\"id4\",\"x\":0,\"y\":0,\"width\":100,\"height\":100,\"color\":\"1\",\"url\":\"https://www.google.com\",\"type\":\"link\"},{\"id\":\"id6\",\"x\":0,\"y\":0,\"width\":100,\"height\":100,\"type\":\"group\"},{\"id\":\"id3\",\"x\":0,\"y\":0,\"width\":100,\"height\":100,\"color\":\"1\",\"file\":\"dir/to/path/file.png\",\"subpath\":\"#here\",\"type\":\"file\"},{\"id\":\"id8\",\"x\":0,\"y\":0,\"width\":100,\"height\":100,\"background\":\"path/to/image.png\",\"backgroundStyle\":\"cover\",\"type\":\"group\"},{\"id\":\"id\",\"x\":0,\"y\":0,\"width\":100,\"height\":100,\"color\":\"1\",\"text\":\"Test\",\"type\":\"text\"}],\"edges\":[{\"id\":\"edge2\",\"fromNode\":\"node3\",\"toNode\":\"node4\",\"color\":\"5\",\"label\":\"edge label\",\"toSide\":\"left\",\"toEnd\":\"arrow\"},{\"id\":\"edge1\",\"fromNode\":\"node1\",\"toNode\":\"node2\",\"toSide\":\"left\",\"toEnd\":\"arrow\"}]}".to_string();
//! let canvas: JsonCanvas = s.parse().unwrap();
//!
//! let _s = canvas.to_string();
//! ```
//!
//! ## Complete example
//!
//! ```
//!    use hex_color::HexColor;
//!    use jsoncanvas::color::Color;
//!    use jsoncanvas::edge::{Edge, End, Side};
//!    use jsoncanvas::node::{
//!        Background, BackgroundStyle, FileNode, GroupNode, LinkNode, Node, TextNode,
//!    };
//!    use jsoncanvas::JsonCanvas;
//!    use std::path::PathBuf;
//!    use url::Url;
//!
//!    // Color
//!    let color1 = Color::Preset("red".to_string());
//!    let color2 = Color::Color(HexColor::parse("#ff0000").unwrap());
//!
//!    let serialized_color1 = serde_json::to_string(&color1).unwrap();
//!    let serialized_color2 = serde_json::to_string(&color2).unwrap();
//!
//!    println!("serialized1 = {}", serialized_color1);
//!    println!("serialized2 = {}", serialized_color2);
//!
//!    // Text Node
//!    let node1: Node = Node::Text(TextNode::new(
//!        "id".parse().unwrap(),
//!        0,
//!        0,
//!        100,
//!        100,
//!        Some(Color::Preset("red".to_string())),
//!        "This is a test".to_string(),
//!    ));
//!
//!    // File Node
//!    let node2: Node = Node::File(FileNode::new(
//!        "id2".parse().unwrap(),
//!        0,
//!        0,
//!        100,
//!        100,
//!        Some(Color::Preset("red".to_string())),
//!        PathBuf::from("dir/to/path/file.png"),
//!        None,
//!    ));
//!    let node3: Node = Node::File(FileNode::new(
//!        "id3".parse().unwrap(),
//!        0,
//!        0,
//!        100,
//!        100,
//!        Some(color1),
//!        PathBuf::from("dir/to/path/file.png"),
//!        Some("#here".parse().unwrap()),
//!    ));
//!
//!    // Link Node
//!    let node4: Node = Node::Link(LinkNode::new(
//!        "id4".parse().unwrap(),
//!        0,
//!        0,
//!        100,
//!        100,
//!        Some(Color::Preset("red".to_string())),
//!        Url::parse("https://julienduroure.com").unwrap(),
//!    ));
//!
//!    // Group Node
//!    let node5: Node = Node::Group(GroupNode::new(
//!        "id5".parse().unwrap(),
//!        0,
//!        0,
//!        100,
//!        100,
//!        Some(color2),
//!        Some("Label".to_string()),
//!        None,
//!    ));
//!    let node6: Node = Node::Group(GroupNode::new(
//!        "id6".parse().unwrap(),
//!        0,
//!        0,
//!        100,
//!        100,
//!        None,
//!        None,
//!        None,
//!    ));
//!    let node7: Node = Node::Group(GroupNode::new(
//!        "id7".parse().unwrap(),
//!        0,
//!        0,
//!        100,
//!        100,
//!        None,
//!        None,
//!        Some(Background::new(PathBuf::from("path/to/image.png"), None)),
//!    ));
//!    let node8: Node = Node::Group(GroupNode::new(
//!        "id8".parse().unwrap(),
//!        0,
//!        0,
//!        100,
//!        100,
//!        None,
//!        None,
//!        Some(Background::new(
//!            PathBuf::from("path/to/image.png"),
//!            Some(BackgroundStyle::Cover),
//!        )),
//!    ));
//!
//!    let serialized_node1: String = serde_json::to_string(&node1).unwrap();
//!    let serialized_node2 = serde_json::to_string(&node2).unwrap();
//!    let serialized_node3 = serde_json::to_string(&node3).unwrap();
//!    let serialized_node4 = serde_json::to_string(&node4).unwrap();
//!    let serialized_node5 = serde_json::to_string(&node5).unwrap();
//!    let serialized_node6 = serde_json::to_string(&node6).unwrap();
//!    let serialized_node7 = serde_json::to_string(&node7).unwrap();
//!    let serialized_node8 = serde_json::to_string(&node8).unwrap();
//!
//!    println!("serialized node 1= {}", serialized_node1);
//!    println!("serialized node 2= {}", serialized_node2);
//!    println!("serialized node 3= {}", serialized_node3);
//!    println!("serialized node 4= {}", serialized_node4);
//!    println!("serialized node 5= {}", serialized_node5);
//!    println!("serialized node 6= {}", serialized_node6);
//!    println!("serialized node 7= {}", serialized_node7);
//!    println!("serialized node 8= {}", serialized_node8);
//!
//!    // Edge
//!
//!    let edge1 = Edge::new(
//!        "edge1".parse().unwrap(),
//!        "id".parse().unwrap(),
//!        None,
//!        None,
//!        "id2".parse().unwrap(),
//!        Some(Side::Left),
//!        Some(End::Arrow),
//!        None,
//!        None,
//!    );
//!    let edge2 = Edge::new(
//!        "edge2".parse().unwrap(),
//!        "id3".parse().unwrap(),
//!        None,
//!        None,
//!        "id4".parse().unwrap(),
//!        Some(Side::Left),
//!        Some(End::Arrow),
//!        Some(Color::Preset("cyan".to_string())),
//!        Some("edge label".to_string()),
//!    );
//!
//!    let serialized_edge1 = serde_json::to_string(&edge1).unwrap();
//!    let serialized_edge2 = serde_json::to_string(&edge2).unwrap();
//!
//!    println!("serialized edge 1= {}", serialized_edge1);
//!    println!("serialized edge 2= {}", serialized_edge2);
//!
//!    // JSON Canvas
//!    let mut canvas = JsonCanvas::default();
//!
//!    let empty_canvas = canvas.to_string();
//!    println!("empty canvas = {}", empty_canvas);
//!    canvas = empty_canvas.parse().unwrap();
//!
//!    canvas.add_node(node1).unwrap();
//!    canvas.add_node(node2).unwrap();
//!    canvas.add_node(node3).unwrap();
//!    canvas.add_node(node4).unwrap();
//!    canvas.add_node(node5).unwrap();
//!    canvas.add_node(node6).unwrap();
//!    canvas.add_node(node7).unwrap();
//!    canvas.add_node(node8).unwrap();
//!
//!    canvas.add_edge(edge1).unwrap();
//!    canvas.add_edge(edge2).unwrap();
//!
//!    let serialized_canvas = canvas.to_string();
//!
//!    println!("serialized canvas = {}", serialized_canvas);
//!
//!    let jsoncanvas_deserialized: JsonCanvas = serialized_canvas.parse().unwrap();
//!    println!("deserialized canvas = {:?}", jsoncanvas_deserialized);
//! ```
//!
//! ## Available structs
//!
//! ```
//! use hex_color::HexColor;
//! use jsoncanvas::color::Color;
//! use jsoncanvas::edge::{Edge, End, Side};
//! use jsoncanvas::node::{
//!     Background, BackgroundStyle, FileNode, GroupNode, LinkNode, Node, TextNode,
//! };
//! use jsoncanvas::JsonCanvas;
//! use std::path::PathBuf;
//! use url::Url;
//! ```

// pub type NodeId = String;
// pub type EdgeId = String;
pub type PixelCoordinate = i64;
pub type PixelDimension = u64;

pub mod color;
pub mod edge;
mod id;
pub mod jsoncanvas;
pub mod node;

pub use id::{EdgeId, NodeId};
pub use jsoncanvas::JsonCanvas;
pub use jsoncanvas::JsonCanvasError;
pub use node::{Background, BackgroundStyle, FileNode, GroupNode, LinkNode, Node, TextNode};

#[cfg(test)]
mod test {
    use hex_color::HexColor;

    #[test]
    fn test() {
        use super::color::Color;
        use super::edge::{Edge, End, Side};
        use super::jsoncanvas::JsonCanvas;
        use super::node::{
            Background, BackgroundStyle, FileNode, GroupNode, LinkNode, Node, TextNode,
        };
        use std::path::PathBuf;
        use url::Url;

        // Color
        let color1 = Color::Preset("red".to_string());
        let color2 = Color::Color(HexColor::parse("#ff0000").unwrap());

        // Text Node
        let node1: Node = TextNode::new(
            "id".parse().unwrap(),
            0,
            0,
            100,
            100,
            Some(Color::Preset("red".to_string())),
            "This is a test".to_string(),
        )
        .into();

        // File Node
        let node2: Node = FileNode::new(
            "id2".parse().unwrap(),
            0,
            0,
            100,
            100,
            Some(Color::Preset("red".to_string())),
            PathBuf::from("dir/to/path/file.png"),
            None,
        )
        .into();
        let node3: Node = FileNode::new(
            "id3".parse().unwrap(),
            0,
            0,
            100,
            100,
            Some(color1),
            PathBuf::from("dir/to/path/file.png"),
            Some("#here".to_string()),
        )
        .into();

        // Link Node
        let node4: Node = LinkNode::new(
            "id4".parse().unwrap(),
            0,
            0,
            100,
            100,
            Some(Color::Preset("red".to_string())),
            Url::parse("https://julienduroure.com").unwrap(),
        )
        .into();

        // Group Node
        let node5: Node = GroupNode::new(
            "id5".parse().unwrap(),
            0,
            0,
            100,
            100,
            Some(color2),
            Some("Label".to_string()),
            None,
        )
        .into();
        let node6: Node =
            GroupNode::new("id6".parse().unwrap(), 0, 0, 100, 100, None, None, None).into();
        let node7: Node = GroupNode::new(
            "id7".parse().unwrap(),
            0,
            0,
            100,
            100,
            None,
            None,
            Some(Background::new(PathBuf::from("path/to/image.png"), None)),
        )
        .into();
        let node8: Node = GroupNode::new(
            "id8".parse().unwrap(),
            0,
            0,
            100,
            100,
            None,
            None,
            Some(Background::new(
                PathBuf::from("path/to/image.png"),
                Some(BackgroundStyle::Cover),
            )),
        )
        .into();

        // Edge

        let edge1 = Edge::new(
            "edge1".parse().unwrap(),
            "id".parse().unwrap(),
            None,
            None,
            "id2".parse().unwrap(),
            Some(Side::Left),
            Some(End::Arrow),
            None,
            None,
        );
        let edge2 = Edge::new(
            "edge2".parse().unwrap(),
            "id3".parse().unwrap(),
            None,
            None,
            "id4".parse().unwrap(),
            Some(Side::Left),
            Some(End::Arrow),
            Some(Color::Preset("cyan".to_string())),
            Some("edge label".to_string()),
        );

        // JSON Canvas
        let mut canvas = JsonCanvas::default();
        canvas.add_node(node1).unwrap();
        canvas.add_node(node2).unwrap();
        canvas.add_node(node3).unwrap();
        canvas.add_node(node4).unwrap();
        canvas.add_node(node5).unwrap();
        canvas.add_node(node6).unwrap();
        canvas.add_node(node7).unwrap();
        canvas.add_node(node8).unwrap();

        canvas.add_edge(edge1).unwrap();
        canvas.add_edge(edge2).unwrap();

        let serialized_canvas = canvas.to_string();

        println!("serialized canvas = {}", serialized_canvas);

        ///////////////////////////// Deserialization /////////////////////////////

        // let deserialized_node1: Node = serde_json::from_str(&serialized_node1).unwrap();
        // println!("deserialized node 1= {:?}", deserialized_node1);

        // let deseralied_edge1: Edge = serde_json::from_str(&serialized_edge1).unwrap();
        // println!("deserialized edge 1= {:?}", deseralied_edge1);

        let _jsoncanvas_deserialized: JsonCanvas = serialized_canvas.parse().unwrap();
    }
}
