use anyhow::Result;
use std::path::Path;

pub fn solve_puzzle(input_path: &Path) -> Result<()> {
    let input = std::fs::read_to_string(input_path)?;
    println!("Day10, Part1: {}", part1(parse_input(&input)));
    println!("Day10, Part2: {}", part2(parse_input(&input)));
    Ok(())
}

fn parse_input(input: &str) -> Vec<String> {
    todo!()
}

fn part1(input: Vec<String>) -> u32 {
    todo!()
}

fn part2(input: Vec<String>) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
    "};

    #[test]
    fn test_part1() {
        assert_eq!(0, part1(parse_input(INPUT)));
    }
}
