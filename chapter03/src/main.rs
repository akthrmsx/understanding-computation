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
            FiniteAutomatonRule::new(1, Some('a'), 2),
            FiniteAutomatonRule::new(1, Some('b'), 1),
            FiniteAutomatonRule::new(2, Some('a'), 2),
            FiniteAutomatonRule::new(2, Some('b'), 3),
            FiniteAutomatonRule::new(3, Some('a'), 3),
            FiniteAutomatonRule::new(3, Some('b'), 3),
        ]),
    );

    println!("{}", design.accepts("a"));
    println!("{}", design.accepts("baa"));
    println!("{}", design.accepts("baba"));

    let design = NondeterministicFiniteAutomatonDesign::new(
        1,
        [4].into(),
        NondeterministicFiniteAutomatonRuleBook::new(vec![
            FiniteAutomatonRule::new(1, Some('a'), 1),
            FiniteAutomatonRule::new(1, Some('b'), 1),
            FiniteAutomatonRule::new(1, Some('b'), 2),
            FiniteAutomatonRule::new(2, Some('a'), 3),
            FiniteAutomatonRule::new(2, Some('b'), 3),
            FiniteAutomatonRule::new(3, Some('a'), 4),
            FiniteAutomatonRule::new(3, Some('b'), 4),
        ]),
    );

    println!("{}", design.accepts("bab"));
    println!("{}", design.accepts("bbbbb"));
    println!("{}", design.accepts("bbabb"));

    let design = NondeterministicFiniteAutomatonDesign::new(
        1,
        [2, 4].into(),
        NondeterministicFiniteAutomatonRuleBook::new(vec![
            FiniteAutomatonRule::new(1, None, 2),
            FiniteAutomatonRule::new(1, None, 4),
            FiniteAutomatonRule::new(2, Some('a'), 3),
            FiniteAutomatonRule::new(3, Some('a'), 2),
            FiniteAutomatonRule::new(4, Some('a'), 5),
            FiniteAutomatonRule::new(5, Some('a'), 6),
            FiniteAutomatonRule::new(6, Some('a'), 4),
        ]),
    );

    println!("{}", design.accepts("aa"));
    println!("{}", design.accepts("aaa"));
    println!("{}", design.accepts("aaaaa"));
    println!("{}", design.accepts("aaaaaa"));
}
