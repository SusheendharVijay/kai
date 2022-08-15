mod expression;
pub mod scanner;
mod token;

#[cfg(test)]
mod tests {
    use super::expression::*;
    use super::scanner::*;
    use super::token::*;
    #[test]
    fn it_parses_parenthesis() {
        let mut scanner = Scanner::new("()");

        let _ = scanner.scan_tokens();

        let left_paren = Token::new(TokenType::LeftParen, "(", 1);
        let right_paren = Token::new(TokenType::RightParen, ")", 1);
        let eof = Token::new(TokenType::Eof, "", 1);

        assert_eq!(vec![left_paren, right_paren, eof], scanner.tokens)
    }

    #[test]
    fn it_pretty_prints() {
        let a = Box::new(Expression::Literal(TokenType::Number(3.0)));
        let b = Box::new(Expression::Literal(TokenType::Number(6.0)));
        let exp = Expression::Binary(a, TokenType::Plus, b);
        let una = Expression::Unary(TokenType::Minus, Box::new(exp));
        let group = Expression::Grouping(Box::new(una));

        println!("{}", group);
    }
}
