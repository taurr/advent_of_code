use anyhow::Result;
use std::path::Path;

pub fn solve_puzzle(input_path: &Path) -> Result<()> {
    let input = include_str!("../assets/day.txt");
    println!("Part1: {}", part1(parse_input(input)?)?);
    println!("Part2: {}", part2(parse_input(input)?)?);
    Ok(())
}

fn parse_input(input: &str) -> Result<Vec<String>> {
    todo!()
}

fn part1(input: Vec<String>) -> Result<usize> {
    todo!()
}

fn part2(input: Vec<String>) -> Result<usize> {
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
