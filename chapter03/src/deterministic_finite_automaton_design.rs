use crate::{
    deterministic_finite_automaton::DeterministicFiniteAutomaton,
    deterministic_finite_automaton_rule_book::DeterministicFiniteAutomatonRuleBook,
    finite_automaton_state::FiniteAutomatonState,
};

#[derive(Debug, Clone, PartialEq)]
pub struct DeterministicFiniteAutomatonDesign<T> {
    start_state: T,
    accept_states: Vec<T>,
    rule_book: DeterministicFiniteAutomatonRuleBook<T>,
}

impl<T> DeterministicFiniteAutomatonDesign<T> {
    pub fn new(
        start_state: T,
        accept_states: Vec<T>,
        rule_book: DeterministicFiniteAutomatonRuleBook<T>,
    ) -> Self {
        Self {
            start_state,
            accept_states,
            rule_book,
        }
    }
}

impl<T: FiniteAutomatonState> DeterministicFiniteAutomatonDesign<T> {
    pub fn accepts<S: Into<String>>(&self, string: S) -> bool {
        let mut automaton = DeterministicFiniteAutomaton::from(self.clone());
        automaton.read_string(string);
        automaton.accepting()
    }
}

impl<T> From<DeterministicFiniteAutomatonDesign<T>> for DeterministicFiniteAutomaton<T> {
    fn from(design: DeterministicFiniteAutomatonDesign<T>) -> Self {
        DeterministicFiniteAutomaton::new(
            design.start_state,
            design.accept_states,
            design.rule_book,
        )
    }
}
