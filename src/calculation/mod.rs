pub mod parser;
pub mod token;

use std::fmt::Display;

use derive_more::{Display, From};

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
    Num(f64),
    Expr {
        left: Box<Value>,
        op: Operator,
        right: Box<Value>,
    },
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Var => write!(f, "x"),
            Self::Num(n) => write!(f, "{n}"),
            Self::Expr { left, op, right } => {
                if let Self::Expr { .. } = **left {
                    write!(f, "({})", left)?;
                } else {
                    write!(f, "{}", left)?;
                }

                write!(f, " {} ", op)?;

                if let Self::Expr { .. } = **right {
                    write!(f, "({})", right)
                } else {
                    write!(f, "{}", right)
                }
            }
        }
    }
}

impl Value {
    pub fn evaluate(&self, x: f64) -> Option<f64> {
        match self {
            Self::Var => Some(x),
            Self::Num(n) => Some(*n),
            Self::Expr { left, right, op } => {
                let left = left.evaluate(x)?;
                let right = right.evaluate(x)?;
                match op {
                    Operator::Add => Some(left + right),
                    Operator::Sub => Some(left - right),
                    Operator::Mul => Some(left * right),
                    Operator::Div => (right != 0.0).then_some(left / right),
                }
            }
        }
    }
}

// #[derive(PartialEq, Debug)]
// pub struct Expression {
//     left: Value,
//     op: Operator,
//     right: Value,
// }
//
// impl Display for Expression {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         if let Value::Expr(e) = &self.left {
//             write!(f, "({})", e)?;
//         } else {
//             write!(f, "{}", self.left)?;
//         }
//
//         write!(f, " {} ", self.op)?;
//
//         if let Value::Expr(e) = &self.right {
//             write!(f, "({})", e)
//         } else {
//             write!(f, "{}", self.right)
//         }
//     }
// }
//
// impl Expression {
//     pub fn evaluate(&self, x: f64) -> Option<f64> {
//         let left = self.left.evaluate(x)?;
//         let right = self.right.evaluate(x)?;
//         match self.op {
//             Operator::Add => Some(left + right),
//             Operator::Sub => Some(left - right),
//             Operator::Mul => Some(left * right),
//             Operator::Div => (right != 0.0).then_some(left / right),
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expression_evaluation() {
        let my_expr = Value::Expr {
            left: Box::new(1.0.into()),
            op: Operator::Add,
            right: Box::new(Value::Var),
        };
        assert_eq!(my_expr.evaluate(5.0), Some(6.0));

        let my_expr = Value::Expr {
            left: Box::new(my_expr),
            op: Operator::Add,
            right: Box::new(Value::Var),
        };
        assert_eq!(my_expr.evaluate(3.0), Some(7.0));

        let my_expr = Value::Expr {
            left: Box::new(Value::Var),
            op: Operator::Div,
            right: Box::new(0.0.into()),
        };
        assert!(my_expr.evaluate(3.0).is_none());
    }

    #[test]
    fn test_format() {
        let my_expr = Value::Expr {
            left: Box::new(1.0.into()),
            op: Operator::Add,
            right: Box::new(Value::Var),
        };
        assert_eq!(format!("{}", my_expr), String::from("1 + x"));

        let my_expr = Value::Expr {
            left: Box::new(my_expr),
            op: Operator::Add,
            right: Box::new(Value::Var),
        };
        assert_eq!(format!("{}", my_expr), String::from("(1 + x) + x"));
    }
}
