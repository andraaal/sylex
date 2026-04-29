mod builtin;
mod expr;
mod interpreter;
mod lexer;
mod parser;
mod token;
mod value;

use std::io::Write;
use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::token::{Token, TokenType};

fn main() {
    // let test = "print(1 + 2);\nprint(1 == 1);\nprint(1 >= 2);\nprint(2 / 3.1);\nprint(1.5 * 2);\nprint(5 - 3);\nprint(10 > 5);\nprint(3 < 4);\nprint(1.0 / 2);\nprint(1 / 2);\nprint(sin(1.3));\nprint(cos(3.4));\nprint(tan(1.1));";

    let args = std::env::args_os().collect::<Vec<_>>();

    match args.len() {
        1 => {
            // make repl here
            loop {
                print!("> ");
                std::io::stdout().flush().unwrap();
                let mut input = String::new();
                match std::io::stdin().read_line(&mut input) {
                    Ok(_) => exec(&input, true),
                    Err(e) => println!("Error reading input: {}", e),
                }
            }
        }
        2 => {
            let filename = &args[1];
            match std::fs::read_to_string(filename) {
                Ok(contents) => exec(&contents, false),
                Err(e) => println!("Error reading file {}: {}", filename.to_string_lossy(), e),
            }
        }
        _ => {
            println!("Usage: {} [filename]", args[0].to_string_lossy());
            println!("If no filename is provided, REPL will start");
        }
    };


}

fn exec(input: &str, repl: bool) {
    let lex = Lexer::new(input);
    let tokens = lex.lex();
    tokens.iter().for_each(|token| match token.token_type {
        TokenType::Invalid(ref msg) => println!("Invalid token at {}: {}", token.start, msg),
        _ => {}
    });
    let filtered = tokens
        .into_iter()
        .filter(|result| match result.token_type {
            TokenType::Comment(_) | TokenType::Invalid(_) => false,
            _ => true,
        })
        .collect::<Vec<_>>();
    let parser = Parser::new(filtered);
    let res = parser.parse(!repl);

    if !res.errors.is_empty() {
        println!("\nErrors:");
        for error in &res.errors {
            println!("{}", error);
        }
    }

    if res.errors.is_empty() {
        let mut interpreter = Interpreter::new();
        for expr in res.exprs {
            match interpreter.interpret(expr) {
                Ok(val) => {
                    if repl {
                        println!("{}", val)
                    }
                }
                Err(e) => println!("Interpretation error: {}", e),
            }
        }
    }
}
