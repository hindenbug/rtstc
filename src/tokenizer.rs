pub mod tokenizer {

    #[derive(Debug, PartialEq)]
    pub enum TokenType {
        OpenParen,
        CloseParen,
        Name,
        Number,
        String,
        TokenError
    }

    #[derive(Debug, PartialEq)]
    pub struct Token {
        type_: TokenType,
        value: String,
    }

    pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
        let mut tokens: Vec<Token> = vec![];

        while let Some(ch) = input.chars().next() {
            match ch {
                ch if ch.is_whitespace() => continue,
                '(' => tokens.push(Token { type_: TokenType::OpenParen, value: ch.to_string() }),
                ')' => tokens.push(Token { type_: TokenType::CloseParen, value: ch.to_string() }),
                'a'...'z' => tokens.push(Token { type_: TokenType::Name, value: ch.to_string() }),
                '0'...'9' => tokens.push(Token { type_: TokenType::Number, value: ch.to_string() }),
                '"' => tokens.push(Token { type_: TokenType::String, value: ch.to_string() }),
                _ => tokens.push(Token { type_: TokenType::TokenError, value: format!("I don't know what this character is: {}", ch) }),
            }
        }

        println!("{:?}", tokens);

        Ok(tokens)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_tokenizer() {
            assert_eq!(
                tokenizer("("),
                Ok(vec![
                    Token {
                        type_: TokenType::OpenParen,
                        value: "(".to_string(),
                    }
                ])
            )
        }
    }
}
