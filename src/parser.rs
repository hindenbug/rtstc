use std::slice::Iter;

#[derive(Debug, PartialEq)]
pub enum Node {
    Program { body: Vec<Node> },
    CallExpression { name: String, params: Vec<Node> },
    StringLiteral { value: String },
    NumberLiteral { value: String },
}

fn walk(
    token: &::tokenizer::Token,
    tokens_iter: &mut Iter<::tokenizer::Token>,
) -> Result<Node, String> {
    match token.type_ {
        ::tokenizer::TokenType::Number => Ok(Node::NumberLiteral {
            value: token.value.to_string(),
        }),
        ::tokenizer::TokenType::String => Ok(Node::StringLiteral {
            value: token.value.to_string(),
        }),
        ::tokenizer::TokenType::OpenParen => {
            if let Some(tkn) = tokens_iter.next() {
                match tkn.type_ {
                    ::tokenizer::TokenType::Operation => {
                        let mut params: Vec<Node> = vec![];

                        while let Some(c) = tokens_iter.clone().peekable().peek() {
                            match c.type_ {
                                ::tokenizer::TokenType::CloseParen => {
                                    tokens_iter.next().unwrap();
                                    break;
                                }
                                _ => match walk(&tokens_iter.next().unwrap(), tokens_iter) {
                                    Ok(node) => params.push(node),
                                    Err(value) => return Err(value),
                                },
                            }
                        }

                        Ok(Node::CallExpression {
                            name: tkn.value.to_string(),
                            params: params,
                        })
                    }
                    _ => return Err("Error".to_string()),
                }
            } else {
                return Err("Error".to_string());
            }
        }
        _ => return Err("Error".to_string()),
    }
}

pub fn parse(tokens: &Vec<::tokenizer::Token>) -> Result<Node, String> {
    let mut nodes = vec![];
    let mut tokens_iter = tokens.into_iter();

    while let Some(token) = tokens_iter.next() {
        match walk(&token, &mut tokens_iter) {
            Ok(node) => nodes.push(node),
            _ => return Err("Parsing Error".to_string()),
        }
    }

    Ok(Node::Program { body: nodes })
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
                value: "2".to_string(),
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
            parse(&tokens),
            Ok(Node::Program {
                body: vec![
                    Node::CallExpression {
                        name: "add".to_string(),
                        params: vec![
                            Node::NumberLiteral {
                                value: "2".to_string(),
                            },
                            Node::CallExpression {
                                name: "subtract".to_string(),
                                params: vec![
                                    Node::NumberLiteral {
                                        value: "4".to_string(),
                                    },
                                    Node::NumberLiteral {
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
