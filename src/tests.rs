#[cfg(test)]
mod tests {
    use crate::asg::{ASG, Node};
    use crate::interpreter::execute;

    #[test]
    fn test_literal_add() {
        // x = 1 + 2
        let n1 = Node::literal_int(1, 1);
        let n2 = Node::literal_int(2, 2);
        let n3 = Node::binary_add(3, 1, 2);

        let asg = ASG {
            nodes: vec![n1, n2, n3],
            entry: 3,
        };

        let mut output = Vec::new();
        let result = execute(&asg);
        assert!(result.is_ok());
        // Для расширения: можно добавить сбор вывода и проверку результата
    }

    #[test]
    fn test_conditional_then() {
        // if (1) { 42 } else { 99 }
        let n1 = Node::literal_int(1, 1);
        let n2 = Node::literal_int(2, 42);
        let n3 = Node::literal_int(3, 99);
        let n4 = Node::conditional(4, 1, 2, 3);

        let asg = ASG {
            nodes: vec![n1, n2, n3, n4],
            entry: 4,
        };

        let result = execute(&asg);
        assert!(result.is_ok());
        // Можно также проверять, что результат entry node == 42
    }

    #[test]
    fn test_conditional_else() {
        // if (0) { 42 } else { 99 }
        let n1 = Node::literal_int(1, 0);
        let n2 = Node::literal_int(2, 42);
        let n3 = Node::literal_int(3, 99);
        let n4 = Node::conditional(4, 1, 2, 3);

        let asg = ASG {
            nodes: vec![n1, n2, n3, n4],
            entry: 4,
        };

        let result = execute(&asg);
        assert!(result.is_ok());
        // Можно также проверять, что результат entry node == 99
    }
}
