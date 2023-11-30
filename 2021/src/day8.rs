use anyhow::Result;
use itertools::Itertools;
use std::{collections::HashSet, fs::read_to_string, path::Path};

pub fn solve_puzzle(input_path: &Path) -> Result<()> {
    let input = read_to_string(input_path)?;
    let input = parse_input(&input);
    println!("Day8, Part1: {}", part1(&input));
    println!("Day8, Part2: {}", part2(&input));
    Ok(())
}

fn part1(input: &[Vec<String>]) -> i32 {
    input
        .iter()
        .flat_map(|v| {
            v[10..].iter().map(|s| match s.len() {
                2 | 4 | 3 | 7 => 1,
                _ => 0,
            })
        })
        .sum()
}

fn part2(input: &[Vec<String>]) -> usize {
    input
        .iter()
        .map(|input| {
            let (input, output) = input.split_at(10);
            let mut input = input
                .iter()
                .map(|l| l.chars().collect::<HashSet<char>>())
                .collect_vec();
            let output = output
                .iter()
                .map(|l| l.chars().collect::<HashSet<char>>())
                .collect_vec();

            let one = input.remove(
                input
                    .iter()
                    .find_position(|s| s.len() == 2)
                    .map(|(p, _)| p)
                    .unwrap(),
            );
            let four = input.remove(
                input
                    .iter()
                    .find_position(|s| s.len() == 4)
                    .map(|(p, _)| p)
                    .unwrap(),
            );
            let seven = input.remove(
                input
                    .iter()
                    .find_position(|s| s.len() == 3)
                    .map(|(p, _)| p)
                    .unwrap(),
            );
            let eight = input.remove(
                input
                    .iter()
                    .find_position(|s| s.len() == 7)
                    .map(|(p, _)| p)
                    .unwrap(),
            );

            let six = input.remove(
                input
                    .iter()
                    .find_position(|s| s.len() == 6 && s.difference(&one).count() == 5)
                    .map(|(p, _)| p)
                    .unwrap(),
            );

            let nine = input.remove(
                input
                    .iter()
                    .find_position(|s| s.len() == 6 && s.difference(&four).count() == 2)
                    .map(|(p, _)| p)
                    .unwrap(),
            );

            let zero = input.remove(
                input
                    .iter()
                    .find_position(|s| s.len() == 6)
                    .map(|(p, _)| p)
                    .unwrap(),
            );

            let three = input.remove(
                input
                    .iter()
                    .find_position(|s| s.difference(&seven).count() == 2)
                    .map(|(p, _)| p)
                    .unwrap(),
            );

            let five = input.remove(
                input
                    .iter()
                    .find_position(|s| s.difference(&four).count() == 2)
                    .map(|(p, _)| p)
                    .unwrap(),
            );

            let two = input.remove(0);

            let ciphers = [zero, one, two, three, four, five, six, seven, eight, nine];

            output
                .into_iter()
                .map(|o| {
                    ciphers
                        .iter()
                        .position(|c| c.symmetric_difference(&o).count() == 0)
                        .unwrap()
                })
                .zip([1000usize, 100, 10, 1])
                .map(|(a, b)| a * b)
                .sum::<usize>()
        })
        .sum()
}

fn parse_input(input: &str) -> Vec<Vec<String>> {
    let input = input
        .lines()
        .map(|l| {
            l.replace(" | ", " ")
                .split(' ')
                .map(|s| s.to_string())
                .collect_vec()
        })
        .collect_vec();
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = indoc::indoc!(
        "
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
    );

    #[test]
    fn test_part1() {
        let input = parse_input(EXAMPLE_INPUT);
        assert_eq!(26, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = parse_input(EXAMPLE_INPUT);
        assert_eq!(61229, part2(&input));
    }
}
