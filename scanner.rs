// #[path = "token.rs"]
// mod token;
#![allow(dead_code)]
#![allow(unused_imports)]
use crate::lox;
use crate::token;
use std::str::FromStr;
use token::{Literal, Token, TokenTemp, TokenType};

use std::collections::HashMap;

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = [
        ("and", TokenType::AND),
        ("class", TokenType::CLASS),
        ("else", TokenType::ELSE),
        ("false", TokenType::FALSE),
        ("for", TokenType::FOR),
        ("fun", TokenType::FUN),
        ("if", TokenType::IF),
        ("nil", TokenType::NIL),
        ("or", TokenType::OR),
        ("print", TokenType::PRINT),
        ("return", TokenType::RETURN),
        ("super", TokenType::SUPER),
        ("this", TokenType::THIS),
        ("true", TokenType::TRUE),
        ("var", TokenType::VAR),
        ("while", TokenType::WHILE),
    ]
    .iter()
    .cloned()
    .collect();
}

pub struct Scanner {
    // pub source: Vec<char>,
    source: String,
    tokens: Vec<Token>,
    start: u32,
    current: u32,
    line: u32,
}

impl Scanner {
    pub fn new(input: String) -> Scanner {
        Scanner {
            source: input,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn get_tokens(self) -> Vec<Token> {
        self.tokens.clone()
    }

    pub fn scan_tokens(&mut self) {
        // println!("In scan_tokens");
        // while !isAtEnd(self.current, self.source.len()) {
        while !self.is_at_end() {
            // println!("At start of scan_tokens");
            self.start = self.current;
            self.scan_token();
        }
        let t = Token {
            token_type: TokenType::EOF,
            lexeme: String::new(),
            literal: None,
            line: self.line,
        };
        // println!("Adding final token: {}", t);
        // Add an end of line token to help make it cleaner!
        // self.tokens.push(Token {
        //     token_type: TokenType::EOF,
        //     lexeme: String::new(),
        //     literal: None,
        //     line: self.line,
        // });
        self.tokens.push(t);
    }

    fn is_at_end(&mut self) -> bool {
        self.current >= self.source.len() as u32
    }

    fn scan_token(&mut self) {
        // println!("In scan_token");
        let c: char = self.advance();
        match c {
            '(' => self.add_token(TokenType::LEFTPAREN, None),
            ')' => self.add_token(TokenType::RIGHTPAREN, None),
            '{' => self.add_token(TokenType::LEFTBRACE, None),
            '}' => self.add_token(TokenType::RIGHTBRACE, None),
            ',' => self.add_token(TokenType::COMMA, None),
            '.' => self.add_token(TokenType::DOT, None),
            '-' => self.add_token(TokenType::MINUS, None),
            '+' => self.add_token(TokenType::PLUS, None),
            ';' => self.add_token(TokenType::SEMICOLON, None),
            '*' => self.add_token(TokenType::STAR, None),
            '!' => {
                let t_t = if self.match_c('=') {
                    TokenType::BANGEQUAL
                } else {
                    TokenType::BANG
                };
                self.add_token(t_t, None);
            }
            '=' => {
                let t_t = if self.match_c('=') {
                    TokenType::EQUALEQUAL
                } else {
                    TokenType::EQUAL
                };
                self.add_token(t_t, None);
            }
            '<' => {
                let t_t = if self.match_c('=') {
                    TokenType::LESSEQUAL
                } else {
                    TokenType::LESS
                };
                self.add_token(t_t, None);
            }
            '>' => {
                let t_t = if self.match_c('=') {
                    TokenType::GREATEREQUAL
                } else {
                    TokenType::GREATER
                };
                self.add_token(t_t, None);
            }
            '/' => {
                if self.match_c('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH, None);
                }
            }
            ' ' => (),
            '\r' => (),
            '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string(),
            c if is_digit(c) => self.number(),
            c if is_alpha(c) => self.identifier(),
            _ => lox::error(self.line, "Unexpected character".to_string()),
        }
    }

    fn number(&mut self) {
        println!("In number");
        while is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && is_digit(self.peek_two()) {
            self.advance();

            while is_digit(self.peek()) {
                self.advance();
            }
        }
        let num = match self.source.get(self.start as usize..self.current as usize) {
            Some(i) => i,
            None => panic!("Panicked in number, when calling .get() with:\nself.source: {} self.start: {}, self.current, {}.", self.source, self.start as usize, self.current as usize)
        };

        if let Ok(i) = num.parse::<i32>() {
            self.add_token(TokenType::NUMBER, Some(Literal::Number(i)));
        } else if let Ok(i) = num.parse::<f32>() {
            self.add_token(TokenType::NUMBER, Some(Literal::Decimal(i)));
        } else {
            panic!("Got a none number when parsing in number. Num: {}", num);
        }
    }

    fn advance(&mut self) -> char {
        // println!("In advance");
        self.current += 1;
        //self.source[(self.current - 1)]
        let c = match self.source.get((self.current-1) as usize..self.current as usize) {
                Some(i) => i,
                None => panic!("Panicked in advance, when doing .get() with:\nself.source: {} self.current-1: {}, self.current, {}.", self.source, (self.current - 1) as usize, self.current as usize)
            };
        match char::from_str(c) {
            Ok(i) => i,
            Err(e) => panic!("Panicked in advance, when doing ..from_str with:\nself.source: {} self.current-1: {}, self.current, {}. Causing error: {}", self.source, (self.current - 1) as usize, self.current as usize, e)
        }
        // self.source.remove((self.current - 1) as usize)
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        // println!("In add_token");
        // let substring = &self.source[self.start as usize..self.current as usize];

        //let substring = &self.source[self.start as usize..self.current as usize];
        //let substring = self.sub_string(self.start, self.current);
        let s_string = match self
            .source
            .get(self.start as usize..self.current as usize) {
                Some(i) => i,
                None => panic!("Panicked in add_token, when doing .get() with:\nself.source: {} self.start: {}, self.current, {}.", self.source, self.start as usize, self.current as usize)
            };

        self.tokens.push(Token {
            token_type,
            lexeme: s_string.to_string(),
            literal,
            line: self.line,
        });
    }

    fn match_c(&mut self, c: char) -> bool {
        // println!("In match_c");
        // used for operators that have to have `c` as the char before it, like ==. The second = needs the first one.
        if self.is_at_end() {
            return false;
        }
        // if self.source.chars().nth(self.current as usize).unwrap() != c {
        //     return false;
        // }
        // if self.char_at(self.current).unwrap() != c {
        //     return false;
        // }
        // let temp = match self.char_at(self.current) {
        //     Some(i) => i,
        //     None => panic!("Panicked in match_c, index out of range mostlikely!"),
        // };
        let temp = match self.source.get(self.current as usize.. (self.current + 1) as usize){
             Some(i) => {
                //char::from_str(i).unwrap()
                match char::from_str(i) {
                    Ok(c) => c,
                    Err(e) => panic!("Panicked in match_c, when doing ..from_str with:\nself.source: {} self.current: {}, self.current + 1, {}.\n caused error: {}", self.source, self.current as usize, (self.current + 1) as usize, e)
                }
            },
            None => panic!("Panicked in match_c, when doing .get() with:\nself.source: {} self.current: {}, self.current + 1, {}.", self.source, self.current as usize, (self.current + 1) as usize),
        };

        if temp != c {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&mut self) -> char {
        // println!("In peek");
        if self.is_at_end() {
            return '\0';
        }
        // self.source.chars().nth(self.current as usize).unwrap()
        // match self.char_at(self.current) {
        //     Some(i) => i,
        //     None => panic!("Panicked in peak, index out of range mostlikely!"),
        // }
        match self.source.get(self.current as usize..(self.current +1)as usize){
             Some(i) => {match char::from_str(i) {
                    Ok(c) => c,
                    Err(e) => panic!("Panicked in peek, when doing .get() with:\nself.source: {} self.current: {}, self.current + 1, {}.\n caused error: {}", self.source, self.current as usize, (self.current + 1) as usize, e)
                }},
            None => panic!("Panicked in peek, when doing .get() with:\nself.source: {} self.current: {}, self.current + 1, {}.", self.source, self.current as usize, (self.current + 1) as usize),
        }
    }

    fn peek_two(&mut self) -> char {
        // println!("In peek_two");
        if self.current + 1 >= self.source.len() as u32 {
            return '\0';
        }

        // match self
        //     .source
        //     .get(self.current as usize..(self.current + 1) as usize)
        // {
        //     Some(i) => char::from_str(i).unwrap(),
        //     None => panic!("Panicked in peak_two, index out of range mostlikely!"),
        // }
        match self.source.get(self.current as usize..(self.current +1)as usize){
             Some(i) => {match char::from_str(i) {
                    Ok(c) => c,
                    Err(e) => panic!("Panicked in peek_two, when doing .from_str with:\nself.source: {} self.current: {}, self.current + 1, {}.\n caused error: {}", self.source, self.current as usize, (self.current + 1) as usize, e)
                }},
            None => panic!("Panicked in peek_two, when doing .get() with:\nself.source: {} self.current: {}, self.current + 1, {}.", self.source, self.current as usize, (self.current + 1) as usize),
        }
        // match self.char_at(self.current + 1) {
        // Some(i) => i,
        // None => panic!("Panicked in peak_two, index out of range mostlikely!"),
        // }
        // self.source
        //     .chars()
        //     .nth((self.current + 1) as usize)
        //     .unwrap()
    }

    fn string(&mut self) {
        // println!("In string");
        // Go until the line ends and the closing " is reached
        while self.peek() != '"' && self.is_at_end() {
            // Go to next line if " hasn't been reached yet
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        // Missing the closing "
        if self.is_at_end() {
            lox::error(self.line, "Unterminated string.".to_string());
        }
        // " has been reached, so go to its index (since the loop checked when the next char was ")
        self.advance();
        // Gets the value surronded by the quotes
        //let value = self.sub_string(self.start + 1, self.current - 1);
        // let value = self
        //     .source
        //     .get((self.start + 1) as usize..(self.current - 1) as usize)
        //     .unwrap()
        //     .to_string();
        let value = match self.source.get((self.start + 1) as usize..(self.current - 1) as usize) {
            Some(i) => i.to_string(),
            None => panic!("Panicked in string, when doing .get() with:\nself.source: {} self.start + 1: {}, self.current - 1, {}.",self.source, (self.start +1) as usize, (self.current -1 ) as usize)
        };
        //&self.source[(self.start + 1) as usize..(self.current - 1) as usize].to_string();
        self.add_token(TokenType::STRING, Some(Literal::Letters(value)));
    }

    fn identifier(&mut self) { 
        println!("In identifier");
        while is_alpha_numeric(self.peek()) {
            self.advance();
        }
        // let text: String = self.sub_string(self.start, self.current);
        let temp = self.source.clone();
        // let text = temp
        //     .get((self.start + 1) as usize..(self.current - 1) as usize)
        //     .unwrap();
        let text = match temp.get((self.start + 1) as usize..(self.current - 1) as usize) {
            Some(i) => i,
            None => panic!("Panicked in string, when doing .get() with:\nself.source: {} self.start + 1: {}, self.current - 1, {}.",self.source, (self.start +1) as usize, (self.current -1 ) as usize)
        };
        let t_t = KEYWORDS.get(&text);

        match t_t {
            Some(i) => self.add_token(i.clone(), None),
            None => self.add_token(TokenType::IDENTIFIER, None),
        }
    }

    // fn get(&mut self, start: u32, end: u32)  -> &str{
    //     match self.source.get(start as usize, end as usize) {
    //         Some(i) => i,
    //         None => panic!("Panicked in peek_two, when doing .get() with:\nself.source: {} self.current: {}, self.current + 1, {}.", self.source, self.current as usize, (self.current + 1) as usize),
    //     }
    // }

    // fn sub_string(&mut self, start: u32, end: u32) -> String {
    //     println!("In sub_string");
    //     match Result(self.source[start as usize..end as usize]) {
    //         Ok(i) => i.to_string(),
    //         Err(e) => panic!("Tried get_tokensting substring! Source: {} Start:{} End: {}", self.source, start, end),
    //     }
    //     // self.source[start as usize..end as usize].to_string()
    // }

    // fn char_at(&mut self, n: u32) -> Option<char> {
    //     println!("In char_at");
    //     self.source.chars().nth(n as usize)
    // }
}
// fn isAtEnd(current: u32, src_len: usize) -> bool{
//         current >= src_len as u32
// }
fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_alpha(c: char) -> bool {
    c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '_'
}

fn is_alpha_numeric(c: char) -> bool {
    is_digit(c) || is_alpha(c)
}
