use crate::gen8::states::UndergroundState;
use crate::parents::filters::{Filter, StateFilter, WildFilter, WildStateFilter};
use crate::parents::states::{EggGeneratorState, GeneratorState, WildGeneratorState};

/// Checks if static encounters match the filter criteria
#[derive(Copy, Clone, Default)]
pub struct StateFilter8 {
    /// Base state filter data
    pub base: StateFilter,
}

impl StateFilter8 {
    /// Contruct a new [`StateFilter8`] struct
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
    pub fn compare_egg_state(&self, state: &EggGeneratorState) -> bool {
        if self.base.skip {
            return true;
        }

        if (self.base.ability != 255 && self.base.ability != state.base.base.ability)
            || (self.base.gender != 255 && self.base.gender != state.base.base.gender)
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

    /// Determines if the generator `state` meets the filter criteria
    pub fn compare_generator_state(&self, state: &GeneratorState) -> bool {
        if self.base.skip {
            return true;
        }

        if (self.base.ability != 255 && self.base.ability != state.base.ability)
            || (self.base.gender != 255 && self.base.gender != state.base.gender)
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

impl Filter for StateFilter8 {
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

/// Checks if the wild encounters match the filter criteria
#[derive(Copy, Clone, Default)]
pub struct WildStateFilter8 {
    /// Base wild state filter data
    pub base: WildStateFilter,
}

impl WildStateFilter8 {
    /// Construct a new [`WildStateFilter8`] struct
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

    /// Determines if the wild `state` meets the filter criteria
    pub fn compare_generator_state(&self, state: &WildGeneratorState) -> bool {
        if self.base.base.skip {
            return true;
        }

        if (self.base.base.ability != 255 && self.base.base.ability != state.base.base.ability)
            || (self.base.base.gender != 255 && self.base.base.gender != state.base.base.ability)
            || (!self.base.base.powers[state.base.base.hidden_power as usize])
            || (!self.base.base.natures[state.base.base.nature as usize])
            || (self.base.base.shiny != 255
                && (!(self.base.base.shiny & state.base.base.shiny)) != 0)
        {
            return false;
        }

        for (i, &iv) in state.base.base.ivs.iter().enumerate() {
            if iv < self.base.base.min[i] || iv > self.base.base.max[i] {
                return false;
            }
        }

        true
    }
}

impl Filter for WildStateFilter8 {
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

impl WildFilter for WildStateFilter8 {
    fn get_encounter_slots(&self) -> [bool; 12] {
        self.base.encounter_slots
    }
}

/// Checks if the underground encounters match the filter criteria
#[derive(Copy, Clone, Default)]
pub struct UndergroundStateFilter<'a> {
    /// Base state filter data
    pub base: StateFilter,
    /// Pokemon species to filter by. This list **must be sorted**.
    pub species: &'a [u16],
}

impl<'a> UndergroundStateFilter<'a> {
    /// Construct a new [`UndergroundStateFilter`] struct
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
        species: &'a [u16],
    ) -> Self {
        Self {
            base: StateFilter::new(gender, ability, shiny, skip, min, max, natures, powers),
            species,
        }
    }

    /// Determines if the underground `state` meets the filter criteria
    pub fn compare_underground_state(&self, state: &UndergroundState) -> bool {
        if self.base.skip {
            return true;
        }

        if (self.base.ability != 255 && self.base.ability != state.base.base.ability)
            || (self.base.gender != 255 && self.base.gender != state.base.base.gender)
            || (!self.base.natures[state.base.base.nature as usize])
            || self.species.binary_search(&state.species).is_err()
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
}
impl Filter for UndergroundStateFilter<'_> {
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
