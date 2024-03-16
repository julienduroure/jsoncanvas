use jsoncanvas::{color::{Color, PresetColor}, jsoncanvas::JsonCanvas, node::{BackGround, BackgroundStyle, FileNode, GroupNode, LinkNode, Node, NodeType, TextNode}};
use jsoncanvas::edge::{Edge, End, Side};
use serde_json;
use url::Url;
use hex_color::HexColor;

fn main() {

    ///////////////////////////// Serialization /////////////////////////////

    // Color
    let color1 = Color::Preset(PresetColor::Red);
    let color2 = Color::Color(HexColor::parse("#ff0000").unwrap());

    let serialized_color1 = serde_json::to_string(&color1).unwrap();
    let serialized_color2 = serde_json::to_string(&color2).unwrap();

    println!("serialized1 = {}", serialized_color1);
    println!("serialized2 = {}", serialized_color2);

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
    node4.set(NodeType::Link(LinkNode::new(Url::parse("https://julienduroure.com").unwrap())));

    // Group Node
    let mut node5: Node = Node::new("id5".to_string(), 0, 0, 100, 100, Some(color2));
    node5.set(NodeType::Group(GroupNode::new(Some("Label".to_string()), None)));

    let mut node6: Node = Node::new("id6".to_string(), 0, 0, 100, 100, None);
    node6.set(NodeType::Group(GroupNode::new(None, None)));

    let mut node7: Node = Node::new("id7".to_string(), 0, 0, 100, 100, None);
    node7.set(NodeType::Group(GroupNode::new(None, Some(BackGround::new("path/to/image.png".to_string(), None)))));

    let mut node8: Node = Node::new("id8".to_string(), 0, 0, 100, 100, None);
    node8.set(NodeType::Group(GroupNode::new(None, Some(BackGround::new("path/to/image.png".to_string(), Some(BackgroundStyle::Cover))))));

    let serialized_node1: String = serde_json::to_string(&node1).unwrap();
    let serialized_node2 = serde_json::to_string(&node2).unwrap();
    let serialized_node3 = serde_json::to_string(&node3).unwrap();
    let serialized_node4 = serde_json::to_string(&node4).unwrap();
    let serialized_node5 = serde_json::to_string(&node5).unwrap();
    let serialized_node6 = serde_json::to_string(&node6).unwrap();
    let serialized_node7 = serde_json::to_string(&node7).unwrap();
    let serialized_node8 = serde_json::to_string(&node8).unwrap();

    println!("serialized node 1= {}", serialized_node1);
    println!("serialized node 2= {}", serialized_node2);
    println!("serialized node 3= {}", serialized_node3);
    println!("serialized node 4= {}", serialized_node4);
    println!("serialized node 5= {}", serialized_node5);
    println!("serialized node 6= {}", serialized_node6);
    println!("serialized node 7= {}", serialized_node7);
    println!("serialized node 8= {}", serialized_node8);


    // Edge

    let edge1 = Edge::new("edge1".to_string(), "node1".to_string(), None, None, "node2".to_string(), Some(Side::Left), Some(End::Arrow), None, None);
    let edge2 = Edge::new("edge2".to_string(), "node3".to_string(), None, None, "node4".to_string(), Some(Side::Left), Some(End::Arrow), Some(Color::Preset(PresetColor::Cyan)), Some("edge label".to_string()));

    let serialized_edge1 = serde_json::to_string(&edge1).unwrap();
    let serialized_edge2 = serde_json::to_string(&edge2).unwrap();

    println!("serialized edge 1= {}", serialized_edge1);
    println!("serialized edge 2= {}", serialized_edge2);

    // JSON Canvas
    let mut canvas = jsoncanvas::jsoncanvas::JsonCanvas::new();
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

    let jsoncanvas_deserialized: jsoncanvas::jsoncanvas::JsonCanvas = JsonCanvas::from_string(serialized_canvas);
    println!("deserialized canvas = {:?}", jsoncanvas_deserialized);

}







