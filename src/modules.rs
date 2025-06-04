//! Модуль `modules`
//!
//! Заглушка для поддержки модулей в Synapse.
//!
//! TODO:
//! - Реализовать реальное разрешение импортов и экспортов.

use crate::asg::ASG;

/// Проверить корректность модуля.
///
/// На данный момент реализовано как заглушка.
pub fn check_module(asg: &ASG) {
    println!("Modules: checking module with {} nodes.", asg.nodes.len());
    // TODO: Реализовать полноценную проверку модуля.
}
