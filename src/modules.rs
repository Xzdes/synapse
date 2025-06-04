//! Модуль `modules`
//!
//! Поддержка модульной системы Synapse:
//! - ModuleRoot
//! - ImportDeclaration, ExportDeclaration
//! - Импорт/экспорт символов
//! - Базовая модель разрешения имён (Name Resolution)

use serde::{Deserialize, Serialize};

use crate::asg::NodeID;
use crate::{SynapseError, SynapseResult};

/// Представление модуля.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    /// Идентификатор корневого узла модуля.
    pub root_node: NodeID,
    /// Имя модуля (для удобства).
    pub name: String,
    /// Символы, экспортируемые из модуля.
    pub exports: Vec<String>,
    /// Символы, импортируемые в модуль.
    pub imports: Vec<String>,
}

impl Module {
    /// Создать новый модуль.
    pub fn new(root_node: NodeID, name: &str) -> Self {
        Self {
            root_node,
            name: name.to_string(),
            exports: Vec::new(),
            imports: Vec::new(),
        }
    }

    /// Добавить экспорт.
    pub fn add_export(&mut self, symbol: &str) {
        self.exports.push(symbol.to_string());
    }

    /// Добавить импорт.
    pub fn add_import(&mut self, symbol: &str) {
        self.imports.push(symbol.to_string());
    }

    /// Проверить, экспортируется ли символ.
    pub fn is_exported(&self, symbol: &str) -> bool {
        self.exports.contains(&symbol.to_string())
    }

    /// Проверить, импортируется ли символ.
    pub fn is_imported(&self, symbol: &str) -> bool {
        self.imports.contains(&symbol.to_string())
    }

    /// Разрешить имя в модуле.
    pub fn resolve_name(&self, name: &str) -> SynapseResult<NodeID> {
        if self.is_exported(name) || self.is_imported(name) {
            // Здесь можно добавить реальное сопоставление с NodeID.
            Ok(self.root_node)
        } else {
            Err(SynapseError::Module(format!(
                "Symbol `{}` not found in module `{}`",
                name, self.name
            )))
        }
    }
}
