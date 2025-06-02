// src/main.rs

//! CLI-интерпретатор Synapse. Загружает граф (ASG) и запускает его выполнение.
//! Для примера здесь строится вручную простейший граф с новым узлом типа FFI.

mod asg;
mod nodecodes;
mod types;
mod interpreter;

use asg::{Node, ASG};
use nodecodes::NodeType;
use types::{SynType};
use interpreter::interpret_node;

fn main() {
    // Пример: создаём небольшой граф с разными типами узлов

    // 1. Узел FFI (демонстрация stub-обработки)
    let ffi_node = Node {
        id: 1,
        code: NodeType::FFI as u16,
        value: Some(serde_json::json!({"func": "system_time"})),
        ty: Some(SynType::Any),
    };

    // 2. Обычный узел (например, Print)
    let print_node = Node {
        id: 2,
        code: NodeType::Print as u16,
        value: Some(serde_json::json!("Hello, Synapse!")),
        ty: Some(SynType::String),
    };

    // 3. Новые типы: Variable, Function
    let var_node = Node {
        id: 3,
        code: NodeType::Variable as u16,
        value: Some(serde_json::json!({"name": "x"})),
        ty: Some(SynType::Int),
    };

    let func_node = Node {
        id: 4,
        code: NodeType::Function as u16,
        value: Some(serde_json::json!({"name": "main"})),
        ty: None,
    };

    // Собираем граф
    let graph = ASG {
        nodes: vec![ffi_node, print_node, var_node, func_node],
        edges: vec![], // В этом демо ребер нет
    };

    // Интерпретируем каждый узел
    for node in &graph.nodes {
        match interpret_node(node) {
            Ok(_) => {}
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }
}
