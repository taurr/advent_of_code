use std::iter::from_fn;

use crate::custom_error::AocError;
use tracing::{debug, info, trace};

#[tracing::instrument(level = "trace", skip(input))]
pub fn process(input: &str) -> anyhow::Result<u32, AocError> {
    let calibration_value = input.lines().map(process_line).sum();
    info!(calibration_value);
    Ok(calibration_value)
}

#[tracing::instrument(level = "trace")]
fn process_line(line: &str) -> u32 {
    let mut index = 0;
    let mut line_iter = from_fn(move || loop {
        let lookup = [
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
            ("zero", 0),
        ];
        let indexed_line = &line[index..];
        index += 1;
        let digit = lookup
            .into_iter()
            .find(|(prefix, _)| indexed_line.starts_with(prefix))
            .map(|(prefix, digit)| {
                let size = prefix.len();
                trace!(indexed_line, prefix, size, digit);
                digit
            })
            .or_else(|| {
                let digit = indexed_line.chars().next().and_then(|c| c.to_digit(10));
                trace!(indexed_line, digit);
                digit
            });

        if digit.is_none() && index < line.len() {
            continue;
        }
        break digit;
    });

    if let Some(first) = line_iter.next() {
        let last = line_iter.last().unwrap_or(first);
        let value = first * 10 + last;
        debug!(value);
        value
    } else {
        panic!("No digits found");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use rstest::*;

    #[fixture]
    #[once]
    #[tracing::instrument(level = "trace", skip())]
    fn trace() -> () {
        tracing_subscriber::fmt::init();
    }

    #[rstest]
    #[tracing::instrument(level = "trace", skip())]
    fn test_overlapping_numbers(_trace: &()) -> Result<()> {
        let input = indoc::indoc! {r#"oneight"#};
        assert_eq!(18, process(input)?);
        Ok(())
    }

    #[rstest]
    #[tracing::instrument(level = "trace", skip())]
    fn test_process(_trace: &()) -> Result<()> {
        let input = indoc::indoc! {r#"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen"#};
        assert_eq!(281, process(input)?);
        Ok(())
    }
}
