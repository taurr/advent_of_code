use anyhow::Result;
use itertools::Itertools;
use std::{fs::read_to_string, path::Path};

pub fn solve_puzzle(input_path: &Path) -> Result<()> {
    let input = parse_input(&read_to_string(input_path)?);
    println!("Day7, Part1: {}", part1(&input));
    println!("Day7, Part2: {}", part2(&input));
    Ok(())
}

fn part1(input: &[i32]) -> i32 {
    minmax_range(input)
        .map(|pos| {
            input
                .iter()
                .fold(0, |fuel, crab_pos| fuel + (crab_pos - pos).abs())
        })
        .min()
        .unwrap()
}

fn part2(input: &[i32]) -> i32 {
    fn triangular(n: i32) -> i32 {
        n * (n + 1) / 2
    }

    minmax_range(input)
        .map(|pos| {
            input.iter().fold(0, |fuel, crab_pos| {
                fuel + triangular((crab_pos - pos).abs())
            })
        })
        .min()
        .unwrap()
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|s| s.parse::<i32>().expect("unable to parse number"))
        .collect_vec()
}

fn minmax_range(input: &[i32]) -> std::ops::RangeInclusive<i32> {
    match input.iter().minmax() {
        itertools::MinMaxResult::NoElements => 0..=0,
        itertools::MinMaxResult::OneElement(p) => *p..=*p,
        itertools::MinMaxResult::MinMax(a, b) => *a..=*b,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = parse_input("16,1,2,0,4,2,7,1,2,14");
        assert_eq!(37, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = parse_input("16,1,2,0,4,2,7,1,2,14");
        assert_eq!(168, part2(&input));
    }
}
