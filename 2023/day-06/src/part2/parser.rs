use nom::{
    bytes::complete::tag, character::complete::*, combinator::*, multi::*, sequence::*, IResult,
};
use tracing::debug;

pub fn parse(input: &str) -> IResult<&str, (u64, u64)> {
    let (input, times) = delimited(
        tag("Time:"),
        fold_many1(
            preceded(space1, digit1),
            String::default,
            |mut acc, item| {
                acc.push_str(item);
                acc
            },
        ),
        opt(line_ending),
    )(input)?;
    let (input, distances) = delimited(
        tag("Distance:"),
        fold_many1(
            preceded(space1, digit1),
            String::default,
            |mut acc, item| {
                acc.push_str(item);
                acc
            },
        ),
        opt(line_ending),
    )(input)?;
    debug!(?times, ?distances);
    Ok((input, (times.parse().unwrap(), distances.parse().unwrap())))
}
