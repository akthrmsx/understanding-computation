use chapter03::regular_expression::regular_expression_pattern::RegularExpressionPattern as Pattern;

fn main() {
    let pattern = Pattern::Repeat(Box::new(Pattern::Concatenate(
        Box::new(Pattern::Literal('a')),
        Box::new(Pattern::Choose(
            Box::new(Pattern::Empty),
            Box::new(Pattern::Literal('b')),
        )),
    )));

    println!("{}", pattern.matches(""));
    println!("{}", pattern.matches("a"));
    println!("{}", pattern.matches("ab"));
    println!("{}", pattern.matches("aba"));
    println!("{}", pattern.matches("abab"));
    println!("{}", pattern.matches("abaab"));
    println!("{}", pattern.matches("abba"));
}
