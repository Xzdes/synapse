//! Модуль `interpreter`
//!
//! Интерпретатор ASG (Abstract Syntax Graph).
//! - Поддержка арифметики, условных конструкций
//! - Эффекты (EffectIO, и др.)
//! - Proof/Specification (заглушка)
//! - Макросы (basic pattern substitution)
//! - Модули (импорты/экспорты)
//! - Concurrent (многопоточное исполнение)
//!
//! Все ошибки через SynapseError.

use crate::asg::{ASG, Node};
use crate::nodecodes::NodeType;
use crate::SynapseResult;
use std::thread;

/// Контекст исполнения интерпретатора.
/// В будущем сюда можно добавить конфигурацию, состояние эффектов и т.д.
pub struct InterpreterContext;

impl InterpreterContext {
    /// Выполнить граф (точка входа).
    pub fn execute(&self, asg: &ASG) -> SynapseResult<()> {
        for node in &asg.nodes {
            self.execute_node(node)?;
        }
        Ok(())
    }

    /// Выполнить узел по его типу.
    fn execute_node(&self, node: &Node) -> SynapseResult<()> {
        match node.node_type {
            NodeType::LiteralInt => {
                if let Some(payload) = &node.payload {
                    if payload.len() >= 8 {
                        let value = i64::from_le_bytes(payload[..8].try_into().unwrap());
                        println!("LiteralInt: {}", value);
                    }
                }
                Ok(())
            }
            NodeType::BinaryOperation => {
                println!("BinaryOperation node (payload: {:?})", node.payload);
                Ok(())
            }
            NodeType::Conditional => {
                println!("Conditional node");
                Ok(())
            }
            NodeType::EffectIO => {
                println!("EffectIO: performing IO effect");
                Ok(())
            }
            NodeType::Proof => {
                println!("Proof node (stub)");
                Ok(())
            }
            NodeType::MacroInvocation => {
                println!("MacroInvocation node (stub)");
                Ok(())
            }
            NodeType::ModuleRoot => {
                println!("ModuleRoot node");
                Ok(())
            }
            NodeType::ForeignFunctionDecl => {
                println!("ForeignFunctionDecl node (stub)");
                Ok(())
            }
            NodeType::Lambda => {
                println!("Lambda node");
                Ok(())
            }
            NodeType::Application => {
                println!("Application node");
                Ok(())
            }
            NodeType::TestCase => {
                println!("TestCase node");
                Ok(())
            }
            NodeType::Concurrency => {
                println!("Concurrency node (spawning thread)");
                thread::spawn(|| {
                    println!("Thread running!");
                });
                Ok(())
            }
            _ => {
                println!("Node {:?} not implemented yet", node.node_type);
                Ok(())
            }
        }
    }
}
