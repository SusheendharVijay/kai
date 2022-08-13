use std::fmt::Display;

use yansi::Paint;

/// Token type enum
#[derive(Debug, Clone)]
pub enum TokenType<'a> {
    //Single char tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    SemiColon,
    Minus,
    Plus,
    Slash,
    Star,
    // One or two char tokens
    Not,
    NotEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // literals
    Identifier,
    String(&'a str),
    Number(f32),
    //Keywords
    And,
    For,
    If,
    Else,
    While,
    Class,
    Nil,
    Or,
    False,
    True,
    Var,
    Print,
    Return,
    This,
    Super,
    Eof,
}

/// Token struct
#[derive(Debug)]
pub struct Token<'a> {
    pub token_type: TokenType<'a>,
    pub lexeme: &'a str,
    pub line: usize,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType<'a>, lexeme: &'a str, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            line,
        }
    }

    // pub fn to_string(&self) -> String {}
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<{}, {}: \'{}\' at line [{}]>",
            Paint::yellow(&self.token_type),
            Paint::blue("literal"),
            Paint::green(&self.lexeme),
            Paint::yellow(&self.line)
        )
    }
}

impl<'a> Display for TokenType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::String(val) => write!(f, "String, lexeme:{}", Paint::green(val)),
            TokenType::Number(val) => write!(f, "Number, lexeme:{}", Paint::green(val)),
            _ => write!(f, "{:?}", self),
        }
    }
}
