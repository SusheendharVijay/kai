use std::fmt::{format, Display};

use crate::token::TokenType;

#[derive(Debug, PartialEq)]
pub enum Expression<'a> {
    Binary(Box<Expression<'a>>, TokenType<'a>, Box<Expression<'a>>),
    Unary(TokenType<'a>, Box<Expression<'a>>),
    Grouping(Box<Expression<'a>>),
    Literal(TokenType<'a>),
    Ternary(
        Box<Expression<'a>>,
        Box<Expression<'a>>,
        Box<Expression<'a>>,
    ),
}

pub trait ReversePolish {
    fn print_rpn(&self) -> String;
}

impl<'a> Display for Expression<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(_) => {}
            Self::Grouping(_) => {}
            _ => write!(f, "(")?,
        };

        match self {
            Self::Binary(left, token, right) => write!(f, "{} {} {}", left, token, right)?,
            Self::Unary(token, right) => write!(f, "{} {}", token, right)?,
            Self::Grouping(exp) => write!(f, "({})", exp)?,
            Self::Literal(token) => write!(f, "{}", token)?,
            Self::Ternary(exp1, exp2, exp3) => write!(f, "{} ? {} : {}", exp1, exp2, exp3)?,
        };

        match self {
            Self::Literal(_) => {}
            Self::Grouping(_) => {}
            _ => write!(f, ")")?,
        };
        Ok(())
    }
}

impl<'a> ReversePolish for Expression<'a> {
    fn print_rpn(&self) -> String {
        match self {
            Self::Binary(left, token, right) => {
                format!("{} {} {}", left.print_rpn(), right.print_rpn(), token)
            }
            Self::Unary(token, right) => format!("{} {}", right.print_rpn(), token),
            Self::Grouping(exp) => format!("( {} )", exp.print_rpn()),
            Self::Literal(token) => format!("{}", token),
            Self::Ternary(e1, e2, e3) => format!(
                "{} {} {} : ?",
                e1.print_rpn(),
                e2.print_rpn(),
                e3.print_rpn(),
            ),
        }
    }
}
