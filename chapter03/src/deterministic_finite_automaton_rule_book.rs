use crate::{
    finite_automaton_rule::FiniteAutomatonRule, finite_automaton_state::FiniteAutomatonState,
};

#[derive(Debug, Clone)]
pub struct DeterministicFiniteAutomatonRuleBook<T> {
    rules: Vec<FiniteAutomatonRule<T>>,
}

impl<T> DeterministicFiniteAutomatonRuleBook<T> {
    pub fn new(rules: Vec<FiniteAutomatonRule<T>>) -> Self {
        Self { rules }
    }
}

impl<T: FiniteAutomatonState> DeterministicFiniteAutomatonRuleBook<T> {
    pub fn next_state(&self, state: T, character: char) -> Option<T> {
        self.rule_for(state, character).map(|rule| rule.follow())
    }

    pub fn rule_for(&self, state: T, character: char) -> Option<FiniteAutomatonRule<T>> {
        self.rules
            .iter()
            .find(|rule| rule.applies_to(state.clone(), character))
            .cloned()
    }
}
