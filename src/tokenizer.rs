pub mod tokenizer {

    #[derive(Debug, PartialEq)]
    pub enum TokenType {
        OpenParen,
        CloseParen,
        Operation,
        Number,
        String,
        TokenError,
    }

    #[derive(Debug, PartialEq)]
    pub struct Token {
        type_: TokenType,
        value: String,
    }

    pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
        let mut tokens: Vec<Token> = vec![];

        let mut chars_ = input.chars();

        while let Some(ch) = chars_.next() {
            if ch.is_whitespace() {
                continue;
            }

            match ch {
                '(' => tokens.push(Token {
                    type_: TokenType::OpenParen,
                    value: ch.to_string(),
                }),
                ')' => tokens.push(Token {
                    type_: TokenType::CloseParen,
                    value: ch.to_string(),
                }),
                'a'...'z' => {
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
                '"' => tokens.push(Token {
                    type_: TokenType::String,
                    value: ch.to_string(),
                }),
                _ => tokens.push(Token {
                    type_: TokenType::TokenError,
                    value: format!("I don't know what this character is: {}", ch),
                }),
            }
        }

        Ok(tokens)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_tokenizer() {
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
    }
}
