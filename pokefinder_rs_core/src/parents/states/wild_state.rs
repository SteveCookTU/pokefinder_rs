use crate::parents::states::{State, StateT};
use crate::parents::PersonalInfo;

/// State struct that provides additional wild information
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct WildState {
    /// Base pokemon information
    pub base: State,
    /// Pokemon item
    pub item: u16,
    /// Pokemon species
    pub species: u16,
    /// Pokemon encounter slot
    pub encounter_slot: u8,
    /// Pokemon form
    pub form: u8,
}

impl WildState {
    /// Constructs a new [`WildState`] struct
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
        species: u16,
        form: u8,
        info: &PersonalInfo,
    ) -> Self {
        Self {
            base: State::new(pid, ivs, ability, gender, level, nature, shiny, info),
            item,
            species,
            encounter_slot,
            form,
        }
    }

    /// Constructs a new [`WildState`] struct with an EC different than the PID
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
        species: u16,
        form: u8,
        info: &PersonalInfo,
    ) -> Self {
        Self {
            base: State::new_with_ec(ec, pid, ivs, ability, gender, level, nature, shiny, info),
            item,
            species,
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

/// State struct that provides additional information from a wild generator
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct WildGeneratorState {
    /// Base wild pokemon information
    pub base: WildState,
    /// Advances of the state
    pub advances: u32,
}

impl WildGeneratorState {
    /// Construct a new [`WildGeneratorState`] struct
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
        species: u16,
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
                species,
                form,
                info,
            ),
            advances,
        }
    }

    /// Construct a new [`WildGeneratorState`] struct with an EC different than the PID
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
        species: u16,
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
                species,
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

/// State struct that provides additional information from a wild searcher
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct WildSearcherState {
    /// Base wild pokemon information
    pub base: WildState,
    /// Seed of the state
    pub seed: u32,
}

impl WildSearcherState {
    /// Construct a new [`WildSearcherState`] struct
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
        species: u16,
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
                species,
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
