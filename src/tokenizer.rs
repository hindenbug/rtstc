#[derive(Debug, PartialEq)]
pub enum TokenType {
    OpenParen,
    CloseParen,
    Operation,
    Number,
    String,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub type_: TokenType,
    pub value: String,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = vec![];

    let mut chars_ = input.chars();

    while let Some(ch) = chars_.next() {
        match ch {
            ch if ch.is_whitespace() => continue,
            '(' => tokens.push(Token {
                type_: TokenType::OpenParen,
                value: ch.to_string(),
            }),
            ')' => tokens.push(Token {
                type_: TokenType::CloseParen,
                value: ch.to_string(),
            }),
            'a'...'z' | 'A'...'Z' => {
                let mut name = String::new();
                name.push(ch);

                while let Some(&'a'...'z') = chars_.clone().peekable().peek() {
                    name.push(chars_.next().unwrap())
                }

                tokens.push(Token {
                    type_: TokenType::Operation,
                    value: name,
                })
            }
            '0'...'9' => {
                let mut number = String::new();
                number.push(ch);

                while let Some(&'0'...'9') = chars_.clone().peekable().peek() {
                    number.push(chars_.next().unwrap())
                }

                tokens.push(Token {
                    type_: TokenType::Number,
                    value: number,
                })
            }
            '"' => {
                let mut str_ = String::new();

                while let Some(c) = chars_.clone().peekable().peek() {
                    match c {
                        &'"' => {
                            chars_.next().unwrap();
                            break;
                        }
                        _ => str_.push(chars_.next().unwrap()),
                    }
                }

                tokens.push(Token {
                    type_: TokenType::String,
                    value: str_,
                });
            }
            _ => return Err(format!("Error : {}", ch)),
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer_1() {
        assert_eq!(
            tokenize("(add 22 (subtract 4 2))"),
            Ok(vec![
                Token {
                    type_: TokenType::OpenParen,
                    value: "(".to_string(),
                },
                Token {
                    type_: TokenType::Operation,
                    value: "add".to_string(),
                },
                Token {
                    type_: TokenType::Number,
                    value: "22".to_string(),
                },
                Token {
                    type_: TokenType::OpenParen,
                    value: "(".to_string(),
                },
                Token {
                    type_: TokenType::Operation,
                    value: "subtract".to_string(),
                },
                Token {
                    type_: TokenType::Number,
                    value: "4".to_string(),
                },
                Token {
                    type_: TokenType::Number,
                    value: "2".to_string(),
                },
                Token {
                    type_: TokenType::CloseParen,
                    value: ")".to_string(),
                },
                Token {
                    type_: TokenType::CloseParen,
                    value: ")".to_string(),
                },
            ])
        )
    }

    #[test]
    fn test_tokenizer_2() {
        assert_eq!(
            tokenize("(concat \"add\" \"this\")"),
            Ok(vec![
                Token {
                    type_: TokenType::OpenParen,
                    value: "(".to_string(),
                },
                Token {
                    type_: TokenType::Operation,
                    value: "concat".to_string(),
                },
                Token {
                    type_: TokenType::String,
                    value: "add".to_string(),
                },
                Token {
                    type_: TokenType::String,
                    value: "this".to_string(),
                },
                Token {
                    type_: TokenType::CloseParen,
                    value: ")".to_string(),
                },
            ])
        )
    }
}
