use sylex::{Lexer, Parser};

fn main() {
    let source = "1 + 2 / 3 - 4 * 1.1;";
    let lex = Lexer::new(source);
    let tokens = lex.lex();
    let parser = Parser::new(tokens);
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
