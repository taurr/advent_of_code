mod parser;
mod range_map;
mod seed_map;

use parser::parse;
use rayon::iter::ParallelIterator;
use tracing::*;

use crate::custom_error::AocError;

// NAIVE APPROACH!!!
// Can be solved a lot more efficient by considering overlapping ranges and create a projection
// directly from seed into location.

#[tracing::instrument(level = "trace", skip(_input))]
pub fn process(_input: &str) -> Result<u32, AocError> {
    let (_, seed_map) = parse(_input).map_err(|e| AocError::ParserError(e.to_string()))?;

    let min = seed_map
        .seeds
        .destination_ranges()
        .map(|seed| u32::from(seed_map.find_location_from_seed(seed)))
        .min()
        .ok_or(AocError::ProcessError("No minimum value found".to_string()))?;
    Ok(min)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use rstest::*;
    use test_log::test;

    #[fixture]
    fn input() -> &'static str {
        indoc::indoc! {r#"
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
        "#}
    }

    #[tracing::instrument(level = "trace", skip())]
    #[test(rstest)]
    fn test_process(input: &str) -> Result<()> {
        assert_eq!(46, process(input)?);
        Ok(())
    }
}
