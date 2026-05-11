mod lexer;
mod parser;
mod token;
mod error;

use crate::lexer::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("test.json")?;
    println!("\nContent of test.json:");
    println!("{}", content);

    let lexer = Lexer::tokenize(content.chars())?;
    println!("\nTokens:");
    for token in lexer.tokens {
        println!("{:?}", token);
    }
    Ok(())
}


