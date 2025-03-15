use crate::regular_expression::regular_expression_pattern::RegularExpressionPattern as Pattern;
use anyhow::{Result, bail};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, none_of},
    combinator::{eof, map},
    sequence::delimited,
};

pub fn parse_pattern(input: &str) -> Result<Pattern> {
    match map((parse_choose, eof), |(pattern, _)| pattern).parse(input) {
        Ok((_, pattern)) => Ok(pattern),
        Err(_) => bail!("failed to parse"),
    }
}

fn parse_choose(input: &str) -> IResult<&str, Pattern> {
    alt((
        map(
            (parse_concatenate_or_empty, char('|'), parse_choose),
            |(first, _, second)| Pattern::Choose(Box::new(first), Box::new(second)),
        ),
        parse_concatenate_or_empty,
    ))
    .parse(input)
}

fn parse_concatenate_or_empty(input: &str) -> IResult<&str, Pattern> {
    alt((parse_concatenate, parse_empty)).parse(input)
}

fn parse_concatenate(input: &str) -> IResult<&str, Pattern> {
    alt((
        map((parse_repeat, parse_concatenate), |(first, second)| {
            Pattern::Concatenate(Box::new(first), Box::new(second))
        }),
        parse_repeat,
    ))
    .parse(input)
}

fn parse_empty(input: &str) -> IResult<&str, Pattern> {
    map(tag(""), |_| Pattern::Empty).parse(input)
}

fn parse_repeat(input: &str) -> IResult<&str, Pattern> {
    alt((
        map((parse_brackets, char('*')), |(pattern, _)| {
            Pattern::Repeat(Box::new(pattern))
        }),
        parse_brackets,
    ))
    .parse(input)
}

fn parse_brackets(input: &str) -> IResult<&str, Pattern> {
    alt((
        map(delimited(char('('), parse_choose, char(')')), |pattern| {
            pattern
        }),
        parse_literal,
    ))
    .parse(input)
}

fn parse_literal(input: &str) -> IResult<&str, Pattern> {
    map(none_of("|*()\\"), Pattern::Literal).parse(input)
}
