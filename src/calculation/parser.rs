use super::{Operator, Value, token::Token};
use derive_more::Display;

#[derive(Debug, PartialEq, Display)]
pub enum CalculationParseError {
    #[display("Unexpected end of line in the calculation")]
    UnexpectedEOL,
    #[display("Unexpected token {_0} in the calculation")]
    UnexpectedToken(Token),
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Iterator for Parser {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let tok = self.tokens.get(self.pos)?;
        self.pos += 1;
        Some(*tok)
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    pub fn parse_expression(&mut self) -> Result<Value, CalculationParseError> {
        let mut left = self.parse_term()?;

        while let Some(Token::Operator(Operator::Add | Operator::Sub)) = self.peek() {
            let op = if let Token::Operator(op) = self.next().unwrap() {
                op
            } else {
                unreachable!()
            };
            let right = self.parse_term()?;
            left = Value::Expr {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Value, CalculationParseError> {
        let mut left = self.parse_factor()?;

        while let Some(Token::Operator(Operator::Mul | Operator::Div)) = self.peek() {
            let op = if let Token::Operator(op) = self.next().unwrap() {
                op
            } else {
                unreachable!()
            };
            let right = self.parse_factor()?;
            left = Value::Expr {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Value, CalculationParseError> {
        match self.next() {
            Some(Token::Number(n)) => Ok(Value::Num(n)),
            Some(Token::Variable) => Ok(Value::Var),
            Some(Token::LeftParenthese) => {
                let expr = self.parse_expression()?;
                match self.next() {
                    Some(Token::RightParenthese) => Ok(expr),
                    Some(tok) => Err(CalculationParseError::UnexpectedToken(tok)),
                    None => Err(CalculationParseError::UnexpectedEOL),
                }
            }
            Some(tok) => Err(CalculationParseError::UnexpectedToken(tok)),
            None => Err(CalculationParseError::UnexpectedEOL),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::token::token_list;
    use super::*;

    fn parse_ok(expr: &str) -> Value {
        let tokens = token_list(expr).expect("tokenization failed");
        let mut parser = Parser::new(tokens);
        parser.parse_expression().expect("parsing failed")
    }

    #[test]
    fn test_simple_addition() {
        let value = parse_ok("1 + 2");
        assert_eq!(format!("{}", value), "1 + 2");
        assert_eq!(value.evaluate(0.0), Some(3.0));
    }

    #[test]
    fn test_operator_precedence() {
        let value = parse_ok("1 + 2 * 3");
        assert_eq!(format!("{}", value), "1 + (2 * 3)");
        assert_eq!(value.evaluate(0.0), Some(7.0));

        let value = parse_ok("10 - 4 / 2");
        assert_eq!(format!("{}", value), "10 - (4 / 2)");
        assert_eq!(value.evaluate(0.0), Some(8.0));
    }

    #[test]
    fn test_parentheses_override_precedence() {
        let value = parse_ok("(1 + 2) * 3");
        assert_eq!(format!("{}", value), "(1 + 2) * 3");
        assert_eq!(value.evaluate(0.0), Some(9.0));
    }

    #[test]
    fn test_variable_handling() {
        let value = parse_ok("x * 2 + 1");
        assert_eq!(format!("{}", value), "(x * 2) + 1");
        assert_eq!(value.evaluate(3.0), Some(7.0));
    }

    #[test]
    fn test_nested_expressions() {
        let value = parse_ok("x * (1 + 2 * x)");
        assert_eq!(format!("{}", value), "x * (1 + (2 * x))");
        assert_eq!(value.evaluate(2.0), Some(10.0));
    }

    #[test]
    fn test_error() {
        let mut parser = Parser::new(token_list("(x + 2").unwrap());
        let result = parser.parse_expression();
        assert_eq!(result, Err(CalculationParseError::UnexpectedEOL));

        let mut parser = Parser::new(token_list("x +* 2").unwrap());
        assert_eq!(
            parser.parse_expression(),
            Err(CalculationParseError::UnexpectedToken(Token::Operator(
                Operator::Mul
            )))
        );
    }

    #[test]
    fn test_single_expr() {
        assert_eq!(parse_ok("42").evaluate(0.0), Some(42.0));
        assert_eq!(parse_ok("x").evaluate(5.5), Some(5.5));
    }
}
