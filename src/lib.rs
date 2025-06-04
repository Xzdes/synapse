#![warn(missing_docs)]

//! # Synapse
//!
//! Synapse — AI-friendly programming language with ASG representation.
//! This is the main library crate for Synapse.

pub mod asg;
pub mod concurrency;
pub mod concurrency_async;
pub mod effects;
pub mod ffi;
pub mod interpreter;
pub mod macros;
pub mod modules;
pub mod node_factories;
pub mod nodecodes;

/// Proof module: Provides SMT-based proof system.
pub mod proof;

/// Proof DSL module: High-level interface for SMT interactions.
pub mod proof_dsl;

pub mod syn1;
pub mod syn1_writer;
pub mod testing;
pub mod types;

/// Result type for all Synapse operations.
pub type SynapseResult<T> = Result<T, SynapseError>;

/// Synapse's core error type.
#[derive(Debug, thiserror::Error)]
pub enum SynapseError {
    /// Generic error type for concurrency operations.
    #[error("Concurrency error: {0}")]
    Concurrency(String),

    /// Generic error type for file effects (IO).
    #[error("Effect error: {0}")]
    Effect(String),

    /// Serialization/deserialization errors.
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Generic error type for proof or SMT interactions.
    #[error("Proof error: {0}")]
    Proof(String),

    /// Unknown or unhandled error.
    #[error("Unknown error: {0}")]
    Other(String),
}
