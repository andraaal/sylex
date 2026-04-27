mod lexer;
mod token;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn simple_tokens() {
        let lexer = Lexer::new("+ - * / : ; ? = == > >= < <= ! != { } [ ] ( ) % && ||");
        let tokens = lexer.lex();
        let token_types: Vec<_> = tokens.into_iter().map(|t| t.token_type).collect();
        assert_eq!(token_types, vec![
            token::TokenType::Plus,
            token::TokenType::Minus,
            token::TokenType::Asterisk,
            token::TokenType::Slash,
            token::TokenType::Colon,
            token::TokenType::Semicolon,
            token::TokenType::QuestionMark,
            token::TokenType::Equal,
            token::TokenType::DoubleEqual,
            token::TokenType::Greater,
            token::TokenType::GreaterEqual,
            token::TokenType::Lesser,
            token::TokenType::LesserEqual,
            token::TokenType::Bang,
            token::TokenType::BangEqual,
            token::TokenType::LeftBrace,
            token::TokenType::RightBrace,
            token::TokenType::LeftBracket,
            token::TokenType::RightBracket,
            token::TokenType::LeftParen,
            token::TokenType::RightParen,
            token::TokenType::Percent,
            token::TokenType::DoubleAmpersand,
            token::TokenType::DoublePipe,
        ]);
    }

    #[test]
    fn int_float_tokens() {
        let lexer = Lexer::new("1+1.0");
        let tokens = lexer.lex();
        let token_types: Vec<_> = tokens.into_iter().map(|t| t.token_type).collect();
        assert_eq!(token_types, vec![
            token::TokenType::Number(1),
            token::TokenType::Plus,
            token::TokenType::Float(1.0),
        ])
    }
}
