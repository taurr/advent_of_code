use anyhow::Result;
use itertools::Itertools;
use std::{iter::repeat, path::Path, str::FromStr};

pub fn solve_puzzle(input_path: &Path) -> Result<()> {
    let input = std::fs::read_to_string(input_path)?;
    let input = input.split_terminator('\n').collect_vec();
    println!("Day4, Part1: {}", part1(&input));
    println!("Day4, Part2: {}", part2(&input));
    Ok(())
}

fn part1<T: ToString + AsRef<str>>(input: &[T]) -> u32 {
    let numbers = NumberPool::from_str(&input[0]);
    let mut boards = extract_board_numbers(&input[1..])
        .map(BingoBoard::<5, 5>::from)
        .collect_vec();

    for number in numbers {
        boards.iter_mut().for_each(|b| b.mark(number));
        if let Some(winner) = boards.iter().find(|b| b.is_winner()) {
            let unmarked_sum = winner
                .unmarked_numbers()
                .iter()
                .map(|n| *n as u32)
                .fold1(|a, b| a + b)
                .unwrap_or_default();
            return unmarked_sum * number as u32;
        }
    }
    0
}

fn part2<T: ToString + AsRef<str>>(input: &[T]) -> u32 {
    let numbers = NumberPool::from_str(&input[0]);
    let mut boards = extract_board_numbers(&input[1..])
        .map(BingoBoard::<5, 5>::from)
        .collect_vec();

    for number in numbers {
        if boards.len() > 1 {
            boards.iter_mut().for_each(|b| b.mark(number));
            boards.retain(|b| !b.is_winner());
        } else {
            boards[0].mark(number);
            let unmarked_sum = boards[0]
                .unmarked_numbers()
                .iter()
                .map(|n| *n as u32)
                .fold1(|a, b| a + b)
                .unwrap_or_default();
            if boards[0].is_winner() {
                return unmarked_sum * number as u32;
            }
        }
    }
    0
}

type BingoNumber = u8;

struct NumberPool(Vec<BingoNumber>);

impl NumberPool {
    pub fn from_str(input: impl AsRef<str>) -> Self {
        let numbers = input
            .as_ref()
            .split_terminator(',')
            .filter_map(|n| BingoNumber::from_str(n.trim()).ok())
            .rev()
            .collect();
        Self(numbers)
    }
}

impl Iterator for NumberPool {
    type Item = BingoNumber;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

#[derive(Debug, Eq, Clone)]
enum BoardField {
    Unmarked(BingoNumber),
    Marked(BingoNumber),
}

impl From<BingoNumber> for BoardField {
    fn from(n: BingoNumber) -> Self {
        BoardField::Unmarked(n)
    }
}

impl PartialEq for BoardField {
    fn eq(&self, other: &Self) -> bool {
        self.number() == other.number()
    }
}

impl BoardField {
    fn number(&self) -> BingoNumber {
        match self {
            Self::Unmarked(l0) | Self::Marked(l0) => *l0,
        }
    }

    fn mark(&mut self) {
        *self = Self::Marked(self.number());
    }

    pub fn is_marked(&self) -> bool {
        matches!(self, Self::Marked(_))
    }
}

#[derive(Debug, PartialEq)]
struct BingoBoard<const C: usize, const R: usize> {
    board: Vec<BoardField>,
}

impl<ITT: IntoIterator<Item = BingoNumber>, const C: usize, const R: usize> From<ITT>
    for BingoBoard<C, R>
{
    fn from(numbers: ITT) -> Self {
        let numbers = numbers.into_iter().chain(repeat(0)).map(BoardField::from);
        Self {
            board: numbers.take(C * R).collect_vec(),
        }
    }
}

impl<const C: usize, const R: usize> BingoBoard<C, R> {
    pub fn mark(&mut self, num: BingoNumber) {
        for field in self.board.iter_mut() {
            if field.number() == num {
                field.mark();
            }
        }
    }

    pub fn is_winner(&self) -> bool {
        // check rows
        for r in 0..R {
            let x = { 0..C }
                .map(|c| {
                    let index = r * C + c;
                    &self.board[index]
                })
                .collect_vec();
            if x.iter().all(|f| f.is_marked()) {
                return true;
            }
        }
        // check columns
        for c in 0..C {
            let x = { 0..R }
                .map(|r| {
                    let index = r * C + c;
                    &self.board[index]
                })
                .collect_vec();
            if x.iter().all(|f| f.is_marked()) {
                return true;
            }
        }
        false
    }

    pub fn unmarked_numbers(&self) -> Vec<BingoNumber> {
        self.board
            .iter()
            .filter_map(|f| match f {
                BoardField::Marked(_) => None,
                BoardField::Unmarked(num) => Some(*num),
            })
            .collect_vec()
    }
}

fn extract_board_numbers<T>(input: &[T]) -> impl Iterator<Item = Vec<BingoNumber>> + '_
where
    T: ToString,
{
    let itt = input.iter().map(|x| x.to_string()).batching(|it| {
        use itertools::FoldWhile::{Continue, Done};
        let numbers = it
            .skip_while(|l| l.trim().is_empty())
            .fold_while(vec![], |mut acc, l| {
                if l.trim().is_empty() {
                    Done(acc)
                } else {
                    acc.push(l);
                    Continue(acc)
                }
            })
            .into_inner();
        if numbers.is_empty() {
            None
        } else {
            Some(
                numbers
                    .into_iter()
                    .join(" ")
                    .split_whitespace()
                    .filter_map(|n| BingoNumber::from_str(n.trim()).ok())
                    .collect_vec(),
            )
        }
    });
    itt
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    fn example_input() -> Vec<String> {
        [
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
            "",
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
            "",
            " 3 15  0  2 22",
            " 9 18 13 17  5",
            "19  8  7 25 23",
            "20 11 10 24  4",
            "14 21 16 12  6",
            "",
            "14 21 17 24  4",
            "10 16 15  9 19",
            "18  8 23 26 20",
            "22 11 13  6  5",
            " 2  0 12  3  7",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    #[test]
    fn numbers_pool() {
        let input = &example_input()[0];
        let pool = NumberPool::from_str(input);
        assert_eq!(
            vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1
            ],
            pool.collect_vec()
        );
    }

    #[test]
    fn test_part1() {
        let input = example_input();
        let result = part1(&input);
        assert_eq!(4512, result);
    }

    #[test]
    fn test_part2() {
        let input = example_input();
        let result = part2(&input);
        assert_eq!(1924, result);
    }
}
