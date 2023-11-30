use std::str::Lines;

use itertools::Itertools;
#[allow(unused)]
use tracing::{debug, error, info, instrument, warn};

fn main() {
    let input = std::fs::read_to_string(r"assets/day1.txt").unwrap();
    println!("Task1 = {}", task1(input.lines()));
    println!("Task2 = {}", task2(input.lines()));
}

fn task1(input: Lines) -> usize {
    input
        .batching(|itt| {
            itt.map_while(|line| -> Option<usize> { line.parse().ok() })
                .sum1()
        })
        .max()
        .unwrap()
}

fn task2(input: Lines) -> usize {
    input
        .batching(|itt| {
            itt.map_while(|line| -> Option<usize> { line.parse().ok() })
                .sum1::<usize>()
        })
        .sorted_by(|a, b| b.cmp(a))
        .take(3)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn task1_example() {
        const INPUT: &str = indoc! {r#"
            1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000
        "#};

        assert_eq!(
            task1(INPUT.lines()),
            vec![6000, 4000, 11000, 24000, 10000]
                .into_iter()
                .max()
                .unwrap()
        );
    }
}
