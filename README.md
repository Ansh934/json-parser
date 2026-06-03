# JSON Parser in Rust

A simple, lightweight, from-scratch JSON parser written in Rust. This project reads JSON files, tokenizes the input strings, and parses them into an Abstract Syntax Tree (AST) representation using custom tokens.

## Features

- **Lexical Analysis (Lexer):** Tokenizes incoming streams of characters into logical JSON elements (Strings, Numbers, Booleans, Punctuation, Null).
- **Parsing (Parser):** Validates and constructs an in-memory representation of the JSON data structure. Handles nested Objects and Arrays.
- **Robust Error Handling:** Uses `thiserror` to provide descriptive errors indicating where parsing or syntax failures happened.
- **Order Preservation:** Uses `indexmap` internally to retain the insertion order of object keys, making debugging and structured output easier.

## Project Structure

- `src/main.rs`: The entry point that reads a file, tokenizes it, and runs the parsing logic.
- `src/lexer.rs`: Contains the `Lexer` implementation to convert raw text into `Token` variants.
- `src/parser.rs`: Contains the `Parser` implementation to construct the parsed object tree.
- `src/token.rs`: Defines the `Token` enums.
- `src/error.rs`: Custom error definitions (Lexer and Parser errors).
- `src/constants.rs`: Helpful generic constants for parsing (e.g., punctuation marks like `{`, `}`, `[`, `]`).

## Getting Started

### Prerequisites

You need Rust and Cargo installed. If you haven't installed them, follow the instructions at [rustup.rs](https://rustup.rs/).

### Running the Parser

By default, the program looks for a file passed as the first argument. If no arguments are passed, it defaults to evaluating `tests/default_test.json`.

```bash
# Run with the default test file
cargo run

# Run with a specific JSON file
cargo run -- tests/extreme_test.json
```

### Example Output

Given a file like:
```json
{
  "key": "value",
  "number": 123
}
```

The output will display:
1. The raw file content
2. The list of generated tokens
3. The beautifully formatted, parsed representation of the JSON tree

## Development and TODOs

- Handle escape characters in strings
- Handle complex numbers better
- Handle errors better instead of panicking
- Add more test cases for edge cases and error cases
- Refactor Constants
- Iterator implementation for Lexer (in progress!)

## Dependencies

- **[indexmap](https://crates.io/crates/indexmap):** A hash table with consistent order. Used over `HashMap` to retain JSON key order.
- **[thiserror](https://crates.io/crates/thiserror):** A popular crate for handling and deriving `Error` types cleanly in Rust.

## License

This project is Public domain and available under the 'Unlicense' License.
