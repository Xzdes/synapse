//! # Synapse
//!
//! Основной модуль проекта Synapse. Подключает все остальные модули,
//! предоставляя доступ ко всем ключевым структурам, типам и функциям.
//!
//! Этот проект реализует спецификацию Synapse 0.2, включая:
//! - ASG (Abstract Syntax Graph) как основную модель программ
//! - Полный набор NodeType и EdgeType
//! - Расширенную систему типов
//! - Эффекты, FFI, Proof, Macros
//! - Интерпретатор и средства сериализации SYN1
//! - Модули, тестирование, компилятор
//! - Поддержку AI API
//!
//! Все публичные структуры сериализуемы через serde.
//! Ошибки возвращаются через стандартный Result<T, E>.

#![warn(missing_docs)]

pub mod asg;
pub mod nodecodes;
pub mod types;
pub mod syn1;
pub mod syn1_writer;
pub mod interpreter;
pub mod node_factories;
pub mod modules;
pub mod effects;
pub mod proof;
pub mod ffi;
pub mod macros;
pub mod concurrency;
pub mod testing;
pub mod compiler;
pub mod ai_api;

/// Тип ошибки верхнего уровня для всей библиотеки.
#[derive(Debug, thiserror::Error)]
pub enum SynapseError {
    /// Ошибка сериализации/десериализации.
    #[error("Serialization error: {0}")]
    Serialization(String),
    /// Ошибка интерпретатора.
    #[error("Interpreter error: {0}")]
    Interpreter(String),
    /// Ошибка типов.
    #[error("Type error: {0}")]
    Type(String),
    /// Ошибка модуля/импорта.
    #[error("Module error: {0}")]
    Module(String),
    /// Ошибка эффектов.
    #[error("Effect error: {0}")]
    Effect(String),
    /// Ошибка проверки (proof).
    #[error("Proof error: {0}")]
    Proof(String),
    /// Ошибка FFI.
    #[error("FFI error: {0}")]
    Ffi(String),
    /// Ошибка макросов.
    #[error("Macro error: {0}")]
    Macro(String),
    /// Ошибка многопоточности.
    #[error("Concurrency error: {0}")]
    Concurrency(String),
    /// Ошибка тестирования.
    #[error("Testing error: {0}")]
    Testing(String),
    /// Ошибка компиляции.
    #[error("Compiler error: {0}")]
    Compiler(String),
    /// Общая ошибка.
    #[error("General error: {0}")]
    General(String),
}

pub type SynapseResult<T> = Result<T, SynapseError>;
