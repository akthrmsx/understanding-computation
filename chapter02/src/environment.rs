use crate::expression::Expression;
use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    variables: HashMap<String, Expression>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: &str, value: Expression) {
        self.variables.insert(key.into(), value);
    }

    pub fn get(&self, key: &str) -> Expression {
        match self.variables.get(key) {
            Some(value) => value.clone(),
            None => panic!("key '{}' is not found", key),
        }
    }
}

impl Display for Environment {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{{ {} }}",
            self.variables
                .iter()
                .map(|(k, v)| format!("{} = {}", k, v))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
