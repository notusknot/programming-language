use crate::throw_error;
use crate::tokenizer::TokenType::*;
use crate::tokenizer::*;
use std::collections::HashMap;
use std::string::String;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    keywords: HashMap<String, TokenType>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        let mut keywords = HashMap::new();
        keywords.insert("and".to_string(), And);
        keywords.insert("class".to_string(), Class);
        keywords.insert("else".to_string(), Else);
        keywords.insert("false".to_string(), False);
        keywords.insert("for".to_string(), For);
        keywords.insert("fun".to_string(), Fun);
        keywords.insert("if".to_string(), If);
        keywords.insert("nil".to_string(), Nil);
        keywords.insert("or".to_string(), Or);
        keywords.insert("print".to_string(), Print);
        keywords.insert("return".to_string(), Return);
        keywords.insert("super".to_string(), Super);
        keywords.insert("this".to_string(), This);
        keywords.insert("true".to_string(), True);
        keywords.insert("var".to_string(), Var);
        keywords.insert("while".to_string(), While);

        Scanner {
            source,
            tokens: vec![],
            keywords,
            current: 0,
            start: 0,
            line: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: Eof,
            lexeme: "".to_string(),
            literal: None,
            line: self.line,
        });
        self.tokens.clone()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let character = self.peek();
        self.current = self.current + 1;
        character
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        let c = self.source.as_str().chars().nth(self.current).unwrap();
        c
    }

    fn peek_two(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        let c = self.source.as_str().chars().nth(self.current + 1).unwrap();
        c
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = &self.source[(self.start)..(self.current)];
        self.tokens.push(Token {
            token_type,
            lexeme: text.to_string(),
            literal: None,
            line: self.line,
        });
    }

    pub fn scan_token(&mut self) {
        let c = self.advance();

        match c {
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

            '\"' => self.string(),

            '0'..='9' => self.number(),

            'a'..='z' | 'A'..='Z' => self.identifier(),

            ' ' | '\r' | '\t' => (), // Ignore whitespace

            '\n' => self.line = self.line + 1,

            _ => throw_error(self.line, "Unexpected character"),
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.advance();
        }

        if self.is_at_end() {
            throw_error(self.line, "Unterminated string");
            return;
        }

        // the closing "
        self.advance();

        // immutable borrow
        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token(StringLiteral(value.to_string()));
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_two().is_digit(10) {
            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let number_text = &self.source[self.start..self.current];
        let value: f32 = number_text.parse().expect("Failed to parse number");
        self.add_token(Number(value));
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let text = &self.source.as_str()[self.start..self.current];
        let mut token_type: Option<&TokenType> = self.keywords.get(text);

        if token_type.is_none() {
            token_type = Some(&Identifier);
        }

        self.add_token(token_type.unwrap().clone());
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
