use std::io::Write;
use crate::interpreter::InterpretError;
use crate::value::{Number, Value};

pub fn call_builtin(name: String, args: Vec<Value>) -> Result<Value, InterpretError> {
    match name.as_str() {
        "print" => {
            for arg in args {
                print!("{} ", arg);
            }
            println!();
            std::io::stdout().flush().unwrap();
            Ok(Value::Bool(true)) // Return true for successful print, since there is no null (yet)
        }
        "sin" => {
            if args.len() != 1 {
                return Err(arg_len_error(1, args.len()));
            }
            match args[0] {
                Value::Number(Number::Int(num)) => {
                    Ok(Value::Number(Number::Float((num as f64).sin())))
                }
                Value::Number(Number::Float(num)) => Ok(Value::Number(Number::Float(num.sin()))),
                _ => {
                    Err(arg_type_error(
                        "Int or Float",
                        &vec![&args[0]],
                    ))
                }
            }
        }
        "cos" => {
            if args.len() != 1 {
                return Err(arg_len_error(1, args.len()));
            }
            match args[0] {
                Value::Number(Number::Int(num)) => {
                    Ok(Value::Number(Number::Float((num as f64).cos())))
                }
                Value::Number(Number::Float(num)) => Ok(Value::Number(Number::Float(num.cos()))),
                _ => {
                    Err(arg_type_error(
                        "Int or Float",
                        &vec![&args[0]],
                    ))
                }
            }
        }
        "tan" => {
            if args.len() != 1 {
                return Err(arg_len_error(1, args.len()));
            }
            match args[0] {
                Value::Number(Number::Int(num)) => {
                    Ok(Value::Number(Number::Float((num as f64).tan())))
                }
                Value::Number(Number::Float(num)) => Ok(Value::Number(Number::Float(num.tan()))),
                _ => {
                    Err(arg_type_error(
                        "Int or Float",
                        &vec![&args[0]],
                    ))
                }
            }
        }
        "exit" => {
            std::process::exit(0);
        }
        _ => Err(InterpretError {
            message: format!("unknown builtin {}", name),
        }),
    }
}

fn arg_len_error(expected: usize, actual: usize) -> InterpretError {
    InterpretError {
        message: format!("expected {} arguments, got {}", expected, actual),
    }
}

fn arg_type_error(expected: &str, actual: &Vec<&Value>) -> InterpretError {
    InterpretError {
        message: format!("expected arguments of type {:?}, got {:?}", expected, actual),
    }
}