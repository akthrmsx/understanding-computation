use environment::Environment;
use expression::Expression;
use machine::Machine;
use statement::Statement;

mod environment;
mod expression;
mod machine;
mod reducer;
mod statement;

pub fn main() {
    let statement = Statement::While(
        Expression::LessThan(
            Box::new(Expression::Variable("x".into())),
            Box::new(Expression::Number(5)),
        ),
        Box::new(Statement::Assign(
            "x".into(),
            Expression::Multiply(
                Box::new(Expression::Variable("x".into())),
                Box::new(Expression::Number(3)),
            ),
        )),
    );

    let mut environment = Environment::new();
    environment.insert("x", Expression::Number(1));

    Machine::new(statement, environment).run();
}
