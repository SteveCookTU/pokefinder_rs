use crate::parents::states::{GeneratorState, StateT};
use crate::parents::PersonalInfo;

/// Generator state for Gen8 underground encounters
#[derive(Copy, Clone)]
pub struct UndergroundState {
    /// Base generator state data
    pub base: GeneratorState,
    /// Pokemon egg move
    pub egg_move: u16,
    /// Pokemon item
    pub item: u16,
    /// Pokemon species
    pub species: u16,
}

impl UndergroundState {
    /// Construct a new [`UndergroundState`] struct
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        advances: u32,
        ec: u32,
        pid: u32,
        ivs: [u8; 6],
        ability: u8,
        gender: u8,
        level: u8,
        nature: u8,
        shiny: u8,
        egg_move: u16,
        item: u16,
        species: u16,
        info: &PersonalInfo,
    ) -> Self {
        Self {
            base: GeneratorState::new_with_ec(
                advances, ec, pid, ivs, ability, gender, level, nature, shiny, info,
            ),
            egg_move,
            item,
            species,
        }
    }
}

impl StateT for UndergroundState {
    fn update_stats(&mut self, info: &PersonalInfo) {
        self.base.update_stats(info);
    }
}
