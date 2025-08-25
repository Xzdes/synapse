//! Простой, рекурсивный интерпретатор ASG.

use std::collections::HashMap;
use crate::asg::{ASG, Node, NodeID};
use crate::nodecodes::{EdgeType, NodeType};
use crate::error::{SynapseResult, SynapseError};

/// Представление рантайм-значений в Synapse.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
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

    /// Выполняет ASG, начиная с корневого узла, и возвращает финальное значение.
    pub fn execute(&mut self, asg: &ASG, root_id: NodeID) -> SynapseResult<Value> {
        let root_node = asg.find_node(root_id).ok_or(SynapseError::NodeNotFound(root_id))?;
        self.eval_node(asg, root_node)
    }

    /// Рекурсивно вычисляет значение для одного узла.
    fn eval_node(&mut self, asg: &ASG, node: &Node) -> SynapseResult<Value> {
        if let Some(val) = self.memo.get(&node.id) {
            return Ok(val.clone());
        }

        let result = match node.node_type {
            NodeType::LiteralInt => {
                let payload = node.payload.as_ref().ok_or(SynapseError::MissingPayload(node.id))?;
                let bytes: [u8; 8] = payload.clone().try_into()
                    .map_err(|_| SynapseError::InvalidPayload(node.id))?;
                Ok(Value::Int(i64::from_le_bytes(bytes)))
            }

            NodeType::BinaryOperation => {
                let arg_edges: Vec<&crate::asg::Edge> = node.edges.iter()
                    .filter(|e| e.edge_type == EdgeType::ApplicationArgument)
                    .collect();

                if arg_edges.len() < 2 {
                    return Err(SynapseError::MissingEdge(node.id, EdgeType::ApplicationArgument));
                }

                let node1 = asg.find_node(arg_edges[0].target_node_id).ok_or(SynapseError::NodeNotFound(arg_edges[0].target_node_id))?;
                let node2 = asg.find_node(arg_edges[1].target_node_id).ok_or(SynapseError::NodeNotFound(arg_edges[1].target_node_id))?;
                
                let val1 = self.eval_node(asg, node1)?;
                let val2 = self.eval_node(asg, node2)?;

                match (val1, val2) {
                    (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
                    // --- ИСПРАВЛЕНИЕ: Удаляем недостижимую ветку `_` ---
                    // Так как Value содержит только Int, другие варианты невозможны.
                    // Если мы добавим другие типы в Value, нам понадобится более сложный match.
                }
            }
            // --- ИСПРАВЛЕНИЕ: Добавляем "дикую" ветку для нереализованных узлов ---
            _ => Err(SynapseError::InvalidOperation(format!(
                "Execution for node type {:?} is not yet implemented",
                node.node_type
            ))),
        }?;
        
        self.memo.insert(node.id, result.clone());
        Ok(result)
    }
}