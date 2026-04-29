use sylex::{Lexer, Parser, TokenType};
use sylex::interpreter::Interpreter;

fn main() {
    let source = "1 + 2;\n1 == 1;\n1 >= 2;\n2 / 3.1;\n1.5 * 2;\n5 - 3;\n10 > 5;\n3 < 4;\n1.0 / 2;\n1 / 2;\n";
    let lex = Lexer::new(source);
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
    let res = parser.parse();

    println!("\nErrors:");
    for error in &res.errors {
        println!("{}", error);
    }

    println!("\n\nCode:");
    println!("\n{}", source);

    if res.errors.is_empty() {
        let mut interpreter = Interpreter::new();
        for expr in res.exprs {
            match interpreter.interpret(expr) {
                Ok(val) => println!("Result: {:?}", val),
                Err(e) => println!("Interpretation error: {}", e),
            }
        }
    }
}
