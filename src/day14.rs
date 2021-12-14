use anyhow::{bail, Result};
use itertools::{Itertools, MinMaxResult};
use std::collections::HashMap;

use self::parser::{parse, ParsedInput};

mod parser;

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
    fn replace_genomes(genome: Vec<char>, rules: &HashMap<(char, char), char>) -> Vec<char> {
        let mut new_genome = Vec::with_capacity(genome.len() * 2);
        let mut genome_iterator = genome.into_iter().peekable();

        while let Some(g1) = genome_iterator.next() {
            new_genome.push(g1);
            if let Some(g2) = genome_iterator.peek() {
                if let Some(g) = rules.get(&(g1, *g2)) {
                    new_genome.push(*g);
                }
            }
        }
        new_genome
    }

    let mut genome = input.start;
    for _ in 0..10 {
        genome = replace_genomes(genome, &input.rules);
    }

    let counts = genome.into_iter().counts_by(|c| c);
    match counts.values().copied().minmax() {
        MinMaxResult::NoElements => bail!("no min & max"),
        MinMaxResult::OneElement(_) => bail!("only 1 element"),
        MinMaxResult::MinMax(min, max) => Ok(max - min),
    }
}

fn part2(input: ParsedInput) -> Result<usize> {
    fn update_genome_pairs(input: &ParsedInput, freqs: &mut HashMap<(char, char), usize>) {
        for (k, v) in freqs.clone().into_iter() {
            if let Some(g) = input.rules.get(&k) {
                *freqs.entry(k).or_default() -= v;
                *freqs.entry((k.0, *g)).or_default() += v;
                *freqs.entry((*g, k.1)).or_default() += v;
            }
        }
    }

    // update genome pair frequencies
    let mut genome_pair_frequencies = HashMap::new();
    input
        .start
        .iter()
        .copied()
        .tuple_windows()
        .for_each(|pair| *genome_pair_frequencies.entry(pair).or_default() += 1);

    for _ in 0..40 {
        update_genome_pairs(&input, &mut genome_pair_frequencies);
    }

    // count characters (all chars except start/end will be counted twice)
    let mut counts = HashMap::<char, usize>::new();
    counts.insert(input.start[0], 1);
    *counts
        .entry(input.start[input.start.len() - 1])
        .or_default() += 1;
    for ((k1, k2), v) in genome_pair_frequencies.into_iter() {
        *counts.entry(k1).or_default() += v;
        *counts.entry(k2).or_default() += v;
    }

    match counts.into_iter().map(|(_, v)| v / 2).minmax() {
        MinMaxResult::NoElements => bail!("no min & max"),
        MinMaxResult::OneElement(_) => bail!("only 1 element"),
        MinMaxResult::MinMax(min, max) => Ok(max - min),
    }
}

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
