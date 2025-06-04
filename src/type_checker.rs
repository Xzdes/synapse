//! Модуль `type_checker`
//!
//! Реализация системы типов и вывода типов для Synapse.
//! В будущем здесь будет полноценная проверка согласованности типов,
//! вывод типов (Hindley-Milner) и проверка эффектов.
//!
//! Планируется поддержка:
//! - Вывод типов (type inference).
//! - Проверка согласованности типов.
//! - Сообщения об ошибках типов.

use crate::asg::{ASG, Node};
use crate::{SynapseError, SynapseResult};
use crate::types::SynType;

/// Проверка согласованности типов для ASG (заглушка).
pub fn check_types(asg: &ASG) -> SynapseResult<()> {
    println!("Type Checker: checking types for ASG with {} nodes.", asg.nodes.len());
    // TODO: Реализовать настоящую проверку согласованности типов
    Ok(())
}

/// Вывод типов (Hindley-Milner, заглушка).
pub fn infer_types(asg: &ASG) -> SynapseResult<Vec<(u64, SynType)>> {
    println!("Type Checker: inferring types for ASG with {} nodes.", asg.nodes.len());
    // TODO: Реализовать настоящую систему вывода типов
    Ok(Vec::new())
}
