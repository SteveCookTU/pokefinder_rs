use crate::parents::PersonalInfo;
use crate::util;

const ORDER: [usize; 6] = [0, 1, 2, 5, 3, 4];
const CHAR_ORDER: [usize; 11] = [0, 1, 2, 3, 4, 5, 0, 1, 2, 3, 4];

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Default, Debug)]
pub struct State {
    pub ec: u32,
    pub pid: u32,
    pub stats: [u16; 6],
    pub ability_index: u16,
    pub ivs: [u8; 6],
    pub ability: u8,
    pub characteristic: u8,
    pub gender: u8,
    pub hidden_power: u8,
    pub hidden_power_strength: u8,
    pub level: u8,
    pub nature: u8,
    pub shiny: u8,
}

impl State {
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

pub trait StateT {
    fn update_stats(&mut self, info: &PersonalInfo);
}

#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct GeneratorState {
    pub base: State,
    pub advances: u32,
}

impl GeneratorState {
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

#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct SearcherState {
    pub base: State,
    pub seed: u32,
}

impl SearcherState {
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
