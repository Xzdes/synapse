// src/asg.rs

use serde::{Serialize, Deserialize};
use crate::types::SynType;

/// Узел (Node) в ASG-графе Synapse.
/// Каждый узел может содержать значение, код (тип действия) и (теперь) опциональный тип.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: u64,                          // Уникальный идентификатор узла
    pub code: u16,                        // Код типа узла (NodeType as u16)
    pub value: Option<serde_json::Value>, // Значение (например, литерал, имя переменной и т.д.)
    pub ty: Option<SynType>,              // Новый параметр: тип узла (опционально)
    // Можно добавить еще поля по необходимости (например, source location и др.)
}

/// Ребро (Edge) в ASG-графе.
/// Связывает два узла, может иметь тип (EdgeType).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub from: u64, // id исходного узла
    pub to: u64,   // id целевого узла
    pub code: u16, // тип ребра (EdgeType as u16)
    // Можно добавить поля: label, weight, и др.
}

/// Вся структура ASG-графа.
/// Включает список узлов и ребер.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ASG {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

