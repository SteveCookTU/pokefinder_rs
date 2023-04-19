use crate::gen3::states::EggState3;
use crate::parents::filters::{Filter, StateFilter, WildFilter};
use crate::parents::states::{
    GeneratorState, SearcherState, WildGeneratorState, WildSearcherState,
};

#[derive(Copy, Clone)]
pub struct StateFilter3 {
    pub base: StateFilter,
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
            base: StateFilter {
                natures,
                powers,
                max,
                min,
                skip,
                ability,
                gender,
                shiny,
            },
        }
    }

    pub fn compare_egg_state(&self, state: &EggState3) -> bool {
        if self.base.skip {
            return true;
        }

        if !self.base.powers[state.base.base.base.hidden_power as usize]
            || !self.base.natures[state.base.base.base.nature as usize]
            || (self.base.shiny != 255 && self.base.shiny & state.base.base.base.nature == 0)
        {
            return false;
        }

        for i in 0..6 {
            let iv = state.base.base.base.ivs[i];
            if iv < self.base.min[i] || iv > self.base.max[i] {
                return false;
            }
        }

        true
    }

    pub fn compare_generator_state(&self, state: &GeneratorState) -> bool {
        if self.base.skip {
            return true;
        }

        if (self.base.ability != 255 && self.base.ability != state.base.ability)
            || (self.base.gender != 255 && self.base.gender != state.base.gender)
            || (!self.base.powers[state.base.hidden_power as usize])
            || (!self.base.natures[state.base.nature as usize])
            || (self.base.shiny != 255 && (!(self.base.shiny & state.base.shiny)) != 0)
        {
            return false;
        }

        for (i, &iv) in state.base.ivs.iter().enumerate() {
            if iv < self.base.min[i] || iv > self.base.max[i] {
                return false;
            }
        }

        true
    }

    pub fn compare_searcher_state(&self, state: &SearcherState) -> bool {
        !((self.base.ability != 255 && self.base.ability != state.base.ability)
            || (self.base.gender != 255 && self.base.gender != state.base.gender)
            || (!self.base.powers[state.base.hidden_power as usize])
            || (self.base.shiny != 255 && (!(self.base.shiny & state.base.shiny)) != 0))
    }
}

impl Filter for StateFilter3 {
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

impl Filter for WildStateFilter3 {
    #[inline]
    fn get_natures(&self) -> [bool; 25] {
        self.natures
    }
    #[inline]
    fn get_powers(&self) -> [bool; 16] {
        self.powers
    }
    #[inline]
    fn get_max(&self) -> [u8; 6] {
        self.max
    }
    #[inline]
    fn get_min(&self) -> [u8; 6] {
        self.min
    }
    #[inline]
    fn skip(&self) -> bool {
        self.skip
    }
    #[inline]
    fn get_ability(&self) -> u8 {
        self.ability
    }
    #[inline]
    fn get_gender(&self) -> u8 {
        self.gender
    }
    #[inline]
    fn get_shiny(&self) -> u8 {
        self.shiny
    }
}

impl WildFilter for WildStateFilter3 {
    #[inline]
    fn get_encounter_slots(&self) -> [bool; 12] {
        self.encounter_slots
    }
}
