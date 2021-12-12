use anyhow::Result;
use itertools::Itertools;
use petgraph::{graph::NodeIndex, Graph, Undirected};
use std::{collections::HashMap, path::Path};

pub fn solve_puzzle(_input_path: &Path) -> Result<()> {
    println!(
        "Day12, Part1: {}",
        part1(parse_input(include_str!("../assets/day12.txt")))
    );
    println!(
        "Day12, Part2: {}",
        part2(parse_input(include_str!("../assets/day12.txt")))
    );
    Ok(())
}

#[derive(PartialEq, PartialOrd)]
enum CaveSize {
    Small,
    Big,
}

impl From<String> for CaveSize {
    fn from(name: String) -> Self {
        if name == name.to_lowercase() {
            CaveSize::Small
        } else {
            CaveSize::Big
        }
    }
}

struct CaveData {
    #[allow(dead_code)]
    name: String,
    size: CaveSize,
}

fn parse_input(
    input: &str,
) -> (
    HashMap<String, NodeIndex>,
    Graph<CaveData, usize, Undirected>,
) {
    let mut node_indices = HashMap::new();
    let mut graph = Graph::<CaveData, usize, Undirected>::new_undirected();
    for (a, b) in input
        .lines()
        .map(|l| l.split_once('-').expect("No seperator in line"))
    {
        let a = a.to_owned();
        let b = b.to_owned();

        let a_ix = node_indices.get(&a).copied().unwrap_or_else(|| {
            let ix = graph.add_node(CaveData {
                name: a.clone(),
                size: a.clone().into(),
            });
            node_indices.insert(a, ix);
            ix
        });
        let b_ix = node_indices.get(&b).copied().unwrap_or_else(|| {
            let ix = graph.add_node(CaveData {
                name: b.clone(),
                size: b.clone().into(),
            });
            node_indices.insert(b, ix);
            ix
        });
        graph.add_edge(a_ix, b_ix, 1);
    }
    (node_indices, graph)
}

fn part1(
    (node_indices, graph): (
        HashMap<String, NodeIndex>,
        Graph<CaveData, usize, Undirected>,
    ),
) -> usize {
    let start = *node_indices.get("start").unwrap();
    let end = *node_indices.get("end").unwrap();

    let mut final_paths: Vec<Vec<NodeIndex>> = Vec::new();
    let mut investigating_paths = vec![vec![start]];

    while let Some(path) = investigating_paths.pop() {
        for node in graph.neighbors(path[path.len() - 1]) {
            if node == start {
                // back at start - don't keep this path
                continue;
            }

            if node == end {
                // reached end - don't investigate further
                let mut path = path.clone();
                path.push(node);
                final_paths.push(path);
                continue;
            }

            let node_weight = graph.node_weight(node).unwrap();
            if node_weight.size == CaveSize::Small
                && path
                    .iter()
                    .filter(|&&idx| graph.node_weight(idx).unwrap().size == CaveSize::Small)
                    .any(|&idx| idx == node)
            {
                continue;
            }

            let mut path = path.clone();
            path.push(node);
            investigating_paths.push(path);
        }
    }

    final_paths.len()
}

fn part2(
    (node_indices, graph): (
        HashMap<String, NodeIndex>,
        Graph<CaveData, usize, Undirected>,
    ),
) -> usize {
    let start = *node_indices.get("start").unwrap();
    let end = *node_indices.get("end").unwrap();

    let mut final_paths: Vec<Vec<NodeIndex>> = Vec::new();
    let mut investigating_paths = vec![vec![start]];

    while let Some(path) = investigating_paths.pop() {
        for node in graph.neighbors(path[path.len() - 1]) {
            if node == start {
                // back at start - don't keep this path
                continue;
            }

            if node == end {
                // reached end - don't investigate further
                let mut path = path.clone();
                path.push(node);
                final_paths.push(path);
                continue;
            }

            let mut path = path.clone();
            path.push(node);
            let node_weight = graph.node_weight(node).unwrap();
            if node_weight.size == CaveSize::Small {
                let counts = path
                    .iter()
                    .filter(|&&idx| graph.node_weight(idx).unwrap().size == CaveSize::Small)
                    .counts_by(|&idx| idx);
                if let Some(3) = counts.get(&node) {
                    // too many small caves visited - don't investigate further
                    continue;
                }
                if counts.values().filter(|&&v| v == 2).count() > 1 {
                    // too many small caves visited - don't investigate further
                    continue;
                }
            }

            investigating_paths.push(path);
        }
    }
    // for path in final_paths.iter() {
    //     println!(
    //         "{}",
    //         path.iter()
    //             .map(|&idx| &graph.node_weight(idx).unwrap().name)
    //             .join(", ")
    //     );
    // }
    final_paths.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT_SMALL: &str = indoc! {"
        start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end
    "};

    const INPUT_MEDIUM: &str = indoc! {"
        dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc
    "};

    const INPUT_LARGE: &str = indoc! {"
        fs-end
        he-DX
        fs-he
        start-DX
        pj-DX
        end-zg
        zg-sl
        zg-pj
        pj-he
        RW-he
        fs-DX
        pj-RW
        zg-RW
        start-pj
        he-WI
        zg-he
        pj-fs
        start-RW
    "};

    #[test]
    fn test_part1_small() {
        assert_eq!(10, part1(parse_input(INPUT_SMALL)));
    }

    #[test]
    fn test_part1_medium() {
        assert_eq!(19, part1(parse_input(INPUT_MEDIUM)));
    }

    #[test]
    fn test_part1_large() {
        assert_eq!(226, part1(parse_input(INPUT_LARGE)));
    }

    #[test]
    fn test_part2_small() {
        assert_eq!(36, part2(parse_input(INPUT_SMALL)));
    }

    #[test]
    fn test_part2_medium() {
        assert_eq!(103, part2(parse_input(INPUT_MEDIUM)));
    }

    #[test]
    fn test_part2_large() {
        assert_eq!(3509, part2(parse_input(INPUT_LARGE)));
    }
}
