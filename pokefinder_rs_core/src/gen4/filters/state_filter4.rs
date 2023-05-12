use crate::gen4::states::{
    EggGeneratorState4, GeneratorState4, SearcherState4, WildGeneratorState4, WildSearcherState4,
};
use crate::parents::filters::{Filter, StateFilter, WildFilter, WildStateFilter};

#[derive(Copy, Clone, Default)]
pub struct StateFilter4 {
    pub base: StateFilter,
}

impl StateFilter4 {
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
            base: StateFilter::new(gender, ability, shiny, skip, min, max, natures, powers),
        }
    }

    pub fn compare_egg_state(&self, state: &EggGeneratorState4) -> bool {
        if self.base.skip {
            return true;
        }

        if !self.base.powers[state.base.base.base.hidden_power as usize] {
            return false;
        }

        for (i, &iv) in state.base.base.base.ivs.iter().enumerate() {
            if iv < self.base.min[i] || iv > self.base.max[i] {
                return false;
            }
        }

        true
    }

    pub fn compare_generator_state(&self, state: &GeneratorState4) -> bool {
        if self.base.skip {
            return true;
        }

        if (self.base.ability != 255 && self.base.ability != state.base.base.ability)
            || (self.base.gender != 255 && self.base.gender != state.base.base.gender)
            || (!self.base.powers[state.base.base.hidden_power as usize])
            || (!self.base.natures[state.base.base.nature as usize])
            || (self.base.shiny != 255 && (!(self.base.shiny & state.base.base.shiny)) != 0)
        {
            return false;
        }

        for (i, &iv) in state.base.base.ivs.iter().enumerate() {
            if iv < self.base.min[i] || iv > self.base.max[i] {
                return false;
            }
        }

        true
    }

    pub fn compare_searcher_state(&self, state: &SearcherState4) -> bool {
        !((self.base.ability != 255 && self.base.ability != state.base.base.ability)
            || (self.base.gender != 255 && self.base.gender != state.base.base.gender)
            || (!self.base.powers[state.base.base.hidden_power as usize])
            || (self.base.shiny != 255 && (!(self.base.shiny & state.base.base.shiny)) != 0))
    }
}

impl Filter for StateFilter4 {
    fn get_natures(&self) -> [bool; 25] {
        self.base.natures
    }

    fn get_powers(&self) -> [bool; 16] {
        self.base.powers
    }

    fn get_max(&self) -> [u8; 6] {
        self.base.max
    }

    fn get_min(&self) -> [u8; 6] {
        self.base.min
    }

    fn skip(&self) -> bool {
        self.base.skip
    }

    fn get_ability(&self) -> u8 {
        self.base.ability
    }

    fn get_gender(&self) -> u8 {
        self.base.gender
    }

    fn get_shiny(&self) -> u8 {
        self.base.shiny
    }
}

#[derive(Copy, Clone, Default)]
pub struct WildStateFilter4 {
    pub base: WildStateFilter,
}

impl WildStateFilter4 {
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
            base: WildStateFilter::new(
                gender,
                ability,
                shiny,
                skip,
                min,
                max,
                natures,
                powers,
                encounter_slots,
            ),
        }
    }

    pub fn compare_generator_state(&self, state: &WildGeneratorState4) -> bool {
        if self.base.base.skip {
            return true;
        }

        if (self.base.base.ability != 255 && self.base.base.ability != state.base.base.base.ability)
            || (self.base.base.gender != 255
                && self.base.base.gender != state.base.base.base.ability)
            || (!self.base.base.powers[state.base.base.base.hidden_power as usize])
            || (self.base.base.shiny != 255
                && (!(self.base.base.shiny & state.base.base.base.shiny)) != 0)
        {
            return false;
        }

        for (i, &iv) in state.base.base.base.ivs.iter().enumerate() {
            if iv < self.base.base.min[i] || iv > self.base.base.max[i] {
                return false;
            }
        }

        true
    }

    pub fn compare_searcher_state(&self, state: &WildSearcherState4) -> bool {
        !((self.base.base.ability != 255 && self.base.base.ability != state.base.base.base.ability)
            || (self.base.base.gender != 255
                && self.base.base.gender != state.base.base.base.gender)
            || (!self.base.base.powers[state.base.base.base.hidden_power as usize])
            || (self.base.base.shiny != 255
                && (!(self.base.base.shiny & state.base.base.base.shiny)) != 0))
    }
}

impl Filter for WildStateFilter4 {
    fn get_natures(&self) -> [bool; 25] {
        self.base.get_natures()
    }

    fn get_powers(&self) -> [bool; 16] {
        self.base.get_powers()
    }

    fn get_max(&self) -> [u8; 6] {
        self.base.get_max()
    }

    fn get_min(&self) -> [u8; 6] {
        self.base.get_min()
    }

    fn skip(&self) -> bool {
        self.base.skip()
    }

    fn get_ability(&self) -> u8 {
        self.base.get_ability()
    }

    fn get_gender(&self) -> u8 {
        self.base.get_gender()
    }

    fn get_shiny(&self) -> u8 {
        self.base.get_shiny()
    }
}

impl WildFilter for WildStateFilter4 {
    fn get_encounter_slots(&self) -> [bool; 12] {
        self.base.encounter_slots
    }
}
