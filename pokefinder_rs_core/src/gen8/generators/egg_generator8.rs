use crate::enums::Method;
use crate::gen8::filters::StateFilter8;
use crate::gen8::Profile8;
use crate::parents::generators::EggGenerator;
use crate::parents::states::EggGeneratorState;
use crate::parents::{personal_loader, Daycare, Profile};
use crate::rng::{RNGList, XoroshiroBDSP, Xorshift};

/// Egg generator for Gen8
#[derive(Clone)]
pub struct EggGenerator8<'a, 'b, 'c> {
    /// Base egg generator data
    pub base: EggGenerator<'a, 'b, 'c, Profile8, StateFilter8>,
    /// If the profile has the shiny charm unlocked
    pub shiny_charm: bool,
}

fn gen(rng: &mut Xorshift) -> u32 {
    rng.next_range(0x80000000, 0x7fffffff)
}

impl<'a, 'b, 'c> EggGenerator8<'a, 'b, 'c> {
    /// Construct a new [`EggGenerator8`] struct
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        initial_advances: u32,
        max_advances: u32,
        delay: u32,
        compatability: u8,
        daycare: &'c Daycare,
        profile: &'a Profile8,
        filter: &'b StateFilter8,
    ) -> Self {
        let mut new = Self {
            base: EggGenerator::new(
                initial_advances,
                max_advances,
                delay,
                Method::None,
                compatability,
                daycare,
                profile,
                filter,
            ),
            shiny_charm: profile.get_shiny_charm(),
        };
        new.base.base.tsv = (profile.get_tid() & 0xFFF0) ^ profile.get_sid();
        new
    }

    /// Generate [`EggGeneratorState`] states
    pub fn generate(&self, seed0: u64, seed1: u64) -> Vec<EggGeneratorState> {
        let base = personal_loader::get_personal_info(
            self.base.base.profile.get_version(),
            self.base.daycare.get_egg_species(),
            0,
        );
        let (male, female) = match self.base.daycare.get_egg_species() {
            29 | 32 => (
                Some(personal_loader::get_personal_info(
                    self.base.base.profile.get_version(),
                    32,
                    0,
                )),
                Some(personal_loader::get_personal_info(
                    self.base.base.profile.get_version(),
                    29,
                    0,
                )),
            ),
            313 | 314 => (
                Some(personal_loader::get_personal_info(
                    self.base.base.profile.get_version(),
                    313,
                    0,
                )),
                Some(personal_loader::get_personal_info(
                    self.base.base.profile.get_version(),
                    314,
                    0,
                )),
            ),
            _ => (None, None),
        };

        let rng = Xorshift::new_with_initial_advances(
            seed0,
            seed1,
            self.base
                .base
                .initial_advances
                .wrapping_add(self.base.base.delay),
        );
        let mut rng_list = RNGList::<u32, Xorshift, 2>::new(rng, Some(gen));

        let mut pid_rolls = 0;
        if self.base.daycare.get_masuda() {
            pid_rolls += 6;
        }
        if self.shiny_charm {
            pid_rolls += 2;
        }

        let inheritance_count = if self.base.daycare.get_parent_item(0) == 8
            || self.base.daycare.get_parent_item(1) == 8
        {
            5
        } else {
            3
        };

        let mut states = vec![];
        let mut cnt = 0;
        while cnt <= self.base.base.max_advances {
            if ((rng_list.next() % 100) as u8) < self.base.compatability {
                let mut seed = rng_list.next() as u64;
                if (seed & 0x80000000) != 0 {
                    seed |= 0xffffffff00000000;
                }

                let mut rng = XoroshiroBDSP::new(seed);
                let mut info = base;
                let gender = if matches!(self.base.daycare.get_egg_species(), 29 | 32 | 313 | 314) {
                    let gender = rng.next_u32(2) as u8;
                    if gender != 0 {
                        info = female.unwrap();
                    } else {
                        info = male.unwrap();
                    }
                    gender
                } else {
                    match base.get_gender() {
                        255 => 2,
                        254 => 1,
                        0 => 0,
                        _ => u8::from((rng.next_u32(252) as u8 + 1) < base.get_gender()),
                    }
                };

                let mut nature = rng.next_u32(25) as u8;
                if self.base.daycare.get_everstone_count() == 2 {
                    nature = self.base.daycare.get_parent_nature(rng.next_u32(2) as u8);
                } else if self.base.daycare.get_parent_item(0) == 1 {
                    nature = self.base.daycare.get_parent_nature(0);
                } else if self.base.daycare.get_parent_item(1) == 1 {
                    nature = self.base.daycare.get_parent_nature(1);
                }

                let parent_ability = self.base.daycare.get_parent_ability(
                    if self.base.daycare.get_parent_gender(1) == 3 {
                        0
                    } else {
                        1
                    },
                );
                let mut ability = rng.next_u32(100) as u8;
                if parent_ability == 2 {
                    ability = if ability < 20 {
                        0
                    } else if ability < 40 {
                        1
                    } else {
                        2
                    };
                } else if parent_ability == 1 {
                    ability = if ability < 20 { 0 } else { 1 };
                } else {
                    ability = if ability < 80 { 0 } else { 1 };
                }

                let mut inheritance = [0; 6];
                let mut i = 0;
                while i < inheritance_count {
                    let index = rng.next_u32(6) as usize;
                    if inheritance[index] == 0 {
                        inheritance[index] = rng.next_u32(2) as u8 + 1;
                        i += 1;
                    }
                }

                let mut ivs = [0; 6];
                for (i, iv) in ivs.iter_mut().enumerate() {
                    let mut rand = rng.next_u32(32) as u8;
                    if inheritance[i] == 1 {
                        rand = self.base.daycare.get_parent_iv(0, i);
                    } else if inheritance[i] == 2 {
                        rand = self.base.daycare.get_parent_iv(1, i);
                    }
                    *iv = rand;
                }

                let ec = rng.next_u32(0xffffffff);

                let mut pid = 0;
                let mut psv = 0;
                for _ in 0..pid_rolls {
                    pid = rng.next_u32(0xffffffff);
                    psv = ((pid >> 16) ^ (pid & 0xfff0)) as u16;
                    if (psv ^ self.base.base.tsv) < 16 {
                        break;
                    }
                }
                let shiny = (psv ^ self.base.base.tsv) < 16;

                let state = EggGeneratorState::new_with_ec(
                    self.base.base.initial_advances.wrapping_add(cnt),
                    ec,
                    pid,
                    ivs,
                    ability,
                    gender,
                    1,
                    nature,
                    u8::from(shiny),
                    inheritance,
                    info,
                );
                if self.base.base.filter.compare_egg_state(&state) {
                    states.push(state);
                }
            }
            cnt += 1;
            rng_list.advance_state();
        }

        states
    }
}
