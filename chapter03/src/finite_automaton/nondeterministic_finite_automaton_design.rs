use super::{
    finite_automaton_rule::FiniteAutomatonRule, finite_automaton_state::FiniteAutomatonState,
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
    pub fn without_rule_book(start_state: T, accept_states: HashSet<T>) -> Self {
        Self {
            start_state,
            accept_states,
            rule_book: NondeterministicFiniteAutomatonRuleBook::default(),
        }
    }

    pub fn add_rule(&mut self, rule: FiniteAutomatonRule<T>) {
        self.rule_book.add_rule(rule);
    }
}

impl<T: FiniteAutomatonState> NondeterministicFiniteAutomatonDesign<T> {
    pub fn start_state(&self) -> T {
        self.start_state.clone()
    }

    pub fn accept_states(&self) -> HashSet<T> {
        self.accept_states.clone()
    }

    pub fn rule_book(&self) -> NondeterministicFiniteAutomatonRuleBook<T> {
        self.rule_book.clone()
    }

    pub fn accepts<S: Into<String>>(&self, string: S) -> bool {
        let mut automaton = NondeterministicFiniteAutomaton::from(self.clone());
        automaton.read_string(string);
        automaton.accepting()
    }

    pub fn merge_rule_book(&mut self, rule_book: NondeterministicFiniteAutomatonRuleBook<T>) {
        for rule in rule_book.rules() {
            self.add_rule(rule);
        }
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
