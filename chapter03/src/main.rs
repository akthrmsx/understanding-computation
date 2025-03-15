use anyhow::Result;
use chapter03::parser::parse_pattern;

fn main() -> Result<()> {
    let pattern = parse_pattern("(a(|b))*")?;

    println!("{}", pattern.matches("abaab"));
    println!("{}", pattern.matches("abba"));

    Ok(())
}
