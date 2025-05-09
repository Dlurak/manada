use std::str::FromStr;

use super::Operator;
use derive_more::Display;
use rust_decimal::Decimal;

#[derive(PartialEq, Debug, Clone, Copy, Display)]
pub enum Token {
    #[display("NUMBER({_0})")]
    Number(Decimal),
    #[display("OPERATOR({_0})")]
    Operator(Operator),
    #[display("VARIABLE")]
    Variable,
    #[display("LEFT-PARENTHESE")]
    LeftParenthese,
    #[display("RIGHT-PARENTHESE")]
    RightParenthese,
}

#[derive(PartialEq, Eq, Debug, Display)]
pub enum TokenizeError {
    #[display("Invalid charachter {char} in calculation")]
    InvalidChar {
        serialized_string: String,
        char: char,
        position: usize,
    },
}

pub fn token_list(unparsed: &str) -> Result<Vec<Token>, TokenizeError> {
    let serialized = unparsed.replace(' ', "");

    let mut tokens = Vec::new();
    let chars = serialized.chars().collect::<Vec<_>>();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        let is_negative_number = c == '-' && {
            chars.get(i + 1).is_some_and(|&nc| nc.is_ascii_digit())
                && (matches!(
                    tokens.last(),
                    None | Some(Token::Operator(_)) | Some(Token::LeftParenthese)
                ))
        };
        let is_number = c.is_ascii_digit() || is_negative_number;

        if is_number {
            let start = i;
            if c == '-' {
                i += 1;
            }
            while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                i += 1;
            }
            let num_str: String = chars[start..i].iter().collect();
            let num = Decimal::from_str(&num_str).unwrap();
            tokens.push(Token::Number(num));
        } else {
            let next = match c {
                'x' => Token::Variable,
                '+' => Token::Operator(Operator::Add),
                '-' => Token::Operator(Operator::Sub),
                '*' => Token::Operator(Operator::Mul),
                '/' => Token::Operator(Operator::Div),
                '(' | '{' | '[' => Token::LeftParenthese,
                ')' | '}' | ']' => Token::RightParenthese,
                char => {
                    return Err(TokenizeError::InvalidChar {
                        serialized_string: serialized,
                        char,
                        position: i,
                    });
                }
            };
            tokens.push(next);
            i += 1;
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    // TODO: Only have this library in test builds
    use rust_decimal_macros::dec;

    #[test]
    fn test_tokenizer() {
        assert_eq!(
            token_list("(x + x) * (-5.5--3)").unwrap(),
            vec![
                Token::LeftParenthese,
                Token::Variable,
                Token::Operator(Operator::Add),
                Token::Variable,
                Token::RightParenthese,
                Token::Operator(Operator::Mul),
                Token::LeftParenthese,
                Token::Number(dec!(-5.5)),
                Token::Operator(Operator::Sub),
                Token::Number(dec!(-3.0)),
                Token::RightParenthese,
            ]
        );
        assert_eq!(
            token_list("[] h 1"),
            Err(TokenizeError::InvalidChar {
                serialized_string: String::from("[]h1"),
                char: 'h',
                position: 2
            })
        );

        assert_eq!(token_list("-5").unwrap(), vec![Token::Number(dec!(-5.0))]);
    }
}
