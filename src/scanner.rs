use crate::tokenizer::{KeywordType, KeywordType::*, Span, Token, TokenType, TokenType::*};
use std::collections::HashMap;
use std::str::Chars;

pub struct Cursor<'input> {
    chars: Chars<'input>,
    byte_pos: usize,
}

impl<'input> Cursor<'input> {
    pub fn advance(&mut self) -> Option<char> {
        // Bump the character iterator
        let c = self.chars.next();
        // Bump the byte position
        self.byte_pos += c.map(char::len_utf8).unwrap_or_default();
        c
    }

    pub fn peek(&self) -> Option<char> {
        // Cloning a [`Chars`] iterator is cheap.
        self.chars.clone().next()
    }

    pub fn peek_two(&self) -> Option<char> {
        let mut cloned = self.chars.clone();
        cloned.next();
        cloned.next()
    }

    pub fn skip_while(&mut self, predicate: fn(char) -> bool) {
        // Record the remaining input bytes before skipping
        let start_length = self.chars.as_str().len();
        while matches!(self.peek(), Some(c) if predicate(c)) {
            // Notice how this doesn't call [`Cursor::next`] directly.
            // This way we can batch the byte_pos update.
            self.chars.next();
        }
        let final_length = self.chars.as_str().len();
        self.byte_pos += start_length - final_length;
    }

    fn advance_if(&mut self, expected: char) -> bool {
        if self.peek().unwrap_or_default() == expected {
            self.advance();
            true
        } else {
            false
        }
    }
}

pub struct Scanner<'input> {
    source: &'input str,
    keywords: HashMap<&'input str, KeywordType>,
    cursor: Cursor<'input>,
}

impl<'input> Iterator for Scanner<'input> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.scan_token()
    }
}

impl<'input> Scanner<'input> {
    pub fn new(source: &'input str) -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("and", And);
        keywords.insert("class", Class);
        keywords.insert("else", Else);
        keywords.insert("false", False);
        keywords.insert("for", For);
        keywords.insert("fun", Fun);
        keywords.insert("if", If);
        keywords.insert("nil", Nil);
        keywords.insert("or", Or);
        keywords.insert("print", Print);
        keywords.insert("return", Return);
        keywords.insert("super", Super);
        keywords.insert("this", This);
        keywords.insert("true", True);
        keywords.insert("var", Var);
        keywords.insert("while", While);

        Self {
            source,
            keywords,
            cursor: Cursor {
                chars: source.chars(),
                byte_pos: 0,
            },
        }
    }

    pub fn scan_token(&mut self) -> Option<Token> {
        let start = self.cursor.byte_pos;
        let token_type = match self.cursor.advance()? {
            '"' => self.string()?, // string literals
            '0'..='9' => self.number()?,
            'a'..='z' | 'A'..='Z' => self.identifier(start),
            c if c.is_whitespace() => self.whitespace(),
            '(' => LeftParen,
            ')' => RightParen,
            '{' => LeftBrace,
            '}' => RightBrace,
            ',' => Comma,
            '.' => Dot,
            '-' => Minus,
            '+' => Plus,
            ';' => Semicolon,
            '*' => Star,
            '!' => {
                if self.cursor.advance_if('=') {
                    BangEqual
                } else {
                    Bang
                }
            }
            '=' => {
                if self.cursor.advance_if('=') {
                    EqualEqual
                } else {
                    Equal
                }
            }
            '<' => {
                if self.cursor.advance_if('=') {
                    LessEqual
                } else {
                    Less
                }
            }
            '>' => {
                if self.cursor.advance_if('=') {
                    GreaterEqual
                } else {
                    Greater
                }
            }

            '/' => {
                if self.cursor.advance_if('/') {
                    while self.cursor.peek()? != '\n' {
                        self.cursor.advance()?;
                    }
                    None?
                } else {
                    Slash
                }
            }
            '\n' => None?,

            _ => Unknown,
        };
        let span = Span::from(start..self.cursor.byte_pos);

        Some(Token { token_type, span })
    }

    fn string(&mut self) -> Option<TokenType> {
        while self.cursor.peek()? != '"' {
            self.cursor.advance();
        }

        /*
        if self.cursor.is_at_end() {
            panic!("unterminated string");
        }
                */

        // the closing "
        self.cursor.advance();

        Some(StringLiteral)
    }

    fn number(&mut self) -> Option<TokenType> {
        while self.cursor.peek()?.is_ascii_digit() {
            self.cursor.advance();
        }

        if self.cursor.peek()? == '.' && self.cursor.peek_two()?.is_ascii_digit() {
            self.cursor.advance();

            while self.cursor.peek()?.is_ascii_digit() {
                self.cursor.advance();
            }
        }

        let number_text = &self.source[0..self.cursor.byte_pos];
        let value: f32 = number_text.parse().expect("Failed to parse number");
        Some(Number(value))
    }

    fn identifier(&mut self, start: usize) -> TokenType {
        self.cursor.skip_while(char::is_alphanumeric);

        let text = &self.source[start..self.cursor.byte_pos];

        match self.keywords.get(text).copied() {
            Some(keyword) => Keyword(keyword),
            None => Identifier,
        }
    }

    fn whitespace(&mut self) -> TokenType {
        self.cursor.skip_while(char::is_whitespace);
        Whitespace
    }
}
