use crate::gen5::states::{DreamRadarState, EggState5};
use crate::parents::filters::{Filter, StateFilter};
use crate::parents::states::GeneratorState;

/// Checks if encounters match the filter criteria
#[derive(Copy, Clone, Default)]
pub struct StateFilter5 {
    /// Base state filter data
    pub base: StateFilter,
}

impl StateFilter5 {
    /// Construct a new [`StateFilter5`] struct
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

    /// Determines if the egg `state` meets the filter criteria
    pub fn compare_egg_state(&self, state: &EggState5) -> bool {
        if self.base.skip {
            return true;
        }

        if (self.base.ability != 255 && self.base.ability != state.base.base.base.ability)
            || (self.base.gender != 255 && self.base.gender != state.base.base.base.gender)
            || (!self.base.powers[state.base.base.base.hidden_power as usize])
            || (!self.base.natures[state.base.base.base.nature as usize])
            || (self.base.shiny != 255 && (!(self.base.shiny & state.base.base.base.shiny)) != 0)
        {
            return false;
        }

        for (i, &iv) in state.base.base.base.ivs.iter().enumerate() {
            if iv < self.base.min[i] || iv > self.base.max[i] {
                return false;
            }
        }

        true
    }

    /// Determines if the generator `state` meets the filter criteria
    pub fn compare_dream_radar_state(&self, state: &DreamRadarState) -> bool {
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

    /// Determines if the searcher `state` meets the filter criteria
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
}

impl Filter for StateFilter5 {
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
