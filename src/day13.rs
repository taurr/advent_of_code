use anyhow::Result;
use image::{GrayImage, Luma};
use itertools::Itertools;
use std::path::PathBuf;

use self::parser::{parse, Fold, ParsedInput};

mod parser;

pub fn solve_puzzle() -> Result<()> {
    const INPUT: &str = include_str!("../assets/day13.txt");
    println!("Part1: {}", part1(parse_input(INPUT)?)?);
    part2(parse_input(INPUT)?)?;
    println!("Part2: it's an image!",);
    Ok(())
}

fn parse_input(input: &'static str) -> Result<ParsedInput> {
    let (_, parsed) = parse(input)?;
    Ok(parsed)
}

fn part1(mut input: ParsedInput) -> Result<usize> {
    follow_instructions(input.instructions.into_iter().take(1), &mut input.dots);

    // finalize
    Ok(input.dots.into_iter().unique().count())
}

fn part2(mut input: ParsedInput) -> Result<()> {
    follow_instructions(input.instructions.into_iter(), &mut input.dots);

    // finalize as an image
    let (max_x, max_y) = input
        .dots
        .iter()
        .fold((0, 0), |(x1, y1), (x2, y2)| (x1.max(*x2), y1.max(*y2)));
    let mut img = GrayImage::new(max_x + 1, max_y + 1);
    for (x, y) in input.dots.into_iter() {
        img.put_pixel(x, y, Luma([255]));
    }

    // save image
    let mut pb = PathBuf::from("assets_day13");
    if !pb.exists() {
        std::fs::create_dir(&pb)?;
    }
    pb.push("part2.png");

    #[cfg(feature = "visualize")]
    img.save(pb.as_path())?;

    Ok(())
}

fn follow_instructions(instructions: impl Iterator<Item = Fold>, dots: &mut Vec<(u32, u32)>) {
    for instr in instructions {
        match instr {
            Fold::AlongX(n) => dots.iter_mut().for_each(|(x, _)| {
                if *x > n {
                    *x -= 2 * (*x - n);
                }
            }),
            Fold::AlongY(n) => dots.iter_mut().for_each(|(_, y)| {
                if *y > n {
                    *y -= 2 * (*y - n);
                }
            }),
        }
    }
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
