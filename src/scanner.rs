use crate::throw_error;
use crate::tokenizer::TokenType::*;
use crate::tokenizer::*;
use std::string::String;

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: i32,
    current: i32,
    line: i32,
}

impl Scanner {
    fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            let start = self.current;
            self.scan_tokens();
        }

        self.tokens.push(Token {
            token_type: Eof,
            lexeme: "".to_string(),
            line: self.line,
        });
        self.tokens.clone()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as i32
    }

    fn advance(&self) -> char {
        let pos: usize = (self.current + 1).try_into().unwrap();
        self.source.chars().nth(pos).unwrap()
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        let c = self
            .source
            .as_str()
            .chars()
            .nth(self.current as usize)
            .unwrap();
        c
    }

    fn add_token(&mut self, token_type: TokenType) {
        //TODO there has to be a better way to do this
        let text: String = self
            .source
            .chars()
            .skip(self.start as usize)
            .take((self.current - self.start) as usize)
            .collect();

        self.tokens.push(Token {
            token_type,
            lexeme: text,
            line: self.line,
        })
    }

    fn scan_token(&mut self) {
        let character = self.advance();
        match character {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            '-' => self.add_token(Minus),
            '+' => self.add_token(Plus),
            ';' => self.add_token(Semicolon),
            '*' => self.add_token(Star),
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '!' => {
                if self.match_next('=') {
                    self.add_token(BangEqual)
                } else {
                    self.add_token(Bang)
                }
            }
            '=' => {
                if self.match_next('=') {
                    self.add_token(EqualEqual)
                } else {
                    self.add_token(Equal)
                }
            }
            '<' => {
                if self.match_next('=') {
                    self.add_token(LessEqual)
                } else {
                    self.add_token(Less)
                }
            }
            '>' => {
                if self.match_next('=') {
                    self.add_token(GreaterEqual)
                } else {
                    self.add_token(Greater)
                }
            }
            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(Slash);
                }
            }
            _ => throw_error(self.line, "Unexpected character"),
        }
    }

    fn match_next(&mut self, expected: char) -> bool {
        if !self.is_at_end() && self.peek() == expected {
            self.advance();
            true
        } else {
            false
        }
    }
}
