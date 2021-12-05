use anyhow::Result;
use serde::Deserialize;
use std::path::Path;

use crate::read_csv;

#[derive(Debug, Deserialize, PartialEq)]
struct Day2Input {
    direction: Direction,
    steps: i32,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum Direction {
    Up,
    Down,
    Forward,
}

#[derive(Debug, PartialEq, Default)]
struct SubmarinePosition {
    horizontal: i32,
    vertical: i32,
    aim: i32,
}

pub fn solve_puzzle(input_path: &Path) -> Result<()> {
    let input = read_csv(input_path, &["direction", "steps"], b' ')?;
    println!("Day2, Part1: {}", part1(&input));
    println!("Day2, Part2: {}", part2(&input));
    Ok(())
}

fn part1(input: &[Day2Input]) -> i32 {
    let r = input.iter().fold(
        SubmarinePosition::default(),
        |mut acc, Day2Input { direction, steps }| {
            match direction {
                Direction::Up => acc.vertical -= steps,
                Direction::Down => acc.vertical += steps,
                Direction::Forward => acc.horizontal += steps,
            };
            acc
        },
    );
    r.horizontal * r.vertical
}

fn part2(input: &[Day2Input]) -> i32 {
    let r = input.iter().fold(
        SubmarinePosition::default(),
        |mut acc, Day2Input { direction, steps }| {
            match direction {
                Direction::Up => acc.aim -= steps,
                Direction::Down => acc.aim += steps,
                Direction::Forward => {
                    acc.horizontal += steps;
                    acc.vertical += acc.aim * steps;
                }
            };
            acc
        },
    );
    r.horizontal * r.vertical
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{create_input, read_csv};
    use anyhow::Result;

    fn example_input() -> Result<Vec<Day2Input>, anyhow::Error> {
        let dir = tempfile::tempdir()?;
        let input_path = create_input(
            &dir,
            &[
                "forward 5",
                "down 5",
                "forward 8",
                "up 3",
                "down 8",
                "forward 2",
            ],
        )?;
        let input = read_csv(input_path.as_path(), &["direction", "steps"], b' ')?;
        Ok(input)
    }

    #[test]
    fn test_part1() -> Result<()> {
        let input = example_input()?;
        assert_eq!(150, part1(&input));
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let input = example_input()?;
        assert_eq!(900, part2(&input));
        Ok(())
    }
}
