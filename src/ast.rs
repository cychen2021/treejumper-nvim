//! AST representation for navigating code structures.

use serde::{Deserialize, Serialize};

/// Represents a node in the code AST.
///
/// # Examples
///
/// ```ignore
/// let node = CodeNode {
///     kind: "function_definition".to_string(),
///     name: Some("my_function".to_string()),
///     start_row: 5,
///     start_col: 0,
///     end_row: 15,
///     end_col: 1,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CodeNode {
    /// The type of the node (e.g., "function_definition", "class_declaration")
    pub kind: String,
    /// Optional name of the node
    pub name: Option<String>,
    /// Starting row (0-indexed)
    pub start_row: usize,
    /// Starting column (0-indexed)
    pub start_col: usize,
    /// Ending row (0-indexed)
    pub end_row: usize,
    /// Ending column (0-indexed)
    pub end_col: usize,
}

impl CodeNode {
    /// Creates a new code node.
    pub fn new(
        kind: String,
        name: Option<String>,
        start_row: usize,
        start_col: usize,
        end_row: usize,
        end_col: usize,
    ) -> Self {
        Self {
            kind,
            name,
            start_row,
            start_col,
            end_row,
            end_col,
        }
    }

    /// Returns the line span of this node.
    pub fn line_span(&self) -> (usize, usize) {
        (self.start_row, self.end_row)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_node_creation() {
        let node = CodeNode::new(
            "function".to_string(),
            Some("main".to_string()),
            0,
            0,
            10,
            1,
        );
        assert_eq!(node.kind, "function");
        assert_eq!(node.name, Some("main".to_string()));
        assert_eq!(node.line_span(), (0, 10));
    }
}
