pub fn transform(ast: ::parser::Node) {

    let mut new_ast = ::parser::Node::Program { body: []};

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_1() {
        let ast = ::parser::Node::Program {
            body: vec![
                ::parser::Node::CallExpression {
                    name: "add".to_string(),
                    params: vec![
                        ::parser::Node::NumberLiteral {
                            value: "2".to_string(),
                        },
                        ::parser::Node::CallExpression {
                            name: "subtract".to_string(),
                            params: vec![
                                ::parser::Node::NumberLiteral {
                                    value: "4".to_string(),
                                },
                                ::parser::Node::NumberLiteral {
                                    value: "2".to_string(),
                                },
                            ],
                        },
                    ],
                },
            ],
        };

        assert_eq!(transform(ast), {})
    }
}
