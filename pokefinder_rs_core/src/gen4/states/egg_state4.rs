use crate::parents::states::{EggGeneratorState, StateT};
use crate::parents::PersonalInfo;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Default)]
pub struct EggGeneratorState4 {
    pub base: EggGeneratorState,
    pub pickup_advances: u32,
    pub call: u8,
    pub chatot: u8,
}

impl EggGeneratorState4 {
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

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct EggSearcherState4<'a> {
    pub seed: u32,
    pub state: &'a EggGeneratorState4,
}

impl<'a> EggSearcherState4<'a> {
    pub fn new(seed: u32, state: &'a EggGeneratorState4) -> Self {
        Self { seed, state }
    }
}
