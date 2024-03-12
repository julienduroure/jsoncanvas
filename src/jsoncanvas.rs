use std::collections::HashMap;

use crate::node::Node;
use crate::edge::Edge;

#[derive(Debug)]
pub enum JsonCanvasError {
    AlreadyExists,
}

pub struct JsonCanvas {
    nodes: HashMap<String, Node>,
    edges: HashMap<String, Edge>
}


impl JsonCanvas {
    pub fn new() -> JsonCanvas {
        JsonCanvas {
            nodes: HashMap::new(),
            edges: HashMap::new()
        }
    }

    pub fn add_node(&mut self, node: Node) -> Result<(), JsonCanvasError> {
        if self.nodes.contains_key(&node.id) {
            return Err(JsonCanvasError::AlreadyExists);
        }
        self.nodes.insert(node.id.clone(), node);
        Ok(())
    }

    pub fn add_edge(&mut self, edge: Edge) -> Result<(), JsonCanvasError>{
        if self.edges.contains_key(&edge.id) {
            return Err(JsonCanvasError::AlreadyExists);
        }
        self.edges.insert(edge.id.clone(), edge);
        Ok(())
    }

    pub fn get_node(&mut self, id: String) -> Option<&mut Node> {
        self.nodes.get_mut(&id)
    }
}
