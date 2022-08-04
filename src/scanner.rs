use std::thread::current;
use thiserror::Error;

use crate::token::{Token, TokenType};
// use color_eyre::Result;

#[derive(Debug)]
pub struct Scanner {
    pub source_code: String,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

#[derive(Debug, Error)]
pub enum ScannerError {
    #[error("Invalid char")]
    InvalidCharacter(usize, usize),
}

impl Scanner {
    pub fn new(source_code: String) -> Self {
        Scanner {
            source_code,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<(), ScannerError> {
        for (curr, c) in self.source_code.clone().char_indices() {
            self.start = curr;
            self.scan_token(c)?
        }

        self.add_token(TokenType::Eof, None);
        Ok(())
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source_code.len()
    }

    fn scan_token(&mut self, c: char) -> Result<(), ScannerError> {
        match c {
            ')' => Ok(self.add_token(TokenType::RightParen, None)),
            '(' => Ok(self.add_token(TokenType::LeftParen, None)),
            '{' => Ok(self.add_token(TokenType::LeftBrace, None)),
            '}' => Ok(self.add_token(TokenType::RightBrace, None)),
            ';' => Ok(self.add_token(TokenType::LeftParen, None)),
            '.' => Ok(self.add_token(TokenType::Dot, None)),
            '/' => Ok(self.add_token(TokenType::Slash, None)),
            '*' => Ok(self.add_token(TokenType::Star, None)),
            '+' => Ok(self.add_token(TokenType::Plus, None)),
            '-' => Ok(self.add_token(TokenType::Minus, None)),
            ',' => Ok(self.add_token(TokenType::Comma, None)),
            '\n' => {
                self.line += 1;
                Ok(())
            }
            _ => Err(ScannerError::InvalidCharacter(self.start, self.line)),
        }
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<String>) {
        self.tokens.push(Token {
            token_type,
            lexeme: self
                .source_code
                .get(self.start..(self.start + 1))
                .unwrap()
                .to_string(),
            literal,
            line: self.line,
        })
    }
    pub fn print_tokens(&self) {
        println!("{:?}", self.tokens)
    }
}
