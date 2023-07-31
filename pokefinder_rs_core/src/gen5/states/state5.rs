use crate::parents::states::{GeneratorState, StateT};
use crate::parents::PersonalInfo;

#[derive(Copy, Clone)]
pub struct State5 {
    pub base: GeneratorState,
    pub chatot: u8,
}

impl State5 {
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
            chatot: (prng / 82) as u8,
        }
    }
}

impl StateT for State5 {
    fn update_stats(&mut self, info: &PersonalInfo) {
        self.base.update_stats(info);
    }
}
