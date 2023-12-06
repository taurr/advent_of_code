use itertools::Itertools;
use tracing::*;

use crate::custom_error::AocError;

mod parser;

pub fn process(input: &str) -> Result<u32, AocError> {
    let (_, input) = parser::parse(input).map_err(|e| AocError::ParserError(e.to_string()))?;

    let input = input.collect_vec();

    let r = input.into_iter().map(|(time, distance)| {
        let time = time as f64;
        let distance = distance as f64;
        let h = ((time) - (time * time - 4. * distance).sqrt()) / 2.0;

        let h_max = time / 2.0;

        let first_win = if h.fract() < f64::EPSILON {
            h.ceil() + 1.
        } else {
            h.ceil()
        };
        let last_win = (h_max + h_max - h).ceil();
        let diff = last_win - first_win;
        trace!(time, distance, h, h_max, first_win, last_win, diff);

        diff as u32
    });

    Ok(r.into_iter().product())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use rstest::*;
    use test_log::test;

    #[test(rstest)]
    fn test_process() -> Result<()> {
        let input = indoc::indoc! {r#"
            Time:      7  15   30
            Distance:  9  40  200
        "#};
        assert_eq!(288, process(input)?);
        Ok(())
    }
}
