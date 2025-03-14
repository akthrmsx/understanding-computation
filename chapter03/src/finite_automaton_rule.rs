use crate::finite_automaton_state::FiniteAutomatonState;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FiniteAutomatonRule<T> {
    state: T,
    character: Option<char>,
    next_state: T,
}

impl<T> FiniteAutomatonRule<T> {
    pub fn new(state: T, character: Option<char>, next_state: T) -> Self {
        Self {
            state,
            character,
            next_state,
        }
    }
}

impl<T: FiniteAutomatonState> FiniteAutomatonRule<T> {
    pub fn applies_to(&self, state: T, character: Option<char>) -> bool {
        self.state == state && self.character == character
    }

    pub fn follow(&self) -> T {
        self.next_state.clone()
    }
}
