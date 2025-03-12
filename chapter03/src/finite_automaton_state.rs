pub trait FiniteAutomatonState: Clone + PartialEq {}

impl FiniteAutomatonState for usize {}
