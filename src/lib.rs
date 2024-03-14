pub mod node;
pub mod edge;
pub mod color;
pub mod jsoncanvas;


#[cfg(test)]
mod test {
    #[test]
    fn test() {

        use super::jsoncanvas::JsonCanvas;
        use super::node::{Node, NodeType, TextNode, FileNode, LinkNode, GroupNode, BackGround, BackgroundStyle};
        use super::color::{Color, PresetColor};
        use super::edge::{Edge, End, Side};


        // Color
        let color1 = Color::Preset(PresetColor::Red);
        let color2 = Color::Color("#ff0000".to_string());

        // Text Node

        let node_type_text = NodeType::Text(TextNode::new("Test".to_string()));

        let mut node1 = Node::new("id".to_string(), 0, 0, 100, 100, Some(Color::Preset(PresetColor::Red)));
        node1.set(node_type_text);

        // File Node
        let mut node2: Node = Node::new("id2".to_string(), 0, 0, 100, 100, Some(Color::Preset(PresetColor::Red)));
        node2.set(NodeType::File(FileNode::new("dir/to/path/file.png".to_string(), None)));

        let mut node3: Node = Node::new("id3".to_string(), 0, 0, 100, 100, Some(color1));
        node3.set(NodeType::File(FileNode::new("dir/to/path/file.png".to_string(), Some("#here".to_string()))));

        // Link Node
        let mut node4: Node = Node::new("id4".to_string(), 0, 0, 100, 100, Some(Color::Preset(PresetColor::Red)));
        node4.set(NodeType::Link(LinkNode::new("https://julienduroure.com".to_string())));

        // Group Node
        let mut node5: Node = Node::new("id5".to_string(), 0, 0, 100, 100, Some(color2));
        node5.set(NodeType::Group(GroupNode::new(Some("Label".to_string()), None)));

        let mut node6: Node = Node::new("id6".to_string(), 0, 0, 100, 100, None);
        node6.set(NodeType::Group(GroupNode::new(None, None)));

        let mut node7: Node = Node::new("id7".to_string(), 0, 0, 100, 100, None);
        node7.set(NodeType::Group(GroupNode::new(None, Some(BackGround::new("path/to/image.png".to_string(), None)))));

        let mut node8: Node = Node::new("id8".to_string(), 0, 0, 100, 100, None);
        node8.set(NodeType::Group(GroupNode::new(None, Some(BackGround::new("path/to/image.png".to_string(), Some(BackgroundStyle::Cover))))));


        // Edge

        let edge1 = Edge::new("edge1".to_string(), "node1".to_string(), None, None, "node2".to_string(), Some(Side::Left), Some(End::Arrow), None, None);
        let edge2 = Edge::new("edge2".to_string(), "node3".to_string(), None, None, "node4".to_string(), Some(Side::Left), Some(End::Arrow), Some(Color::Preset(PresetColor::Cyan)), Some("edge label".to_string()));

        // JSON Canvas
        let mut canvas = JsonCanvas::new();
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

        let serialized_canvas = serde_json::to_string(&canvas).unwrap();

        println!("serialized canvas = {}", serialized_canvas);


        ///////////////////////////// Deserialization /////////////////////////////

        // let deserialized_node1: Node = serde_json::from_str(&serialized_node1).unwrap();
        // println!("deserialized node 1= {:?}", deserialized_node1);

        // let deseralied_edge1: Edge = serde_json::from_str(&serialized_edge1).unwrap();
        // println!("deserialized edge 1= {:?}", deseralied_edge1);

        let _jsoncanvas_deserialized: JsonCanvas = serde_json::from_str(&serialized_canvas).unwrap();

    }
}
