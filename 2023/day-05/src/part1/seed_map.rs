use derive_more::*;

use super::range_map::RangeMap;

#[derive(Debug, Constructor, Clone, Copy, PartialEq, Eq, From, Into)]
pub struct Soil(u32);

#[derive(Debug, Constructor, Clone, Copy, PartialEq, Eq, From, Into)]
pub struct Fertilizer(u32);

#[derive(Debug, Constructor, Clone, Copy, PartialEq, Eq, From, Into)]
pub struct Water(u32);

#[derive(Debug, Constructor, Clone, Copy, PartialEq, Eq, From, Into)]
pub struct Light(u32);

#[derive(Debug, Constructor, Clone, Copy, PartialEq, Eq, From, Into)]
pub struct Temperature(u32);

#[derive(Debug, Constructor, Clone, Copy, PartialEq, Eq, From, Into)]
pub struct Humidity(u32);

#[derive(Debug, Constructor, Clone, Copy, PartialEq, Eq, From, Into)]
pub struct Location(u32);

#[derive(Debug)]
pub struct SeedMap {
    pub seeds: Vec<u32>,
    pub seed_to_soil: RangeMap,
    pub soil_to_fertilizer: RangeMap,
    pub fertilizer_to_water: RangeMap,
    pub water_to_light: RangeMap,
    pub light_to_temperature: RangeMap,
    pub temperature_to_humidity: RangeMap,
    pub humidity_to_location: RangeMap,
}

impl SeedMap {
    pub fn find_location_from_seed(&self, seed: u32) -> Location {
        self.find_location(self.find_humidity(self.find_temperature(
            self.find_light(self.find_water(self.find_fertilizer(self.find_soil(seed)))),
        )))
    }

    pub fn find_soil(&self, seed: u32) -> Soil {
        self.seed_to_soil.find_destination(seed).into()
    }
    pub fn find_fertilizer(&self, soil: Soil) -> Fertilizer {
        self.soil_to_fertilizer.find_destination(soil.into()).into()
    }
    pub fn find_water(&self, fertilizer: Fertilizer) -> Water {
        self.fertilizer_to_water
            .find_destination(fertilizer.into())
            .into()
    }
    pub fn find_light(&self, water: Water) -> Light {
        self.water_to_light.find_destination(water.into()).into()
    }
    pub fn find_temperature(&self, light: Light) -> Temperature {
        self.light_to_temperature
            .find_destination(light.into())
            .into()
    }
    pub fn find_humidity(&self, temperature: Temperature) -> Humidity {
        self.temperature_to_humidity
            .find_destination(temperature.into())
            .into()
    }
    pub fn find_location(&self, humidity: Humidity) -> Location {
        self.humidity_to_location
            .find_destination(humidity.into())
            .into()
    }
}
