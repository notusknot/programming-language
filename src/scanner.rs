use crate::tokenizer::{
    KeywordType,
    KeywordType::{And, Class, Else, For, Fun, If, Or, Print, Return, Super, This, Var, While},
    Span, Token, TokenType,
    TokenType::{
        Bang, BangEqual, Comma, Comment, Dot, Equal, EqualEqual, Greater, GreaterEqual, Identifier,
        Keyword, LeftBrace, LeftParen, Less, LessEqual, Minus, Number, Plus, RightBrace,
        RightParen, Semicolon, Slash, Star, StringLiteral, Unknown, Whitespace,
    },
};
use std::str::Chars;

#[derive(Debug)]
pub struct Cursor<'source> {
    chars: Chars<'source>,
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
        if self.peek().unwrap() == expected {
            self.advance();
            true
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct Scanner<'input> {
    source: &'input str,
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
        Self {
            source,
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
            '0'..='9' => self.number(start)?,
            'a'..='z' | 'A'..='Z' => self.identifier_or_keyword(start),
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
                match self.cursor.peek() {
                    Some('/') => {
                        self.cursor.skip_while(|c| c != '\n'); // Comment ends at the end of line
                        Comment
                    }
                    _ => Slash,
                }
            }

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

    fn number(&mut self, _start: usize) -> Option<TokenType> {
        while self.cursor.peek()?.is_ascii_digit() {
            self.cursor.advance();
        }

        if self.cursor.peek()? == '.' && self.cursor.peek_two()?.is_ascii_digit() {
            self.cursor.advance();

            while self.cursor.peek()?.is_ascii_digit() {
                self.cursor.advance();
            }
        }

        //let number_text = &self.source[start..self.cursor.byte_pos];
        //let value: f32 = number_text.parse().expect("Failed to parse number");
        //Some(Number(value))
        Some(Number)
    }

    fn identifier_or_keyword(&mut self, start: usize) -> TokenType {
        self.cursor.skip_while(char::is_alphanumeric);

        let text = &self.source[start..self.cursor.byte_pos];

        match text {
            "class" => Keyword(Class),
            "and" => Keyword(And),
            "else" => Keyword(Else),
            "false" => Keyword(KeywordType::False),
            "for" => Keyword(For),
            "fun" => Keyword(Fun),
            "if" => Keyword(If),
            "nil" => Keyword(KeywordType::Nil),
            "or" => Keyword(Or),
            "print" => Keyword(Print),
            "return" => Keyword(Return),
            "super" => Keyword(Super),
            "this" => Keyword(This),
            "true" => Keyword(KeywordType::True),
            "var" => Keyword(Var),
            "while" => Keyword(While),
            _ => Identifier,
        }
    }

    fn whitespace(&mut self) -> TokenType {
        self.cursor.skip_while(char::is_whitespace);
        Whitespace
    }
}
