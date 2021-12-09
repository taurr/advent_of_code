use anyhow::Result;
use itertools::Itertools;
use std::path::Path;

pub fn solve_puzzle(input_path: &Path) -> Result<()> {
    let input = std::fs::read_to_string(input_path)?;
    let input = parse_input(&input);
    println!("Day9, Part1: {}", part1(&input));
    println!("Day9, Part2: {}", part2(&input));
    Ok(())
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect_vec()
        })
        .collect_vec()
}

fn part1(input: &[Vec<u8>]) -> u32 {
    let max_y = input.len() - 1;
    let max_x = input[0].len() - 1;

    let xy_window = { 0..=max_x }.flat_map(|x| { 0..=max_y }.map(move |y| (x, y)));

    let local_minima = xy_window.filter_map(|(x, y)| {
        let local_minimum_x = { (x as i32 - 1).max(0) as usize..=(x + 1).min(max_x) }
            .filter(|&lx| lx != x)
            .map(|x| input[y][x])
            .min()
            .unwrap();
        let local_minimum_y = { (y as i32 - 1).max(0) as usize..=(y + 1).min(max_y) }
            .filter(|&ly| ly != y)
            .map(|y| input[y][x])
            .min()
            .unwrap();
        let local_minimum = local_minimum_x.min(local_minimum_y);

        let local_value = input[y][x];
        if local_value < local_minimum {
            Some(local_value)
        } else {
            None
        }
    });

    local_minima.map(|m| 1 + m as u32).sum()
}

fn flood_fill<T: Copy + PartialOrd>(val: T, threshold: T, xy: (usize, usize), map: &mut [Vec<T>]) {
    let mut q = vec![xy];
    while let Some((x, y)) = q.pop() {
        if map[y][x] < threshold {
            map[y][x] = val;
            if x > 0 {
                q.push((x - 1, y))
            }
            if x < (map[0].len() - 1) {
                q.push((x + 1, y))
            }
            if y > 0 {
                q.push((x, y - 1))
            }
            if y < (map.len() - 1) {
                q.push((x, y + 1))
            }
        }
    }
}

fn part2(input: &[Vec<u8>]) -> u32 {
    let max_y = input.len() - 1;
    let max_x = input[0].len() - 1;
    let mut bassins = input.iter().cloned().collect_vec();

    const THRESHOLD: u8 = 9;

    let mut last_counter = THRESHOLD + 1;
    let mut inside = false;
    let mut counter = 0;

    for y in 0..=max_y {
        for x in 0..=max_x {
            if bassins[y][x] < THRESHOLD {
                if !inside {
                    inside = true;
                    last_counter += 1;
                    counter = last_counter;
                };
                flood_fill(counter, THRESHOLD, (x, y), &mut bassins);
            } else {
                inside = false;
            }
        }
    }

    let x = bassins
        .into_iter()
        .flat_map(|v| v.into_iter())
        .filter(|&x| x > THRESHOLD)
        .counts_by(|v| v);
    let mut x = x.into_values().collect_vec();
    x.sort_unstable();
    x.into_iter().rev().take(3).product::<usize>() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = indoc::indoc! {"
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
    "};

    #[test]
    fn test_part1() {
        let input = parse_input(INPUT);
        let result = part1(&input);
        assert_eq!(15, result);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(INPUT);
        let result = part2(&input);
        assert_eq!(1134, result);
    }
}
