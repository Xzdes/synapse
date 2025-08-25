//! Простой, рекурсивный интерпретатор ASG.

use std::collections::HashMap;
use crate::asg::{ASG, Node, NodeID};
use crate::nodecodes::{EdgeType, NodeType};
use crate::error::{SynapseResult, SynapseError};
use crate::runtime::diff_tensor::DifferentiableTensor;
use crate::ops::tensor_ops;

/// Представление рантайм-значений в Synapse.
#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Tensor(DifferentiableTensor),
}

/// Контекст выполнения, хранит вычисленные значения для каждого узла.
#[derive(Default)]
pub struct Interpreter {
    memo: HashMap<NodeID, Value>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self::default()
    }

    /// Выполняет ASG, вычисляя все узлы, и возвращает значение корневого.
    pub fn execute(&mut self, asg: &ASG, root_id: NodeID) -> SynapseResult<Value> {
        for node in &asg.nodes {
            self.eval_node(asg, node)?;
        }
        self.memo.get(&root_id).cloned().ok_or(SynapseError::NodeNotFound(root_id))
    }
    
    /// Вычисляет значение для одного узла и сохраняет его в кэш.
    // --- ИСПРАВЛЕНИЕ: Добавляем `_` к неиспользуемому `asg` ---
    fn eval_node(&mut self, _asg: &ASG, node: &Node) -> SynapseResult<()> {
        if self.memo.contains_key(&node.id) {
            return Ok(());
        }

        let result_value = match node.node_type {
            NodeType::LiteralInt => {
                let payload = node.payload.as_ref().ok_or(SynapseError::MissingPayload(node.id))?;
                let bytes: [u8; 8] = payload.clone().try_into()
                    .map_err(|_| SynapseError::InvalidPayload(node.id))?;
                Value::Int(i64::from_le_bytes(bytes))
            }
            NodeType::LiteralTensor => {
                let payload = node.payload.as_ref().ok_or(SynapseError::MissingPayload(node.id))?;
                let bytes: [u8; 4] = payload.clone().try_into()
                    .map_err(|_| SynapseError::InvalidPayload(node.id))?;
                let val = f32::from_le_bytes(bytes);
                let tensor = DifferentiableTensor::new(ndarray::arr0(val).into_dyn(), true);
                Value::Tensor(tensor)
            }
            NodeType::BinaryOperation => {
                let arg_edges: Vec<&crate::asg::Edge> = node.edges.iter()
                    .filter(|e| e.edge_type == EdgeType::ApplicationArgument)
                    .collect();
                if arg_edges.len() < 2 { return Err(SynapseError::MissingEdge(node.id, EdgeType::ApplicationArgument)); }

                let val1 = self.get_value_for_node(arg_edges[0].target_node_id)?;
                let val2 = self.get_value_for_node(arg_edges[1].target_node_id)?;

                match (val1, val2) {
                    (Value::Int(a), Value::Int(b)) => Value::Int(a + b),
                    _ => return Err(SynapseError::TypeError("Expected two Integers for BinaryOperation".to_string())),
                }
            }
            NodeType::TensorAdd => {
                let arg_edges: Vec<&crate::asg::Edge> = node.edges.iter()
                    .filter(|e| e.edge_type == EdgeType::ApplicationArgument)
                    .collect();
                if arg_edges.len() < 2 { return Err(SynapseError::MissingEdge(node.id, EdgeType::ApplicationArgument)); }

                let val1 = self.get_value_for_node(arg_edges[0].target_node_id)?;
                let val2 = self.get_value_for_node(arg_edges[1].target_node_id)?;

                match (val1, val2) {
                    (Value::Tensor(a), Value::Tensor(b)) => Value::Tensor(tensor_ops::add(a, b)),
                    _ => return Err(SynapseError::TypeError("Expected two Tensors for TensorAdd".to_string())),
                }
            }
        };

        self.memo.insert(node.id, result_value);
        Ok(())
    }

    fn get_value_for_node(&self, node_id: NodeID) -> SynapseResult<&Value> {
        self.memo.get(&node_id).ok_or(SynapseError::NodeNotFound(node_id))
    }
}