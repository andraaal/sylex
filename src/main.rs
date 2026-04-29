use sylex::{Lexer, Parser, TokenType};

fn main() {
    let source = "1 + 2 / 3 - 4 * 1.1;";
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
    println!("Tokens:");
    for expr in &res.exprs {
        println!("{:?}", expr);
    }
    println!("\nErrors:");
    for error in &res.errors {
        println!("{}", error);
    }
}
