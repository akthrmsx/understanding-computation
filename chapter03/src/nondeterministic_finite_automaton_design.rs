use crate::{
    finite_automaton_state::FiniteAutomatonState,
    nondeterministic_finite_automaton::NondeterministicFiniteAutomaton,
    nondeterministic_finite_automaton_rule_book::NondeterministicFiniteAutomatonRuleBook,
};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct NondeterministicFiniteAutomatonDesign<T> {
    start_state: T,
    accept_states: HashSet<T>,
    rule_book: NondeterministicFiniteAutomatonRuleBook<T>,
}

impl<T> NondeterministicFiniteAutomatonDesign<T> {
    pub fn new(
        start_state: T,
        accept_states: HashSet<T>,
        rule_book: NondeterministicFiniteAutomatonRuleBook<T>,
    ) -> Self {
        Self {
            start_state,
            accept_states,
            rule_book,
        }
    }
}

impl<T: FiniteAutomatonState> NondeterministicFiniteAutomatonDesign<T> {
    pub fn accepts<S: Into<String>>(&self, string: S) -> bool {
        let mut automaton = NondeterministicFiniteAutomaton::from(self.clone());
        automaton.read_string(string);
        automaton.accepting()
    }
}

impl<T: FiniteAutomatonState> From<NondeterministicFiniteAutomatonDesign<T>>
    for NondeterministicFiniteAutomaton<T>
{
    fn from(design: NondeterministicFiniteAutomatonDesign<T>) -> Self {
        NondeterministicFiniteAutomaton::new(
            [design.start_state].into(),
            design.accept_states,
            design.rule_book,
        )
    }
}
