use nom::{
    bytes::complete::tag, character::complete::*, combinator::*, multi::*, sequence::*, IResult,
};
use tracing::*;

use super::{
    range_map::{RangeMap, RangeMapEntry},
    seed_map::SeedMap,
};

#[tracing::instrument(level = "trace")]
pub fn headline<'a>(headline_tag: &'a str) -> impl FnMut(&str) -> IResult<&str, ()> + 'a {
    move |input| map(tuple((tag(headline_tag), line_ending)), |_| ())(input)
}

#[tracing::instrument(level = "trace", skip(input))]
pub fn space_separated_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(space1, u32)(input)
}

#[tracing::instrument(level = "trace", skip(input))]
pub fn range_map(input: &str) -> IResult<&str, RangeMap> {
    terminated(
        map(many1(range_map_entry), |entries| RangeMap { entries }),
        multispace0,
    )(input)
}

#[tracing::instrument(level = "trace", skip(input))]
pub fn range_map_entry(input: &str) -> IResult<&str, RangeMapEntry> {
    let (input, destination_start) = terminated(u32, space1)(input)?;
    let (input, source_start) = terminated(u32, space1)(input)?;
    let (input, range_length) = terminated(u32, line_ending)(input)?;

    Ok((
        input,
        RangeMapEntry {
            source_start,
            destination_start,
            range_length,
        },
    ))
}

#[tracing::instrument(level = "trace", skip(input))]
pub fn parse(input: &str) -> IResult<&str, SeedMap> {
    trace!(?input);

    let (input, seeds) = preceded(
        tag("seeds: "),
        terminated(space_separated_numbers, multispace1),
    )(input)?;
    trace!(?seeds, ?input);

    let (input, _) = headline("seed-to-soil map:")(input)?;
    let (input, seed_to_soil) = range_map(input)?;
    trace!(?seed_to_soil, ?input);

    let (input, _) = headline("soil-to-fertilizer map:")(input)?;
    let (input, soil_to_fertilizer) = range_map(input)?;
    trace!(?soil_to_fertilizer, ?input);

    let (input, _) = headline("fertilizer-to-water map:")(input)?;
    let (input, fertilizer_to_water) = range_map(input)?;
    trace!(?fertilizer_to_water, ?input);

    let (input, _) = headline("water-to-light map:")(input)?;
    let (input, water_to_light) = range_map(input)?;
    trace!(?water_to_light, ?input);

    let (input, _) = headline("light-to-temperature map:")(input)?;
    let (input, light_to_temperature) = range_map(input)?;
    trace!(?light_to_temperature, ?input);

    let (input, _) = headline("temperature-to-humidity map:")(input)?;
    let (input, temperature_to_humidity) = range_map(input)?;
    trace!(?temperature_to_humidity, ?input);

    let (input, _) = headline("humidity-to-location map:")(input)?;
    let (input, humidity_to_location) = range_map(input)?;
    trace!(?humidity_to_location, ?input);

    Ok((
        input,
        SeedMap {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        },
    ))
}
