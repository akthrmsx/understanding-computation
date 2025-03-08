use crate::expression::Expression;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Expression(Expression),
    Assign(String, Expression),
    If(Expression, Box<Statement>, Box<Statement>),
    Sequence(Box<Statement>, Box<Statement>),
    While(Expression, Box<Statement>),
    Nothing,
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Expression(expression) => write!(f, "{}", expression),
            Self::Assign(name, expression) => write!(f, "{} = {}", name, expression),
            Self::If(condition, consequence, alternative) => {
                write!(
                    f,
                    "if ( {} ) {{ {} }} else {{ {} }}",
                    condition, consequence, alternative
                )
            }
            Self::Sequence(first, second) => write!(f, "{}; {}", first, second),
            Self::While(condition, body) => write!(f, "while ( {} ) {{ {} }}", condition, body),
            Self::Nothing => write!(f, "nothing"),
        }
    }
}
