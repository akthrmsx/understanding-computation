use std::{fmt::Debug, hash::Hash};

pub trait FiniteAutomatonState: Debug + Clone + PartialEq + Eq + Hash {}

impl FiniteAutomatonState for usize {}
