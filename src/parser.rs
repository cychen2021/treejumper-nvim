//! Tree-sitter parser wrapper for supported languages.

use crate::ast::CodeNode;
use crate::error::{Error, Result};

/// Supported programming languages.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    C,
    Cpp,
    Python,
    Rust,
    CSharp,
}

impl Language {
    /// Parses a language from a string.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let lang = Language::from_str("rust")?;
    /// assert_eq!(lang, Language::Rust);
    /// ```
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "c" => Ok(Language::C),
            "cpp" | "c++" => Ok(Language::Cpp),
            "python" | "py" => Ok(Language::Python),
            "rust" | "rs" => Ok(Language::Rust),
            "csharp" | "c#" | "cs" => Ok(Language::CSharp),
            _ => Err(Error::UnsupportedLanguage(s.to_string())),
        }
    }

    /// Returns the file extensions for this language.
    pub fn file_extensions(&self) -> &'static [&'static str] {
        match self {
            Language::C => &["c", "h"],
            Language::Cpp => &["cpp", "cc", "cxx", "hpp", "h"],
            Language::Python => &["py"],
            Language::Rust => &["rs"],
            Language::CSharp => &["cs"],
        }
    }
}

/// Parser for extracting AST nodes from source code.
pub struct Parser {
    language: Language,
}

impl Parser {
    /// Creates a new parser for the given language.
    pub fn new(language: Language) -> Self {
        Self { language }
    }

    /// Parses the given source code and returns top-level nodes.
    ///
    /// # Arguments
    ///
    /// * `_source` - The source code to parse
    ///
    /// # Returns
    ///
    /// A vector of top-level code nodes
    pub fn parse(&self, _source: &str) -> Result<Vec<CodeNode>> {
        // This is a placeholder implementation.
        // The actual implementation will use tree-sitter to parse the code.
        match self.language {
            Language::C | Language::Cpp | Language::Python | Language::Rust | Language::CSharp => {
                Ok(Vec::new())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_from_str() {
        assert_eq!(Language::from_str("rust").unwrap(), Language::Rust);
        assert_eq!(Language::from_str("python").unwrap(), Language::Python);
        assert_eq!(Language::from_str("c++").unwrap(), Language::Cpp);
    }

    #[test]
    fn test_unsupported_language() {
        assert!(Language::from_str("unknown").is_err());
    }

    #[test]
    fn test_file_extensions() {
        assert!(Language::Rust.file_extensions().contains(&"rs"));
        assert!(Language::Python.file_extensions().contains(&"py"));
    }

    #[test]
    fn test_parser_creation() {
        let parser = Parser::new(Language::Rust);
        assert_eq!(parser.language, Language::Rust);
    }
}
