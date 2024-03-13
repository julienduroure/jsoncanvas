use std::collections::HashMap;

use crate::node::Node;
use crate::edge::Edge;

use serde::Serialize;

use serde::ser::{SerializeStruct, Serializer};

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


impl Serialize for JsonCanvas {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("JsonCanvas", 2)?;
        let nodes_vec: Vec<&Node> = self.nodes.values().collect();
        let edges_vec: Vec<&Edge> = self.edges.values().collect();
        state.serialize_field("nodes", &nodes_vec)?;
        state.serialize_field("edges", &edges_vec)?;
        state.end()
    }
}
