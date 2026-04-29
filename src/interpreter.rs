use crate::expr::Expr;
use crate::value::{Number, Value};
use thiserror::Error;

pub struct Interpreter {}

#[derive(Error, Clone, Debug)]
#[error("error in interpreter: {message}")]
pub struct InterpretError {
    pub message: String,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    pub fn interpret(&mut self, expr: Expr) -> Result<Value, InterpretError> {
        self.eval(expr)
    }

    fn eval(&mut self, expr: Expr) -> Result<Value, InterpretError> {
        match expr {
            Expr::Ternary(cond, then_branch, else_branch) => {
                let condition = self.eval(*cond)?;
                if condition.is_truthy() {
                    self.eval(*then_branch)
                } else {
                    self.eval(*else_branch)
                }
            }
            Expr::Call(name, args) => {
                let mut evaluated_args = Vec::new();
                for arg in *args {
                    evaluated_args.push(self.eval(arg)?);
                }
                crate::builtin::call_builtin(name, evaluated_args)
            }
            Expr::Plus(a, b) => self.eval_numeric_op(
                *a,
                *b,
                |a, b| Value::Number(Number::Float(a + b)),
                |a, b| Value::Number(Number::Int(a + b)),
                "+",
            ),
            Expr::Minus(a, b) => self.eval_numeric_op(
                *a,
                *b,
                |a, b| Value::Number(Number::Float(a - b)),
                |a, b| Value::Number(Number::Int(a - b)),
                "-",
            ),
            Expr::Mul(a, b) => self.eval_numeric_op(
                *a,
                *b,
                |a, b| Value::Number(Number::Float(a * b)),
                |a, b| Value::Number(Number::Int(a * b)),
                "*",
            ),
            Expr::Div(a, b) => self.eval_numeric_op(
                *a,
                *b,
                |a, b| Value::Number(Number::Float(a / b)),
                |a, b| Value::Number(Number::Float(a as f64 / b as f64)), // Always divide as float
                "/",
            ),
            Expr::Greater(a, b) => self.eval_numeric_op(
                *a,
                *b,
                |a, b| Value::Bool(a > b),
                |a, b| Value::Bool(a > b),
                ">",
            ),
            Expr::Less(a, b) => self.eval_numeric_op(
                *a,
                *b,
                |a, b| Value::Bool(a < b),
                |a, b| Value::Bool(a < b),
                "<",
            ),
            Expr::Eq(a, b) => Ok(Value::Bool(self.eval(*a)? == self.eval(*b)?)),
            Expr::Neq(a, b) => Ok(Value::Bool(self.eval(*a)? != self.eval(*b)?)),
            Expr::And(a, b) => {
                let left = self.eval(*a)?;
                if left.is_truthy() {
                    self.eval(*b)
                } else {
                    Ok(left)
                }
            }
            Expr::Or(a, b) => {
                let left = self.eval(*a)?;
                if left.is_truthy() {
                    Ok(left)
                } else {
                    self.eval(*b)
                }
            }
            Expr::GreaterEq(a, b) => self.eval_numeric_op(
                *a,
                *b,
                |a, b| Value::Bool(a >= b),
                |a, b| Value::Bool(a >= b),
                ">=",
            ),
            Expr::LessEq(a, b) => self.eval_numeric_op(
                *a,
                *b,
                |a, b| Value::Bool(a <= b),
                |a, b| Value::Bool(a <= b),
                "<=",
            ),
            Expr::Int(i) => Ok(Value::Number(Number::Int(i))),
            Expr::Float(f) => Ok(Value::Number(Number::Float(f))),
            Expr::Bool(b) => Ok(Value::Bool(b)),
            Expr::Literal(l) => Ok(Value::Str(l)),
        }
    }

    fn eval_numeric_op(
        &mut self,
        left: Expr,
        right: Expr,
        fclosure: fn(f64, f64) -> Value,
        iclosure: fn(i64, i64) -> Value,
        op: &str,
    ) -> Result<Value, InterpretError> {
        let left_val = self.eval(left)?;
        let right_val = self.eval(right)?;
        if let (&Value::Number(l), &Value::Number(r)) = (&left_val, &right_val) {
            match (l, r) {
                (Number::Int(a), Number::Int(b)) => Ok(iclosure(a, b)),
                (Number::Float(a), Number::Int(b)) => Ok(fclosure(a, b as f64)),
                (Number::Int(a), Number::Float(b)) => Ok(fclosure(a as f64, b)),
                (Number::Float(a), Number::Float(b)) => Ok(fclosure(a, b)),
            }
        } else {
            Err(InterpretError {
                message: format!(
                    "cannot use operator {} with non-number arguments {:?} or {:?}",
                    op, left_val, right_val
                ),
            })
        }
    }
}
