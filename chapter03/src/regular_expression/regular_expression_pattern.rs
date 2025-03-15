use super::regular_expression_context::RegularExpressionContext as Context;
use crate::finite_automaton::{
    finite_automaton_rule::FiniteAutomatonRule as Rule,
    nondeterministic_finite_automaton_design::NondeterministicFiniteAutomatonDesign as Design,
};

#[derive(Debug, Clone)]
pub enum RegularExpressionPattern {
    Empty,
    Literal(char),
    Concatenate(Box<Self>, Box<Self>),
    Choose(Box<Self>, Box<Self>),
    Repeat(Box<Self>),
}

impl RegularExpressionPattern {
    pub fn matches<S: Into<String>>(&self, string: S) -> bool {
        let mut context = Context::default();
        let design = self.to_design(&mut context);
        design.accepts(string)
    }

    fn to_design(&self, context: &mut Context) -> Design<usize> {
        match self {
            Self::Empty => {
                let start_state = context.next_state();
                let accept_state = context.next_state();
                let accept_states = [accept_state].into();

                let mut design = Design::without_rule_book(start_state, accept_states);
                design.add_rule(Rule::new(start_state, None, accept_state));

                design
            }
            Self::Literal(character) => {
                let start_state = context.next_state();
                let accept_state = context.next_state();
                let accept_states = [accept_state].into();

                let mut design = Design::without_rule_book(start_state, accept_states);
                design.add_rule(Rule::new(start_state, Some(*character), accept_state));

                design
            }
            Self::Concatenate(first, second) => {
                let first = first.to_design(context);
                let second = second.to_design(context);

                let start_state = first.start_state();
                let accept_states = second.accept_states();

                let mut design = Design::without_rule_book(start_state, accept_states);
                design.merge_rule_book(first.rule_book());
                design.merge_rule_book(second.rule_book());

                for first_accept_state in first.accept_states() {
                    design.add_rule(Rule::new(first_accept_state, None, second.start_state()));
                }

                design
            }
            Self::Choose(first, second) => {
                let first = first.to_design(context);
                let second = second.to_design(context);

                let start_state = context.next_state();
                let accept_state = context.next_state();
                let accept_states = [accept_state].into();

                let mut design = Design::without_rule_book(start_state, accept_states);
                design.merge_rule_book(first.rule_book());
                design.merge_rule_book(second.rule_book());
                design.add_rule(Rule::new(start_state, None, first.start_state()));
                design.add_rule(Rule::new(start_state, None, second.start_state()));

                for first_accept_state in first.accept_states() {
                    design.add_rule(Rule::new(first_accept_state, None, accept_state));
                }

                for second_accept_state in second.accept_states() {
                    design.add_rule(Rule::new(second_accept_state, None, accept_state));
                }

                design
            }
            Self::Repeat(pattern) => {
                let pattern = pattern.to_design(context);

                let start_state = context.next_state();
                let accept_states = pattern
                    .accept_states()
                    .union(&[start_state].into())
                    .cloned()
                    .collect();

                let mut design = Design::without_rule_book(start_state, accept_states);
                design.merge_rule_book(pattern.rule_book());
                design.add_rule(Rule::new(start_state, None, pattern.start_state()));

                for pattern_accept_state in pattern.accept_states() {
                    design.add_rule(Rule::new(pattern_accept_state, None, pattern.start_state()));
                }

                design
            }
        }
    }
}
