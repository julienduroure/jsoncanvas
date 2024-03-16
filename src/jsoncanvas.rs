use std::collections::HashMap;

use crate::node::GenericNodeInfo;
use crate::node::Node;
use crate::edge::Edge;

use serde::{Serialize, Serializer, Deserialize, Deserializer};

#[derive(Debug)]
pub enum JsonCanvasError {
    AlreadyExists,
    NodeNotExists,
}


/// JsonCanvas
///
/// Main struct for the canvas
///
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonCanvas {
    #[serde(serialize_with = "serialize_as_vec_node", deserialize_with = "deserialize_as_map_node")]
    nodes: HashMap<String, Node>,
    #[serde(serialize_with = "serialize_as_vec_edge", deserialize_with = "deserialize_as_map_edge")]
    edges: HashMap<String, Edge>
}
fn serialize_as_vec_node<S>(data: &HashMap<String, Node>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let vec: Vec<&Node> = data.values().collect();
    vec.serialize(serializer)
}

fn deserialize_as_map_node<'de, D>(deserializer: D) -> Result<HashMap<String, Node>, D::Error>
where
    D: Deserializer<'de>,
{
    let vec: Vec<Node> = Vec::deserialize(deserializer)?;
    let map: HashMap<String, Node> = vec.into_iter().map(|node| (node.id().clone(), node)).collect();
    Ok(map)
}

fn serialize_as_vec_edge<S>(data: &HashMap<String, Edge>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let vec: Vec<&Edge> = data.values().collect();
    vec.serialize(serializer)
}

fn deserialize_as_map_edge<'de, D>(deserializer: D) -> Result<HashMap<String, Edge>, D::Error>
where
    D: Deserializer<'de>,
{
    let vec: Vec<Edge> = Vec::deserialize(deserializer)?;
    let map: HashMap<String, Edge> = vec.into_iter().map(|node| (node.id.clone(), node)).collect();
    Ok(map)
}


impl JsonCanvas {
    pub fn new() -> JsonCanvas {
        JsonCanvas {
            nodes: HashMap::new(),
            edges: HashMap::new()
        }
    }

    pub fn add_node(&mut self, node: Node) -> Result<(), JsonCanvasError> {
        if self.nodes.contains_key(node.id()) {
            return Err(JsonCanvasError::AlreadyExists);
        }
        self.nodes.insert(node.id().clone(), node);
        Ok(())
    }

    pub fn add_edge(&mut self, edge: Edge) -> Result<(), JsonCanvasError>{
        if self.edges.contains_key(&edge.id) {
            return Err(JsonCanvasError::AlreadyExists);
        }

        if !self.nodes.contains_key(&edge.from_node) {
            return Err(JsonCanvasError::NodeNotExists);
        }

        if !self.nodes.contains_key(&edge.to_node) {
            return Err(JsonCanvasError::NodeNotExists);
        }

        self.edges.insert(edge.id.clone(), edge);
        Ok(())
    }

    pub fn get_node(&mut self, id: String) -> Option<&mut Node> {
        self.nodes.get_mut(&id)
    }

    pub fn get_edge(&mut self, id: String) -> Option<&mut Edge> {
        self.edges.get_mut(&id)
    }

    /// Serialize the JsonCanvas to a string
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    /// Deserialize the JsonCanvas from a string
    pub fn from_string(s: String) -> JsonCanvas {
        serde_json::from_str(&s).unwrap()
    }
}
