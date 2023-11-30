use anyhow::{bail, Result};
use ndarray::{Array2, Axis};
use petgraph::algo::astar;
use petgraph::graphmap::UnGraphMap;

use self::parser::parse_input;

mod parser;

pub fn solve_puzzle() -> Result<()> {
    const INPUT: &str = include_str!("../assets/day15.txt");
    println!("Part1: {}", part1(parse_input(INPUT)?)?);
    println!("Part2: {}", part2(parse_input(INPUT)?)?);
    Ok(())
}

fn part1(node_weights: Array2<u16>) -> Result<usize> {
    // create a undirected graph with u32 for node index, and () for edge weights
    let mut graph = UnGraphMap::<u32, ()>::new();

    // create connections in the graph
    let row_size = node_weights.len_of(Axis(1)) as usize;
    let windows_per_row = row_size - 1;
    node_weights
        .windows((2, 2))
        .into_iter()
        .enumerate()
        .for_each(|(i, _)| {
            let a_idx = (i % windows_per_row + i / windows_per_row * row_size) as u32;
            let b_idx = a_idx + 1;
            let c_idx = a_idx + row_size as u32;
            let d_idx = c_idx + 1;

            graph.add_edge(a_idx, b_idx, ());
            graph.add_edge(c_idx, d_idx, ());
            graph.add_edge(a_idx, c_idx, ());
            graph.add_edge(b_idx, d_idx, ());
        });

    let top_left = 0;
    let bottom_right = node_weights.len() as u32 - 1;
    match astar(
        &graph,
        top_left,
        |n| n == bottom_right,
        |e| node_weights.as_slice().unwrap()[e.1 as usize],
        |_n| 0,
    ) {
        Some((cost, _nodes)) => Ok(cost as usize),
        None => bail!("no path through the graph!"),
    }
}

fn part2(input: Array2<u16>) -> Result<usize> {
    let (dim_y, dim_x) = input.dim();
    let expanded = Array2::from_shape_fn((dim_y * 5, dim_x * 5), |(y, x)| {
        let v = input[[y % dim_y, x % dim_x]];
        let dx = (x / dim_x) as u16;
        let dy = (y / dim_y) as u16;
        let r = v + dx + dy;
        if r > 9 {
            r % 10 + 1
        } else {
            r
        }
    });
    part1(expanded)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581
    "};

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(40, part1(parse_input(INPUT)?)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(315, part2(parse_input(INPUT)?)?);
        Ok(())
    }
}
