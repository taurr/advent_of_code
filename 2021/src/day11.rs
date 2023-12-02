use anyhow::Result;
use image::{GrayImage, Luma};
use itertools::Itertools;
use std::path::PathBuf;

pub fn solve_puzzle() -> Result<()> {
    const INPUT: &str = include_str!("../assets/day12.txt");
    println!("Part1: {}", part1(parse_input(INPUT)?)?);
    println!("Part2: {}", part2(parse_input(INPUT)?)?);
    Ok(())
}

const ENERGY_INCREASE: u8 = 1;
const ENERGY_FLASH_THRESHOLD: u8 = 9;
const ENERGY_FLASH: u8 = 255;

fn parse_input(input: &str) -> Result<GrayImage> {
    let height = input.lines().count() as u32;
    let container = input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as u8)
        .collect_vec();
    GrayImage::from_raw(container.len() as u32 / height, height, container)
        .ok_or(anyhow::anyhow!("Failed to create image"))
}

struct FrameCount(usize, PathBuf, String);
impl FrameCount {
    fn frames(&self) -> usize {
        self.0
    }

    fn new(dir: &str, name_format: &str) -> Result<Self> {
        let pb = PathBuf::from(dir);
        if !pb.exists() {
            std::fs::create_dir(&pb)?;
        }
        Ok(Self(0, pb, name_format.to_owned()))
    }

    #[cfg(feature = "visualize")]
    fn save(&mut self, img: &GrayImage) -> Result<()> {
        use image::imageops::FilterType;

        let mut pb = self.1.clone();
        pb.push(format!("{}.{:03}.png", self.2, self.0));
        let img = image::imageops::resize(img, 256, 256, FilterType::Nearest);
        img.save(&pb)?;
        self.0 += 1;
        Ok(())
    }

    #[cfg(not(feature = "visualize"))]
    fn save(&mut self, _: &GrayImage) -> Result<()> {
        self.0 += 1;
        Ok(())
    }
}

fn part1(mut img: GrayImage) -> Result<usize> {
    let mut flash_count = 0;

    for _ in 0..100 {
        let mut flashes = increase_energy_and_flash(&mut img);
        while !flashes.is_empty() {
            flash_count += flashes.len();
            flashes = flash_adjacent(&mut img, flashes);
        }
        reset_flashes(&mut img);
    }

    Ok(flash_count)
}

fn part2(mut img: GrayImage) -> Result<usize> {
    let mut frame = FrameCount::new("assets_day11", "part2")?;
    frame.save(&img)?;

    loop {
        let mut flashes = increase_energy_and_flash(&mut img);
        while !flashes.is_empty() {
            flashes.len();
            flashes = flash_adjacent(&mut img, flashes);
        }
        frame.save(&img)?;
        reset_flashes(&mut img);
        if img.pixels().all(|p| p.0[0] == 0) {
            return Ok(frame.frames() - 1);
        }
    }
}

fn increase_energy_and_flash(img: &mut GrayImage) -> Vec<(u32, u32)> {
    img.enumerate_pixels_mut()
        .filter_map(|(x, y, p)| {
            if p.0[0] <= (ENERGY_FLASH - ENERGY_INCREASE) {
                p.0[0] += ENERGY_INCREASE;
                if p.0[0] > ENERGY_FLASH_THRESHOLD {
                    *p = Luma([ENERGY_FLASH]);
                    Some((x, y))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect_vec()
}

fn flash_adjacent(img: &mut GrayImage, flashes: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    let mut new_flashes = vec![];
    for (x, y) in flashes.into_iter() {
        for xt in x.max(1) - 1..=(x + 1).min(img.width() - 1) {
            for yt in y.max(1) - 1..=(y + 1).min(img.height() - 1) {
                if xt == x && yt == y {
                    continue;
                }
                let p = img.get_pixel_mut(xt, yt);
                if p.0[0] <= (ENERGY_FLASH - ENERGY_INCREASE) {
                    p.0[0] += ENERGY_INCREASE;
                    if p.0[0] > ENERGY_FLASH_THRESHOLD {
                        *p = Luma([ENERGY_FLASH]);
                        new_flashes.push((xt, yt))
                    }
                }
            }
        }
    }
    new_flashes
}

fn reset_flashes(img: &mut GrayImage) {
    for p in img.pixels_mut() {
        if p.0[0] == ENERGY_FLASH {
            *p = Luma([0]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        5483143223
        2745854711
        5264556173
        6141336146
        6357385478
        4167524645
        2176841721
        6882881134
        4846848554
        5283751526
    "};

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(1656, part1(parse_input(INPUT)?)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(195, part2(parse_input(INPUT)?)?);
        Ok(())
    }
}
