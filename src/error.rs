
use thiserror::Error;
use crate::nodecodes::EdgeType;

/// Основной тип `Result` для библиотеки.
pub type SynapseResult<T> = Result<T, SynapseError>;

/// Перечисление всех возможных ошибок.
#[derive(Error, Debug)]
pub enum SynapseError {
    #[error("Node with ID {0} not found in ASG")]
    NodeNotFound(u64),

    #[error("Node {0} is missing required payload")]
    MissingPayload(u64),

    #[error("Node {0} has invalid payload (e.g., wrong size)")]
    InvalidPayload(u64),
    
    #[error("Node {0} is missing required edge of type {1:?}")]
    MissingEdge(u64, EdgeType),

    #[error("Type mismatch during execution: {0}")]
    TypeError(String),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
}