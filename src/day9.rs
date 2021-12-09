use anyhow::Result;
use image::{GrayImage, ImageBuffer, Luma, Pixel, Rgb, RgbImage};
use itertools::Itertools;
use palette::{FromColor, Hsv, RgbHue, Srgb};
use rand::Rng;
use std::{ops::DerefMut, path::Path};

pub fn solve_puzzle(input_path: &Path) -> Result<()> {
    let input = std::fs::read_to_string(input_path)?;
    println!("Day9, Part1: {}", part1(parse_input(&input)?));
    println!("Day9, Part2: {}", part2(parse_input(&input)?));
    Ok(())
}

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

fn part1(map: GrayImage) -> u32 {
    let xy_window = { 0..map.width() }.flat_map(|x| { 0..map.height() }.map(move |y| (x, y)));

    let local_minima = xy_window.filter_map(|(x, y)| {
        let v = map.get_pixel(x, y).0[0];
        if !(x > 0 && v > map.get_pixel(x - 1, y).0[0]
            || x < (map.width() - 1) && v > map.get_pixel(x + 1, y).0[0]
            || y > 0 && v > map.get_pixel(x, y - 1).0[0]
            || y < (map.height() - 1) && v > map.get_pixel(x, y + 1).0[0])
        {
            Some(v)
        } else {
            None
        }
    });

    local_minima.map(|m| 1 + m as u32).sum()
}

fn part2(mut map: GrayImage) -> usize {
    const THRESHOLD: u8 = 9;
    let mut filler = Luma([THRESHOLD + 1]);
    let mut sizes = vec![];
    for y in 0..map.height() {
        for x in 0..map.width() {
            let pixels_filled = flood_fill(filler, |p| p.0[0] < THRESHOLD, (x, y), &mut map);
            if pixels_filled > 0 {
                filler = Luma([filler.0[0] + 1]);
            }
            sizes.push(pixels_filled);
        }
    }

    let mut rgb = RgbImage::new(map.width(), map.height());
    let mut rng = rand::thread_rng();
    let colors = { THRESHOLD..filler.0[0] }
        .map(|_| rng.gen_range(0f32..359f32))
        .map(|hue| Hsv::new(RgbHue::from_degrees(hue), 1., 1.))
        .map(Srgb::from_color)
        .map(|srgb| {
            let (r, g, b) = srgb.into_components();
            Rgb([(r * 255.) as u8, (g * 255.) as u8, (b * 255.) as u8])
        })
        .collect_vec();
    for y in 0..map.height() {
        for x in 0..map.width() {
            let g = map.get_pixel(x, y).0[0];
            if g > THRESHOLD {
                rgb.put_pixel(x, y, colors[(g - THRESHOLD - 1) as usize]);
            }
        }
    }
    rgb.save("day9_part2_viz.png").unwrap();

    sizes.sort_unstable();
    sizes.into_iter().rev().take(3).product()
}

fn flood_fill<F, P, Container>(
    val: P,
    func: F,
    xy: (u32, u32),
    map: &mut ImageBuffer<P, Container>,
) -> usize
where
    P: Pixel + 'static,
    Container: DerefMut<Target = [P::Subpixel]>,
    F: Fn(&P) -> bool,
{
    let mut pixels: usize = 0;
    let mut q = vec![xy];
    while let Some((x, y)) = q.pop() {
        if func(map.get_pixel(x, y)) {
            map.put_pixel(x, y, val);
            pixels += 1;
            if x > 0 {
                q.push((x - 1, y))
            }
            if x < (map.width() - 1) {
                q.push((x + 1, y))
            }
            if y > 0 {
                q.push((x, y - 1))
            }
            if y < (map.height() - 1) {
                q.push((x, y + 1))
            }
        }
    }

    pixels
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    const INPUT: &str = indoc::indoc! {"
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
    "};

    #[test]
    fn test_part1() -> Result<()> {
        let input = parse_input(INPUT);
        let result = part1(input?);
        assert_eq!(15, result);
        Ok(())
    }

    #[test]
    fn example_part2() -> Result<()> {
        let input = parse_input(INPUT);
        let result = part2(input?);
        assert_eq!(1134, result);
        Ok(())
    }
}
