//! the lexer
use crate::token::*;

/// the main lexer structure
pub struct Lexer {
    pub input: &'static str,
    pub pos: usize,
    pub read_pos: usize,
    pub ch: char,
}

impl Lexer {
    /// create a new lexer from the given input
    pub fn new(input: &'static str) -> Lexer {
        Lexer {
            input,
            pos: 0,
            read_pos: 0,
            ch: '\0',
        }
    }

    /// reads the next token
    pub fn next_token(&mut self) -> Token {
        self.read_char();
        match self.ch {
            '=' => Token {
                token_type: ASSIGN,
                literal: self.ch,
            },
            ';' => Token {
                token_type: SEMICOLON,
                literal: self.ch,
            },
            '(' => Token {
                token_type: LPAREN,
                literal: self.ch,
            },
            ')' => Token {
                token_type: RPAREN,
                literal: self.ch,
            },
            ',' => Token {
                token_type: COMMA,
                literal: self.ch,
            },
            '+' => Token {
                token_type: PLUS,
                literal: self.ch,
            },
            '{' => Token {
                token_type: LBRACE,
                literal: self.ch,
            },
            '}' => Token {
                token_type: RBRACE,
                literal: self.ch,
            },
            _ => Token {
                token_type: EOF,
                literal: '\0',
            },
        }
    }

    /// reads the chatacter position
    pub fn read_char(&mut self) {
        if self.read_pos >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_pos).unwrap();
        }
        self.pos = self.read_pos;
        self.read_pos += 1;
    }
}
