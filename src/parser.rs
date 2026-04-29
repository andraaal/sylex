use crate::expr::Expr;
use crate::Location;
use crate::{Token, TokenType};
use std::fmt::{Display, Formatter};
use std::iter::Peekable;
use std::vec::IntoIter;
use thiserror::Error;

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
    exprs: Vec<Expr>,
    errors: Vec<ParseError>,
    context: ParseContext, // Track current context for errors
}

#[derive(Error, Debug, Clone)]
#[error("error in context {context} at {start} until {end}:  \n{message}")]
pub struct ParseError {
    pub start: Location,
    pub end: Location,
    pub message: String,
    pub context: ParseContext,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParseContext {
    Root,
    Statement,
    Expression,
    PrefixExpr,
    InfixExpr,
}

impl Display for ParseContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseContext::Root => write!(f, "root"),
            ParseContext::Statement => write!(f, "statement"),
            ParseContext::PrefixExpr => write!(f, "prefix expression"),
            ParseContext::InfixExpr => write!(f, "infix expression"),
            ParseContext::Expression => write!(f, "expression"),
        }
    }
}

pub struct ParseResult {
    pub exprs: Vec<Expr>,
    pub errors: Vec<ParseError>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens.into_iter().peekable(),
            exprs: Vec::new(),
            errors: Vec::new(),
            context: ParseContext::Root,
        }
    }

    /// Parse and return expressions and errors
    pub fn parse(mut self) -> ParseResult {
        while self.peek().is_some() {
            match self.stmt() {
                Ok(expr) => self.exprs.push(expr),
                Err(err) => {
                    self.errors.push(err);
                    self.synchronize();
                }
            }
        }

        ParseResult {
            exprs: self.exprs,
            errors: self.errors,
        }
    }

    /// Parse a statement (currently only an expression followed by semicolon)
    fn stmt(&mut self) -> Result<Expr, ParseError> {
        let prev_rule = self.context;
        self.context = ParseContext::Statement;

        let expr = self.parse_precedence(0)?;
        self.consume(TokenType::Semicolon, "expected ';' after expression")?;

        self.context = prev_rule;
        Ok(expr)
    }

    /// Main expression parsing function
    fn parse_precedence(&mut self, min_precedence: u32) -> Result<Expr, ParseError> {
        let prev_rule = self.context;
        self.context = ParseContext::Expression;
        let mut lhs;

        // Start of expression
        if let Some(next) = self.next() {
            if let Some(rule) = Self::get_starting_rule(&next.token_type)
                && rule.prefix
            {
                lhs = self.prefix(next)?;
            } else {
                self.context = prev_rule;
                return Err(ParseError {
                    start: next.start,
                    end: next.end,
                    message: format!(
                        "unexpected token: {:?}, expected prefix operator",
                        next.token_type
                    ),
                    context: self.context,
                });
            }
        } else {
            self.context = prev_rule;
            return Err(self.error_at_eof("unexpected EOF".to_string()));
        }

        // Infix expression with a higher precedence than the current one
        while let Some(next) = self.peek() {
            if let Some(rule) = Self::get_chaining_rule(&next.token_type)
                && (rule.precedence > min_precedence
                    || (rule.precedence >= min_precedence
                        && matches!(rule.associativity, Associativity::Right)))
            {
                let next = self.next().unwrap();
                lhs = self.infix(next, lhs, rule)?;
            } else {
                break;
            }
        }

        self.context = prev_rule;
        Ok(lhs)
    }

    /// Parse a prefix or nilfix operator
    fn prefix(&mut self, token: Token) -> Result<Expr, ParseError> {
        let prev_rule = self.context;
        self.context = ParseContext::PrefixExpr;

        let result = match token.token_type {
            TokenType::Number(n) => Ok(Expr::Int(n)),
            TokenType::Float(f) => Ok(Expr::Float(f)),
            TokenType::Literal(l) => Ok(Expr::Literal(l)),
            TokenType::LeftParen => {
                let expr = self.parse_precedence(0)?;
                self.consume(TokenType::RightParen, "expected ')' after expression")?;
                Ok(expr)
            }
            tk => Err(ParseError {
                start: token.start,
                end: token.end,
                message: format!("unexpected token: {}, expected prefix operator", tk),
                context: self.context,
            }),
        };
        self.context = prev_rule;
        result
    }

    fn infix(&mut self, token: Token, lhs: Expr, rule: &Rule) -> Result<Expr, ParseError> {
        let prev_rule = self.context;
        self.context = ParseContext::InfixExpr;

        let result = match token.token_type {
            TokenType::Minus => {
                let rhs = self.parse_precedence(rule.precedence)?;
                Ok(Expr::Minus(Box::new(lhs), Box::new(rhs)))
            }
            TokenType::Plus => {
                let rhs = self.parse_precedence(rule.precedence)?;
                Ok(Expr::Plus(Box::new(lhs), Box::new(rhs)))
            }
            TokenType::Asterisk => {
                let rhs = self.parse_precedence(rule.precedence)?;
                Ok(Expr::Mul(Box::new(lhs), Box::new(rhs)))
            }
            TokenType::Slash => {
                let rhs = self.parse_precedence(rule.precedence)?;
                Ok(Expr::Div(Box::new(lhs), Box::new(rhs)))
            }
            TokenType::Greater => {
                let rhs = self.parse_precedence(rule.precedence)?;
                Ok(Expr::Greater(Box::new(lhs), Box::new(rhs)))
            }
            TokenType::GreaterEqual => {
                let rhs = self.parse_precedence(rule.precedence)?;
                Ok(Expr::GreaterEq(Box::new(lhs), Box::new(rhs)))
            }
            TokenType::Lesser => {
                let rhs = self.parse_precedence(rule.precedence)?;
                Ok(Expr::Less(Box::new(lhs), Box::new(rhs)))
            }
            TokenType::LesserEqual => {
                let rhs = self.parse_precedence(rule.precedence)?;
                Ok(Expr::LessEq(Box::new(lhs), Box::new(rhs)))
            }
            TokenType::DoubleEqual => {
                let rhs = self.parse_precedence(rule.precedence)?;
                Ok(Expr::Eq(Box::new(lhs), Box::new(rhs)))
            }
            TokenType::BangEqual => {
                let rhs = self.parse_precedence(rule.precedence)?;
                Ok(Expr::Neq(Box::new(lhs), Box::new(rhs)))
            }
            TokenType::DoublePipe => {
                let rhs = self.parse_precedence(rule.precedence)?;
                Ok(Expr::Or(Box::new(lhs), Box::new(rhs)))
            }
            TokenType::DoubleAmpersand => {
                let rhs = self.parse_precedence(rule.precedence)?;
                Ok(Expr::And(Box::new(lhs), Box::new(rhs)))
            }
            TokenType::ThreeWay => {
                let rhs = self.parse_precedence(rule.precedence)?;
                Ok(Expr::ThreeWay(Box::new(lhs), Box::new(rhs)))
            }
            TokenType::ThreeWayReverse => {
                let rhs = self.parse_precedence(rule.precedence)?;
                Ok(Expr::ThreeWayReverse(Box::new(lhs), Box::new(rhs)))
            }
            tk => {
                let err = Err(ParseError {
                    start: token.start,
                    end: token.end,
                    message: format!("unexpected token: {}, expected infix operator", tk),
                    context: self.context,
                });
                err
            }
        };

        self.context = prev_rule;
        result
    }

    /// Require a specific token type; consume it if present, otherwise return an error
    fn consume(&mut self, expected: TokenType, message: &str) -> Result<Token, ParseError> {
        match self.peek() {
            Some(token) if token.token_type == expected => Ok(self.next().unwrap()),
            Some(token) => Err(ParseError {
                start: token.start,
                end: token.end,
                message: format!("{}: got '{}'", message, token.token_type),
                context: self.context,
            }),
            None => Err(self.error_at_eof(format!("{}: got EOF", message))),
        }
    }

    /// Skip tokens until we find a synchronization point (=semicolon)
    fn synchronize(&mut self) {
        while let Some(token) = self.peek() {
            match token.token_type {
                TokenType::Semicolon => {
                    self.next();
                    break;
                }
                _ => {
                    self.next();
                }
            }
        }
    }

    fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    fn error_at_eof(&self, message: String) -> ParseError {
        ParseError {
            start: Location {
                line: 0,
                col: 0,
                index: 0,
            },
            end: Location {
                line: 0,
                col: 0,
                index: 0,
            },
            message,
            context: self.context,
        }
    }

    /// Get nilfix/prefix rules
    fn get_starting_rule(tkt: &TokenType) -> Option<&'static Rule> {
        let rule = match tkt {
            TokenType::Number(_)
            | TokenType::Float(_)
            | TokenType::Literal(_)
            | TokenType::LeftParen => &PRIMARY_RULE,
            TokenType::Bang | TokenType::Minus => &UNARY_RULE,
            _ => return None,
        };
        Some(rule)
    }

    /// Get infix/postfix rules
    fn get_chaining_rule(tkt: &TokenType) -> Option<&'static Rule> {
        let rule: &'static Rule = match tkt {
            TokenType::Plus | TokenType::Minus => &TERM_RULE,
            TokenType::Asterisk | TokenType::Slash => &FACTOR_RULE,
            TokenType::Greater
            | TokenType::GreaterEqual
            | TokenType::Lesser
            | TokenType::LesserEqual => &COMPARISON_RULE,
            TokenType::DoubleEqual | TokenType::BangEqual => &EQUALITY_RULE,
            _ => return None,
        };
        Some(rule)
    }
}

/// Rules for rules: Either prefix or infix (or both) has to be true, otherwise the rule will always lead to parse errors. Rules with the same precedence have to have the same associativity.
#[derive(Debug, Clone, Copy)]
struct Rule {
    prefix: bool,
    infix: bool,
    precedence: u32,
    associativity: Associativity, // Only matters when it is an infix operation
}

#[derive(Debug, Clone, Copy)]
enum Associativity {
    Left,
    Right,
}

const PRIMARY_RULE: Rule = Rule {
    prefix: true,
    infix: false,
    precedence: 100,
    associativity: Associativity::Left,
};

const TERM_RULE: Rule = Rule {
    prefix: true,
    infix: false,
    precedence: 50,
    associativity: Associativity::Left,
};

const FACTOR_RULE: Rule = Rule {
    prefix: true,
    infix: false,
    precedence: 60,
    associativity: Associativity::Left,
};

const COMPARISON_RULE: Rule = Rule {
    prefix: false,
    infix: true,
    precedence: 40,
    associativity: Associativity::Left,
};

const EQUALITY_RULE: Rule = Rule {
    prefix: false,
    infix: true,
    precedence: 30,
    associativity: Associativity::Left,
};

const UNARY_RULE: Rule = Rule {
    prefix: true,
    infix: false,
    precedence: 70,
    associativity: Associativity::Left,
};
