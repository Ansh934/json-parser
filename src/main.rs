mod lexer;
mod parser;
mod token;
mod error;

use std::env;
use crate::lexer::*;
use crate::parser::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = env::args().nth(1).unwrap_or_else(|| "default_test.json".into());
    println!("\nReading file: {}", file_path);
    let content = std::fs::read_to_string(&file_path)?;
    println!("Content of {}: \n{}", file_path, content);

    let lexer = Lexer::tokenize(content.chars())?;
    println!("\nTokens:");
    for token in &lexer.tokens {
        print!("{:?} | ", token);
    }
    println!();

    let parser = Parser::parse(lexer.tokens)?;
    println!("\nParsed JSON:");
    println!("{:#?}", parser);
    Ok(())
}


