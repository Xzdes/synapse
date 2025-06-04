//! Модуль `macros`
//!
//! Поддержка макросов в Synapse:
//! - MacroDefinition
//! - MacroInvocation
//!
//! Простейший движок подстановки шаблонов (pattern/substitution).

use serde::{Deserialize, Serialize};

use crate::SynapseResult;

/// Определение макроса.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroDefinition {
    /// Имя макроса.
    pub name: String,
    /// Тело макроса (может быть строкой или сериализованной структурой).
    pub body: String,
}

/// Вызов макроса.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroInvocation {
    /// Имя макроса.
    pub name: String,
    /// Аргументы макроса (простая строка на этом этапе).
    pub arguments: Vec<String>,
}

/// Выполнить макрос (stub).
pub fn execute_macro(def: &MacroDefinition, invocation: &MacroInvocation) -> SynapseResult<String> {
    println!("Expanding macro `{}` with args: {:?}", def.name, invocation.arguments);

    // Заглушка для pattern/substitution.
    let mut expanded = def.body.clone();
    for (i, arg) in invocation.arguments.iter().enumerate() {
        let placeholder = format!("${}", i + 1);
        expanded = expanded.replace(&placeholder, arg);
    }

    Ok(expanded)
}
