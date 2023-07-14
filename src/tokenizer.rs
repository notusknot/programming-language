use std::ops::Range;

pub enum Object {
    Num(f64),
    Str(String),
    Nil,
    True,
    False,
}

#[derive(Copy, Clone, Debug)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    StringLiteral,
    Number(f32),

    // Keywords.
    Keyword(KeywordType),
    Unknown,

    Whitespace,
}

#[derive(Copy, Clone, Debug)]
pub enum KeywordType {
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

/// A byte range representing a location in a source string.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Span {
    /// The (inclusive) start position of the span in bytes.
    pub start: usize,
    /// The (exclusive) end position of the span in bytes.
    pub end: usize,
}

impl From<Range<usize>> for Span {
    fn from(value: Range<usize>) -> Self {
        Self {
            start: value.start,
            end: value.end,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub span: Span,
}
/*
impl Token {
    pub fn new(token_type: TokenType, line) -> Self {
        Self {
            token_type:
        }
    }
}
*/
