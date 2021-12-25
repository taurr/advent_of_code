use anyhow::Result;

use self::parser::{parse_input, ParsedInput};

pub fn solve_puzzle() -> Result<()> {
    const INPUT: &str = include_str!("../assets/day.txt");
    println!("Part1: {}", part1(parse_input(INPUT)?)?);
    println!("Part2: {}", part2(parse_input(INPUT)?)?);
    Ok(())
}

mod parser {
    use nom::{
        bytes::complete::tag,
        character::complete::*,
        multi::{fold_many0, many1},
        IResult,
    };

    pub(crate) struct ParsedInput {}

    pub(crate) fn parse_input(input: &'static str) -> anyhow::Result<ParsedInput> {
        todo!();
    }
}

fn part1(input: ParsedInput) -> Result<usize> {
    todo!()
}

fn part2(input: ParsedInput) -> Result<usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
    "};

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(0, part1(parse_input(INPUT)?)?);
        Ok(())
    }
}
