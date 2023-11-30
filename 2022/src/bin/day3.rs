use std::{borrow::ToOwned, str::Lines};

use itertools::Itertools;
#[allow(unused)]
use tracing::{debug, error, info, instrument, warn};

fn main() {
    let input = std::fs::read_to_string(r"assets/day3.txt").unwrap();
    println!("Task1 = {}", task1(input.lines()));
    println!("Task2 = {}", task2(input.lines()));
}

fn map_to_priority(c: char) -> usize {
    (match c {
        'A'..='Z' => (c as u8) - b'A' + 27,
        'a'..='z' => (c as u8) - b'a' + 1,
        _ => panic!("Invalid input"),
    }) as usize
}

fn task1(input: Lines) -> usize {
    let input = input.map(|l| {
        let items = l.chars().collect_vec();
        let (a, b) = items.split_at(items.len() / 2);
        (Vec::from(a), Vec::from(b))
    });
    input
        .map(|(a, b)| a.iter().filter(|&c| b.contains(c)).copied().collect_vec())
        .map(|v| map_to_priority(v[0]))
        .sum()
}

fn task2(input: Lines) -> usize {
    find_badges(input).map(map_to_priority).sum()
}

fn find_badges(input: Lines) -> impl Iterator<Item = char> {
    input
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let x = chunk.map(ToOwned::to_owned).collect_vec();
            let (first, remainder) = x.split_first().unwrap();
            let group = first
                .chars()
                .find(|c| remainder.iter().all(|rc| rc.contains(*c)))
                .unwrap();
            group
        })
        .collect_vec()
        .into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use tracing_test::traced_test;

    #[test]
    fn priority() {
        assert_eq!(1, map_to_priority('a'));
        assert_eq!(26, map_to_priority('z'));
        assert_eq!(27, map_to_priority('A'));
        assert_eq!(52, map_to_priority('Z'));
    }

    #[test]
    #[traced_test]
    fn task1_example() {
        const INPUT: &str = indoc! {r#"
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw
        "#};

        assert_eq!(task1(INPUT.lines()), 157);
    }

    #[test]
    #[traced_test]
    fn task2_example() {
        const INPUT: &str = indoc! {r#"
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw
        "#};

        assert_eq!(
            find_badges(INPUT.lines()).take(2).collect_vec(),
            vec!['r', 'Z']
        );
    }
}
