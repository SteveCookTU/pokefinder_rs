use crate::enums::{Game, Method};
use crate::gen5::filters::StateFilter5;
use crate::gen5::states::EggState5;
use crate::gen5::Profile5;
use crate::parents::filters::Filter;
use crate::parents::generators::EggGenerator;
use crate::parents::{personal_loader, Daycare, PersonalInfo, Profile};
use crate::rng::{MTFast, BWRNG};
use crate::util;

#[derive(Clone)]
pub struct EggGenerator5<'a, 'b, 'c> {
    pub base: EggGenerator<'a, 'b, 'c, Profile5, StateFilter5>,
    pub ditto: bool,
    pub everstone: u8,
    pub parent_ability: u8,
    pub power_item: u8,
    pub rolls: u8,
}

fn get_gender(pid: u32, info: &PersonalInfo) -> u8 {
    match info.get_gender() {
        255 => 2,
        254 => 1,
        0 => 0,
        i => (((pid & 255) as u8) < i) as u8,
    }
}

fn get_shiny(pid: u32, tsv: u16) -> u8 {
    let psv = ((pid >> 16) ^ (pid & 0xFFFF)) as u16;
    if tsv == psv {
        2
    } else if (tsv ^ psv) < 8 {
        1
    } else {
        0
    }
}

#[inline]
fn is_shiny(pid: u32, tsv: u16) -> bool {
    ((((pid >> 16) ^ (pid & 0xFFFF)) as u16) ^ tsv) < 8
}

impl<'a, 'b, 'c> EggGenerator5<'a, 'b, 'c> {
    pub fn new(
        initial_advances: u32,
        max_advances: u32,
        delay: u32,
        daycare: &'c Daycare,
        profile: &'a Profile5,
        filter: &'b StateFilter5,
    ) -> Self {
        Self {
            base: EggGenerator::new(
                initial_advances,
                max_advances,
                delay,
                Method::None,
                0,
                daycare,
                profile,
                filter,
            ),
            ditto: daycare.get_ditto(),
            everstone: daycare.get_everstone_count(),
            parent_ability: daycare.get_parent_ability(1),
            power_item: daycare.get_power_item_count(),
            rolls: if profile.get_shiny_charm() { 2 } else { 0 }
                + if daycare.get_masuda() { 5 } else { 0 },
        }
    }

    pub fn generate(&self, seed: u64) -> Vec<EggState5> {
        match self.base.base.profile.get_version() {
            Game::BLACK | Game::WHITE => self.generate_bw(seed),
            Game::BLACK2 | Game::WHITE2 => self.generate_bw2(seed),
            _ => vec![],
        }
    }

    fn generate_bw(&self, seed: u64) -> Vec<EggState5> {
        let base = personal_loader::get_personal_info(
            self.base.base.profile.get_version(),
            self.base.daycare.get_egg_species(),
            0,
        );
        let (male, female) = if matches!(self.base.daycare.get_egg_species(), 29 | 32) {
            (
                personal_loader::get_personal_info(self.base.base.profile.get_version(), 32, 0),
                personal_loader::get_personal_info(self.base.base.profile.get_version(), 29, 0),
            )
        } else if matches!(self.base.daycare.get_egg_species(), 313 | 314) {
            (
                personal_loader::get_personal_info(self.base.base.profile.get_version(), 313, 0),
                personal_loader::get_personal_info(self.base.base.profile.get_version(), 314, 0),
            )
        } else {
            (base, base)
        };

        let mut mt = MTFast::<13, 16, true>::new((seed >> 32) as u32, 7);
        let mut mt_ivs = [0; 6];
        mt_ivs.iter_mut().for_each(|iv| *iv = mt.next() as u8);

        let advances = util::utilities5::initial_advances(seed, self.base.base.profile);
        let mut rng = BWRNG::new_with_initial_advances(
            seed,
            advances
                .wrapping_add(self.base.base.initial_advances)
                .wrapping_add(self.base.base.delay),
        );

        let mut states = vec![];

        for cnt in 0..=self.base.base.max_advances {
            let mut go = rng;

            let info = if matches!(self.base.daycare.get_egg_species(), 29 | 32) {
                if go.next_u32_max(2) != 0 {
                    male
                } else {
                    female
                }
            } else if matches!(self.base.daycare.get_egg_species(), 313 | 314) {
                if go.next_u32_max(2) != 0 {
                    female
                } else {
                    male
                }
            } else {
                base
            };

            let mut nature = go.next_u32_max(25) as u8;

            if self.everstone != 0 && go.next_u32_max(2) == 1 {
                if self.everstone == 2 {
                    nature = self
                        .base
                        .daycare
                        .get_parent_nature(go.next_u32_max(2) as u8);
                } else {
                    let parent = if self.base.daycare.get_parent_item(0) == 1 {
                        0
                    } else {
                        1
                    };
                    nature = self.base.daycare.get_parent_nature(parent);
                }
            }

            let mut hidden_ability = go.next_u32_max(100) >= 40 && self.parent_ability == 2;

            if self.ditto {
                go.advance(1);
                hidden_ability = false;
            }

            let mut inheritance_count = 0;
            let mut ivs = mt_ivs;
            let mut inheritance = [0; 6];
            if self.power_item != 0 {
                inheritance_count = 1;
                if self.power_item == 2 {
                    let parent = go.next_u32_max(2) as u8;
                    let item = self.base.daycare.get_parent_item(parent) as usize;

                    ivs[item - 2] = self.base.daycare.get_parent_iv(parent, item - 2);
                    inheritance[item - 2] = parent + 1;
                } else {
                    let parent = if self.base.daycare.get_parent_item(0) >= 2
                        && self.base.daycare.get_parent_item(1) <= 7
                    {
                        0
                    } else {
                        1
                    };
                    let item = self.base.daycare.get_parent_item(parent) as usize;

                    ivs[item - 2] = self.base.daycare.get_parent_iv(parent, item - 2);
                    inheritance[item - 2] = parent + 1;
                }
            }

            while inheritance_count < 3 {
                let index = go.next_u32_max(6) as usize;
                let parent = go.next_u32_max(2) as u8;

                if inheritance[index] == 0 {
                    inheritance_count += 1;
                    ivs[index] = self.base.daycare.get_parent_iv(parent, index);
                    inheritance[index] = parent + 1;
                }
            }

            let mut pid = go.next_u32_max(0xFFFFFFFF);
            let mut i = 0;
            while i < self.rolls && !is_shiny(pid, self.base.base.tsv) {
                pid = go.next_u32_max(0xFFFFFFFF);
                i += 1;
            }

            let ability = if hidden_ability {
                2
            } else {
                ((pid >> 16) & 1) as u8
            };

            let state = EggState5::new(
                rng.next_u32_max(0x1FFFF) as u16,
                advances
                    .wrapping_add(self.base.base.initial_advances)
                    .wrapping_add(cnt),
                pid,
                ivs,
                ability,
                get_gender(pid, info),
                nature,
                get_shiny(pid, self.base.base.tsv),
                inheritance,
                info,
            );

            if self.base.base.filter.compare_egg_state(&state) {
                states.push(state);
            }
        }

        states
    }

    fn generate_bw2(&self, seed: u64) -> Vec<EggState5> {
        let mut states = vec![];

        let mut mt = MTFast::<4, 8, false>::new((seed >> 32) as u32, 2);

        let mut egg_seed = (mt.next() as u64) << 32;
        egg_seed |= mt.next() as u64;

        let (mut state, info) = self.generate_bw2_egg(egg_seed);

        if self
            .base
            .base
            .filter
            .compare_ability(state.base.base.base.ability)
            && self
                .base
                .base
                .filter
                .compare_nature(state.base.base.base.nature)
            && self.base.base.filter.compare_iv(state.base.base.base.ivs)
        {
            let advances = util::utilities5::initial_advances(seed, self.base.base.profile);
            let mut rng = BWRNG::new_with_initial_advances(
                seed,
                advances
                    .wrapping_add(self.base.base.initial_advances)
                    .wrapping_add(self.base.base.delay),
            );
            for cnt in 0..=self.base.base.max_advances {
                let mut go = rng;

                let mut pid = go.next_u32();
                if (((pid >> 16) & 1) as u8) != state.base.base.base.ability {
                    pid ^= 0x10000;
                }

                let mut i = 0;
                while i < self.rolls && !is_shiny(pid, self.base.base.tsv) {
                    pid = go.next_u32();
                    if (((pid >> 16) & 1) as u8) != state.base.base.base.ability {
                        pid ^= 0x10000;
                    }
                    i += 1;
                }

                state.update(
                    rng.next_u32_max(0x1FFF) as u16,
                    advances
                        .wrapping_add(self.base.base.initial_advances)
                        .wrapping_add(cnt),
                    pid,
                    get_gender(pid, info),
                    get_shiny(pid, self.base.base.tsv),
                );
                if self
                    .base
                    .base
                    .filter
                    .compare_gender(state.base.base.base.gender)
                    && self
                        .base
                        .base
                        .filter
                        .compare_shiny(state.base.base.base.shiny)
                {
                    states.push(state);
                }
            }
        }

        states
    }

    fn generate_bw2_egg(&self, seed: u64) -> (EggState5, &'static PersonalInfo) {
        let mut rng = BWRNG::new(seed);

        let info = if matches!(self.base.daycare.get_egg_species(), 29 | 32) {
            if rng.next_u32_max(2) != 0 {
                personal_loader::get_personal_info(self.base.base.profile.get_version(), 32, 0)
            } else {
                personal_loader::get_personal_info(self.base.base.profile.get_version(), 29, 0)
            }
        } else if matches!(self.base.daycare.get_egg_species(), 313 | 314) {
            if rng.next_u32_max(2) != 0 {
                personal_loader::get_personal_info(self.base.base.profile.get_version(), 314, 0)
            } else {
                personal_loader::get_personal_info(self.base.base.profile.get_version(), 313, 0)
            }
        } else {
            personal_loader::get_personal_info(
                self.base.base.profile.get_version(),
                self.base.daycare.get_egg_species(),
                0,
            )
        };

        let mut nature = rng.next_u32_max(25) as u8;

        if self.everstone == 2 {
            nature = self
                .base
                .daycare
                .get_parent_nature(rng.next_u32_max(2) as u8);
        } else if self.everstone == 1 {
            let parent = if self.base.daycare.get_parent_item(0) == 1 {
                0
            } else {
                1
            };
            nature = self.base.daycare.get_parent_nature(parent);
        }

        let ability = if !self.ditto {
            let ability_rand = rng.next_u32_max(100);
            match self.parent_ability {
                0 => {
                    if ability_rand < 80 {
                        0
                    } else {
                        1
                    }
                }
                1 => {
                    if ability_rand < 20 {
                        0
                    } else {
                        1
                    }
                }
                _ => {
                    if ability_rand < 20 {
                        0
                    } else if ability_rand < 40 {
                        1
                    } else {
                        2
                    }
                }
            }
        } else {
            rng.advance(1);
            rng.next_u32_max(2) as u8
        };

        let mut inheritance_count = 0;
        let mut ivs = [0; 6];
        let mut inheritance = [0; 6];
        if self.power_item != 0 {
            inheritance_count = 1;
            if self.power_item == 2 {
                let parent = rng.next_u32_max(2) as u8;
                let item = self.base.daycare.get_parent_item(parent) as usize;

                ivs[item - 2] = self.base.daycare.get_parent_iv(parent, item - 2);
                inheritance[item - 2] = parent + 1;
            } else {
                let parent = if self.base.daycare.get_parent_item(0) >= 2
                    && self.base.daycare.get_parent_item(1) <= 7
                {
                    0
                } else {
                    1
                };
                let item = self.base.daycare.get_parent_item(parent) as usize;

                ivs[item - 2] = self.base.daycare.get_parent_iv(parent, item - 2);
                inheritance[item - 2] = parent + 1;
            }
        }

        while inheritance_count < 3 {
            let index = rng.next_u32_max(6) as usize;
            let parent = rng.next_u32_max(2) as u8;

            if inheritance[index] == 0 {
                inheritance_count += 1;
                ivs[index] = self.base.daycare.get_parent_iv(parent, index);
                inheritance[index] = parent + 1;
            }
        }

        for i in 0..6 {
            if inheritance[i] == 0 {
                ivs[i] = rng.next_u32_max(32) as u8;
            }
        }

        (
            EggState5::new_ivs(ivs, ability, nature, inheritance, info),
            info,
        )
    }
}
