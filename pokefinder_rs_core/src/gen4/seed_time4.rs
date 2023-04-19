use crate::gen4::HGSSRoamer;
use crate::util::{utilities4, DateTime};

#[derive(Copy, Clone, Default)]
pub struct SeedTime4 {
    pub date_time: DateTime,
    pub delay: u32,
}

impl SeedTime4 {
    pub fn new(date_time: DateTime, delay: u32) -> Self {
        Self { date_time, delay }
    }
}

#[derive(Copy, Clone, Default)]
pub struct SeedTimeCalibrate4 {
    pub base: SeedTime4,
    pub roamer: Option<HGSSRoamer>,
    pub seed: u32,
}

impl SeedTimeCalibrate4 {
    pub fn new(date_time: DateTime, delay: u32) -> Self {
        let seed = utilities4::calc_seed(&date_time, delay);
        Self {
            base: SeedTime4::new(date_time, delay),
            roamer: None,
            seed,
        }
    }

    pub fn new_with_roamer(
        date_time: DateTime,
        delay: u32,
        roamers: [bool; 3],
        routes: [u8; 3],
    ) -> Self {
        let seed = utilities4::calc_seed(&date_time, delay);
        Self {
            base: SeedTime4::new(date_time, delay),
            roamer: Some(HGSSRoamer::new(seed, roamers, routes)),
            seed,
        }
    }

    pub fn get_sequence(&self) -> String {
        if let Some(roamer) = &self.roamer {
            utilities4::get_calls(self.seed, roamer.skips)
        } else {
            utilities4::coin_flips(self.seed)
        }
    }
}
