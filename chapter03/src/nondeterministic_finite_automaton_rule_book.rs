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
    pub fn next_state(&self, states: HashSet<T>, character: Option<char>) -> HashSet<T> {
        states
            .iter()
            .flat_map(|state| self.follow_rules_for(state.clone(), character))
            .collect()
    }

    pub fn follow_rules_for(&self, state: T, character: Option<char>) -> HashSet<T> {
        self.rule_for(state, character)
            .iter()
            .map(|rule| rule.follow())
            .collect()
    }

    pub fn rule_for(&self, state: T, character: Option<char>) -> Vec<FiniteAutomatonRule<T>> {
        self.rules
            .iter()
            .filter(|rule| rule.applies_to(state.clone(), character))
            .cloned()
            .collect()
    }

    pub fn follow_free_move(&self, states: HashSet<T>) -> HashSet<T> {
        let more_states = self.next_state(states.clone(), None);

        if more_states.is_subset(&states) {
            states
        } else {
            self.follow_free_move(states.union(&more_states).cloned().collect())
        }
    }
}
