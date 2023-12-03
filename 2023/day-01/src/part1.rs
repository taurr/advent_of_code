use crate::custom_error::AocError;
use itertools::Itertools;
use tracing::{debug, info};

#[tracing::instrument(level = "trace", skip(input))]
pub fn process(input: &str) -> Result<u32, AocError> {
    let calibration_value = input
        .lines()
        .map(process_line)
        .process_results(|iter| iter.sum())?;
    info!(calibration_value);
    Ok(calibration_value)
}

#[tracing::instrument(level = "trace", skip(line))]
fn process_line(line: &str) -> Result<u32, AocError> {
    let mut digit_iterator = line.chars().filter_map(|c| c.to_digit(10));
    let first = digit_iterator
        .next()
        .ok_or(AocError::invalid_input(line, "expected at least 1 digit"))?;
    let last = digit_iterator.last().unwrap_or(first);

    let value = first * 10 + last;
    debug!(value);
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use rstest::*;

    #[tracing::instrument(level = "trace", skip())]
    #[rstest]
    fn test_process() -> Result<()> {
        let input = indoc::indoc! {r#"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet"#};
        assert_eq!(142, process(input)?);
        Ok(())
    }
}
