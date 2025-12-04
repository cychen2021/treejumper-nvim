/// Integration tests for TreeJumper

#[cfg(test)]
mod tests {
    use treejumper_nvim::ast::CodeNode;
    use treejumper_nvim::navigator::Navigator;
    use treejumper_nvim::parser::Language;

    #[test]
    fn test_language_parsing() {
        let result = Language::from_str("rust");
        assert!(result.is_ok());
    }

    #[test]
    fn test_navigator_with_multiple_nodes() {
        let nodes = vec![
            CodeNode::new("fn".to_string(), Some("main".to_string()), 0, 0, 10, 1),
            CodeNode::new(
                "struct".to_string(),
                Some("Config".to_string()),
                12,
                0,
                20,
                1,
            ),
            CodeNode::new("impl".to_string(), None, 22, 0, 30, 1),
        ];

        let mut nav = Navigator::new(nodes);

        // Test navigation through nodes
        assert_eq!(nav.current().unwrap().name, Some("main".to_string()));

        nav.next();
        assert_eq!(nav.current().unwrap().name, Some("Config".to_string()));

        nav.next();
        assert_eq!(nav.current().unwrap().name, None);

        nav.previous();
        assert_eq!(nav.current().unwrap().name, Some("Config".to_string()));
    }

    #[test]
    fn test_find_nodes_at_line_multiple() {
        let nodes = vec![
            CodeNode::new(
                "class".to_string(),
                Some("MyClass".to_string()),
                0,
                0,
                20,
                1,
            ),
            CodeNode::new("fn".to_string(), Some("method1".to_string()), 2, 4, 8, 5),
            CodeNode::new("fn".to_string(), Some("method2".to_string()), 10, 4, 15, 5),
        ];

        let nav = Navigator::new(nodes);

        // Line 3 should be in MyClass and method1
        let found = nav.find_nodes_at_line(3);
        assert_eq!(found.len(), 2);

        // Line 12 should be in MyClass and method2
        let found = nav.find_nodes_at_line(12);
        assert_eq!(found.len(), 2);

        // Line 25 should not be in any node
        let found = nav.find_nodes_at_line(25);
        assert_eq!(found.len(), 0);
    }
}
