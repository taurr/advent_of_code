use nom::{
    bytes::complete::tag, character::complete::*, combinator::*, multi::*, sequence::*, IResult,
};


pub fn parse(input: &str) -> IResult<&str, impl Iterator<Item = (u32, u32)>> {
    let (input, times) = delimited(
        tag("Time:"),
        preceded(space1, separated_list1(space1, u32)),
        opt(line_ending),
    )(input)?;
    let (input, distances) = delimited(
        tag("Distance:"),
        preceded(space1, separated_list1(space1, u32)),
        opt(line_ending),
    )(input)?;
    let result = times.into_iter().zip(distances);
    Ok((input, result))
}
