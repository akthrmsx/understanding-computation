use crate::{environment::Environment, expression::Expression, statement::Statement};

pub trait Evaluator {
    fn evaluate(&self, environment: &mut Environment) -> Self;
}

impl Evaluator for Expression {
    fn evaluate(&self, environment: &mut Environment) -> Self {
        match self {
            expression @ Self::Number(_) => expression.clone(),
            expression @ Self::Boolean(_) => expression.clone(),
            Self::Variable(value) => environment.get(value),
            Self::Add(left, right) => {
                let left = left.evaluate(environment);
                let right = right.evaluate(environment);
                Self::Number(left.unwrap_number() + right.unwrap_number())
            }
            Self::Subtract(left, right) => {
                let left = left.evaluate(environment);
                let right = right.evaluate(environment);
                Self::Number(left.unwrap_number() - right.unwrap_number())
            }
            Self::Multiply(left, right) => {
                let left = left.evaluate(environment);
                let right = right.evaluate(environment);
                Self::Number(left.unwrap_number() * right.unwrap_number())
            }
            Self::Divide(left, right) => {
                let left = left.evaluate(environment);
                let right = right.evaluate(environment);
                Self::Number(left.unwrap_number() / right.unwrap_number())
            }
            Self::Not(expression) => {
                let expression = expression.evaluate(environment);
                Self::Boolean(!expression.unwrap_boolean())
            }
            Self::And(left, right) => {
                let left = left.evaluate(environment);
                let right = right.evaluate(environment);
                Self::Boolean(left.unwrap_boolean() && right.unwrap_boolean())
            }
            Self::Or(left, right) => {
                let left = left.evaluate(environment);
                let right = right.evaluate(environment);
                Self::Boolean(left.unwrap_boolean() || right.unwrap_boolean())
            }
            Self::Equal(left, right) => {
                let left = left.evaluate(environment);
                let right = right.evaluate(environment);
                Self::Boolean(left == right)
            }
            Self::NotEqual(left, right) => {
                let left = left.evaluate(environment);
                let right = right.evaluate(environment);
                Self::Boolean(left != right)
            }
            Self::LessThan(left, right) => {
                let left = left.evaluate(environment);
                let right = right.evaluate(environment);
                Self::Boolean(left.unwrap_number() < right.unwrap_number())
            }
            Self::LessThanOrEqual(left, right) => {
                let left = left.evaluate(environment);
                let right = right.evaluate(environment);
                Self::Boolean(left.unwrap_number() <= right.unwrap_number())
            }
            Self::GreaterThan(left, right) => {
                let left = left.evaluate(environment);
                let right = right.evaluate(environment);
                Self::Boolean(left.unwrap_number() > right.unwrap_number())
            }
            Self::GreaterThanOrEqual(left, right) => {
                let left = left.evaluate(environment);
                let right = right.evaluate(environment);
                Self::Boolean(left.unwrap_number() >= right.unwrap_number())
            }
        }
    }
}

impl Evaluator for Statement {
    fn evaluate(&self, environment: &mut Environment) -> Self {
        match self {
            Self::Expression(expression) => Self::Expression(expression.evaluate(environment)),
            Self::Assign(name, expression) => {
                let expression = expression.evaluate(environment);
                environment.insert(name, expression);
                Self::Nothing
            }
            Self::If(condition, consequence, alternative) => {
                let condition = condition.evaluate(environment);

                if condition.unwrap_boolean() {
                    consequence.evaluate(environment)
                } else {
                    alternative.evaluate(environment)
                }
            }
            Self::Sequence(first, second) => {
                first.evaluate(environment);
                second.evaluate(environment);
                Self::Nothing
            }
            statement @ Self::While(condition, body) => {
                let condition = condition.evaluate(environment);

                if condition.unwrap_boolean() {
                    body.evaluate(environment);
                    statement.evaluate(environment)
                } else {
                    Self::Nothing
                }
            }
            statement @ Self::Nothing => statement.clone(),
        }
    }
}
