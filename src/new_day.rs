use anyhow::Result;
use std::path::Path;

pub fn solve_puzzle(input_path: &Path) -> Result<()> {
    let input = std::fs::read_to_string(input_path)?;
    let input = parse_input(&input);
    println!("Day4, Part1: {}", part1(&input));
    println!("Day4, Part2: {}", part2(&input));
    Ok(())
}

type PuzzleInput = String;

fn parse_input(_input: &str) -> Vec<PuzzleInput> {
    vec![]
}

fn part1(_input: &[PuzzleInput]) -> u32 {
    0
}

fn part2(_input: &[PuzzleInput]) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    fn example_input() -> String {
        let input = indoc! {"
        "};
        input.to_string()
    }

    #[test]
    fn test_part1() {
        let input = example_input();
        let input = parse_input(&input);
        let result = part1(&input);
        assert_eq!(4512, result);
    }

    #[test]
    fn example_part2() {
        let input = example_input();
        let input = parse_input(&input);
        let result = part2(&input);
        assert_eq!(4512, result);
    }
}
