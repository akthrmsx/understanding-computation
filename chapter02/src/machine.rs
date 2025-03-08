use crate::{environment::Environment, reducer::Reducer, statement::Statement};

#[derive(Debug, Clone, PartialEq)]
pub struct Machine {
    statement: Statement,
    environment: Environment,
}

impl Machine {
    pub fn new(statement: Statement, environment: Environment) -> Self {
        Self {
            statement,
            environment,
        }
    }

    pub fn run(&mut self) {
        while self.statement.is_reducible() {
            println!("{}, {}", self.statement, self.environment);
            self.statement = self.statement.reduce(&mut self.environment);
        }

        println!("{}, {}", self.statement, self.environment);
    }
}
