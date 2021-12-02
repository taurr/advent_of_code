use anyhow::Result;
use itertools::*;
use serde::Deserialize;
use std::path::Path;

use crate::read_csv;

#[derive(Debug, Deserialize, PartialEq)]
struct Day1Input {
    depth: f64,
}

pub fn solve_puzzle(input_path: &Path) -> Result<()> {
    let input = read_csv(input_path, &["depth"], b'\t')?;
    println!("Day1, Part1: {}", day1_part1(&input));
    println!("Day1, Part2: {}", day1_part2(&input));
    Ok(())
}

fn day1_part1(input: &[Day1Input]) -> usize {
    input
        .iter()
        .tuple_windows()
        .filter(|(a, b)| b.depth > a.depth)
        .count()
}

fn day1_part2(input: &[Day1Input]) -> usize {
    input
        .iter()
        .tuple_windows()
        .map(|(a, b, c)| a.depth + b.depth + c.depth)
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{create_input, read_csv};
    use anyhow::Result;

    #[test]
    fn example_day1_part1() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let path = create_input(
            &dir,
            &[
                "199", "200", "208", "210", "200", "207", "240", "269", "260", "263",
            ],
        )?;
        let input = read_csv(path.as_path(), &["depth"], b'\t')?;
        assert_eq!(7, day1_part1(&input));
        Ok(())
    }

    #[test]
    fn example_day1_part2() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let path = create_input(
            &dir,
            &[
                "199", "200", "208", "210", "200", "207", "240", "269", "260", "263",
            ],
        )?;
        let input = read_csv(path.as_path(), &["depth"], b'\t')?;
        assert_eq!(5, day1_part2(&input));
        Ok(())
    }
}
