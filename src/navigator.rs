//! Navigation through code AST.

use crate::ast::CodeNode;

/// Navigator for traversing and jumping through code structures.
pub struct Navigator {
    nodes: Vec<CodeNode>,
    current_index: usize,
}

impl Navigator {
    /// Creates a new navigator with the given nodes.
    pub fn new(nodes: Vec<CodeNode>) -> Self {
        Self {
            nodes,
            current_index: 0,
        }
    }

    /// Finds nodes containing the given line.
    ///
    /// # Arguments
    ///
    /// * `line` - The line number to search for (0-indexed)
    ///
    /// # Returns
    ///
    /// A vector of nodes that contain the given line
    pub fn find_nodes_at_line(&self, line: usize) -> Vec<&CodeNode> {
        self.nodes
            .iter()
            .filter(|node| node.start_row <= line && line <= node.end_row)
            .collect()
    }

    /// Moves to the next node.
    pub fn next(&mut self) -> Option<&CodeNode> {
        if self.current_index < self.nodes.len() - 1 {
            self.current_index += 1;
            Some(&self.nodes[self.current_index])
        } else {
            None
        }
    }

    /// Moves to the previous node.
    pub fn previous(&mut self) -> Option<&CodeNode> {
        if self.current_index > 0 {
            self.current_index -= 1;
            Some(&self.nodes[self.current_index])
        } else {
            None
        }
    }

    /// Returns the current node.
    pub fn current(&self) -> Option<&CodeNode> {
        self.nodes.get(self.current_index)
    }

    /// Returns all nodes.
    pub fn nodes(&self) -> &[CodeNode] {
        &self.nodes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_navigator_creation() {
        let nodes = vec![
            CodeNode::new("function".to_string(), Some("foo".to_string()), 0, 0, 5, 1),
            CodeNode::new("function".to_string(), Some("bar".to_string()), 6, 0, 10, 1),
        ];
        let nav = Navigator::new(nodes);
        assert_eq!(nav.current_index, 0);
    }

    #[test]
    fn test_find_nodes_at_line() {
        let nodes = vec![
            CodeNode::new("function".to_string(), Some("foo".to_string()), 0, 0, 5, 1),
            CodeNode::new("function".to_string(), Some("bar".to_string()), 6, 0, 10, 1),
        ];
        let nav = Navigator::new(nodes);
        let found = nav.find_nodes_at_line(3);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].name, Some("foo".to_string()));
    }

    #[test]
    fn test_navigation() {
        let nodes = vec![
            CodeNode::new("function".to_string(), Some("foo".to_string()), 0, 0, 5, 1),
            CodeNode::new("function".to_string(), Some("bar".to_string()), 6, 0, 10, 1),
        ];
        let mut nav = Navigator::new(nodes);
        assert_eq!(nav.current().unwrap().name, Some("foo".to_string()));
        nav.next();
        assert_eq!(nav.current().unwrap().name, Some("bar".to_string()));
        nav.previous();
        assert_eq!(nav.current().unwrap().name, Some("foo".to_string()));
    }
}
