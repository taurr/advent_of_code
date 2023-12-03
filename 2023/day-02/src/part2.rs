use crate::{custom_error::AocError, parser::parse_games, GamePick};

#[tracing::instrument(level = "trace", skip(input))]
pub fn process(input: &str) -> Result<u32, AocError> {
    let (_, games) = parse_games(input).map_err(|e| AocError::ParserError(e.to_string()))?;
    let games = games.into_iter().map(|game| {
        game.pickings
            .into_iter()
            .fold(GamePick::default(), |mut cube, picking| {
                cube.red = cube.red.max(picking.red);
                cube.green = cube.green.max(picking.green);
                cube.blue = cube.blue.max(picking.blue);
                cube
            })
    });
    let power = games.map(|cube| cube.red * cube.green * cube.blue);
    let power_sum = power.sum::<u32>();
    Ok(power_sum)
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
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#};
        assert_eq!(2286, process(input)?);
        Ok(())
    }
}
