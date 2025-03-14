use crate::{
    finite_automaton_rule::FiniteAutomatonRule, finite_automaton_state::FiniteAutomatonState,
};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct NondeterministicFiniteAutomatonRuleBook<T> {
    rules: Vec<FiniteAutomatonRule<T>>,
}

impl<T> NondeterministicFiniteAutomatonRuleBook<T> {
    pub fn new(rules: Vec<FiniteAutomatonRule<T>>) -> Self {
        Self { rules }
    }
}

impl<T: FiniteAutomatonState> NondeterministicFiniteAutomatonRuleBook<T> {
    pub fn next_state(&self, states: HashSet<T>, character: char) -> HashSet<T> {
        states
            .iter()
            .flat_map(|state| self.follow_rules_for(state.clone(), character))
            .collect()
    }

    pub fn follow_rules_for(&self, state: T, character: char) -> HashSet<T> {
        self.rule_for(state, character)
            .iter()
            .map(|rule| rule.follow())
            .collect()
    }

    pub fn rule_for(&self, state: T, character: char) -> Vec<FiniteAutomatonRule<T>> {
        self.rules
            .iter()
            .filter(|rule| rule.applies_to(state.clone(), character))
            .cloned()
            .collect()
    }
}
