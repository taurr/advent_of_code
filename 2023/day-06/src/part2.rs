use crate::custom_error::AocError;
use tracing::*;

mod parser;

pub fn process(input: &str) -> Result<u64, AocError> {
    let (_, (time, distance)) =
        parser::parse(input).map_err(|e| AocError::ParserError(e.to_string()))?;

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

    Ok(diff as u64)
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
        assert_eq!(71503, process(input)?);
        Ok(())
    }
}
