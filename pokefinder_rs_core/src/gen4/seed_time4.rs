use crate::gen4::HGSSRoamer;
use crate::util::{utilities4, DateTime};

/// State struct for seed to date/time in Gen4
#[derive(Copy, Clone, Default)]
pub struct SeedTime4 {
    /// [`DateTime`]
    pub date_time: DateTime,
    /// Delay
    pub delay: u32,
}

impl SeedTime4 {
    /// Construct a new [`SeedTime4`] struct
    pub fn new(date_time: DateTime, delay: u32) -> Self {
        Self { date_time, delay }
    }
}

/// State struct for seed to date/time calibration in Gen4
#[derive(Copy, Clone, Default)]
pub struct SeedTimeCalibrate4 {
    /// Base seed to date/time data
    pub base: SeedTime4,
    /// Roamer information
    pub roamer: Option<HGSSRoamer>,
    /// State seed
    pub seed: u32,
}

impl SeedTimeCalibrate4 {
    /// Construct a new [`SeedTimeCalibrate4`] struct
    pub fn new(date_time: DateTime, delay: u32) -> Self {
        let seed = utilities4::calc_seed(&date_time, delay);
        Self {
            base: SeedTime4::new(date_time, delay),
            roamer: None,
            seed,
        }
    }

    /// Construct a new [`SeedTimeCalibrate4`] struct including roamer data
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

    /// Returns either the coin flip or call string
    pub fn get_sequence(&self) -> String {
        if let Some(roamer) = &self.roamer {
            utilities4::get_calls(self.seed, roamer.skips)
        } else {
            utilities4::coin_flips(self.seed)
        }
    }
}
