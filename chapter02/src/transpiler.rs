use crate::{expression::Expression, statement::Statement};

pub trait Transpiler {
    fn transpile(&self) -> String;
}

impl Transpiler for Expression {
    fn transpile(&self) -> String {
        match self {
            Self::Number(value) => format!("-> e {{ {} }}", value),
            Self::Boolean(value) => format!("-> e {{ {} }}", value),
            Self::Variable(value) => format!("-> e {{ e[:{}] }}", value),
            Self::Add(left, right) => format!(
                "-> e {{ ({}).call(e) + ({}).call(e) }}",
                left.transpile(),
                right.transpile()
            ),
            Self::Subtract(left, right) => format!(
                "-> e {{ ({}).call(e) - ({}).call(e) }}",
                left.transpile(),
                right.transpile()
            ),
            Self::Multiply(left, right) => format!(
                "-> e {{ ({}).call(e) * ({}).call(e) }}",
                left.transpile(),
                right.transpile()
            ),
            Self::Divide(left, right) => format!(
                "-> e {{ ({}).call(e) / ({}).call(e) }}",
                left.transpile(),
                right.transpile()
            ),
            Self::Not(expression) => format!("-> e {{ !({}).call(e) }}", expression),
            Self::And(left, right) => format!(
                "-> e {{ ({}).call(e) && ({}).call(e) }}",
                left.transpile(),
                right.transpile()
            ),
            Self::Or(left, right) => format!(
                "-> e {{ ({}).call(e) || ({}).call(e) }}",
                left.transpile(),
                right.transpile()
            ),
            Self::Equal(left, right) => format!(
                "-> e {{ ({}).call(e) == ({}).call(e) }}",
                left.transpile(),
                right.transpile()
            ),
            Self::NotEqual(left, right) => format!(
                "-> e {{ ({}).call(e) != ({}).call(e) }}",
                left.transpile(),
                right.transpile()
            ),
            Self::LessThan(left, right) => format!(
                "-> e {{ ({}).call(e) < ({}).call(e) }}",
                left.transpile(),
                right.transpile()
            ),
            Self::LessThanOrEqual(left, right) => format!(
                "-> e {{ ({}).call(e) <= ({}).call(e) }}",
                left.transpile(),
                right.transpile()
            ),
            Self::GreaterThan(left, right) => format!(
                "-> e {{ ({}).call(e) > ({}).call(e) }}",
                left.transpile(),
                right.transpile()
            ),
            Self::GreaterThanOrEqual(left, right) => format!(
                "-> e {{ ({}).call(e) >= ({}).call(e) }}",
                left.transpile(),
                right.transpile()
            ),
        }
    }
}

impl Transpiler for Statement {
    fn transpile(&self) -> String {
        match self {
            Self::Expression(expression) => expression.transpile(),
            Self::Assign(name, expression) => format!(
                "-> e {{ e.merge({{ :{} => ({}).call(e) }}) }}",
                name,
                expression.transpile(),
            ),
            Self::If(condition, consequence, alternative) => {
                format!(
                    "-> e {{ if ({}).call(e) then ({}).call(e) else ({}).call(e) end }}",
                    condition.transpile(),
                    consequence.transpile(),
                    alternative.transpile(),
                )
            }
            Self::Sequence(first, second) => {
                format!(
                    "-> e {{ ({}).call(({}).call(e)) }}",
                    second.transpile(),
                    first.transpile(),
                )
            }
            Self::While(condition, body) => format!(
                "-> e {{ while ({}).call(e); e = ({}).call(e); end; e; }}",
                condition.transpile(),
                body.transpile(),
            ),
            Self::Nothing => "-> e {{ e }}".into(),
        }
    }
}
