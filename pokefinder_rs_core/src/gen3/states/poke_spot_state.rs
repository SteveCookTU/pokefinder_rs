use crate::parents::states::{GeneratorState, StateT};
use crate::parents::PersonalInfo;

#[derive(Copy, Clone)]
pub struct PokeSpotState {
    pub base: GeneratorState,
    pub specie: u16,
    pub encounter_slot: u8,
    pub encounter_advances: u32,
}

impl PokeSpotState {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        advances: u32,
        pid: u32,
        gender: u8,
        shiny: u8,
        encounter_slot: u8,
        specie: u16,
        info: &PersonalInfo,
    ) -> Self {
        Self {
            base: GeneratorState::new(
                advances,
                pid,
                [0; 6],
                0,
                gender,
                0,
                (pid % 25) as u8,
                shiny,
                info,
            ),
            specie,
            encounter_slot,
            encounter_advances: 0,
        }
    }

    pub fn get_encounter_slot(&self) -> u8 {
        self.encounter_slot
    }

    pub fn get_encounter_advances(&self) -> u32 {
        self.encounter_advances
    }

    pub fn species(&self) -> u16 {
        self.specie
    }

    pub fn update(
        &mut self,
        advances: u32,
        ability: u8,
        level: u8,
        ivs: [u8; 6],
        info: &PersonalInfo,
    ) {
        self.encounter_advances = advances;
        self.base.base.ability = ability;
        self.base.base.ability_index = info.get_ability(ability as usize);
        self.base.base.level = level;
        self.base.base.ivs = ivs;
        self.update_stats(info);
    }
}

impl StateT for PokeSpotState {
    fn update_stats(&mut self, info: &PersonalInfo) {
        self.base.update_stats(info);
    }
}
