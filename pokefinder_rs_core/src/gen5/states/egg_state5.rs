use crate::parents::states::{EggGeneratorState, StateT};
use crate::parents::PersonalInfo;

#[derive(Copy, Clone)]
pub struct EggState5 {
    pub base: EggGeneratorState,
    pub chatot: u8,
}

impl EggState5 {
    pub fn new_ivs(
        ivs: [u8; 6],
        ability: u8,
        nature: u8,
        inheritance: [u8; 6],
        info: &PersonalInfo,
    ) -> Self {
        Self {
            base: EggGeneratorState::new(0, 0, ivs, ability, 0, 1, nature, 0, inheritance, info),
            chatot: 0,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        prng: u16,
        advances: u32,
        pid: u32,
        ivs: [u8; 6],
        ability: u8,
        gender: u8,
        nature: u8,
        shiny: u8,
        inheritance: [u8; 6],
        info: &PersonalInfo,
    ) -> Self {
        Self {
            base: EggGeneratorState::new(
                advances,
                pid,
                ivs,
                ability,
                gender,
                1,
                nature,
                shiny,
                inheritance,
                info,
            ),
            chatot: (prng / 82) as u8,
        }
    }

    pub fn update(&mut self, prng: u16, advances: u32, pid: u32, gender: u8, shiny: u8) {
        self.chatot = (prng / 82) as u8;
        self.base.advances = advances;
        self.base.base.base.pid = pid;
        self.base.base.base.gender = gender;
        self.base.base.base.shiny = shiny;
    }
}

impl StateT for EggState5 {
    fn update_stats(&mut self, info: &PersonalInfo) {
        self.base.update_stats(info);
    }
}
