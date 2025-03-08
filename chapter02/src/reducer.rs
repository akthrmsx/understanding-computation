use crate::{environment::Environment, expression::Expression, statement::Statement};

pub trait Reducer {
    fn is_reducible(&self) -> bool;
    fn reduce(&self, environment: &mut Environment) -> Self;
}

impl Reducer for Expression {
    fn is_reducible(&self) -> bool {
        !matches!(self, Self::Number(_) | Self::Boolean(_))
    }

    fn reduce(&self, environment: &mut Environment) -> Self {
        match self {
            expression @ Self::Number(_) => expression.clone(),
            expression @ Self::Boolean(_) => expression.clone(),
            Self::Variable(value) => environment.get(value),
            Self::Add(left, right) => {
                if left.is_reducible() {
                    Self::Add(Box::new(left.reduce(environment)), right.clone())
                } else if right.is_reducible() {
                    Self::Add(left.clone(), Box::new(right.reduce(environment)))
                } else {
                    Self::Number(left.unwrap_number() + right.unwrap_number())
                }
            }
            Self::Subtract(left, right) => {
                if left.is_reducible() {
                    Self::Subtract(Box::new(left.reduce(environment)), right.clone())
                } else if right.is_reducible() {
                    Self::Subtract(left.clone(), Box::new(right.reduce(environment)))
                } else {
                    Self::Number(left.unwrap_number() - right.unwrap_number())
                }
            }
            Self::Multiply(left, right) => {
                if left.is_reducible() {
                    Self::Multiply(Box::new(left.reduce(environment)), right.clone())
                } else if right.is_reducible() {
                    Self::Multiply(left.clone(), Box::new(right.reduce(environment)))
                } else {
                    Self::Number(left.unwrap_number() * right.unwrap_number())
                }
            }
            Self::Divide(left, right) => {
                if left.is_reducible() {
                    Self::Divide(Box::new(left.reduce(environment)), right.clone())
                } else if right.is_reducible() {
                    Self::Divide(left.clone(), Box::new(right.reduce(environment)))
                } else {
                    Self::Number(left.unwrap_number() / right.unwrap_number())
                }
            }
            Self::Not(expression) => {
                if expression.is_reducible() {
                    Self::Not(Box::new(expression.reduce(environment)))
                } else {
                    Self::Boolean(!expression.unwrap_boolean())
                }
            }
            Self::And(left, right) => {
                if left.is_reducible() {
                    Self::And(Box::new(left.reduce(environment)), right.clone())
                } else if right.is_reducible() {
                    Self::And(left.clone(), Box::new(right.reduce(environment)))
                } else {
                    Self::Boolean(left.unwrap_boolean() && right.unwrap_boolean())
                }
            }
            Self::Or(left, right) => {
                if left.is_reducible() {
                    Self::Or(Box::new(left.reduce(environment)), right.clone())
                } else if right.is_reducible() {
                    Self::Or(left.clone(), Box::new(right.reduce(environment)))
                } else {
                    Self::Boolean(left.unwrap_boolean() || right.unwrap_boolean())
                }
            }
            Self::Equal(left, right) => {
                if left.is_reducible() {
                    Self::Equal(Box::new(left.reduce(environment)), right.clone())
                } else if right.is_reducible() {
                    Self::Equal(left.clone(), Box::new(right.reduce(environment)))
                } else {
                    Self::Boolean(left == right)
                }
            }
            Self::NotEqual(left, right) => {
                if left.is_reducible() {
                    Self::NotEqual(Box::new(left.reduce(environment)), right.clone())
                } else if right.is_reducible() {
                    Self::NotEqual(left.clone(), Box::new(right.reduce(environment)))
                } else {
                    Self::Boolean(left != right)
                }
            }
            Self::LessThan(left, right) => {
                if left.is_reducible() {
                    Self::LessThan(Box::new(left.reduce(environment)), right.clone())
                } else if right.is_reducible() {
                    Self::LessThan(left.clone(), Box::new(right.reduce(environment)))
                } else {
                    Self::Boolean(left.unwrap_number() < right.unwrap_number())
                }
            }
            Self::LessThanOrEqual(left, right) => {
                if left.is_reducible() {
                    Self::LessThanOrEqual(Box::new(left.reduce(environment)), right.clone())
                } else if right.is_reducible() {
                    Self::LessThanOrEqual(left.clone(), Box::new(right.reduce(environment)))
                } else {
                    Self::Boolean(left.unwrap_number() <= right.unwrap_number())
                }
            }
            Self::GreaterThan(left, right) => {
                if left.is_reducible() {
                    Self::GreaterThan(Box::new(left.reduce(environment)), right.clone())
                } else if right.is_reducible() {
                    Self::GreaterThan(left.clone(), Box::new(right.reduce(environment)))
                } else {
                    Self::Boolean(left.unwrap_number() > right.unwrap_number())
                }
            }
            Self::GreaterThanOrEqual(left, right) => {
                if left.is_reducible() {
                    Self::GreaterThanOrEqual(Box::new(left.reduce(environment)), right.clone())
                } else if right.is_reducible() {
                    Self::GreaterThanOrEqual(left.clone(), Box::new(right.reduce(environment)))
                } else {
                    Self::Boolean(left.unwrap_number() >= right.unwrap_number())
                }
            }
        }
    }
}

impl Reducer for Statement {
    fn is_reducible(&self) -> bool {
        match self {
            Self::Expression(expression) => expression.is_reducible(),
            Self::Assign(_, _) | Self::If(_, _, _) | Self::Sequence(_, _) | Self::While(_, _) => {
                true
            }
            Self::Nothing => false,
        }
    }

    fn reduce(&self, environment: &mut Environment) -> Self {
        match self {
            Self::Expression(expression) => {
                if expression.is_reducible() {
                    Self::Expression(expression.reduce(environment))
                } else {
                    Self::Expression(expression.clone())
                }
            }
            Self::Assign(name, expression) => {
                if expression.is_reducible() {
                    Self::Assign(name.into(), expression.reduce(environment))
                } else {
                    environment.insert(name, expression.clone());
                    Self::Nothing
                }
            }
            Self::If(condition, consequence, alternative) => {
                if condition.is_reducible() {
                    Self::If(
                        condition.reduce(environment),
                        consequence.clone(),
                        alternative.clone(),
                    )
                } else if condition.unwrap_boolean() {
                    *consequence.clone()
                } else {
                    *alternative.clone()
                }
            }
            Self::Sequence(first, second) => match *first.clone() {
                Self::Nothing => *second.clone(),
                _ => Self::Sequence(Box::new(first.reduce(environment)), second.clone()),
            },
            statement @ Self::While(condition, body) => Self::If(
                condition.clone(),
                Box::new(Self::Sequence(body.clone(), Box::new(statement.clone()))),
                Box::new(Self::Nothing),
            ),
            Self::Nothing => Self::Nothing,
        }
    }
}
