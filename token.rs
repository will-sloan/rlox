#![allow(dead_code)]
use std::fmt;
#[derive(Debug, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LEFTPAREN,
    RIGHTPAREN,
    LEFTBRACE,
    RIGHTBRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,
    // One or two character tokens.
    BANG,
    BANGEQUAL,
    EQUAL,
    EQUALEQUAL,
    GREATER,
    GREATEREQUAL,
    LESS,
    LESSEQUAL,
    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,
    // Keywords
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Number(i32),
    Decimal(f32),
    Letters(String),
}

#[derive(Debug)]
pub struct TokenTemp {
    pub input: char,
}
#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: u32,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} + {} + {:?}",
            self.token_type, self.lexeme, self.literal
        )
    }
}

/*
In scan_tokens
In scan_token
In advance
In identifier
In peek
In advance
In peek
In advance
In peek
In add_token
thread 'main' panicked at 'Panicked in add_token, when doing .get() with:
self.source: rn self.start: 0, self.current, 3.', src/scanner.rs:195:25
*/
