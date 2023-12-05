use std::collections::{BTreeMap, HashSet};

use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, space1, u32},
    combinator::complete,
    multi::fold_many1,
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};

pub fn parse(input: &str) -> IResult<&str, BTreeMap<u32, (HashSet<u32>, HashSet<u32>)>> {
    let (input, result) = complete(fold_many1(
        terminated(line_parser, line_ending),
        BTreeMap::new,
        |mut map, (card, (winning, ours))| {
            map.insert(card, (winning, ours));
            map
        },
    ))(input)?;

    Ok((input, result))
}

fn set(input: &str) -> IResult<&str, HashSet<u32>> {
    fold_many1(preceded(space1, u32), HashSet::new, |mut set, num| {
        set.insert(num);
        set
    })(input)
}

fn line_parser(input: &str) -> IResult<&str, (u32, (HashSet<u32>, HashSet<u32>))> {
    let (input, card) = delimited(tuple((tag("Card"), space1)), u32, tag(":"))(input)?;
    let (input, (winning, ours)) = separated_pair(set, tag(" |"), set)(input)?;
    Ok((input, (card, (winning, ours))))
}
