use anyhow::Result;
use derive_more::{Add, Constructor, Sub};
use image::{Rgb, RgbImage};
use itertools::Itertools;
use regex::Regex;
use std::path::Path;
use std::str::FromStr;

pub fn solve_puzzle(input_path: &Path) -> Result<()> {
    let input = std::fs::read_to_string(input_path)?;
    let input = parse_input(&input);
    println!("Day5, Part1: {}", part1(&input));
    println!("Day5, Part2: {}", part2(&input));
    Ok(())
}

type PuzzleInput = Line;

#[derive(Debug, PartialEq, PartialOrd, Add, Sub, Constructor, Clone, Copy, Default)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Constructor, Clone, Copy)]
struct Line {
    p1: Point,
    p2: Point,
}

fn parse_input(input: &str) -> Vec<PuzzleInput> {
    let re = Regex::new(r"(?P<x1>\d+),(?P<y1>\d+)\s*->\s*(?P<x2>\d+),(?P<y2>\d+)").unwrap();
    input
        .split_terminator('\n')
        .map(|l| {
            let captures = re.captures(l).unwrap();
            let x1 = captures
                .name("x1")
                .map_or(0, |m| i32::from_str(m.as_str()).unwrap());
            let y1 = captures
                .name("y1")
                .map_or(0, |m| i32::from_str(m.as_str()).unwrap());
            let x2 = captures
                .name("x2")
                .map_or(0, |m| i32::from_str(m.as_str()).unwrap());
            let y2 = captures
                .name("y2")
                .map_or(0, |m| i32::from_str(m.as_str()).unwrap());
            Line::new(Point::new(x1, y1), Point::new(x2, y2))
        })
        .collect_vec()
}

#[derive(Debug)]
struct HeatMap {
    cols: usize,
    rows: usize,
    map: Vec<u32>,
}

impl HeatMap {
    fn new(cols: usize, rows: usize) -> Self {
        Self {
            cols,
            rows,
            map: Vec::from_iter(std::iter::repeat(0).take((cols * rows) as usize)),
        }
    }

    fn cols(&self) -> usize {
        self.cols
    }
    fn rows(&self) -> usize {
        self.rows
    }

    fn get_xy(&self, x: usize, y: usize) -> Option<&u32> {
        if x >= self.cols || y >= self.rows {
            return None;
        }
        Some(&self.map[x + y * self.cols])
    }

    fn iter(&self) -> impl Iterator<Item = &u32> {
        self.map.iter()
    }

    fn mark(&mut self, p: Point) -> u32 {
        if p.x < 0 || p.x as usize >= self.cols || p.y < 0 || p.y as usize >= self.rows {
            return 0;
        }

        let idx = p.x + p.y * self.rows as i32;
        self.map[idx as usize] += 1;
        self.map[idx as usize]
    }

    fn generate_minimap(&self, cols: usize, rows: usize) -> HeatMap {
        let cols = cols.min(self.cols);
        let rows = rows.min(self.rows);
        let mut minimap = HeatMap::new(cols, rows);

        for y in 0..self.rows {
            let minimap_y = y * rows / self.rows;
            for x in 0..self.cols {
                let minimap_x = x * cols / self.cols;
                let value = self.map[x + y * self.cols];
                minimap.map[minimap_x + minimap_y * cols] += value;
            }
        }
        minimap
    }
}

impl std::fmt::Display for HeatMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.rows {
            for x in 0..self.cols {
                write!(f, "{}\t", self.map[x + y * self.cols])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn part1(input: &[PuzzleInput]) -> u32 {
    // find heatmap size
    let (x_max, y_max) = input
        .iter()
        .map(|l| (l.p1.x.max(l.p2.x), l.p1.y.max(l.p2.y)))
        .fold1(|a, b| (a.0.max(b.0), a.1.max(b.1)))
        .unwrap_or_default();

    // create heatmap
    let mut heatmap = HeatMap::new((x_max + 1) as usize, (y_max + 1) as usize);
    for line in input {
        if line.p1.y == line.p2.y {
            // horizontal
            let (x1, x2) = (line.p1.x.min(line.p2.x), line.p1.x.max(line.p2.x));
            for x in x1..=x2 {
                heatmap.mark(Point::new(x, line.p1.y));
            }
        } else if line.p1.x == line.p2.x {
            // vertical
            let (y1, y2) = (line.p1.y.min(line.p2.y), line.p1.y.max(line.p2.y));
            for y in y1..=y2 {
                heatmap.mark(Point::new(line.p1.x, y));
            }
        }
    }

    heatmap.iter().filter(|&&h| h > 1).count() as u32
}

fn part2(input: &[PuzzleInput]) -> u32 {
    // find heatmap size
    let (x_max, y_max) = input
        .iter()
        .map(|l| (l.p1.x.max(l.p2.x), l.p1.y.max(l.p2.y)))
        .fold1(|a, b| (a.0.max(b.0), a.1.max(b.1)))
        .unwrap_or_default();

    // create heatmap
    let mut heatmap = HeatMap::new((x_max + 1) as usize, (y_max + 1) as usize);
    for line in input {
        if line.p1.y == line.p2.y {
            // horizontal
            let (x1, x2) = (line.p1.x.min(line.p2.x), line.p1.x.max(line.p2.x));
            for x in x1..=x2 {
                heatmap.mark(Point::new(x, line.p1.y));
            }
        } else if line.p1.x == line.p2.x {
            // vertical
            let (y1, y2) = (line.p1.y.min(line.p2.y), line.p1.y.max(line.p2.y));
            for y in y1..=y2 {
                heatmap.mark(Point::new(line.p1.x, y));
            }
        } else {
            // diagonal
            let (p1, p2) = if line.p1.x <= line.p2.x {
                (line.p1, line.p2)
            } else {
                (line.p2, line.p1)
            };
            let dy = (p2.y - p1.y) as i32 / (p2.x - p1.x) as i32;
            let mut y = p1.y;
            for x in p1.x..=p2.x {
                heatmap.mark(Point::new(x, y));
                y += dy;
            }
        }
    }

    // generate heatmap image!
    let minimap = heatmap.generate_minimap(250, 250);
    let high_value = (*minimap.iter().max().unwrap()).max(1);
    let mut img = RgbImage::new(minimap.cols() as u32, minimap.rows() as u32);
    for y in 0..minimap.rows() {
        for x in 0..minimap.cols() {
            let value = *minimap.get_xy(x, y).unwrap();
            let value = (value * 255 / high_value) as u8;
            img.put_pixel(x as u32, y as u32, Rgb([value, value, value]));
        }
    }
    img.save("heatmap.png").unwrap();

    heatmap.iter().filter(|&&h| h > 1).count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    fn example_input() -> String {
        let input = indoc! {"
            0,9 -> 5,9
            8,0 -> 0,8
            9,4 -> 3,4
            2,2 -> 2,1
            7,0 -> 7,4
            6,4 -> 2,0
            0,9 -> 2,9
            3,4 -> 1,4
            0,0 -> 8,8
            5,5 -> 8,2
        "};
        input.to_string()
    }

    #[test]
    fn test_input() {
        let input = example_input();
        let input = parse_input(&input);
        assert_eq!(
            &vec![Line::new(Point::new(0, 9), Point::new(5, 9)),][0],
            &input[0]
        );
    }

    #[test]
    fn test_part1() {
        let input = example_input();
        let input = parse_input(&input);
        let result = part1(&input);
        assert_eq!(5, result);
    }

    #[test]
    fn example_part2() {
        let input = example_input();
        let input = parse_input(&input);
        let result = part2(&input);
        assert_eq!(12, result);
    }
}
