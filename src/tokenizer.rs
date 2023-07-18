use std::fmt;
use std::ops::Range;

#[derive(Debug)]
pub enum Object {
    Num(f64),
    Str(String),
    Nil,
    True,
    False,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Num(x) => write!(f, "{x}"),
            Object::Str(x) => write!(f, "\"{x}\""),
            Object::Nil => write!(f, "nil"),
            Object::True => write!(f, "true"),
            Object::False => write!(f, "false"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
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
    Number,

    // Keywords.
    Keyword(KeywordType),
    Unknown,
    Eof,

    Whitespace,
    Comment,

    True,
    False,
    Nil,
}

#[derive(Copy, Clone, Debug, PartialEq)]
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub span: Span,
}

impl Token {
    pub fn as_string(&self, source: &str) -> String {
        let start = self.span.start;
        let end = self.span.end;

        format!("{:?}", &source[start..end])
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _start = self.span.start;
        let _end = self.span.end;
        write!(
            f,
            "{:?}",
            self.token_type,
            //self.lexeme,
            /*if let Some(literal) = &self.literal {
                literal.to_string()
            } else {
                "None".to_string()
            }*/
        )
    }
}
