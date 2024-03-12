use jsoncanvas::jsoncanvas::JsonCanvas;
use jsoncanvas::node::TextNode;
use jsoncanvas::node::{{Node, NodeType}};
use jsoncanvas::edge::{Edge, End, Side};

fn main() {
    let mut canvas = JsonCanvas::new();

    let mut node1 = Node::new("node1".to_string(), 0, 0, 100, 100, None);
    node1.set(NodeType::Text(TextNode::new("Test".to_string())));


    let node2 = Node::new("node2".to_string(), 300, 0, 100, 100, None);

    let edge = Edge::new("edge1".to_string(), "node1".to_string(), Some(Side::Right), Some(End::Arrow), "node2".to_string(), Some(Side::Left), Some(End::Arrow), None, None);

    canvas.add_node(node1).unwrap();
    canvas.add_node(node2).unwrap();

    canvas.add_edge(edge).unwrap();


    let node: &mut Node = canvas.get_node("node1".to_string()).unwrap();
    node.set_x(100);
    print!("{:?}", node);
}
