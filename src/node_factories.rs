// src/node_factories.rs

//! Утилиты для быстрого создания типовых узлов Synapse (Node).
//! Используйте эти функции во всех генераторах и тестах!

use crate::asg::Node;
use crate::nodecodes::NodeType;
use crate::types::SynType;
use serde_json::json;

/// Литерал-целое число
pub fn literal_int(id: u64, value: i64) -> Node {
    Node {
        id,
        code: NodeType::Const as u16,
        value: Some(json!(value)),
        ty: Some(SynType::Int),
    }
}

/// Литерал-число с плавающей точкой
pub fn literal_float(id: u64, value: f64) -> Node {
    Node {
        id,
        code: NodeType::Const as u16,
        value: Some(json!(value)),
        ty: Some(SynType::Float),
    }
}

/// Узел сложения (a + b)
pub fn binary_add(id: u64, left: u64, right: u64) -> Node {
    Node {
        id,
        code: NodeType::Add as u16,
        value: Some(json!({"left": left, "right": right})),
        ty: Some(SynType::Int), // по умолчанию Int; можно расширять
    }
}

/// Узел вычитания (a - b)
pub fn binary_sub(id: u64, left: u64, right: u64) -> Node {
    Node {
        id,
        code: NodeType::Sub as u16,
        value: Some(json!({"left": left, "right": right})),
        ty: Some(SynType::Int),
    }
}

/// Print (вывести значение)
pub fn print(id: u64, input: u64) -> Node {
    Node {
        id,
        code: NodeType::Print as u16,
        value: Some(json!({"input": input})),
        ty: None,
    }
}

/// Узел переменной (объявление/использование)
pub fn variable(id: u64, name: &str, ty: SynType) -> Node {
    Node {
        id,
        code: NodeType::Variable as u16,
        value: Some(json!({ "name": name })),
        ty: Some(ty),
    }
}

/// Узел функции (имя и список параметров)
pub fn function(id: u64, name: &str, params: Vec<(&str, SynType)>) -> Node {
    let params_json: Vec<serde_json::Value> = params.iter()
        .map(|(n, t)| json!({ "name": n, "type": t }))
        .collect();

    Node {
        id,
        code: NodeType::Function as u16,
        value: Some(json!({
            "name": name,
            "params": params_json
        })),
        ty: None,
    }
}

/// Узел FFI (external call)
pub fn ffi(id: u64, func: &str, args: Vec<u64>) -> Node {
    Node {
        id,
        code: NodeType::FFI as u16,
        value: Some(json!({ "func": func, "args": args })),
        ty: Some(SynType::Any),
    }
}
