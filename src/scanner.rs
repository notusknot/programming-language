use crate::tokenizer::{Token, TokenType, TokenType::*};
use std::collections::HashMap;
use std::string::String;

pub struct Scanner {
    source: String,
    keywords: HashMap<String, TokenType>,
    start: usize,
    current: usize,
    line: usize,
}

impl Iterator for Scanner {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.scan_token()
    }
}

impl Scanner {
    pub fn new(source: String) -> Self {
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

        Self {
            source,
            keywords,
            current: 0,
            start: 0,
            line: 0,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let character = self.peek();
        self.current += 1;
        character
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        // this unwrap is safe because reaching None means the job is done
        self.source.as_str().chars().nth(self.current).unwrap()
    }

    fn peek_two(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        // this unwrap is safe because reaching None means the job is done
        self.source.as_str().chars().nth(self.current + 1).unwrap()
    }

    fn make_token(&self, token_type: TokenType) -> Option<Token> {
        let text = &self.source[(self.start)..(self.current)];
        Some(Token {
            token_type,
            lexeme: text.to_string(),
            literal: None,
            line: self.line,
        })
    }

    pub fn scan_token(&mut self) -> Option<Token> {
        match self.advance() {
            '\"' => self.string(), // string literals
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' => self.identifier(),
            ' ' | '\r' | '\t' => None, // Ignore whitespace
            '(' => self.make_token(LeftParen),
            ')' => self.make_token(RightParen),
            '{' => self.make_token(LeftBrace),
            '}' => self.make_token(RightBrace),
            ',' => self.make_token(Comma),
            '.' => self.make_token(Dot),
            '-' => self.make_token(Minus),
            '+' => self.make_token(Plus),
            ';' => self.make_token(Semicolon),
            '*' => self.make_token(Star),
            '!' => {
                if self.match_next('=') {
                    self.make_token(BangEqual)
                } else {
                    self.make_token(Bang)
                }
            }
            '=' => {
                if self.match_next('=') {
                    self.make_token(EqualEqual)
                } else {
                    self.make_token(Equal)
                }
            }
            '<' => {
                if self.match_next('=') {
                    self.make_token(LessEqual)
                } else {
                    self.make_token(Less)
                }
            }
            '>' => {
                if self.match_next('=') {
                    self.make_token(GreaterEqual)
                } else {
                    self.make_token(Greater)
                }
            }
            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    None
                } else {
                    self.make_token(Slash)
                }
            }

            '\n' => {
                self.line += 1;
                None
            }
            _ => None,
        }
    }

    fn string(&mut self) -> Option<Token> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            panic!("unterminated string");
        }

        // the closing "
        self.advance();

        let value = &self.source[self.start + 1..self.current - 1];
        self.make_token(StringLiteral(value.to_string()))
    }

    fn number(&mut self) -> Option<Token> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_two().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let number_text = &self.source[self.start..self.current];
        let value: f32 = number_text.parse().expect("Failed to parse number");
        self.make_token(Number(value))
    }

    fn identifier(&mut self) -> Option<Token> {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let text = &self.source.as_str()[self.start..self.current];
        let mut token_type: Option<&TokenType> = self.keywords.get(text);

        if token_type.is_none() {
            token_type = Some(&Identifier);
        }

        self.make_token(token_type.expect("Failed to recognize token type").clone())
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
