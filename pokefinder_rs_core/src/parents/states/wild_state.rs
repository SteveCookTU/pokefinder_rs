use crate::parents::states::{State, StateT};
use crate::parents::PersonalInfo;

#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct WildState {
    pub base: State,
    pub item: u16,
    pub specie: u16,
    pub encounter_slot: u8,
    pub form: u8,
}

impl WildState {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        pid: u32,
        ivs: [u8; 6],
        ability: u8,
        gender: u8,
        level: u8,
        nature: u8,
        shiny: u8,
        encounter_slot: u8,
        item: u16,
        specie: u16,
        form: u8,
        info: &PersonalInfo,
    ) -> Self {
        Self {
            base: State::new(pid, ivs, ability, gender, level, nature, shiny, info),
            item,
            specie,
            encounter_slot,
            form,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_with_ec(
        ec: u32,
        pid: u32,
        ivs: [u8; 6],
        ability: u8,
        gender: u8,
        level: u8,
        nature: u8,
        shiny: u8,
        encounter_slot: u8,
        item: u16,
        specie: u16,
        form: u8,
        info: &PersonalInfo,
    ) -> Self {
        Self {
            base: State::new_with_ec(ec, pid, ivs, ability, gender, level, nature, shiny, info),
            item,
            specie,
            encounter_slot,
            form,
        }
    }
}

impl StateT for WildState {
    fn update_stats(&mut self, info: &PersonalInfo) {
        self.base.update_stats(info)
    }
}

#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct WildGeneratorState {
    pub base: WildState,
    pub advances: u32,
}

impl WildGeneratorState {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
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
        specie: u16,
        form: u8,
        info: &PersonalInfo,
    ) -> Self {
        Self {
            base: WildState::new(
                pid,
                ivs,
                ability,
                gender,
                level,
                nature,
                shiny,
                encounter_slot,
                item,
                specie,
                form,
                info,
            ),
            advances,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_with_ec(
        advances: u32,
        ec: u32,
        pid: u32,
        ivs: [u8; 6],
        ability: u8,
        gender: u8,
        level: u8,
        nature: u8,
        shiny: u8,
        encounter_slot: u8,
        item: u16,
        specie: u16,
        form: u8,
        info: &PersonalInfo,
    ) -> Self {
        Self {
            base: WildState::new_with_ec(
                ec,
                pid,
                ivs,
                ability,
                gender,
                level,
                nature,
                shiny,
                encounter_slot,
                item,
                specie,
                form,
                info,
            ),
            advances,
        }
    }
}

impl StateT for WildGeneratorState {
    fn update_stats(&mut self, info: &PersonalInfo) {
        self.base.update_stats(info);
    }
}

#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct WildSearcherState {
    pub base: WildState,
    pub seed: u32,
}

impl WildSearcherState {
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
        specie: u16,
        form: u8,
        info: &PersonalInfo,
    ) -> Self {
        Self {
            base: WildState::new(
                pid,
                ivs,
                ability,
                gender,
                level,
                nature,
                shiny,
                encounter_slot,
                item,
                specie,
                form,
                info,
            ),
            seed,
        }
    }
}

impl StateT for WildSearcherState {
    fn update_stats(&mut self, info: &PersonalInfo) {
        self.base.update_stats(info);
    }
}
