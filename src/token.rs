use std::fmt::Display;

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
    ThreeWay,
    ThreeWayReverse,
    True,
    False,
    
    Invalid(String),
    Comment(String),
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.col)
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::True => write!(f, "true"),
            TokenType::False => write!(f, "false"),
            TokenType::Identifier(name) => write!(f, "{}", name),
            TokenType::Number(n) => write!(f, "{}", n),
            TokenType::Float(n) => write!(f, "{}", n),
            TokenType::Literal(l) => write!(f, "{}", l),
            TokenType::Plus => write!(f, "+"),
            TokenType::Minus => write!(f, "-"),
            TokenType::Asterisk => write!(f, "*"),
            TokenType::Slash => write!(f, "/"),
            TokenType::Colon => write!(f, ":"),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::QuestionMark => write!(f, "?"),
            TokenType::Equal => write!(f, "="),
            TokenType::DoubleEqual => write!(f, "=="),
            TokenType::Greater => write!(f, ">"),
            TokenType::GreaterEqual => write!(f, ">="),
            TokenType::Lesser => write!(f, "<"),
            TokenType::LesserEqual => write!(f, "<="),
            TokenType::Bang => write!(f, "!"),
            TokenType::BangEqual => write!(f, "!="),
            TokenType::RightBrace => write!(f, "}}"),
            TokenType::RightBracket => write!(f, "]"),
            TokenType::LeftBracket => write!(f, "["),
            TokenType::LeftBrace => write!(f, "{{"),
            TokenType::RightParen => write!(f, ")"),
            TokenType::LeftParen => write!(f, "("),
            TokenType::Percent => write!(f, "%"),
            TokenType::DoubleAmpersand => write!(f, "&&"),
            TokenType::DoublePipe => write!(f, "||"),
            TokenType::ThreeWay => write!(f, "<=>"),
            TokenType::ThreeWayReverse => write!(f, ">=<"),
            TokenType::Invalid(i) => write!(f, "{}", i),
            TokenType::Comment(c) => write!(f, "{}", c),
        }
    }
}