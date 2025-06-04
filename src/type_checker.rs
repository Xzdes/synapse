//! Модуль `type_checker`
//!
//! Базовый механизм проверки и вывода типов для Synapse.
//!
//! Этот модуль реализует:
//! - Проверку корректности типов в ASG.
//! - Вывод типов (Hindley-Milner, заглушка).
//!
//! TODO:
//! - Поддержка эффектов.
//! - Полный вывод типов.
//! - Лайфтаймы, заимствования.

use crate::asg::ASG;

/// Проверка корректности типов в ASG.
///
/// Пока реализовано как заглушка — всегда возвращает Ok.
pub fn check_types(asg: &ASG) -> crate::SynapseResult<()> {
    println!("TypeChecker: checking types for ASG with {} nodes.", asg.nodes.len());
    // TODO: Реализовать полноценную проверку типов.
    Ok(())
}

/// Вывод типов в ASG.
///
/// Пока реализовано как заглушка — всегда возвращает Ok.
pub fn infer_types(asg: &ASG) -> crate::SynapseResult<()> {
    println!("TypeChecker: inferring types for ASG with {} nodes.", asg.nodes.len());
    // TODO: Реализовать полноценный вывод типов.
    Ok(())
}
