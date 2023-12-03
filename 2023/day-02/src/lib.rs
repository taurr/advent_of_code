pub mod custom_error;

pub mod part1;
pub mod part2;

#[derive(Debug, derive_new::new)]
pub struct Game {
    id: u32,
    pickings: Vec<GamePick>,
}

#[derive(Debug, derive_new::new, Default)]
pub struct GamePick {
    red: u32,
    green: u32,
    blue: u32,
}

mod parser;
