use nom::{
    bytes::complete::tag, character::complete::*, combinator::*, multi::*, sequence::*, IResult,
};
use tracing::*;

pub fn parse(input: &str) -> IResult<&str, ()> {
    Ok((input, ()))
}
