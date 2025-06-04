//! Модуль `asg`
//!
//! Содержит основные структуры:
//! - `Node`: Узел графа (NodeType, payload, edges)
//! - `Edge`: Рёбра графа (EdgeType, target_node_id, payload)
//! - `ASG`: Граф в целом
//!
//! Все публичные структуры сериализуемы через serde.

use serde::{Deserialize, Serialize};

use crate::nodecodes::{EdgeType, NodeType};

/// Уникальный идентификатор узла в ASG.
pub type NodeID = u64;

/// Структура, представляющая один узел ASG.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    /// Идентификатор узла.
    pub id: NodeID,
    /// Тип узла.
    pub node_type: NodeType,
    /// Полезная нагрузка (разные типы данных в зависимости от NodeType).
    pub payload: Option<Vec<u8>>,
    /// Исходящие рёбра.
    pub edges: Vec<Edge>,
}

impl Node {
    /// Создать новый узел.
    ///
    /// # Пример
    /// ```
    /// use synapse::asg::{Node, NodeID};
    /// use synapse::nodecodes::NodeType;
    ///
    /// let node = Node::new(1, NodeType::LiteralInt, Some(vec![42]));
    /// assert_eq!(node.id, 1);
    /// ```
    pub fn new(id: NodeID, node_type: NodeType, payload: Option<Vec<u8>>) -> Self {
        Self {
            id,
            node_type,
            payload,
            edges: Vec::new(),
        }
    }

    /// Добавить ребро.
    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.push(edge);
    }
}

/// Структура, представляющая одно ребро ASG.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    /// Тип ребра.
    pub edge_type: EdgeType,
    /// Идентификатор целевого узла.
    pub target_node_id: NodeID,
    /// Полезная нагрузка (разные типы данных в зависимости от EdgeType).
    pub payload: Option<Vec<u8>>,
}

impl Edge {
    /// Создать новое ребро.
    ///
    /// # Пример
    /// ```
    /// use synapse::asg::{Edge, NodeID};
    /// use synapse::nodecodes::EdgeType;
    ///
    /// let edge = Edge::new(EdgeType::DataInput, 2, None);
    /// assert_eq!(edge.target_node_id, 2);
    /// ```
    pub fn new(edge_type: EdgeType, target_node_id: NodeID, payload: Option<Vec<u8>>) -> Self {
        Self {
            edge_type,
            target_node_id,
            payload,
        }
    }
}

/// Структура, представляющая весь граф.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ASG {
    /// Список всех узлов.
    pub nodes: Vec<Node>,
}

impl ASG {
    /// Создать новый пустой граф.
    ///
    /// # Пример
    /// ```
    /// use synapse::asg::ASG;
    ///
    /// let asg = ASG::new();
    /// assert!(asg.nodes.is_empty());
    /// ```
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    /// Добавить узел в граф.
    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    /// Найти узел по идентификатору.
    pub fn find_node(&self, id: NodeID) -> Option<&Node> {
        self.nodes.iter().find(|n| n.id == id)
    }

    /// Найти узел по идентификатору (mutable).
    pub fn find_node_mut(&mut self, id: NodeID) -> Option<&mut Node> {
        self.nodes.iter_mut().find(|n| n.id == id)
    }
}
