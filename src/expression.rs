use std::fmt::Display;

use crate::token::TokenType;
pub enum Expression<'a> {
    Binary(Box<Expression<'a>>, TokenType<'a>, Box<Expression<'a>>),
    Unary(TokenType<'a>, Box<Expression<'a>>),
    Grouping(Box<Expression<'a>>),
    Literal(TokenType<'a>),
}

impl<'a> Display for Expression<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(_) => {}
            _ => write!(f, "(")?,
        };

        match self {
            Self::Binary(left, token, right) => write!(f, "{} {} {}", left, token, right)?,
            Self::Unary(token, right) => write!(f, "{} {}", token, right)?,
            Self::Grouping(exp) => write!(f, "( group {} )", exp)?,
            Self::Literal(token) => write!(f, "{}", token)?,
        };

        match self {
            Self::Literal(_) => {}
            _ => write!(f, ")")?,
        };
        Ok(())
    }
}
