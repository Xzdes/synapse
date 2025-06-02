// src/types.rs

use serde::{Serialize, Deserialize};

/// Примитивные типы Synapse.
/// Это основа для будущей типовой системы.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SynType {
    Int,       // Целое число
    Float,     // Число с плавающей запятой
    Bool,      // Логический тип
    String,    // Строка
    Any,       // Динамический/любой тип (универсальный)
}

impl std::fmt::Display for SynType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SynType::Int => write!(f, "Int"),
            SynType::Float => write!(f, "Float"),
            SynType::Bool => write!(f, "Bool"),
            SynType::String => write!(f, "String"),
            SynType::Any => write!(f, "Any"),
        }
    }
}

/// Базовые ошибки Synapse.
/// Позволяют возвращать ошибки при выполнении кода или работе с графом.
#[derive(Debug)]
pub enum SynError {
    /// Ошибка типов (например, попытка сложить строку и число)
    TypeError(String),

    /// Ошибка времени выполнения (любая логика)
    RuntimeError(String),

    /// Фича не реализована (stub)
    NotImplemented(String),

    // В будущем можно добавить ParseError, IO, Overflow и др.
}
