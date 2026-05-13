#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Token {
    // Structural
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Colon,        // :
    Comma,        // ,
    
    // Primitives
    Str(String),
    Num(f64),
    True,
    False,
    Null,
}
