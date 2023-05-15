use crate::parents::states::{State, StateT};
use crate::parents::PersonalInfo;

/// State struct that provides additional egg information
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Default)]
pub struct EggState {
    /// Base pokemon information
    pub base: State,
    /// Pokemon IV inheritance
    pub inheritance: [u8; 6],
}

impl EggState {
    /// Construct a new [`EggState`] struct
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        pid: u32,
        ivs: [u8; 6],
        ability: u8,
        gender: u8,
        level: u8,
        nature: u8,
        shiny: u8,
        inheritance: [u8; 6],
        info: &PersonalInfo,
    ) -> Self {
        Self {
            base: State::new(pid, ivs, ability, gender, level, nature, shiny, info),
            inheritance,
        }
    }

    /// Construct a new [`EggState`] struct with an EC different than the PID
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
        inheritance: [u8; 6],
        info: &PersonalInfo,
    ) -> Self {
        Self {
            base: State::new_with_ec(ec, pid, ivs, ability, gender, level, nature, shiny, info),
            inheritance,
        }
    }
}

impl StateT for EggState {
    fn update_stats(&mut self, info: &PersonalInfo) {
        self.base.update_stats(info);
    }
}

/// State struct that provides additional information from an egg generator
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct EggGeneratorState {
    /// Base egg pokemon information
    pub base: EggState,
    /// Advances of the state
    pub advances: u32,
}

impl EggGeneratorState {
    /// Construct a new [`EggGeneratorState`] struct
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
        inheritance: [u8; 6],
        info: &PersonalInfo,
    ) -> Self {
        Self {
            base: EggState::new(
                pid,
                ivs,
                ability,
                gender,
                level,
                nature,
                shiny,
                inheritance,
                info,
            ),
            advances,
        }
    }

    /// Construct a new [`EggGeneratorState`] struct with an EC different than the PID
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
        inheritance: [u8; 6],
        info: &PersonalInfo,
    ) -> Self {
        Self {
            base: EggState::new_with_ec(
                ec,
                pid,
                ivs,
                ability,
                gender,
                level,
                nature,
                shiny,
                inheritance,
                info,
            ),
            advances,
        }
    }
}

impl StateT for EggGeneratorState {
    fn update_stats(&mut self, info: &PersonalInfo) {
        self.base.update_stats(info)
    }
}

/// State struct that provides additional information from an egg searcher
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct EggSearcherState {
    /// Base egg pokemon information
    pub base: EggState,
    /// Seed of the state
    pub seed: u32,
}

impl EggSearcherState {
    /// Construct a new [`EggSearcherState`] struct
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
        inheritance: [u8; 6],
        info: &PersonalInfo,
    ) -> Self {
        Self {
            base: EggState::new(
                pid,
                ivs,
                ability,
                gender,
                level,
                nature,
                shiny,
                inheritance,
                info,
            ),
            seed,
        }
    }
}

impl StateT for EggSearcherState {
    fn update_stats(&mut self, info: &PersonalInfo) {
        self.base.update_stats(info)
    }
}
