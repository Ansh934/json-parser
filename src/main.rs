mod lexer;
mod parser;
mod token;
mod error;

use crate::lexer::*;
use crate::parser::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("test.json")?;
    println!("\nContent of test.json:");
    println!("{}", content);

    let lexer = Lexer::tokenize(content.chars())?;
    println!("\nTokens:");
    for token in &lexer.tokens {
        println!("{:?}", token);
    }

    let parser = Parser::parse(lexer.tokens)?;
    println!("\nParsed JSON:");
    println!("{:#?}", parser);
    Ok(())
}


