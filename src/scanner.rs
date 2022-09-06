use std::{collections::HashMap, iter::Peekable, str::CharIndices};
use thiserror::Error;
use yansi::Paint;

use crate::token::{Token, TokenType};
// use color_eyre::Result;

#[derive(Debug)]
pub struct Scanner<'a> {
    pub source_code: &'a str,
    code: Peekable<CharIndices<'a>>,
    pub tokens: Vec<Token<'a>>,
    pub reserved: HashMap<&'a str, TokenType<'a>>,
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
        let mut reserved: HashMap<&str, TokenType<'a>> = HashMap::new();
        reserved.insert("and", TokenType::And);
        reserved.insert("or", TokenType::Or);
        reserved.insert("for", TokenType::For);
        reserved.insert("if", TokenType::If);
        reserved.insert("else", TokenType::Else);
        reserved.insert("while", TokenType::While);
        reserved.insert("Nil", TokenType::Nil);
        reserved.insert("class", TokenType::Class);
        reserved.insert("return", TokenType::Return);
        reserved.insert("super", TokenType::Super);
        reserved.insert("var", TokenType::Var);
        reserved.insert("print", TokenType::Print);
        reserved.insert("this", TokenType::This);
        reserved.insert("true", TokenType::True);
        reserved.insert("false", TokenType::False);

        Scanner {
            source_code: source_code.trim(),
            code: source_code.char_indices().peekable(),
            tokens: vec![],
            reserved,
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
        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "",
            line: self.line,
        });

        Ok(())
    }

    // fn is_at_end(&self) -> bool {
    //     self.current > self.source_code.chars().count() - 1
    // }

    fn print_error(&self, start_index: usize, line_number: usize) {
        let err = Paint::red(format!(
            "Invalid character at char index {} and line number {}",
            start_index, line_number
        ));
        println!("{}", err)
    }

    fn scan_token(&mut self, c: char) -> Result<(), ScannerError> {
        match c {
            ')' => Ok(self.add_token(TokenType::RightParen)),
            '(' => Ok(self.add_token(TokenType::LeftParen)),
            '{' => Ok(self.add_token(TokenType::LeftBrace)),
            '}' => Ok(self.add_token(TokenType::RightBrace)),
            ';' => Ok(self.add_token(TokenType::SemiColon)),
            '.' => Ok(self.add_token(TokenType::Dot)),
            '*' => Ok(self.add_token(TokenType::Star)),
            '+' => Ok(self.add_token(TokenType::Plus)),
            '-' => Ok(self.add_token(TokenType::Minus)),
            ',' => Ok(self.add_token(TokenType::Comma)),
            '!' => {
                if let Some((_, '=')) = self.code.peek() {
                    self.advance();
                    Ok(self.add_token(TokenType::NotEqual))
                } else {
                    Ok(self.add_token(TokenType::Not))
                }
            }
            '?' => Ok(self.add_token(TokenType::Question)),
            ':' => Ok(self.add_token(TokenType::Colon)),
            '>' => {
                if let Some((_, '=')) = self.code.peek() {
                    self.advance();
                    Ok(self.add_token(TokenType::GreaterEqual))
                } else {
                    Ok(self.add_token(TokenType::Greater))
                }
            }
            '<' => {
                if let Some((_, '=')) = self.code.peek() {
                    self.advance();
                    Ok(self.add_token(TokenType::LessEqual))
                } else {
                    Ok(self.add_token(TokenType::Less))
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
                } else if let Some((_, '*')) = self.code.peek() {
                    println!("Ignoring comments at line: [{}]", self.line);
                    // consume the "*"
                    self.advance();
                    /* Hllosdlfkjsldkfjsldfkjl*k */
                    while let Some((_, val)) = self.code.peek() {
                        if *val == '*' {
                            self.advance();
                            if let Some((_, '/')) = self.code.peek() {
                                self.advance();
                                return Ok(());
                            }
                        } else if *val == '\n' {
                            self.line += 1;
                            self.advance();
                        } else {
                            self.advance();
                        }
                    }

                    if let None = self.code.peek() {
                        println!("Unterminated comment at line: [{}]", self.line)
                    }
                    Ok(())
                } else {
                    Ok(self.add_token(TokenType::Slash))
                }
            }
            '=' => {
                if let Some((_, '=')) = self.code.peek() {
                    self.advance();
                    Ok(self.add_token(TokenType::EqualEqual))
                } else {
                    Ok(self.add_token(TokenType::Equal))
                }
            }
            '"' => self.tokenize_string(),
            num if num.is_numeric() => self.tokenize_number(),
            c if (c.is_alphabetic() || c == '_') => self.tokenize_identifier(),
            ' ' => Ok(()),
            '\n' => {
                self.line += 1;
                Ok(self.add_token(TokenType::NewLine))
            }
            '\r' => Ok(()),
            '\t' => Ok(()),
            _ => {
                self.has_error = true;
                Ok(self.print_error(self.start, self.line))
            }
        }
    }

    fn add_token(&mut self, token_type: TokenType<'a>) {
        let lexeme = self
            .source_code
            .get(self.start..self.current + 1)
            .unwrap_or("\n");

        let token_type = match token_type {
            TokenType::String(_) => TokenType::String(lexeme),
            TokenType::Number(_) => TokenType::Number(lexeme.parse::<f32>().unwrap()),
            _ => token_type,
        };

        self.tokens.push(Token {
            token_type,
            lexeme,
            line: self.line,
        })
    }

    fn tokenize_string(&mut self) -> Result<(), ScannerError> {
        while let Some((_, val)) = self.code.peek() {
            if *val == '\n' {
                self.advance();
                self.line += 1;
            } else if *val == '"' {
                self.advance();
                self.add_token(TokenType::String(""));
                return Ok(());
            } else {
                self.advance()
            }
        }
        if let None = self.code.peek() {
            println!(
                "{}",
                Paint::red(format!(
                    "Unterminated string at index: {} and line :[{}]",
                    &self.current, &self.line
                ))
            );
        }
        Ok(())
    }

    fn tokenize_number(&mut self) -> Result<(), ScannerError> {
        while self.code.peek().is_some() && self.code.peek().unwrap().1.is_numeric() {
            self.advance()
        }

        if self.code.peek().is_some() && self.code.peek().unwrap().1 == '.' {
            self.advance()
        }

        while self.code.peek().is_some() && self.code.peek().unwrap().1.is_numeric() {
            self.advance()
        }

        self.add_token(TokenType::Number(0.0));

        Ok(())
    }

    fn tokenize_identifier(&mut self) -> Result<(), ScannerError> {
        self.advance();

        while let Some((_, val)) = self.code.peek() {
            if !val.is_alphanumeric() {
                break;
            }
            self.advance()
        }

        let lexeme = self.source_code.get(self.start..self.current + 1).unwrap();

        if let Some(token_type) = self.reserved.get(lexeme) {
            self.add_token(token_type.clone())
        } else {
            self.add_token(TokenType::Identifier)
        }

        Ok(())
    }

    fn advance(&mut self) {
        let _ = self.code.next();
        self.current += 1;
    }

    pub fn print_tokens(&self) {
        for token in &self.tokens {
            println!("{:?}", token)
        }
    }
}
