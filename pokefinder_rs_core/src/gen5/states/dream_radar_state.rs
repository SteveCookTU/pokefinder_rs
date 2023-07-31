use crate::parents::states::{GeneratorState, StateT};
use crate::parents::PersonalInfo;

#[derive(Copy, Clone)]
pub struct DreamRadarState {
    pub base: GeneratorState,
    pub needle: u8,
}

impl DreamRadarState {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        needle: u8,
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
            needle,
        }
    }
}

impl StateT for DreamRadarState {
    fn update_stats(&mut self, info: &PersonalInfo) {
        self.base.update_stats(info);
    }
}
