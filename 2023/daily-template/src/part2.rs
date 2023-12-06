use crate::{custom_error::AocError, parser::parse};
use tracing::*;

pub fn process(input: &str) -> Result<u32, AocError> {
    let (_, _input) = parse(input).map_err(|e| AocError::ParserError(e.to_string()))?;

    todo!("{{project-name}} - part 2");

    Ok(1)
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

        "#};
        assert_eq!(0, process(input)?);
        Ok(())
    }
}
