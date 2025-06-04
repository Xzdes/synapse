//! Crate root for Synapse.
//!
//! This library defines:
//! - The ASG (Abstract Syntax Graph).
//! - Node and edge types.
//! - Serialization (SYN1 format).
//! - Interpreter.
//! - Proof system.
//! - Modules and effects.
//! - FFI.
//! - Compiler backends (LLVM, Wasm, C, JS).
//! - Type system and checker.
//! - Async concurrency.
//! - Graphviz exporter.

#![warn(missing_docs)]

pub mod ai_api;
pub mod asg;
pub mod compiler;
pub mod concurrency;
pub mod concurrency_async;
pub mod effects;
pub mod ffi;
pub mod interpreter;
pub mod macros;
pub mod modules;
pub mod node_factories;
pub mod nodecodes;
pub mod proof;
pub mod proof_smt;
pub mod syn1;
pub mod syn1_writer;
pub mod testing;
pub mod types;

// Backends
pub mod llvm_backend;
pub mod wasm_backend;
pub mod c_backend;
pub mod js_backend;

// Tools
pub mod tools;

// Type Checker
pub mod type_checker;

use thiserror::Error;

/// Synapse error type.
#[derive(Debug, Error)]
pub enum SynapseError {
    /// Serialization error.
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Type error.
    #[error("Type error: {0}")]
    Type(String),

    /// Proof error.
    #[error("Proof error: {0}")]
    Proof(String),

    /// Effect error.
    #[error("Effect error: {0}")]
    Effect(String),

    /// Concurrency error.
    #[error("Concurrency error: {0}")]
    Concurrency(String),

    /// General error.
    #[error("General error: {0}")]
    General(String),
}

/// Synapse result type.
pub type SynapseResult<T> = Result<T, SynapseError>;

/// Initialize the logger.
pub fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}
