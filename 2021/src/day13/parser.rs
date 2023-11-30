use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, line_ending, multispace0, one_of},
    combinator::opt,
    multi::{fold_many0, many0, many1},
    sequence::{preceded, separated_pair, terminated},
    IResult, Parser,
};

pub(crate) enum Fold {
    AlongX(u32),
    AlongY(u32),
}

pub(crate) struct ParsedInput {
    pub(crate) dots: Vec<(u32, u32)>,
    pub(crate) instructions: Vec<Fold>,
}

pub(crate) fn parse(input: &str) -> IResult<&str, ParsedInput> {
    let (input, dots) = many1(coord)(input)?;
    let (input, _) = multispace0(input)?;
    let (input, instructions) = many0(instruction)(input)?;
    Ok((input, ParsedInput { dots, instructions }))
}

fn uint32(input: &str) -> IResult<&str, u32> {
    let (input, x) = fold_many0(
        one_of("0123456789"),
        || 0,
        |ans, c| ans * 10 + c.to_digit(10).unwrap(),
    )(input)?;
    Ok((input, x))
}

fn coord(input: &str) -> IResult<&str, (u32, u32)> {
    terminated(separated_pair(uint32, char(','), uint32), opt(line_ending))(input)
}

fn instruction(input: &str) -> IResult<&str, Fold> {
    let (input, _) = tag("fold along ")(input)?;
    terminated(
        alt((
            preceded(tag("x="), uint32.map(Fold::AlongX)),
            preceded(tag("y="), uint32.map(Fold::AlongY)),
        )),
        opt(line_ending),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use indoc::indoc;

    #[test]
    fn test_parser1() -> Result<()> {
        assert!(matches!(coord("6,10")?, ("", (6, 10))));
        Ok(())
    }

    #[test]
    fn test_parser2() -> Result<()> {
        assert!(matches!(
            coord(indoc! {"
            6,10
        "})?,
            ("", (6, 10))
        ));
        Ok(())
    }

    #[test]
    fn test_parser3() -> Result<()> {
        assert!(matches!(
            instruction("fold along x=42")?,
            ("", Fold::AlongX(42))
        ));
        Ok(())
    }

    #[test]
    fn test_parser4() -> Result<()> {
        assert!(matches!(
            instruction("fold along y=42")?,
            ("", Fold::AlongY(42))
        ));
        Ok(())
    }
}
