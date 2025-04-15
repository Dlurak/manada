pub mod parser;
pub mod token;

use derive_more::{Display, From};
use rust_decimal::Decimal;
use std::fmt::Display;

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

#[derive(PartialEq, Debug, From)]
pub enum Value {
    Var,
    #[from]
    Num(Decimal),
    Calc {
        left: Box<Value>,
        op: Operator,
        right: Box<Value>,
    },
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Var => write!(f, "x"),
            Self::Num(n) => write!(f, "{}", n.normalize()),
            Self::Calc { left, op, right } => {
                if let Self::Calc { .. } = **left {
                    write!(f, "({})", left)?;
                } else {
                    write!(f, "{}", left)?;
                }

                write!(f, " {} ", op)?;

                if let Self::Calc { .. } = **right {
                    write!(f, "({})", right)
                } else {
                    write!(f, "{}", right)
                }
            }
        }
    }
}

impl Value {
    pub fn evaluate(&self, x: Decimal) -> Option<Decimal> {
        match self {
            Self::Var => Some(x),
            Self::Num(n) => Some(*n),
            Self::Calc { left, right, op } => {
                let left = left.evaluate(x)?;
                let right = right.evaluate(x)?;
                match op {
                    Operator::Add => Some(left + right),
                    Operator::Sub => Some(left - right),
                    Operator::Mul => Some(left * right),
                    Operator::Div => (!right.is_zero()).then(|| left / right),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_expression_evaluation() {
        let my_expr = Value::Calc {
            left: Box::new(dec!(1.0).into()),
            op: Operator::Add,
            right: Box::new(Value::Var),
        };
        assert_eq!(my_expr.evaluate(dec!(5.0)), Some(dec!(6.0)));

        let my_expr = Value::Calc {
            left: Box::new(my_expr),
            op: Operator::Add,
            right: Box::new(Value::Var),
        };
        assert_eq!(my_expr.evaluate(dec!(3.0)), Some(dec!(7.0)));

        let my_expr = Value::Calc {
            left: Box::new(Value::Var),
            op: Operator::Div,
            right: Box::new(dec!(0.0).into()),
        };
        assert!(my_expr.evaluate(dec!(3.0)).is_none());
    }

    #[test]
    fn test_format() {
        let my_expr = Value::Calc {
            left: Box::new(dec!(1.0).into()),
            op: Operator::Add,
            right: Box::new(Value::Var),
        };
        assert_eq!(format!("{}", my_expr), String::from("1 + x"));

        let my_expr = Value::Calc {
            left: Box::new(my_expr),
            op: Operator::Add,
            right: Box::new(Value::Var),
        };
        assert_eq!(format!("{}", my_expr), String::from("(1 + x) + x"));
    }
}
