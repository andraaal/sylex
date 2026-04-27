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
        while let Some(c) = self.source.next() {
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
                    // Do comments here.
                }
                c => {
                    if c.is_whitespace() {
                        // Skip whitespace; it has no semantics aside from delimiting other tokens
                    } else if c.is_digit(10) {
                        let mut acc = c.to_string();
                        while let Some(next) = self.peek().filter(|d| d.is_digit(10)) {
                            acc.push(*next);
                            self.next();
                        }
                        if Some(&'.') == self.peek() {
                            acc.push('.');
                            self.next();

                            while let Some(next) = self.peek().filter(|d| d.is_digit(10)) {
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
                        self.emit(TokenType::Invalid(format!("lexer: Invalid character '{}'", c)));
                    }
                }
            }
        }

        println!("{:?}", self.tokens);

        self.tokens
    }
}
