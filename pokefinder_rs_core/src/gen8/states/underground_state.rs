use crate::parents::states::{GeneratorState, StateT};
use crate::parents::PersonalInfo;

#[derive(Copy, Clone)]
pub struct UndergroundState {
    pub base: GeneratorState,
    pub egg_move: u16,
    pub item: u16,
    pub specie: u16,
}

impl UndergroundState {
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
        specie: u16,
        info: &PersonalInfo,
    ) -> Self {
        Self {
            base: GeneratorState::new_with_ec(
                advances, ec, pid, ivs, ability, gender, level, nature, shiny, info,
            ),
            egg_move,
            item,
            specie,
        }
    }
}

impl StateT for UndergroundState {
    fn update_stats(&mut self, info: &PersonalInfo) {
        self.base.update_stats(info);
    }
}
