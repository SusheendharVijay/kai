use std::{iter::Peekable, str::CharIndices, thread::current};
use thiserror::Error;
use yansi::Paint;

use crate::token::{Token, TokenType};
// use color_eyre::Result;

#[derive(Debug)]
pub struct Scanner<'a> {
    pub source_code: &'a str,
    code: Peekable<CharIndices<'a>>,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    has_error: bool,
}

#[derive(Debug, Error)]
pub enum ScannerError {
    #[error("Invalid char")]
    InvalidCharacter(usize, usize),
}

impl<'a> Scanner<'a> {
    pub fn new(source_code: &'a str) -> Self {
        Scanner {
            source_code,
            code: source_code.char_indices().peekable(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            has_error: false,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<(), ScannerError> {
        while let Some((idx, c)) = self.code.next() {
            self.start = idx;
            self.current = idx;
            self.scan_token(c)?
        }

        self.start = self.current;
        self.add_token(TokenType::Eof, None);
        Ok(())
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source_code.len()
    }

    fn print_error(&self, start_index: usize, line_number: usize) {
        let err = Paint::red(format!(
            "Invalid character at char index {} and line number {}",
            start_index, line_number
        ));
        println!("{}", err)
    }

    fn scan_token(&mut self, c: char) -> Result<(), ScannerError> {
        match c {
            ')' => Ok(self.add_token(TokenType::RightParen, None)),
            '(' => Ok(self.add_token(TokenType::LeftParen, None)),
            '{' => Ok(self.add_token(TokenType::LeftBrace, None)),
            '}' => Ok(self.add_token(TokenType::RightBrace, None)),
            ';' => Ok(self.add_token(TokenType::SemiColon, None)),
            '.' => Ok(self.add_token(TokenType::Dot, None)),
            '*' => Ok(self.add_token(TokenType::Star, None)),
            '+' => Ok(self.add_token(TokenType::Plus, None)),
            '-' => Ok(self.add_token(TokenType::Minus, None)),
            ',' => Ok(self.add_token(TokenType::Comma, None)),
            '!' => {
                if let Some((_, '=')) = self.code.peek() {
                    self.advance();
                    Ok(self.add_token(TokenType::NotEqual, None))
                } else {
                    Ok(self.add_token(TokenType::Not, None))
                }
            }
            '>' => {
                if let Some((_, '=')) = self.code.peek() {
                    self.advance();
                    Ok(self.add_token(TokenType::GreaterEqual, None))
                } else {
                    Ok(self.add_token(TokenType::Greater, None))
                }
            }
            '<' => {
                if let Some((_, '=')) = self.code.peek() {
                    self.advance();
                    Ok(self.add_token(TokenType::LessEqual, None))
                } else {
                    Ok(self.add_token(TokenType::Less, None))
                }
            }
            '/' => {
                if let Some((_, '/')) = self.code.peek() {
                    println!("Ignoring comments at line [{}]", self.line);
                    while let Some((_, val)) = self.code.next() {
                        if val == '\n' {
                            self.line += 1;
                            break;
                        };
                    }
                    Ok(())
                } else {
                    Ok(self.add_token(TokenType::Slash, None))
                }
            }
            '\n' => {
                self.line += 1;
                Ok(())
            }
            _ => {
                self.has_error = true;
                Ok(self.print_error(self.start, self.line))
            }
        }
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<String>) {
        self.tokens.push(Token {
            token_type,
            lexeme: self
                .source_code
                .get(self.start..self.current + 1)
                .unwrap()
                .to_string(),
            literal,
            line: self.line,
        })
    }

    fn advance(&mut self) {
        let _ = self.code.next();
        self.current += 1;
    }

    pub fn print_tokens(&self) {
        for token in &self.tokens {
            println!("{}", token)
        }
    }
}
