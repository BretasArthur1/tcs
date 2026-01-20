//! Error types for TCS compiler

use thiserror::Error;

#[derive(Debug, Error)]
pub enum TcsError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error at line {line}, column {column}: {msg}")]
    ParseError {
        msg: String,
        line: usize,
        column: usize,
    },

    #[error("Verification error: {0}")]
    VerificationError(String),

    #[error("Code generation error: {0}")]
    CodeGenError(String),
}
