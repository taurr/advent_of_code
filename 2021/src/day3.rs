use anyhow::Result;
use serde::Deserialize;
use std::{collections::HashMap, path::Path};

use crate::read_csv;

#[derive(Debug, Deserialize, PartialEq)]
struct Day3Input {
    bits: String,
}

pub fn solve_puzzle(input_path: &Path) -> Result<()> {
    let input = read_csv(input_path, &["bits"], b' ')?;
    println!("Day3, Part1: {}", part1(&input));
    println!("Day3, Part2: {}", part2(&input));
    Ok(())
}

fn part1(input: &[Day3Input]) -> u32 {
    let input = input
        .iter()
        .map(|i| i.bits.clone())
        .collect::<Vec<String>>();
    let bits_in_input = input[0].chars().count();

    let (lines_in_input, bit_counts) = count_ones_by_bitnumber(&input);
    let mut gamma = 0u32;
    let mut epsilon = 0u32;
    for bit in 0..bits_in_input {
        let count = bit_counts[&bit];
        let (mc, lc) = if lines_in_input - count < count {
            (1, 0)
        } else {
            (0, 1)
        };
        gamma |= mc << bit;
        epsilon |= lc << bit;
    }
    gamma * epsilon
}

fn part2(input: &[Day3Input]) -> u32 {
    let input = input
        .iter()
        .map(|i| i.bits.clone())
        .collect::<Vec<String>>();
    let oxygen = get_oxygen(&input);
    let co2 = get_co2_scrubber(&input);
    oxygen * co2
}

fn count_ones_by_bitnumber(input: &[String]) -> (u32, HashMap<usize, u32>) {
    let mut bit_map: HashMap<usize, u32> = HashMap::new();
    let mut lines = 0;
    for line in input.iter() {
        lines += 1;
        let len = line.chars().count();
        for (bit, value) in line.chars().enumerate() {
            let bit = len - bit - 1;
            let value = match value {
                '0' => 0,
                _ => 1,
            };
            if let Some(entry) = bit_map.get_mut(&bit) {
                *entry += value;
            } else {
                bit_map.insert(bit, value);
            }
        }
    }

    (lines, bit_map)
}

fn fold_by_bits_from_right(
    mut input: Vec<String>,
    func: impl Fn(usize, usize, &[String]) -> char,
) -> Option<String> {
    let bits_in_input = input.get(0).map(|l| l.chars().count()).unwrap_or_default();

    for bit in 0..bits_in_input {
        let lookfor = func(bits_in_input, bit, &input);
        input = input
            .into_iter()
            .filter(|line| {
                line.chars()
                    .skip(bit)
                    .map(|c| lookfor == c)
                    .next()
                    .unwrap_or_default()
            })
            .collect();
        if input.len() <= 1 {
            return Some(input[0].clone());
        }
    }
    None
}

fn get_oxygen(input: &[String]) -> u32 {
    fn by_most_significant(bits_in_input: usize, bit: usize, input: &[String]) -> char {
        let (lines_in_input, bit_counts) = count_ones_by_bitnumber(input);
        let count = bit_counts
            .get(&(bits_in_input - bit - 1))
            .copied()
            .unwrap_or_default();
        if count >= lines_in_input - count {
            '1'
        } else {
            '0'
        }
    }

    fold_by_bits_from_right(input.to_vec(), by_most_significant)
        .map(|s| u32::from_str_radix(&s, 2).unwrap_or_default())
        .unwrap_or_default()
}

fn get_co2_scrubber(input: &[String]) -> u32 {
    fn by_least_significant(bits_in_input: usize, bit: usize, input: &[String]) -> char {
        let (lines_in_input, bit_counts) = count_ones_by_bitnumber(input);
        let count = bit_counts
            .get(&(bits_in_input - bit - 1))
            .copied()
            .unwrap_or_default();
        if count >= lines_in_input - count {
            '0'
        } else {
            '1'
        }
    }

    fold_by_bits_from_right(input.to_vec(), by_least_significant)
        .map(|s| u32::from_str_radix(&s, 2).unwrap_or_default())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{create_input, read_csv};
    use anyhow::Result;

    fn example_input() -> Result<Vec<Day3Input>, anyhow::Error> {
        let dir = tempfile::tempdir()?;
        let input_path = create_input(
            &dir,
            &[
                "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
                "11001", "00010", "01010",
            ],
        )?;
        let input = read_csv(input_path.as_path(), &["bits"], b' ')?;
        Ok(input)
    }

    #[test]
    fn test_part1() -> Result<()> {
        let input = example_input()?;
        assert_eq!(198, part1(&input));
        Ok(())
    }

    #[test]
    fn oxygen_co2() -> Result<()> {
        let input = example_input()?;
        let input = input
            .iter()
            .map(|i| i.bits.clone())
            .collect::<Vec<String>>();
        assert_eq!(23, get_oxygen(&input));
        assert_eq!(10, get_co2_scrubber(&input));
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let input = example_input()?;
        assert_eq!(230, part2(&input));
        Ok(())
    }
}
