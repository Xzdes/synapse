// src/tests.rs

//! Тесты для Synapse.
//! Пример: тестируем, что FFI node правильно выдаёт ошибку "NotImplemented".

mod asg;
mod nodecodes;
mod types;
mod interpreter;

use asg::Node;
use nodecodes::NodeType;
use types::{SynType, SynError};
use interpreter::interpret_node;

/// Тест: обработка FFI-узла должна возвращать ошибку NotImplemented
#[test]
fn test_ffi_node_stub() {
    let ffi_node = Node {
        id: 100,
        code: NodeType::FFI as u16,
        value: Some(serde_json::json!({"func": "system_time"})),
        ty: Some(SynType::Any),
    };
    let result = interpret_node(&ffi_node);

    // Проверяем, что результат — ошибка NotImplemented
    match result {
        Err(SynError::NotImplemented(msg)) => {
            assert!(msg.contains("FFI"), "Ошибка должна быть про FFI");
            println!("FFI node test passed! {:?}", msg);
        }
        _ => panic!("FFI node должен возвращать ошибку NotImplemented"),
    }
}
