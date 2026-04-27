#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub start: Location,
    pub end: Location,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Location {
    pub line: usize,
    pub col: usize,
    pub index: usize,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Identifier(String),
    Number(i64),
    Float(f64),
    Literal(String),
    Plus,
    Minus,
    Asterisk,
    Slash,
    Colon,
    Semicolon,
    QuestionMark,
    Equal,
    DoubleEqual,
    Greater,
    GreaterEqual,
    Lesser,
    LesserEqual,
    Bang,
    BangEqual,
    RightBrace,
    RightBracket,
    LeftBracket,
    LeftBrace,
    RightParen,
    LeftParen,
    Percent,
    DoubleAmpersand,
    DoublePipe,
    
    Invalid(String),
    Comment(String),
}
