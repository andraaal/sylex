use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum Value {
    Number(Number),
    Bool(bool),
    Str(String),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Number {
    Int(i64),
    Float(f64),
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Str(s) => !s.is_empty(),
            Value::Number(Number::Int(i)) => *i != 0,
            Value::Number(Number::Float(f)) => *f != 0.0,
            _ => true,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Str(s) => write!(f, "{}", s),
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Int(i) => write!(f, "{}", i),
            Number::Float(n) => write!(f, "{}", n),
        }
    }
}