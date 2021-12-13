use std::path::PathBuf;

use anyhow::Result;
use image::{GrayImage, Luma};
use itertools::Itertools;

use self::parser::{Fold, ParsedInput};

pub fn solve_puzzle() -> Result<()> {
    const INPUT: &str = include_str!("../assets/day13.txt");
    println!("Part1: {}", part1(parse_input(INPUT)?)?);
    part2(parse_input(INPUT)?)?;
    println!("Part2: it's an image!",);
    Ok(())
}

mod parser {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{char, line_ending, multispace0, one_of},
        combinator::opt,
        multi::{fold_many0, many0, many1},
        sequence::{preceded, separated_pair, terminated},
        IResult, Parser,
    };

    pub(crate) enum Fold {
        AlongX(i32),
        AlongY(i32),
    }
    pub(crate) struct ParsedInput {
        pub(crate) dots: Vec<(i32, i32)>,
        pub(crate) instructions: Vec<Fold>,
    }

    fn int32(input: &str) -> IResult<&str, i32> {
        let (input, x) = fold_many0(
            one_of("0123456789"),
            || 0,
            |ans, c| ans * 10 + c.to_digit(10).unwrap(),
        )(input)?;
        Ok((input, x as i32))
    }

    fn coord(input: &str) -> IResult<&str, (i32, i32)> {
        terminated(separated_pair(int32, char(','), int32), opt(line_ending))(input)
    }

    fn instruction(input: &str) -> IResult<&str, Fold> {
        let (input, _) = tag("fold along ")(input)?;
        terminated(
            alt((
                preceded(tag("x="), int32.map(Fold::AlongX)),
                preceded(tag("y="), int32.map(Fold::AlongY)),
            )),
            opt(line_ending),
        )(input)
    }

    pub(crate) fn parser(input: &str) -> IResult<&str, ParsedInput> {
        let (input, dots) = many1(coord)(input)?;
        let (input, _) = multispace0(input)?;
        let (input, instructions) = many0(instruction)(input)?;
        Ok((input, ParsedInput { dots, instructions }))
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use anyhow::Result;
        use indoc::indoc;

        #[test]
        fn test_parser1() -> Result<()> {
            assert!(matches!(coord("6,10")?, ("", (6, 10))));
            Ok(())
        }

        #[test]
        fn test_parser2() -> Result<()> {
            assert!(matches!(
                coord(indoc! {"
            6,10
        "})?,
                ("", (6, 10))
            ));
            Ok(())
        }

        #[test]
        fn test_parser3() -> Result<()> {
            assert!(matches!(
                instruction("fold along x=42")?,
                ("", Fold::AlongX(42))
            ));
            Ok(())
        }

        #[test]
        fn test_parser4() -> Result<()> {
            assert!(matches!(
                instruction("fold along y=42")?,
                ("", Fold::AlongY(42))
            ));
            Ok(())
        }
    }
}

fn parse_input(input: &'static str) -> Result<ParsedInput> {
    let (_, parsed) = parser::parser(input)?;
    Ok(parsed)
}

fn part1(mut input: ParsedInput) -> Result<usize> {
    for instr in input.instructions.iter().take(1) {
        match instr {
            Fold::AlongX(n) => {
                for (x, _) in input.dots.iter_mut() {
                    if *x > *n {
                        *x -= 2 * (*x - n);
                    }
                }
            }
            Fold::AlongY(n) => {
                for (_, y) in input.dots.iter_mut() {
                    if *y > *n {
                        *y -= 2 * (*y - n);
                    }
                }
            }
        }
    }

    // finalize
    Ok(input.dots.into_iter().unique().count())
}

fn part2(mut input: ParsedInput) -> Result<()> {
    for instr in input.instructions.iter() {
        match instr {
            Fold::AlongX(n) => {
                for (x, _) in input.dots.iter_mut() {
                    if *x > *n {
                        *x -= 2 * (*x - n);
                    }
                }
            }
            Fold::AlongY(n) => {
                for (_, y) in input.dots.iter_mut() {
                    if *y > *n {
                        *y -= 2 * (*y - n);
                    }
                }
            }
        }
    }

    // finalize as an image
    let (max_x, max_y) = input
        .dots
        .iter()
        .fold((0, 0), |(x1, y1), (x2, y2)| (x1.max(*x2), y1.max(*y2)));
    let mut img = GrayImage::new(max_x as u32 + 1, max_y as u32 + 1);
    for (x, y) in input.dots.into_iter() {
        img.put_pixel(x as u32, y as u32, Luma([255]));
    }
    let mut pb = PathBuf::from("assets_day13");
    if !pb.exists() {
        std::fs::create_dir(&pb)?;
    }
    pb.push("part2.png");
    img.save(pb.as_path())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        6,10
        0,14
        9,10
        0,3
        10,4
        4,11
        6,0
        6,12
        4,1
        0,13
        10,12
        3,4
        3,0
        8,4
        1,10
        2,14
        8,10
        9,0

        fold along y=7
        fold along x=5
    "};

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(17, part1(parse_input(INPUT)?)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        part2(parse_input(INPUT)?)
    }
}
