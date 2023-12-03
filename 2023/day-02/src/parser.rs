use nom::branch::alt;
use nom::character::complete::u32;
use nom::combinator::map;
use nom::sequence::{delimited, terminated};
use nom::{
    bytes::complete::tag, character::complete::line_ending, multi::separated_list1, IResult,
};

use crate::Game;

use super::CubeCollection;

#[tracing::instrument(level = "trace", skip(input))]
pub fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, games) = separated_list1(line_ending, game)(input)?;
    Ok((input, games))
}

#[tracing::instrument(level = "trace", skip(input))]
fn game(input: &str) -> IResult<&str, Game> {
    let (input, game_id) = delimited(tag("Game "), u32, tag(": "))(input)?;
    let (input, cubes) = separated_list1(tag("; "), game_pick)(input)?;
    Ok((input, Game::new(game_id, cubes)))
}

#[tracing::instrument(level = "trace", skip(input))]
fn game_pick(input: &str) -> IResult<&str, CubeCollection> {
    let mut cube = CubeCollection::default();
    let (input, _) = separated_list1(
        tag(", "),
        alt((
            map(terminated(u32, tag(" red")), |n| {
                cube.red += n;
            }),
            map(terminated(u32, tag(" green")), |n| {
                cube.green += n;
            }),
            map(terminated(u32, tag(" blue")), |n| {
                cube.blue += n;
            }),
        )),
    )(input)?;
    Ok((input, cube))
}
