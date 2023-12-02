use crate::custom_error::AocError;

pub fn process(_input: &str) -> Result<u32, AocError> {
    todo!("day 01 - part 2");
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use rstest::*;

    #[rstest]
    fn test_process() -> Result<()> {
        todo!("haven't built test yet");
        let input = indoc::indoc! {r#"
            "#};
        assert_eq!(0, process(input)?);
        Ok(())
    }
}
