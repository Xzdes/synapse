// src/interpreter.rs

use crate::asg::Node;
use crate::nodecodes::NodeType;
use crate::types::{SynError};

/// Интерпретирует отдельный узел ASG-графа.
/// Возвращает Result<(), SynError> — можно обрабатывать ошибки.
pub fn interpret_node(node: &Node) -> Result<(), SynError> {
    // Если у узла есть тип — выводим его
    if let Some(ref ty) = node.ty {
        println!("Node {} has type: {}", node.id, ty);
    }

    // Основная обработка по коду узла
    match node.code {
        // --- Новые узлы с заглушками ---
        code if code == NodeType::Variable as u16 => {
            println!("Variable node: {:?}", node.value);
            Ok(())
        }
        code if code == NodeType::Function as u16 => {
            println!("Function node: {:?}", node.value);
            Ok(())
        }
        code if code == NodeType::Effect as u16 => {
            println!("Effect node (side effect): {:?}", node.value);
            Ok(())
        }
        code if code == NodeType::PatternMatch as u16 => {
            println!("PatternMatch node: STUB, not implemented yet!");
            Err(SynError::NotImplemented("Pattern matching is not implemented yet.".into()))
        }
        code if code == NodeType::FFI as u16 => {
            println!("FFI node: STUB, not implemented yet!");
            Err(SynError::NotImplemented("FFI is not implemented yet.".into()))
        }
        // --- Примеры существующей логики ---
        code if code == NodeType::Add as u16 => {
            println!("Add node: {:?}", node.value);
            Ok(())
        }
        code if code == NodeType::Print as u16 => {
            println!("Print node: {:?}", node.value);
            Ok(())
        }
        // ... обработка других типов узлов ...

        // Если код неизвестен — ничего не делаем
        _ => Ok(()),
    }
}
