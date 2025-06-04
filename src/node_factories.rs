//! Модуль `node_factories`
//!
//! Фабрики для создания узлов ASG.

use crate::asg::{Node, NodeID};
use crate::nodecodes::NodeType;

/// Создать узел литерала целого.
pub fn literal_int(id: NodeID, value: i64) -> Node {
    Node::new(id, NodeType::LiteralInt, Some(value.to_le_bytes().to_vec()))
}

/// Создать узел литерала строки.
pub fn literal_string(id: NodeID, value: &str) -> Node {
    Node::new(id, NodeType::LiteralString, Some(value.as_bytes().to_vec()))
}

/// Создать бинарную операцию.
pub fn binary_operation(id: NodeID, operator_code: u8) -> Node {
    Node::new(id, NodeType::BinaryOperation, Some(vec![operator_code]))
}

/// Создать узел эффекта.
pub fn perform_effect(id: NodeID, effect_type: NodeType) -> Node {
    Node::new(id, effect_type, None)
}
