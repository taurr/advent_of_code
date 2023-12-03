pub mod custom_error;

pub mod part1;
pub mod part2;

mod parser;

#[derive(Debug, derive_new::new)]
pub struct Game {
    id: u32,
    rounds: Vec<CubeCollection>,
}

#[derive(Debug, derive_new::new, Default)]
pub struct CubeCollection {
    red: u32,
    green: u32,
    blue: u32,
}

impl Game {
    pub fn is_valid(&self, pick: &CubeCollection) -> bool {
        self.rounds
            .iter()
            .all(|cube| cube.red <= pick.red && cube.green <= pick.green && cube.blue <= pick.blue)
    }

    pub fn cubes_in_game(&self) -> CubeCollection {
        self.rounds
            .iter()
            .fold(CubeCollection::default(), |mut cube, picking| {
                cube.red = cube.red.max(picking.red);
                cube.green = cube.green.max(picking.green);
                cube.blue = cube.blue.max(picking.blue);
                cube
            })
    }
}
