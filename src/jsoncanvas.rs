use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::node::GenericNodeInfo;
use crate::node::Node;
use crate::edge::Edge;
use crate::NodeId;
use crate::EdgeId;

use serde::{Serialize, Serializer, Deserialize, Deserializer};

#[derive(Debug)]
pub enum JsonCanvasError {
    AlreadyExists,
    NodeNotExists,
    ParseError
}


/// JsonCanvas
///
/// Main struct for the canvas
///
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonCanvas {
    #[serde(serialize_with = "serialize_as_vec_node", deserialize_with = "deserialize_as_map_node")]
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    nodes: HashMap<NodeId, Node>,
    #[serde(serialize_with = "serialize_as_vec_edge", deserialize_with = "deserialize_as_map_edge")]
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    edges: HashMap<EdgeId, Edge>
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

    pub fn get_node(&mut self, id: NodeId) -> Option<&mut Node> {
        self.nodes.get_mut(&id)
    }

    pub fn get_edge(&mut self, id: EdgeId) -> Option<&mut Edge> {
        self.edges.get_mut(&id)
    }

    pub fn get_nodes(&self) -> &HashMap<NodeId, Node> {
        &self.nodes
    }

    pub fn get_mut_nodes(&mut self) -> &mut HashMap<NodeId, Node> {
        &mut self.nodes
    }

    pub fn get_edges(&self) -> &HashMap<EdgeId, Edge> {
        &self.edges
    }

    pub fn get_mut_edges(&mut self) -> &mut HashMap<EdgeId, Edge> {
        &mut self.edges
    }

}


impl FromStr for JsonCanvas {
    type Err = JsonCanvasError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_str(&s).unwrap())
    }
}

impl Display for JsonCanvas {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
