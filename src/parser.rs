#[derive(Debug, PartialEq)]
pub enum NodeType {
    Program,
    CallExpression,
    StringLiteral,
    NumericalLiteral,
}

#[derive(Debug, PartialEq)]
pub struct Node {
    type_: NodeType,
    value: String,
}

#[derive(Debug, PartialEq)]
pub struct Expression {
    type_: NodeType,
    name: String,
    params: Vec<Node>,
}

#[derive(Debug, PartialEq)]
pub struct Program {
    type_: NodeType,
    body: Vec<Node>,
}

pub fn parse(tokens: Vec<::tokenizer::Token>) -> Result<Program, String> {
    let mut program = Program {
        type_: NodeType::Program,
        body: vec![],
    };

    Ok(Program)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_1() {
        let tokens = vec![
            ::tokenizer::Token {
                type_: ::tokenizer::TokenType::OpenParen,
                value: "(".to_string(),
            },
            ::tokenizer::Token {
                type_: ::tokenizer::TokenType::Operation,
                value: "add".to_string(),
            },
            ::tokenizer::Token {
                type_: ::tokenizer::TokenType::Number,
                value: "22".to_string(),
            },
            ::tokenizer::Token {
                type_: ::tokenizer::TokenType::OpenParen,
                value: "(".to_string(),
            },
            ::tokenizer::Token {
                type_: ::tokenizer::TokenType::Operation,
                value: "subtract".to_string(),
            },
            ::tokenizer::Token {
                type_: ::tokenizer::TokenType::Number,
                value: "4".to_string(),
            },
            ::tokenizer::Token {
                type_: ::tokenizer::TokenType::Number,
                value: "2".to_string(),
            },
            ::tokenizer::Token {
                type_: ::tokenizer::TokenType::CloseParen,
                value: ")".to_string(),
            },
            ::tokenizer::Token {
                type_: ::tokenizer::TokenType::CloseParen,
                value: ")".to_string(),
            },
        ];

        assert_eq!(
            parse(tokens),
            Ok(Program {
                type_: NodeType::Program,
                body: vec![
                    Expression {
                        type_: NodeType::CallExpression,
                        name: "add",
                        params: vec![
                            Node {
                                type_: NodeType::NumberLiteral,
                                value: "2",
                            },
                            Expression {
                                type_: NodeType::CallExpression,
                                name: "subtract",
                                params: vec![
                                    Node {
                                        type_: NodeType::NumberLiteral,
                                        value: "4",
                                    },
                                    Node {
                                        type_: NodeType::NumberLiteral,
                                        value: "2",
                                    },
                                ],
                            },
                        ],
                    },
                ],
            })
        )
    }
}
