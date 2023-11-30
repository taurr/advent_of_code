use anyhow::Result;
use ndarray::{Array, Array2};
use nom::{
    character::complete::{line_ending, one_of},
    combinator::{opt, recognize},
    multi::{fold_many1, many1},
    sequence::terminated,
    IResult, Parser,
};
use std::str::FromStr;

pub(crate) fn parse_input(input: &'static str) -> Result<Array2<u16>> {
    let (_, (cols, node_weights)) = parse(input)?;
    let node_weights = Array::from_shape_vec(
        (node_weights.len() / cols as usize, cols as usize),
        node_weights,
    )?;

    Ok(node_weights)
}

fn row(input: &str) -> IResult<&str, Vec<u16>> {
    terminated(
        many1(recognize(one_of("0123456789")).map(|c: &str| u16::from_str(c).unwrap())),
        opt(line_ending),
    )(input)
}

fn parse(input: &str) -> IResult<&str, (u16, Vec<u16>)> {
    let (input, first_row) = row(input)?;
    let columns = first_row.len() as u16;
    let (input, result) = fold_many1(
        row,
        || {
            let mut v = first_row.clone();
            v.reserve_exact(first_row.len() * first_row.len() - first_row.len());
            v
        },
        |mut res, mut v| {
            res.append(&mut v);
            res
        },
    )(input)?;
    Ok((input, (columns, result)))
}
