use anyhow::Result;
use itertools::Itertools;
use std::path::Path;

pub fn solve_puzzle(input_path: &Path) -> Result<()> {
    let input = std::fs::read_to_string(input_path)?;
    println!("Day10, Part1: {}", part1(parse_input(&input)));
    println!("Day10, Part2: {}", part2(parse_input(&input)));
    Ok(())
}

fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_owned()).collect_vec()
}

fn parse_line(line: &str) -> Result<Vec<char>, char> {
    let mut expected_closing = vec![];

    line.chars()
        .try_for_each(|c| match c {
            '(' => {
                expected_closing.push(')');
                Ok(())
            }
            '[' => {
                expected_closing.push(']');
                Ok(())
            }
            '{' => {
                expected_closing.push('}');
                Ok(())
            }
            '<' => {
                expected_closing.push('>');
                Ok(())
            }
            ')' | ']' | '}' | '>' => {
                if expected_closing
                    .pop()
                    .filter(|&expected| expected == c)
                    .is_some()
                {
                    Ok(())
                } else {
                    Err(c)
                }
            }
            _ => Err(c),
        })
        .and(Ok(expected_closing))
}

fn part1(lines: Vec<String>) -> usize {
    lines
        .iter()
        .map(|l| match parse_line(l) {
            Err(')') => 3,
            Err(']') => 57,
            Err('}') => 1197,
            Err('>') => 25137,
            _ => 0,
        })
        .sum::<usize>()
}

fn part2(lines: Vec<String>) -> usize {
    let mut scores: Vec<usize> = lines
        .iter()
        .map(|l| parse_line(l))
        .filter_map(|r| r.ok())
        .map(|expected| expected.into_iter().rev().collect::<String>())
        .map(|complection| {
            complection.chars().fold(0, |acc, c| {
                acc * 5
                    + match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => 0,
                    }
            })
        })
        .collect_vec();

    scores.sort_unstable();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        [({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]
    "};

    #[test]
    fn test_part1() {
        assert_eq!(26397, part1(parse_input(INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(288957, part2(parse_input(INPUT)));
    }
}
