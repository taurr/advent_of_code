use std::collections::HashMap;

use anyhow::{bail, Result};
use itertools::{Itertools, MinMaxResult};

use self::parser::{parse, ParsedInput};

pub fn solve_puzzle() -> Result<()> {
    const INPUT: &str = include_str!("../assets/day14.txt");
    println!("Part1: {}", part1(parse_input(INPUT)?)?);
    println!("Part2: {}", part2(parse_input(INPUT)?)?);
    Ok(())
}

fn parse_input(input: &'static str) -> Result<ParsedInput> {
    let (_, parsed) = parse(input)?;
    Ok(parsed)
}

fn part1(input: ParsedInput) -> Result<usize> {
    let mut ans = input.start;
    for _ in 0..10 {
        let cap = ans.len() * 2;
        let mut itt = ans.into_iter().peekable();
        ans = Vec::with_capacity(cap);
        while let Some(g1) = itt.next() {
            ans.push(g1);
            if let Some(g2) = itt.peek() {
                if let Some(g) = input.rules.get(&(g1, *g2)) {
                    ans.push(*g);
                }
            }
        }
    }

    println!("ans (len): {:?}", ans.len());
    let counts = ans.into_iter().counts_by(|c| c);
    println!("counts: {:?}", counts);

    match counts.values().copied().minmax() {
        MinMaxResult::NoElements => bail!("no min & max"),
        MinMaxResult::OneElement(_) => bail!("only 1 element"),
        MinMaxResult::MinMax(min, max) => Ok(max - min),
    }
}

fn part2(input: ParsedInput) -> Result<usize> {
    let mut map = HashMap::<(char, char), usize>::new();
    for (a, b) in input.start.iter().tuple_windows() {
        let key = (*a, *b);
        *map.entry(key).or_default() += 1;
    }

    fn replace_genomes(input: &ParsedInput, freqs: HashMap<(char, char), usize>) -> HashMap<(char, char), usize> {
        let mut new_freqs = freqs.clone();
        for (k, v) in freqs.into_iter() {
            if let Some(g) = input.rules.get(&k) {
                *new_freqs.entry(k).or_default() -= v;
                let nk1 = (k.0, *g);
                *new_freqs.entry(nk1).or_default() += v;
                let nk2 = (*g, k.1);
                *new_freqs.entry(nk2).or_default() += v;
            }
        }
        new_freqs
    }

    // update genome pair frequencies
    for _ in 0..40 {
        map = replace_genomes(&input, map);
    }

    // count characters (all chars except start/end will be counted twice)
    let mut map: HashMap<char, usize> =
        map.into_iter()
            .fold(HashMap::new(), |mut map, ((k1, k2), v)| {
                *map.entry(k1).or_default() += v;
                *map.entry(k2).or_default() += v;
                map
            });
    *map.get_mut(&input.start[0]).unwrap() += 1;
    *map.get_mut(&input.start[input.start.len() - 1]).unwrap() += 1;

    match map.into_iter().map(|(_, v)| v/2).minmax() {
        MinMaxResult::NoElements => bail!("no min & max"),
        MinMaxResult::OneElement(_) => bail!("only 1 element"),
        MinMaxResult::MinMax(min, max) => Ok(max - min),
    }
}

mod parser;

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C
    "};

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(1588, part1(parse_input(INPUT)?)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(2188189693529, part2(parse_input(INPUT)?)?);
        Ok(())
    }
}
