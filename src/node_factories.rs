//! Модуль `node_factories`
//!
//! Предоставляет удобные функции-конструкторы для создания узлов ASG:
//! - Литералы
//! - Арифметические операции
//! - Эффекты, Proof, Macros, FFI
//! - Полная совместимость с NodeType
//!
//! Все функции возвращают сериализуемые структуры Node.

use crate::asg::{Node, NodeID};
use crate::nodecodes::NodeType;

/// Создать узел LiteralInt.
pub fn literal_int(id: NodeID, value: i64) -> Node {
    let payload = value.to_le_bytes().to_vec();
    Node::new(id, NodeType::LiteralInt, Some(payload))
}

/// Создать узел LiteralFloat.
pub fn literal_float(id: NodeID, value: f64) -> Node {
    let payload = value.to_le_bytes().to_vec();
    Node::new(id, NodeType::LiteralFloat, Some(payload))
}

/// Создать узел LiteralBool.
pub fn literal_bool(id: NodeID, value: bool) -> Node {
    let payload = vec![if value { 1 } else { 0 }];
    Node::new(id, NodeType::LiteralBool, Some(payload))
}

/// Создать узел BinaryOperation (пример: Add).
pub fn binary_operation(id: NodeID, operator_code: u8) -> Node {
    let payload = vec![operator_code];
    Node::new(id, NodeType::BinaryOperation, Some(payload))
}

/// Создать узел EffectIO.
pub fn effect_io(id: NodeID) -> Node {
    Node::new(id, NodeType::EffectIO, None)
}

/// Создать узел Proof.
pub fn proof(id: NodeID) -> Node {
    Node::new(id, NodeType::Proof, None)
}

/// Создать узел MacroInvocation.
pub fn macro_invocation(id: NodeID) -> Node {
    Node::new(id, NodeType::MacroInvocation, None)
}

/// Создать узел ModuleRoot.
pub fn module_root(id: NodeID) -> Node {
    Node::new(id, NodeType::ModuleRoot, None)
}

/// Создать узел ForeignFunctionDecl.
pub fn foreign_function_decl(id: NodeID) -> Node {
    Node::new(id, NodeType::ForeignFunctionDecl, None)
}

/// Создать узел Concurrency.
pub fn concurrency(id: NodeID) -> Node {
    Node::new(id, NodeType::Concurrency, None)
}
