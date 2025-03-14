use crate::{
    finite_automaton_state::FiniteAutomatonState,
    nondeterministic_finite_automaton_rule_book::NondeterministicFiniteAutomatonRuleBook,
};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct NondeterministicFiniteAutomaton<T> {
    current_states: HashSet<T>,
    accept_states: HashSet<T>,
    rule_book: NondeterministicFiniteAutomatonRuleBook<T>,
}

impl<T> NondeterministicFiniteAutomaton<T> {
    pub fn new(
        current_states: HashSet<T>,
        accept_states: HashSet<T>,
        rule_book: NondeterministicFiniteAutomatonRuleBook<T>,
    ) -> Self {
        Self {
            current_states,
            accept_states,
            rule_book,
        }
    }
}

impl<T: FiniteAutomatonState> NondeterministicFiniteAutomaton<T> {
    pub fn accepting(&self) -> bool {
        !self
            .current_states
            .intersection(&self.accept_states)
            .collect::<HashSet<_>>()
            .is_empty()
    }

    pub fn read_character(&mut self, character: char) {
        self.current_states = self
            .rule_book
            .next_state(self.current_states.clone(), character);
    }

    pub fn read_string<S: Into<String>>(&mut self, string: S) {
        for character in string.into().chars() {
            self.read_character(character);
        }
    }
}
