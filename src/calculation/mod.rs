pub mod parser;
pub mod token;

use std::fmt::Display;

use derive_more::Display;

#[derive(PartialEq, Eq, Debug, Display, Clone, Copy)]
pub enum Operator {
    #[display("+")]
    Add,
    #[display("-")]
    Sub,
    #[display("*")]
    Mul,
    #[display("/")]
    Div,
}

#[derive(Display, PartialEq, Debug)]
pub enum Value {
    #[display("x")]
    Var,
    #[display("{_0}")]
    Num(f64),
    #[display("{_0}")]
    Expr(Box<Expression>),
}

impl<'a> From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Num(value)
    }
}

impl Value {
    pub fn evaluate(&self, x: f64) -> Option<f64> {
        match self {
            Self::Var => Some(x),
            Self::Num(n) => Some(*n),
            Self::Expr(e) => e.evaluate(x),
        }
    }
}

// #[derive(Display, PartialEq, Debug)]
#[derive(PartialEq, Debug)]
// #[display("{left} {op} {right}")]
pub struct Expression {
    left: Value,
    op: Operator,
    right: Value,
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Value::Expr(e) = &self.left {
            write!(f, "({})", e)?;
        } else {
            write!(f, "{}", self.left)?;
        }

        write!(f, " {} ", self.op)?;

        if let Value::Expr(e) = &self.right {
            write!(f, "({})", e)
        } else {
            write!(f, "{}", self.right)
        }
    }
}

impl Expression {
    pub fn evaluate(&self, x: f64) -> Option<f64> {
        let left = self.left.evaluate(x)?;
        let right = self.right.evaluate(x)?;
        match self.op {
            Operator::Add => Some(left + right),
            Operator::Sub => Some(left - right),
            Operator::Mul => Some(left * right),
            Operator::Div => (right != 0.0).then_some(left / right),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expression_evaluation() {
        let my_expr = Expression {
            left: 1.0.into(),
            op: Operator::Add,
            right: Value::Var,
        };
        assert_eq!(my_expr.evaluate(5.0), Some(6.0));

        let my_expr = Expression {
            left: Value::Expr(Box::new(my_expr)),
            op: Operator::Add,
            right: Value::Var,
        };
        assert_eq!(my_expr.evaluate(3.0), Some(7.0));

        let my_expr = Expression {
            left: Value::Var,
            op: Operator::Div,
            right: 0.0.into(),
        };
        assert!(my_expr.evaluate(3.0).is_none());
    }

    #[test]
    fn test_format() {
        let my_expr = Expression {
            left: 1.0.into(),
            op: Operator::Add,
            right: Value::Var,
        };
        assert_eq!(format!("{}", my_expr), String::from("1 + x"));

        let my_expr = Expression {
            left: Value::Expr(Box::new(my_expr)),
            op: Operator::Add,
            right: Value::Var,
        };
        assert_eq!(format!("{}", my_expr), String::from("(1 + x) + x"));
    }
}
