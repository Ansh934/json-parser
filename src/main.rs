mod lexer;
mod parser;
mod token;

use crate::lexer::*;
fn main() {
    let content = std::fs::read_to_string("test.json").unwrap();
    println!("Content of test.json:");
    println!("{}\n", content);
    let lexer = Lexer::tokenize(content.chars());
    println!("Tokens:");
    for token in lexer.tokens {
        println!("{:?}", token);
    }
}


