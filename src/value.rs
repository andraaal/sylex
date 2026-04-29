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
            _ => true,
        }
    }
}