use anyhow::Result;
use std::{fs::read_to_string, path::Path};

const MAX_AGE: usize = 9;

pub fn solve_puzzle(input_path: &Path) -> Result<()> {
    let input = parse_input(&read_to_string(input_path)?);
    println!("Day6, Part1: {}", part1(input.clone()));
    println!("Day6, Part2: {}", part2(input));
    Ok(())
}

fn part1(mut input: Box<[u64]>) -> u64 {
    simulate(&mut input, 80)
}

fn part2(mut input: Box<[u64]>) -> u64 {
    simulate(&mut input, 256)
}

fn parse_input(input: &str) -> Box<[u64]> {
    let mut fish_by_spawn_delay = Box::new([0; MAX_AGE]);
    for age in input
        .split(',')
        .map(|s| s.parse::<u64>().expect("unable to parse age"))
    {
        fish_by_spawn_delay[age as usize % MAX_AGE] += 1;
    }
    fish_by_spawn_delay
}

fn simulate(fish_by_spawn_delay: &mut [u64], days_to_simulate: usize) -> u64 {
    for _ in 0..days_to_simulate {
        let to_be_spawned = fish_by_spawn_delay
            .iter_mut()
            .rev()
            .fold(0, |a, b| std::mem::replace(b, a));
        // re-introduce spawning fish into the population
        fish_by_spawn_delay[6] += to_be_spawned;
        // introduce their offspring
        fish_by_spawn_delay[MAX_AGE - 1] = to_be_spawned;
    }
    fish_by_spawn_delay.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = parse_input("3,4,3,1,2");
        assert_eq!(5934, part1(input));
    }

    #[test]
    fn test_part2() {
        let input = parse_input("3,4,3,1,2");
        assert_eq!(26984457539, part2(input));
    }
}
