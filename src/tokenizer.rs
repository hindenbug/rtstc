pub mod tokenizer {
    pub enum TokenType {
        OpenParen,
        CloseParen,
        Name(String),
        Number(String),
        String(String),
    }

    pub struct Token {
        type_: TokenType,
        value: String
    }

    pub fn tokenizer(input: &str) -> Result<Vec<Token>, String> {
        let mut tokens: Vec<Token> = vec![Token {type_: TokenType::OpenParen, value: "test".to_string()}];

        Ok(tokens)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_tokenizer() {
            assert_eq!(tokenizer("(add 2 (subtract 4 2))"), [
                Token {type_: TokenType::OpenParen, value: "test"}
                //{ type_: Token::OpenParen,  value: '('        },
                //{ type_: Token::Name,   value: "add"      },
                //{ type: Token::Number, value: "2"        },
                //{ type: Token::OpenParen,  value: '('        },
                //{ type: Token::Name,   value: "subtract" },
                //{ type: Token::Number, value: "4"        },
                //{ type: Token::Number, value: "2"        },
                //{ type: Token::CloseParen,  value: ')'        },
                //{ type: Token::CloseParen,  value: ')'        },
            ])
        }
    }
}
