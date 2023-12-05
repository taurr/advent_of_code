use std::collections::HashMap as Map;

#[derive(Debug, derive_more::IsVariant, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MapItem {
    Symbol(char),
    Number(NumberId),
}

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Copy,
    Clone,
    Default,
    Hash,
    derive_more::From,
    derive_more::Deref,
    derive_more::AddAssign,
)]
pub struct NumberId(u32);

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Copy,
    Clone,
    Default,
    Hash,
    derive_more::From,
    derive_more::Add,
    derive_more::AddAssign,
    derive_more::Sub,
    derive_more::SubAssign,
)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

pub fn parse_engine_map(input: &str) -> (Map<Coord, MapItem>, Map<NumberId, u32>) {
    let mut number_map: Map<NumberId, u32> = Default::default();
    let mut item_map: Map<Coord, MapItem> = Default::default();
    let mut number_id = Default::default();

    for (line, input) in input.lines().enumerate() {
        for (column, ch) in input.chars().enumerate() {
            match ch {
                '.' => {}
                ch if ch.is_ascii_digit() => {
                    let digit = ch.to_digit(10).expect("should be a digit");
                    let number_id = match item_map.get(&Coord {
                        x: column as i32 - 1,
                        y: line as i32,
                    }) {
                        Some(&MapItem::Number(number_id)) => {
                            let number = number_map
                                .get_mut(&number_id)
                                .expect("number must exist in map");
                            *number = *number * 10 + digit;
                            number_id
                        }
                        _ => {
                            number_id += 1.into();
                            number_map.insert(number_id, digit);
                            number_id
                        }
                    };
                    item_map.insert(
                        Coord {
                            x: column as i32,
                            y: line as i32,
                        },
                        MapItem::Number(number_id),
                    );
                }
                ch => {
                    item_map.insert(
                        Coord {
                            x: column as i32,
                            y: line as i32,
                        },
                        MapItem::Symbol(ch),
                    );
                }
            }
        }
    }

    (item_map, number_map)
}
