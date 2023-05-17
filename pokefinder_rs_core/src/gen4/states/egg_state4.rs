use crate::parents::states::{EggGeneratorState, StateT};
use crate::parents::PersonalInfo;

/// State struct for Gen4 egg encounters
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Default)]
pub struct EggGeneratorState4 {
    /// Base egg generator state data
    pub base: EggGeneratorState,
    /// Advances of the state
    pub pickup_advances: u32,
    /// Elm/Irwin call value
    pub call: u8,
    /// Chatot pitch value
    pub chatot: u8,
}

impl EggGeneratorState4 {
    /// Construct a new [`EggGeneratorState4`] struct
    pub fn new(advances: u32, pid: u32, gender: u8, shiny: u8, info: &PersonalInfo) -> Self {
        Self {
            base: EggGeneratorState::new(
                advances,
                pid,
                [0; 6],
                (pid & 1) as u8,
                gender,
                1,
                (pid % 25) as u8,
                shiny,
                [0; 6],
                info,
            ),
            ..Default::default()
        }
    }

    /// Updates the egg with data that is calculated on pickup
    pub fn update(
        &mut self,
        prng: u16,
        advances: u32,
        ivs: [u8; 6],
        inheritance: [u8; 6],
        info: &PersonalInfo,
    ) {
        self.call = (prng % 3) as u8;
        self.chatot = (((prng as u32) % 8192).wrapping_mul(100) >> 13) as u8;
        self.pickup_advances = advances;
        self.base.base.base.ivs = ivs;
        self.base.base.inheritance = inheritance;
        self.update_stats(info);
    }
}

impl StateT for EggGeneratorState4 {
    fn update_stats(&mut self, info: &PersonalInfo) {
        self.base.update_stats(info);
    }
}

/// Searcher struct for Gen4 egg encounters
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct EggSearcherState4 {
    /// State seed
    pub seed: u32,
    /// Generator state information
    pub state: EggGeneratorState4,
}

impl EggSearcherState4 {
    /// Construct a new [`EggSearcherState4`] struct
    pub fn new(seed: u32, state: EggGeneratorState4) -> Self {
        Self { seed, state }
    }
}
