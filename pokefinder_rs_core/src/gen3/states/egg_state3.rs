use crate::parents::states::{EggGeneratorState, StateT};
use crate::parents::PersonalInfo;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Default)]
pub struct EggState3 {
    pub base: EggGeneratorState,
    pub pickup_advances: u32,
    pub redraws: u8,
}

impl EggState3 {
    pub fn new_from_pid(
        advances: u32,
        redraws: u8,
        pid: u32,
        gender: u8,
        shiny: u8,
        info: &PersonalInfo,
    ) -> Self {
        Self {
            base: EggGeneratorState::new(
                advances,
                pid,
                [0; 6],
                (pid & 1) as u8,
                gender,
                5,
                (pid % 25) as u8,
                shiny,
                [0; 6],
                info,
            ),
            redraws,
            pickup_advances: 0,
        }
    }

    pub fn new_from_low(advances: u32, low: u16, gender: u8, info: &PersonalInfo) -> Self {
        Self {
            base: EggGeneratorState::new(
                advances,
                low as u32,
                [0; 6],
                (low & 1) as u8,
                gender,
                5,
                0,
                0,
                [0; 6],
                info,
            ),
            redraws: 0,
            pickup_advances: 0,
        }
    }

    pub fn get_pickup_advances(&self) -> u32 {
        self.pickup_advances
    }

    pub fn get_redraws(&self) -> u8 {
        self.redraws
    }

    pub fn update(
        &mut self,
        advances: u32,
        ivs: [u8; 6],
        inheritance: [u8; 6],
        info: &PersonalInfo,
    ) {
        self.pickup_advances = advances;
        self.base.base.base.ivs = ivs;
        self.base.base.inheritance = inheritance;
        self.update_stats(info);
    }

    pub fn update_pid(
        &mut self,
        advances: u32,
        pid: u32,
        shiny: u8,
        ivs: [u8; 6],
        inheritance: [u8; 6],
        info: &PersonalInfo,
    ) {
        self.pickup_advances = advances;
        self.base.base.base.pid = pid;
        self.base.base.base.nature = (pid % 25) as u8;
        self.base.base.base.shiny = shiny;
        self.base.base.base.ivs = ivs;
        self.base.base.inheritance = inheritance;
        self.update_stats(info);
    }
}

impl StateT for EggState3 {
    fn update_stats(&mut self, info: &PersonalInfo) {
        self.base.update_stats(info)
    }
}
