use std::str::{FromStr, Lines};

use derive_more::{Display, Error};
use itertools::Itertools;
#[allow(unused)]
use tracing::{debug, error, info, instrument, warn};

fn main() {
    let input = std::fs::read_to_string(r"assets/day2.txt").unwrap();
    println!("Task1 = {}", task1(parse_input1(input.lines())));
    println!("Task2 = {}", task2(parse_input2(input.lines())));
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum RockPaperScissor {
    Rock,
    Paper,
    Scissor,
}

impl FromStr for RockPaperScissor {
    type Err = ParseError;
    fn from_str(i: &str) -> Result<Self, ParseError> {
        match i {
            "A" | "X" => Ok(RockPaperScissor::Rock),
            "B" | "Y" => Ok(RockPaperScissor::Paper),
            "C" | "Z" => Ok(RockPaperScissor::Scissor),
            _ => Err(ParseError),
        }
    }
}

impl RockPaperScissor {
    pub fn battle(self, other: RockPaperScissor) -> BattleResult {
        match (self, other) {
            (RockPaperScissor::Rock, RockPaperScissor::Scissor)
            | (RockPaperScissor::Scissor, RockPaperScissor::Paper)
            | (RockPaperScissor::Paper, RockPaperScissor::Rock) => BattleResult::Win,
            (RockPaperScissor::Scissor, RockPaperScissor::Rock)
            | (RockPaperScissor::Paper, RockPaperScissor::Scissor)
            | (RockPaperScissor::Rock, RockPaperScissor::Paper) => BattleResult::Loose,
            _ => BattleResult::Draw,
        }
    }

    pub fn pick_a_winner(self) -> Self {
        match self {
            RockPaperScissor::Rock => RockPaperScissor::Paper,
            RockPaperScissor::Paper => RockPaperScissor::Scissor,
            RockPaperScissor::Scissor => RockPaperScissor::Rock,
        }
    }

    pub fn pick_a_looser(self) -> Self {
        match self {
            RockPaperScissor::Rock => RockPaperScissor::Scissor,
            RockPaperScissor::Paper => RockPaperScissor::Rock,
            RockPaperScissor::Scissor => RockPaperScissor::Paper,
        }
    }

    pub fn score(self) -> usize {
        match self {
            RockPaperScissor::Rock => 1,
            RockPaperScissor::Paper => 2,
            RockPaperScissor::Scissor => 3,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum ExpectedOutput {
    Loose,
    Draw,
    Win,
}

impl FromStr for ExpectedOutput {
    type Err = ParseError;
    fn from_str(i: &str) -> Result<Self, ParseError> {
        match i {
            "X" => Ok(ExpectedOutput::Loose),
            "Y" => Ok(ExpectedOutput::Draw),
            "Z" => Ok(ExpectedOutput::Win),
            _ => Err(ParseError),
        }
    }
}

impl ExpectedOutput {
    fn pick_selection(self, opponent: RockPaperScissor) -> RockPaperScissor {
        match self {
            ExpectedOutput::Loose => opponent.pick_a_looser(),
            ExpectedOutput::Draw => opponent,
            ExpectedOutput::Win => opponent.pick_a_winner(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum BattleResult {
    Win,
    Loose,
    Draw,
}

impl BattleResult {
    pub fn score(self) -> usize {
        match self {
            BattleResult::Win => 6,
            BattleResult::Loose => 0,
            BattleResult::Draw => 3,
        }
    }
}

#[derive(Debug, Error, Display, Clone, Copy)]
struct ParseError;

fn parse_input1(input: Lines) -> impl Iterator<Item = (RockPaperScissor, RockPaperScissor)> + '_ {
    input.map(|line| {
        let cols = line.split(' ').collect_vec();
        (cols[0].parse().unwrap(), cols[1].parse().unwrap())
    })
}

fn task1(input: impl Iterator<Item = (RockPaperScissor, RockPaperScissor)>) -> usize {
    input
        .map(|(opponent, me)| me.battle(opponent).score() + me.score())
        .sum()
}

fn parse_input2(input: Lines) -> impl Iterator<Item = (RockPaperScissor, ExpectedOutput)> + '_ {
    input.map(|line| {
        let cols = line.split(' ').collect_vec();
        (cols[0].parse().unwrap(), cols[1].parse().unwrap())
    })
}

fn task2(input: impl Iterator<Item = (RockPaperScissor, ExpectedOutput)>) -> usize {
    input
        .map(|(opponent, expectation)| {
            let me = expectation.pick_selection(opponent);
            me.battle(opponent).score() + me.score()
        })
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
        use RockPaperScissor::{Paper, Rock, Scissor};
        const INPUT: &str = indoc! {r#"
            A Y
            B X
            C Z
        "#};

        assert_eq!(
            parse_input1(INPUT.lines()).collect_vec(),
            vec![(Rock, Paper), (Paper, Rock), (Scissor, Scissor)]
        );

        assert_eq!(task1(parse_input1(INPUT.lines())), 15);
    }

    #[test]
    #[traced_test]
    fn parser2() {
        use ExpectedOutput::{Draw, Loose, Win};
        use RockPaperScissor::{Paper, Rock, Scissor};
        const INPUT: &str = indoc! {r#"
            A Y
            B X
            C Z
        "#};

        assert_eq!(
            parse_input2(INPUT.lines()).collect_vec(),
            vec![(Rock, Draw), (Paper, Loose), (Scissor, Win)]
        );
    }
}
