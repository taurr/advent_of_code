use itertools::Itertools;

use crate::{
    custom_error::AocError,
    parser::{parse_engine_map, Coord, MapItem},
};

pub fn process(input: &str) -> Result<u32, AocError> {
    let (item_map, mut number_map) = parse_engine_map(input);

    let mut try_take = |coord: Coord| -> Option<u32> {
        match item_map.get(&coord) {
            Some(MapItem::Number(id)) => number_map.remove(&id),
            _ => None,
        }
    };

    let numbers = item_map
        .iter()
        .filter(|(_, item)| item.is_symbol())
        .flat_map(|(&coord, _)| {
            vec![
                try_take(coord + Coord { x: -1, y: -1 }),
                try_take(coord + Coord { x: 0, y: -1 }),
                try_take(coord + Coord { x: 1, y: -1 }),
                try_take(coord + Coord { x: -1, y: 0 }),
                try_take(coord + Coord { x: 1, y: 0 }),
                try_take(coord + Coord { x: -1, y: 1 }),
                try_take(coord + Coord { x: 0, y: 1 }),
                try_take(coord + Coord { x: 1, y: 1 }),
            ]
            .into_iter()
        })
        .filter_map(|n| n);

    Ok(numbers.sum1().expect("couldn't calculate sum"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use rstest::*;

    #[rstest]
    fn test_process() -> Result<()> {
        let input = indoc::indoc! {r#"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "#};
        assert_eq!(4361, process(input)?);
        Ok(())
    }
}
