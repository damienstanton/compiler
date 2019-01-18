//! contains the tokens to be used in the lexer

pub type TokenType = &'static str;

/// a token
pub struct Token {
    pub token_type: TokenType,
    pub literal: char,
}

impl Token {
    /// create a new token
    pub fn new() -> Token {
        Token {
            token_type: EOF,
            literal: '\0',
        }
    }
}

pub const ILLEGAL: &str = "ILLEGAL";
pub const EOF: &str = "EOF";

// Idents
pub const IDENT: &str = "IDENT";
pub const INT: &str = "INT";

// Ops
pub const ASSIGN: &str = "=";
pub const PLUS: &str = "+";

// Delimiters
pub const COMMA: &str = ",";
pub const SEMICOLON: &str = ";";
pub const LPAREN: &str = "(";
pub const RPAREN: &str = ")";
pub const LBRACE: &str = "{";
pub const RBRACE: &str = "}";

// Keywords
pub const FUNCTION: &str = "FUNCTION";
pub const LET: &str = "LET";
