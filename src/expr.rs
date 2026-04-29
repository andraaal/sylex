#[derive(Debug)]
pub enum Expr {
    Plus(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Int(i64),
    Float(f64),
    Bool(bool),
    Literal(String),
}