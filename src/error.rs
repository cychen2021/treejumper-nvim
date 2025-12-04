//! Error handling for TreeJumper.

use thiserror::Error;

/// Result type for TreeJumper operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur in TreeJumper operations.
#[derive(Error, Debug)]
pub enum Error {
    #[error("Unsupported language: {0}")]
    UnsupportedLanguage(String),

    #[error("Parser error: {0}")]
    ParseError(String),

    #[error("Navigation error: {0}")]
    NavigationError(String),

    #[error("Tree-sitter error: {0}")]
    TreeSitterError(String),

    #[error("Invalid node: {0}")]
    InvalidNode(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
