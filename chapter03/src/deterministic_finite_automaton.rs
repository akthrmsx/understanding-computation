use crate::{
    deterministic_finite_automaton_rule_book::DeterministicFiniteAutomatonRuleBook,
    finite_automaton_state::FiniteAutomatonState,
};

#[derive(Debug, Clone, PartialEq)]
pub struct DeterministicFiniteAutomaton<T> {
    current_state: T,
    accept_states: Vec<T>,
    rule_book: DeterministicFiniteAutomatonRuleBook<T>,
}

impl<T> DeterministicFiniteAutomaton<T> {
    pub fn new(
        current_state: T,
        accept_states: Vec<T>,
        rule_book: DeterministicFiniteAutomatonRuleBook<T>,
    ) -> Self {
        Self {
            current_state,
            accept_states,
            rule_book,
        }
    }
}

impl<T: FiniteAutomatonState> DeterministicFiniteAutomaton<T> {
    pub fn accepting(&self) -> bool {
        self.accept_states
            .iter()
            .any(|state| *state == self.current_state)
    }

    pub fn read_character(&mut self, character: char) {
        if let Some(state) = self
            .rule_book
            .next_state(self.current_state.clone(), character)
        {
            self.current_state = state;
        }
    }

    pub fn read_string<S: Into<String>>(&mut self, string: S) {
        for character in string.into().chars() {
            self.read_character(character);
        }
    }
}
