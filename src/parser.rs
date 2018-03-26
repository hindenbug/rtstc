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
pub struct Ast {
    type_: NodeType,
    body: Vec<Node>,
}

fn walk(token: &::tokenizer::Token) -> Result<Node, String> {
    match token.type_ {
        ::tokenizer::TokenType::Number => Ok(Node { type_: NodeType::NumericalLiteral, value: token.value }),
        ::tokenizer::TokenType::String => Ok(Node { type_: NodeType::StringLiteral, value: token.value}),
        _ => return Err("Error".to_string()),
    }
}

pub fn parse(tokens: Vec<::tokenizer::Token>) -> Result<Ast, String> {
    let mut nodes = vec![];

    while let Some(token) = tokens.into_iter().next() {
        match walk(&token) {
            Ok(node) => nodes.push(node),
            _ => return Err("Parsing Error".to_string())
        }
    }
    /*
    let mut program = Ast {
        type_: NodeType::Program,
        body: vec![],
    };*/

    Ok(Ast {type_: NodeType::Program, body: nodes})
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
            Ok(Ast {
                type_: NodeType::Program,
                body: vec![
                    Expression {
                        type_: NodeType::CallExpression,
                        name: "add".to_string(),
                        params: vec![
                            Node {
                                type_: NodeType::NumericalLiteral,
                                value: "2".to_string(),
                            },
                            Expression {
                                type_: NodeType::CallExpression,
                                name: "subtract".to_string(),
                                params: vec![
                                    Node {
                                        type_: NodeType::NumericalLiteral,
                                        value: "4".to_string(),
                                    },
                                    Node {
                                        type_: NodeType::NumericalLiteral,
                                        value: "2".to_string(),
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
