use crate::parents::states::{GeneratorState, SearcherState, StateT};
use crate::parents::PersonalInfo;

/// Generator Struct for Gen4 static encounters
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct GeneratorState4 {
    /// Base generator state data
    pub base: GeneratorState,
    /// Elm/Irwin call value
    pub call: u8,
    /// Chatot pitch value
    pub chatot: u8,
}

impl GeneratorState4 {
    /// Construct a new [`GeneratorState4`] struct
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        prng: u16,
        advances: u32,
        pid: u32,
        ivs: [u8; 6],
        ability: u8,
        gender: u8,
        level: u8,
        nature: u8,
        shiny: u8,
        info: &PersonalInfo,
    ) -> Self {
        Self {
            base: GeneratorState::new(
                advances, pid, ivs, ability, gender, level, nature, shiny, info,
            ),
            call: (prng % 3) as u8,
            chatot: (((prng as u32) % 8192).wrapping_mul(100) >> 13) as u8,
        }
    }
}

impl StateT for GeneratorState4 {
    fn update_stats(&mut self, info: &PersonalInfo) {
        self.base.update_stats(info);
    }
}

/// Searcher struct for Gen4 static encounters
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct SearcherState4 {
    /// Base searcher state data
    pub base: SearcherState,
    /// Advances of the state
    pub advances: u32,
}

impl SearcherState4 {
    /// Construct a new [`SearcherState4`] struct
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        seed: u32,
        pid: u32,
        ivs: [u8; 6],
        ability: u8,
        gender: u8,
        level: u8,
        nature: u8,
        shiny: u8,
        info: &PersonalInfo,
    ) -> Self {
        Self {
            base: SearcherState::new(seed, pid, ivs, ability, gender, level, nature, shiny, info),
            advances: 0,
        }
    }
}

impl StateT for SearcherState4 {
    fn update_stats(&mut self, info: &PersonalInfo) {
        self.base.update_stats(info);
    }
}
