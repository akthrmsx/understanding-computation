use deterministic_finite_automaton_design::DeterministicFiniteAutomatonDesign;
use deterministic_finite_automaton_rule_book::DeterministicFiniteAutomatonRuleBook;
use finite_automaton_rule::FiniteAutomatonRule;
use nondeterministic_finite_automaton_design::NondeterministicFiniteAutomatonDesign;
use nondeterministic_finite_automaton_rule_book::NondeterministicFiniteAutomatonRuleBook;

mod deterministic_finite_automaton;
mod deterministic_finite_automaton_design;
mod deterministic_finite_automaton_rule_book;
mod finite_automaton_rule;
mod finite_automaton_state;
mod nondeterministic_finite_automaton;
mod nondeterministic_finite_automaton_design;
mod nondeterministic_finite_automaton_rule_book;

fn main() {
    let design = DeterministicFiniteAutomatonDesign::new(
        1,
        vec![3],
        DeterministicFiniteAutomatonRuleBook::new(vec![
            FiniteAutomatonRule::new(1, 'a', 2),
            FiniteAutomatonRule::new(1, 'b', 1),
            FiniteAutomatonRule::new(2, 'a', 2),
            FiniteAutomatonRule::new(2, 'b', 3),
            FiniteAutomatonRule::new(3, 'a', 3),
            FiniteAutomatonRule::new(3, 'b', 3),
        ]),
    );

    println!("{}", design.accepts("a"));
    println!("{}", design.accepts("baa"));
    println!("{}", design.accepts("baba"));

    let design = NondeterministicFiniteAutomatonDesign::new(
        1,
        [4].into(),
        NondeterministicFiniteAutomatonRuleBook::new(vec![
            FiniteAutomatonRule::new(1, 'a', 1),
            FiniteAutomatonRule::new(1, 'b', 1),
            FiniteAutomatonRule::new(1, 'b', 2),
            FiniteAutomatonRule::new(2, 'a', 3),
            FiniteAutomatonRule::new(2, 'b', 3),
            FiniteAutomatonRule::new(3, 'a', 4),
            FiniteAutomatonRule::new(3, 'b', 4),
        ]),
    );

    println!("{}", design.accepts("bab"));
    println!("{}", design.accepts("bbbbb"));
    println!("{}", design.accepts("bbabb"));
}
