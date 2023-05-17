use crate::parents::states::{StateT, WildGeneratorState, WildSearcherState};
use crate::parents::PersonalInfo;

/// Generator state for Gen4 wild encounters
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct WildGeneratorState4 {
    /// Base wild generator state data
    pub base: WildGeneratorState,
    /// State advances after battle
    pub battle_advances: u32,
    /// Elm/Irwin call value
    pub call: u8,
    /// Chatot pitch value
    pub chatot: u8,
}

impl WildGeneratorState4 {
    /// Construct a new [`WildGeneratorState4`] struct
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        prng: u16,
        battle_advances: u32,
        advances: u32,
        pid: u32,
        ivs: [u8; 6],
        ability: u8,
        gender: u8,
        level: u8,
        nature: u8,
        shiny: u8,
        encounter_slot: u8,
        item: u16,
        species: u16,
        form: u8,
        info: &PersonalInfo,
    ) -> Self {
        Self {
            base: WildGeneratorState::new(
                advances,
                pid,
                ivs,
                ability,
                gender,
                level,
                nature,
                shiny,
                encounter_slot,
                item,
                species,
                form,
                info,
            ),
            battle_advances,
            call: (prng % 3) as u8,
            chatot: (((prng as u32) % 8192).wrapping_mul(100) >> 13) as u8,
        }
    }
}

impl StateT for WildGeneratorState4 {
    fn update_stats(&mut self, info: &PersonalInfo) {
        self.base.update_stats(info);
    }
}

/// Searcher state for Gen4 wild encounters
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct WildSearcherState4 {
    /// Base wild searcher state data
    pub base: WildSearcherState,
    /// Advances of the state
    pub advances: u32,
}

impl WildSearcherState4 {
    /// Construct a new [`WildSearcherState4`]
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
        encounter_slot: u8,
        item: u16,
        species: u16,
        form: u8,
        info: &PersonalInfo,
    ) -> Self {
        Self {
            base: WildSearcherState::new(
                seed,
                pid,
                ivs,
                ability,
                gender,
                level,
                nature,
                shiny,
                encounter_slot,
                item,
                species,
                form,
                info,
            ),
            advances: 0,
        }
    }
}

impl StateT for WildSearcherState4 {
    fn update_stats(&mut self, info: &PersonalInfo) {
        self.base.update_stats(info);
    }
}
