//! Основные структуры Абстрактного Синтаксического Графа (ASG).

use crate::nodecodes::{EdgeType, NodeType};

pub type NodeID = u64;

#[derive(Debug, Clone)]
pub struct Edge {
    pub edge_type: EdgeType,
    pub target_node_id: NodeID,
    pub payload: Option<Vec<u8>>, // Добавим для полноты
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: NodeID,
    pub node_type: NodeType,
    pub payload: Option<Vec<u8>>,
    pub edges: Vec<Edge>,
}

#[derive(Debug, Clone, Default)]
pub struct ASG {
    pub nodes: Vec<Node>,
}

impl ASG {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }
    
    pub fn find_node(&self, id: NodeID) -> Option<&Node> {
        self.nodes.iter().find(|n| n.id == id)
    }
}