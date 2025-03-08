use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Number(usize),
    Boolean(bool),
    Variable(String),
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    Not(Box<Expression>),
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Equal(Box<Expression>, Box<Expression>),
    NotEqual(Box<Expression>, Box<Expression>),
    LessThan(Box<Expression>, Box<Expression>),
    LessThanOrEqual(Box<Expression>, Box<Expression>),
    GreaterThan(Box<Expression>, Box<Expression>),
    GreaterThanOrEqual(Box<Expression>, Box<Expression>),
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Number(value) => write!(f, "{}", value),
            Self::Boolean(value) => write!(f, "{}", value),
            Self::Variable(value) => write!(f, "{}", value),
            Self::Add(left, right) => write!(f, "{} + {}", left, right),
            Self::Subtract(left, right) => write!(f, "{} - {}", left, right),
            Self::Multiply(left, right) => write!(f, "{} * {}", left, right),
            Self::Divide(left, right) => write!(f, "{} / {}", left, right),
            Self::Not(expression) => write!(f, "!{}", expression),
            Self::And(left, right) => write!(f, "{} && {}", left, right),
            Self::Or(left, right) => write!(f, "{} || {}", left, right),
            Self::Equal(left, right) => write!(f, "{} == {}", left, right),
            Self::NotEqual(left, right) => write!(f, "{} != {}", left, right),
            Self::LessThan(left, right) => write!(f, "{} < {}", left, right),
            Self::LessThanOrEqual(left, right) => write!(f, "{} <= {}", left, right),
            Self::GreaterThan(left, right) => write!(f, "{} > {}", left, right),
            Self::GreaterThanOrEqual(left, right) => write!(f, "{} >= {}", left, right),
        }
    }
}

impl Expression {
    pub fn unwrap_number(&self) -> usize {
        match self {
            Self::Number(value) => *value,
            _ => panic!("expression '{}' is not a number", self),
        }
    }

    pub fn unwrap_boolean(&self) -> bool {
        match self {
            Self::Boolean(value) => *value,
            _ => panic!("expression '{}' is not a boolean", self),
        }
    }
}
