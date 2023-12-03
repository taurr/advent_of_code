use crate::{custom_error::AocError, parser::parse_games, GamePick};

#[tracing::instrument(level = "trace", skip(input, total_cubes))]
pub fn process<'a>(input: &'a str, total_cubes: GamePick) -> Result<u32, AocError> {
    let (_, games) = parse_games(input).map_err(|e| AocError::ParserError(e.to_string()))?;
    let valid_games_ids = games.into_iter().filter_map(|game| {
        game.pickings
            .iter()
            .all(|cube| {
                cube.red <= total_cubes.red
                    && cube.green <= total_cubes.green
                    && cube.blue <= total_cubes.blue
            })
            .then_some(game.id)
    });
    let sum = valid_games_ids.sum();
    Ok(sum)
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
        assert_eq!(8, process(input, GamePick::new(12, 13, 14),)?);
        Ok(())
    }
}
