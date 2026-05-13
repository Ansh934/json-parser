mod constants;
mod error;
mod lexer;
mod parser;
mod token;

use crate::lexer::*;
use crate::parser::*;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = env::args()
        .nth(1)
        .unwrap_or_else(|| "tests/default_test.json".into());
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

// TODO
// - [o] Handle escape characters in strings
// - [o] Handle numbers better
// - [o] Handle errors better instead of panicking
// - [o] Add more test cases for edge cases and error cases
// - [o] Refactor Constants
// - [x] Add SourceLocation to tokens for better error reporting
// - [ ] Implement Iterator for Lexer so we can iterate over tokens
// - [ ] Implement a pretty printer for the parsed JSON/ Tree structure GUI