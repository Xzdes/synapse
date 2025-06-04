//! Модуль `types`
//!
//! Расширенная система типов Synapse:
//! - Базовые типы (Int, Float, Bool, String, Unit)
//! - Функции, полиморфизм, трейты
//! - Линейные и borrow-типы
//! - Ошибки типов
//!
//! Все публичные структуры сериализуемы через serde.

use serde::{Deserialize, Serialize};

/// Основной enum для представления типов в Synapse.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SynType {
    /// Целое число.
    Int,
    /// Число с плавающей запятой.
    Float,
    /// Булев тип.
    Bool,
    /// Строка.
    String,
    /// Unit (аналог void).
    Unit,
    /// Функция (арность и возвращаемый тип).
    Function {
        /// Параметры функции.
        parameters: Vec<SynType>,
        /// Тип возвращаемого значения.
        return_type: Box<SynType>,
    },
    /// Переменная типа (для полиморфизма).
    TypeVariable(String),
    /// Обобщённый тип (ForAll).
    ForAll {
        /// Список параметров типа.
        type_params: Vec<String>,
        /// Тело типа.
        body: Box<SynType>,
    },
    /// Record-тип (структура).
    Record(Vec<(String, SynType)>),
    /// Algebraic Data Type (ADT).
    ADT {
        /// Имя ADT.
        name: String,
        /// Варианты.
        variants: Vec<(String, Vec<SynType>)>,
    },
    /// Линейный тип.
    Linear(Box<SynType>),
    /// Shared reference (&T).
    SharedRef(Box<SynType>),
    /// Mutable reference (&mut T).
    MutableRef(Box<SynType>),
    /// Lifetime (абстракция).
    Lifetime(String),
    /// Result<T, E>.
    Result {
        /// Тип Ok.
        ok: Box<SynType>,
        /// Тип Err.
        err: Box<SynType>,
    },
    /// ErrorUnion (T | E).
    ErrorUnion(Box<SynType>, Box<SynType>),
    /// Trait.
    Trait {
        /// Имя трейта.
        name: String,
        /// Методы трейта.
        methods: Vec<TraitMethodDecl>,
    },
    /// Foreign type (FFI).
    Foreign(String),
}

/// Описание метода трейта.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TraitMethodDecl {
    /// Имя метода.
    pub name: String,
    /// Сигнатура метода.
    pub signature: SynType,
}

/// Ошибка типов Synapse.
#[derive(Debug, thiserror::Error)]
pub enum SynTypeError {
    /// Несоответствие типов.
    #[error("Type mismatch: expected {expected:?}, found {found:?}")]
    Mismatch {
        /// Ожидаемый тип.
        expected: SynType,
        /// Найденный тип.
        found: SynType,
    },
    /// Неизвестная переменная типа.
    #[error("Unknown type variable: {0}")]
    UnknownTypeVariable(String),
    /// Ошибка ограничения (constraint).
    #[error("Constraint error: {0}")]
    Constraint(String),
    /// Ошибка borrow-типа.
    #[error("Borrow error: {0}")]
    Borrow(String),
    /// Общая ошибка.
    #[error("General type error: {0}")]
    General(String),
}
