/// Token type enum
#[derive(Debug)]
pub enum TokenType {
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
    Bang,
    BangEqual,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // literals
    Identifier,
    String,
    Number,
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
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<String>,
    pub line: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<String>,
        line: usize,
    ) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "{:?} {} {}",
            self.token_type,
            &self.lexeme,
            self.literal.as_ref().unwrap_or(&"null".to_string())
        )
    }
}
