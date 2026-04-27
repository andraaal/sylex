use crate::token::{Location, Token, TokenType};

pub struct Lexer<'a> {
    source: std::iter::Peekable<std::str::Chars<'a>>,
    tokens: Vec<Token>,
    start: Location,
    current: Location,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            source: source.chars().peekable(),
            tokens: Vec::new(),
            start: Location {
                col: 1,
                line: 1,
                index: 0,
            },
            current: Location {
                line: 1,
                col: 1,
                index: 0,
            },
        }
    }

    fn emit(&mut self, token: TokenType) {
        self.tokens.push(Token {
            token_type: token,
            start: self.start,
            end: self.current,
        });
        self.start = self.current;
    }

    fn next(&mut self) -> Option<char> {
        let c = self.source.next();
        match c {
            Some('\n') => {
                self.current.line += 1;
                self.current.col = 1;
                self.current.index += 1;
            }
            Some(_) => {
                self.current.col += 1;
                self.current.index += 1;
            }
            None => {}
        };
        c
    }

    fn peek(&mut self) -> Option<&char> {
        self.source.peek()
    }

    pub fn lex(mut self) -> Vec<Token> {
        while let Some(c) = self.next() {
            match c {
                '/' => self.emit(TokenType::Slash),
                '+' => self.emit(TokenType::Plus),
                '-' => self.emit(TokenType::Minus),
                '*' => self.emit(TokenType::Asterisk),
                '?' => self.emit(TokenType::QuestionMark),
                ':' => self.emit(TokenType::Colon),
                ';' => self.emit(TokenType::Semicolon),
                '%' => self.emit(TokenType::Percent),
                '(' => self.emit(TokenType::LeftParen),
                ')' => self.emit(TokenType::RightParen),
                '{' => self.emit(TokenType::LeftBrace),
                '}' => self.emit(TokenType::RightBrace),
                '[' => self.emit(TokenType::LeftBracket),
                ']' => self.emit(TokenType::RightBracket),
                '!' => {
                    if let Some('=') = self.peek() {
                        self.next();
                        self.emit(TokenType::BangEqual);
                    } else {
                        self.emit(TokenType::Bang);
                    }
                }
                '<' => {
                    if let Some('=') = self.peek() {
                        self.next();
                        self.emit(TokenType::LesserEqual);
                    } else {
                        self.emit(TokenType::Lesser);
                    }
                }
                '>' => {
                    if let Some('=') = self.peek() {
                        self.next();
                        self.emit(TokenType::GreaterEqual);
                    } else {
                        self.emit(TokenType::Greater);
                    }
                }
                '=' => {
                    if let Some('=') = self.peek() {
                        self.next();
                        self.emit(TokenType::DoubleEqual);
                    } else {
                        self.emit(TokenType::Equal);
                    }
                }
                '|' if self.peek() == Some(&'|') => {
                    self.next();
                    self.emit(TokenType::DoublePipe);
                }
                '&' if self.peek() == Some(&'&') => {
                    self.next();
                    self.emit(TokenType::DoubleAmpersand);
                }
                '#' => {
                    let mut com = "#".to_string();
                    if self.peek() == Some(&'(') {
                        com.push('(');
                        self.next();
                        let mut depth = 1;
                        loop {
                            match self.next() {
                                Some(c) => {
                                    if c == '(' {
                                        depth += 1;
                                    }
                                    if c == ')' {
                                        depth -= 1;
                                        if depth == 0 {
                                            com.push(')');
                                            break;
                                        }
                                    }
                                    com.push(c);
                                }
                                None => {
                                    self.emit(TokenType::Invalid(
                                        "lexer: Unclosed comment".to_string(),
                                    ));
                                    break;
                                }
                            }
                        }
                    } else {
                        loop {
                            match self.next() {
                                None | Some('\n') => break,
                                Some(c) => com.push(c),
                            }
                        }
                    }
                    self.emit(TokenType::Comment(com));
                }
                c => {
                    if c.is_whitespace() {
                        // Skip whitespace; it has no semantics aside from delimiting other tokens
                    } else if c.is_ascii_digit() {
                        let mut acc = c.to_string();
                        while let Some(next) = self.peek().filter(|d| d.is_ascii_digit()) {
                            acc.push(*next);
                            self.next();
                        }
                        if Some(&'.') == self.peek() {
                            acc.push('.');
                            self.next();

                            while let Some(next) = self.peek().filter(|d| d.is_ascii_digit()) {
                                acc.push(*next);
                                self.next();
                            }
                            match acc.parse::<f64>() {
                                Ok(f) => self.emit(TokenType::Float(f)),
                                Err(e) => self.emit(TokenType::Invalid(format!(
                                    "lexer: Invalid float: {}",
                                    e
                                ))),
                            }
                        } else {
                            match acc.parse::<i64>() {
                                Ok(i) => self.emit(TokenType::Number(i)),
                                Err(e) => self.emit(TokenType::Invalid(format!(
                                    "lexer: Invalid integer: {}",
                                    e
                                ))),
                            }
                        }
                    } else if c.is_alphabetic() || c == '_' {
                        let mut acc = c.to_string();
                        while let Some(next) =
                            self.peek().filter(|c| c.is_alphanumeric() || **c == '_')
                        {
                            acc.push(*next);
                            self.next();
                        }
                        self.emit(TokenType::Identifier(acc));
                    } else {
                        self.emit(TokenType::Invalid(format!(
                            "lexer: Invalid character '{}'",
                            c
                        )));
                    }
                }
            }
        }

        self.tokens
    }
}


#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::token::{Location, Token, TokenType};

    #[test]
    fn simple_tokens() {
        let lexer = Lexer::new("+ - * / : ; ? = == > >= < <= ! != { } [ ] ( ) % && ||");
        let tokens = lexer.lex();
        let token_types: Vec<_> = tokens.into_iter().map(|t| t.token_type).collect();
        assert_eq!(
            token_types,
            vec![
                TokenType::Plus,
                TokenType::Minus,
                TokenType::Asterisk,
                TokenType::Slash,
                TokenType::Colon,
                TokenType::Semicolon,
                TokenType::QuestionMark,
                TokenType::Equal,
                TokenType::DoubleEqual,
                TokenType::Greater,
                TokenType::GreaterEqual,
                TokenType::Lesser,
                TokenType::LesserEqual,
                TokenType::Bang,
                TokenType::BangEqual,
                TokenType::LeftBrace,
                TokenType::RightBrace,
                TokenType::LeftBracket,
                TokenType::RightBracket,
                TokenType::LeftParen,
                TokenType::RightParen,
                TokenType::Percent,
                TokenType::DoubleAmpersand,
                TokenType::DoublePipe,
            ]
        );
    }

    #[test]
    fn int_float_tokens() {
        let lexer = Lexer::new("1+1.0");
        let tokens = lexer.lex();
        let token_types: Vec<_> = tokens.into_iter().map(|t| t.token_type).collect();
        assert_eq!(
            token_types,
            vec![TokenType::Number(1), TokenType::Plus, TokenType::Float(1.0),]
        )
    }

    #[test]
    fn identifier_tokens() {
        let lexer = Lexer::new("foo bar123 _baz");
        let tokens = lexer.lex();
        let token_types: Vec<_> = tokens.into_iter().map(|t| t.token_type).collect();
        assert_eq!(
            token_types,
            vec![
                TokenType::Identifier("foo".to_string()),
                TokenType::Identifier("bar123".to_string()),
                TokenType::Identifier("_baz".to_string()),
            ]
        )
    }

    #[test]
    fn comment_tokens() {
        let lexer = Lexer::new("#((invisible))1.00;4<string#invisible_too");
        let tokens = lexer.lex();
        let token_types: Vec<_> = tokens.into_iter().map(|t| t.token_type).collect();
        assert_eq!(
            token_types,
            vec![
                TokenType::Comment("#((invisible))".to_string()),
                TokenType::Float(1.0),
                TokenType::Semicolon,
                TokenType::Number(4),
                TokenType::Lesser,
                TokenType::Identifier("string".to_string()),
                TokenType::Comment("#invisible_too".to_string()),
            ]
        );
    }

    #[test]
    fn token_locations() {
        let lexer = Lexer::new("#comment\n1.42/34*string:from?this");
        let tokens = lexer.lex();
        let solution = vec![
            Token {
                token_type: TokenType::Comment("#comment".to_string()),
                start: Location {
                    line: 1,
                    col: 1,
                    index: 0,
                },
                end: Location {
                    line: 2,
                    col: 1,
                    index: 9,
                },
            },
            Token {
                token_type: TokenType::Float(1.42),
                start: Location {
                    line: 2,
                    col: 1,
                    index: 9,
                },
                end: Location {
                    line: 2,
                    col: 5,
                    index: 13,
                },
            },
            Token {
                token_type: TokenType::Slash,
                start: Location {
                    line: 2,
                    col: 5,
                    index: 13,
                },
                end: Location {
                    line: 2,
                    col: 6,
                    index: 14,
                },
            },
            Token {
                token_type: TokenType::Number(34),
                start: Location {
                    line: 2,
                    col: 6,
                    index: 14,
                },
                end: Location {
                    line: 2,
                    col: 8,
                    index: 16,
                },
            },
            Token {
                token_type: TokenType::Asterisk,
                start: Location {
                    line: 2,
                    col: 8,
                    index: 16,
                },
                end: Location {
                    line: 2,
                    col: 9,
                    index: 17,
                },
            },
            Token {
                token_type: TokenType::Identifier("string".to_string()),
                start: Location {
                    line: 2,
                    col: 9,
                    index: 17,
                },
                end: Location {
                    line: 2,
                    col: 15,
                    index: 23,
                },
            },
            Token {
                token_type: TokenType::Colon,
                start: Location {
                    line: 2,
                    col: 15,
                    index: 23,
                },
                end: Location {
                    line: 2,
                    col: 16,
                    index: 24,
                },
            },
            Token {
                token_type: TokenType::Identifier("from".to_string()),
                start: Location {
                    line: 2,
                    col: 16,
                    index: 24,
                },
                end: Location {
                    line: 2,
                    col: 20,
                    index: 28,
                },
            },
            Token {
                token_type: TokenType::QuestionMark,
                start: Location {
                    line: 2,
                    col: 20,
                    index: 28,
                },
                end: Location {
                    line: 2,
                    col: 21,
                    index: 29,
                },
            },
            Token {
                token_type: TokenType::Identifier("this".to_string()),
                start: Location {
                    line: 2,
                    col: 21,
                    index: 29,
                },
                end: Location {
                    line: 2,
                    col: 25,
                    index: 33,
                },
            },
        ];
        for (i, t) in tokens.into_iter().enumerate() {
            assert_eq!(t, solution[i]);
        }
    }
}
