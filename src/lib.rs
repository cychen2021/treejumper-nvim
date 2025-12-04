//! TreeJumper - A Neovim plugin for AST-based code navigation using tree-sitter.
//!
//! This library provides the core functionality for navigating code through Abstract Syntax Trees
//! using tree-sitter. It supports C, C++, Python, Rust, and C#.

pub mod ast;
pub mod error;
pub mod navigator;
pub mod parser;

pub use ast::CodeNode;
pub use error::{Error, Result};
pub use navigator::Navigator;
pub use parser::Parser;
