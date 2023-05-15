use crate::parents::PersonalInfo;
use crate::util;

const ORDER: [usize; 6] = [0, 1, 2, 5, 3, 4];
const CHAR_ORDER: [usize; 11] = [0, 1, 2, 3, 4, 5, 0, 1, 2, 3, 4];

/// Contains all the common information for a Pokemon across each game
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Default, Debug)]
pub struct State {
    /// Pokemon encryption constant
    pub ec: u32,
    /// Pokemon PID
    pub pid: u32,
    /// Pokemon stats
    pub stats: [u16; 6],
    /// Pokemon ability index
    pub ability_index: u16,
    /// Pokemon IVs
    pub ivs: [u8; 6],
    /// Pokemon ability
    pub ability: u8,
    /// Pokemon characteristic
    pub characteristic: u8,
    /// Pokemon gender
    pub gender: u8,
    /// Pokemon hidden power
    pub hidden_power: u8,
    /// Pokemon hidden power strength
    pub hidden_power_strength: u8,
    /// Pokemon level
    pub level: u8,
    /// Pokemon nature
    pub nature: u8,
    /// Pokemon shininess
    pub shiny: u8,
}

impl State {
    /// Construct a new [`State`] struct
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        pid: u32,
        ivs: [u8; 6],
        ability: u8,
        gender: u8,
        level: u8,
        nature: u8,
        shiny: u8,
        info: &PersonalInfo,
    ) -> Self {
        let mut new = Self {
            ec: pid,
            pid,
            ability_index: info.get_ability(ability as usize),
            ivs,
            ability,
            gender,
            level,
            nature,
            shiny,
            ..Default::default()
        };
        new.update_stats(info);
        new
    }

    /// Construct a new [`State`] struct with an EC different than the PID
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
        info: &PersonalInfo,
    ) -> Self {
        let mut new = Self {
            ec,
            pid,
            ability_index: info.get_ability(ability as usize),
            ivs,
            ability,
            gender,
            level,
            nature,
            shiny,
            ..Default::default()
        };
        new.update_stats(info);
        new
    }
}

impl StateT for State {
    fn update_stats(&mut self, info: &PersonalInfo) {
        let mut h = 0u8;
        let mut p = 0u8;
        let ec_index = (self.ec % 6) as usize;
        let mut char_index = ec_index;
        let mut max_iv = 0;
        let ivs = self.ivs;
        for i in 0..6 {
            h += (ivs[ORDER[i]] & 1) << i;
            p += ((ivs[ORDER[i]] >> 1) & 1) << i;

            let stat = ((2u32
                .wrapping_mul(info.get_stat(i) as u32)
                .wrapping_add(ivs[i] as u32))
            .wrapping_mul(self.level as u32)
                / 100) as u16;
            if i == 0 {
                self.stats[i] = stat + self.level as u16 + 10;
            } else {
                self.stats[i] = util::nature::compute_stat(stat + 5, self.nature, i)
            }
            let index = CHAR_ORDER[ec_index + i];
            if ivs[ORDER[index]] > max_iv {
                char_index = index;
                max_iv = ivs[ORDER[index]];
            }
        }
        self.hidden_power = ((h as u16).wrapping_mul(15) / 63) as u8;
        self.hidden_power_strength = 30u16.wrapping_add((p as u16).wrapping_mul(40) / 63) as u8;
        self.characteristic = (char_index as u8).wrapping_mul(5).wrapping_add(max_iv % 5);
    }
}

/// Trait contains common functions for State structs
pub trait StateT {
    /// Updates extra information such as stats, hidden power, hidden power strength,
    /// and characteristic based on provided pokemon information
    fn update_stats(&mut self, info: &PersonalInfo);
}

/// Struct that provides additional information from a generator
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct GeneratorState {
    /// Base generator pokemon information
    pub base: State,
    /// Advances of the state
    pub advances: u32,
}

impl GeneratorState {
    /// Construct a new [`GeneratorState`] struct
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
        info: &PersonalInfo,
    ) -> Self {
        Self {
            base: State::new(pid, ivs, ability, gender, level, nature, shiny, info),
            advances,
        }
    }

    /// Construct a new [`GeneratorState`] struct with an EC different than the PID
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
        info: &PersonalInfo,
    ) -> Self {
        Self {
            base: State::new_with_ec(ec, pid, ivs, ability, gender, level, nature, shiny, info),
            advances,
        }
    }
}

impl StateT for GeneratorState {
    fn update_stats(&mut self, info: &PersonalInfo) {
        self.base.update_stats(info)
    }
}

/// Struct that provides additional information from a searcher
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct SearcherState {
    /// Base searcher pokemon information
    pub base: State,
    /// Seed of the state
    pub seed: u32,
}

impl SearcherState {
    /// Construct a new [`SearcherState`] struct
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
        info: &PersonalInfo,
    ) -> Self {
        Self {
            base: State::new(pid, ivs, ability, gender, level, nature, shiny, info),
            seed,
        }
    }
}

impl StateT for SearcherState {
    fn update_stats(&mut self, info: &PersonalInfo) {
        self.base.update_stats(info);
    }
}
