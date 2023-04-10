use crate::gen3::states::EggState3;
use crate::parents::filters::{StateFilter, WildStateFilter};
use crate::parents::states::{
    GeneratorState, SearcherState, WildGeneratorState, WildSearcherState,
};

#[derive(Copy, Clone)]
pub struct StateFilter3 {
    gender: u8,
    ability: u8,
    shiny: u8,
    skip: bool,
    min: [u8; 6],
    max: [u8; 6],
    natures: [bool; 25],
    powers: [bool; 16],
}

impl StateFilter3 {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        gender: u8,
        ability: u8,
        shiny: u8,
        skip: bool,
        min: [u8; 6],
        max: [u8; 6],
        natures: [bool; 25],
        powers: [bool; 16],
    ) -> Self {
        Self {
            gender,
            ability,
            shiny,
            skip,
            min,
            max,
            natures,
            powers,
        }
    }

    pub fn compare_egg_state(&self, state: &EggState3) -> bool {
        if self.skip {
            return true;
        }

        if !self.powers[state.base.base.base.hidden_power as usize]
            || !self.natures[state.base.base.base.nature as usize]
            || (self.shiny != 255 && self.shiny & state.base.base.base.nature == 0)
        {
            return false;
        }

        for i in 0..6 {
            let iv = state.base.base.base.ivs[i];
            if iv < self.min[i] || iv > self.max[i] {
                return false;
            }
        }

        true
    }

    pub fn compare_generator_state(&self, state: &GeneratorState) -> bool {
        if self.skip {
            return true;
        }

        if (self.ability != 255 && self.ability != state.base.ability)
            || (self.gender != 255 && self.gender != state.base.gender)
            || (!self.powers[state.base.hidden_power as usize])
            || (!self.natures[state.base.nature as usize])
            || (self.shiny != 255 && (!(self.shiny & state.base.shiny)) != 0)
        {
            return false;
        }

        for (i, &iv) in state.base.ivs.iter().enumerate() {
            if iv < self.min[i] || iv > self.max[i] {
                return false;
            }
        }

        true
    }

    pub fn compare_searcher_state(&self, state: &SearcherState) -> bool {
        !((self.ability != 255 && self.ability != state.base.ability)
            || (self.gender != 255 && self.gender != state.base.gender)
            || (!self.powers[state.base.hidden_power as usize])
            || (self.shiny != 255 && (!(self.shiny & state.base.shiny)) != 0))
    }
}

impl StateFilter for StateFilter3 {
    fn get_natures(&self) -> [bool; 25] {
        self.natures
    }

    fn get_powers(&self) -> [bool; 16] {
        self.powers
    }

    fn get_max(&self) -> [u8; 6] {
        self.max
    }

    fn get_min(&self) -> [u8; 6] {
        self.min
    }

    fn skip(&self) -> bool {
        self.skip
    }

    fn get_ability(&self) -> u8 {
        self.ability
    }

    fn get_gender(&self) -> u8 {
        self.gender
    }

    fn get_shiny(&self) -> u8 {
        self.shiny
    }
}

#[derive(Copy, Clone)]
pub struct WildStateFilter3 {
    gender: u8,
    ability: u8,
    shiny: u8,
    skip: bool,
    min: [u8; 6],
    max: [u8; 6],
    natures: [bool; 25],
    powers: [bool; 16],
    encounter_slots: [bool; 12],
}

impl WildStateFilter3 {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        gender: u8,
        ability: u8,
        shiny: u8,
        skip: bool,
        min: [u8; 6],
        max: [u8; 6],
        natures: [bool; 25],
        powers: [bool; 16],
        encounter_slots: [bool; 12],
    ) -> Self {
        Self {
            gender,
            ability,
            shiny,
            skip,
            min,
            max,
            natures,
            powers,
            encounter_slots,
        }
    }

    pub fn compare_generator_state(&self, state: &WildGeneratorState) -> bool {
        if self.skip {
            return true;
        }

        if (self.ability != 255 && self.ability != state.base.base.ability)
            || (self.gender != 255 && self.gender != state.base.base.ability)
            || (!self.powers[state.base.base.shiny as usize])
            || (self.shiny != 255 && (!(self.shiny & state.base.base.shiny)) != 0)
        {
            return false;
        }

        for (i, &iv) in state.base.base.ivs.iter().enumerate() {
            if iv < self.min[i] || iv > self.max[i] {
                return false;
            }
        }

        true
    }

    pub fn compare_searcher_state(&self, state: &WildSearcherState) -> bool {
        !((self.ability != 255 && self.ability != state.base.base.ability)
            || (self.gender != 255 && self.gender != state.base.base.gender)
            || (!self.powers[state.base.base.hidden_power as usize])
            || (self.shiny != 255 && (!(self.shiny & state.base.base.shiny)) != 0))
    }
}

impl StateFilter for WildStateFilter3 {
    fn get_natures(&self) -> [bool; 25] {
        self.natures
    }

    fn get_powers(&self) -> [bool; 16] {
        self.powers
    }

    fn get_max(&self) -> [u8; 6] {
        self.max
    }

    fn get_min(&self) -> [u8; 6] {
        self.min
    }

    fn skip(&self) -> bool {
        self.skip
    }

    fn get_ability(&self) -> u8 {
        self.ability
    }

    fn get_gender(&self) -> u8 {
        self.gender
    }

    fn get_shiny(&self) -> u8 {
        self.shiny
    }
}

impl WildStateFilter for WildStateFilter3 {
    fn get_encounter_slots(&self) -> [bool; 12] {
        self.encounter_slots
    }
}
