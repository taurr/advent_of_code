use std::collections::HashMap;

use crate::{custom_error::AocError, parser::parse};

pub fn process(input: &str) -> Result<u32, AocError> {
    let (_, cards) = parse(input).map_err(|e| AocError::ParserError(e.to_string()))?;

    let result = cards
        .into_iter()
        .map(|(card, (winning, ours))| (card, winning.intersection(&ours).count() as u32))
        .fold(HashMap::new(), |mut map, (card, wins)| {
            let number_of_cards = *map.entry(card).and_modify(|e| *e += 1u32).or_insert(1);
            for card_won in (card + 1)..=(card + wins) {
                map.entry(card_won)
                    .and_modify(|e| *e += number_of_cards)
                    .or_insert(number_of_cards);
            }
            map
        })
        .into_values()
        .sum::<u32>();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use rstest::*;

    #[rstest]
    fn test_process() -> Result<()> {
        tracing_subscriber::fmt::init();
        let input = indoc::indoc! {r#"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "#};
        assert_eq!(30, process(input)?);
        Ok(())
    }
}
