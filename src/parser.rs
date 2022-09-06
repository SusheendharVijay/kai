use crate::{
    expression::Expression,
    token::{Token, TokenType},
};

#[derive(Debug)]
pub struct Parser<'a> {
    pub tokens: Vec<Token<'a>>,
    pub curr_idx: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Parser {
            tokens,
            curr_idx: 0,
        }
    }

    fn is_at_end(&self) -> bool {
        self.curr_idx >= self.tokens.len()
    }

    fn prev(&self) -> Token<'a> {
        self.tokens[self.curr_idx - 1]
    }

    fn current(&self) -> Token<'a> {
        self.tokens[self.curr_idx]
    }

    pub fn expression(&mut self) -> Expression<'a> {
        self.ternary()
    }

    fn ternary(&mut self) -> Expression<'a> {
        let exp = self.equality();
        if self.is_at_end() {
            return exp;
        }
        if self.current().token_type == TokenType::Question {
            self.curr_idx += 1;
            if self.is_at_end() {
                panic!("No expression found after '?' operator, expected valid ternary expression");
            } else {
                let exp1 = self.equality();
                if self.current().token_type == TokenType::Colon {
                    self.curr_idx += 1;
                    let exp2 = self.equality();
                    return Expression::Ternary(Box::new(exp), Box::new(exp1), Box::new(exp2));
                } else {
                    panic!("Expected valid ternary expression, expected :");
                }
            }
        }

        exp
    }

    fn equality(&mut self) -> Expression<'a> {
        let mut exp = self.comparison();

        if self.is_at_end() {
            return exp;
        }

        while match self.current().token_type {
            TokenType::NotEqual | TokenType::EqualEqual => {
                self.curr_idx += 1;
                true
            }
            _ => false,
        } {
            let operator = self.prev();
            let right = self.comparison();
            exp = Expression::Binary(Box::new(exp), operator.token_type, Box::new(right));
        }

        exp
    }

    fn comparison(&mut self) -> Expression<'a> {
        let mut exp = self.term();
        if self.is_at_end() {
            return exp;
        }

        while match self.current().token_type {
            TokenType::Greater
            | TokenType::GreaterEqual
            | TokenType::Less
            | TokenType::LessEqual => {
                self.curr_idx += 1;
                true
            }
            _ => false,
        } {
            let operator = self.prev();
            let right = self.term();
            exp = Expression::Binary(Box::new(exp), operator.token_type, Box::new(right));
        }
        exp
    }

    fn term(&mut self) -> Expression<'a> {
        let mut exp = self.factor();

        if self.is_at_end() {
            return exp;
        }

        while match self.current().token_type {
            TokenType::Plus | TokenType::Minus => {
                self.curr_idx += 1;
                true
            }
            _ => false,
        } {
            let operator = self.prev();
            let right = self.factor();
            exp = Expression::Binary(Box::new(exp), operator.token_type, Box::new(right));
        }
        exp
    }
    fn factor(&mut self) -> Expression<'a> {
        let mut exp = self.unary();
        if self.is_at_end() {
            return exp;
        }

        while match self.current().token_type {
            TokenType::Slash | TokenType::Star => {
                self.curr_idx += 1;
                true
            }
            _ => false,
        } {
            let operator = self.prev();
            let right = self.unary();
            exp = Expression::Binary(Box::new(exp), operator.token_type, Box::new(right));
        }
        exp
    }
    fn unary(&mut self) -> Expression<'a> {
        match self.current().token_type {
            TokenType::Not | TokenType::Minus => {
                self.curr_idx += 1;
                let operator = self.prev().token_type;
                let exp = self.unary();

                Expression::Unary(operator, Box::new(exp))
            }
            _ => self.primary(),
        }
    }

    fn primary(&mut self) -> Expression<'a> {
        if self.is_at_end() {
            return Expression::Literal(TokenType::Eof);
        }

        self.curr_idx += 1;
        match self.prev().token_type {
            TokenType::Number(val) => Expression::Literal(TokenType::Number(val)),
            TokenType::String(val) => Expression::Literal(TokenType::String(val)),
            TokenType::Identifier => Expression::Literal(TokenType::Identifier),
            TokenType::LeftParen => {
                let exp = self.equality();

                if self.current().token_type != TokenType::RightParen {
                    panic!("Expected ')' at line: {}", self.prev().line);
                } else {
                    self.curr_idx += 1;
                    Expression::Grouping(Box::new(exp))
                }
            }
            _ if self.is_at_end() => Expression::Literal(TokenType::Eof),
            _ => panic!("Unexpected token at line: {}", self.prev().line),
        }
    }
}
