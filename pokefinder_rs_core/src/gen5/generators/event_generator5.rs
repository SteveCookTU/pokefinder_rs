use crate::enums::Method;
use crate::gen5::filters::StateFilter5;
use crate::gen5::states::State5;
use crate::gen5::{Profile5, PGF};
use crate::parents::generators::Generator;
use crate::parents::{personal_loader, PersonalInfo, Profile};
use crate::rng::BWRNG;
use crate::util;

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

#[derive(Clone)]
pub struct EventGenerator5<'a, 'b, 'c> {
    pub base: Generator<'a, 'b, Profile5, StateFilter5>,
    pub pgf: &'c PGF,
    pub wondercard_advances: u8,
}

impl<'a, 'c, 'b> EventGenerator5<'a, 'b, 'c> {
    pub fn new(
        initial_advances: u32,
        max_advances: u32,
        delay: u32,
        pgf: &'c PGF,
        profile: &'a Profile5,
        filter: &'b StateFilter5,
    ) -> Self {
        let mut new = Self {
            base: Generator::new(
                initial_advances,
                max_advances,
                delay,
                Method::None,
                profile,
                filter,
            ),
            pgf,
            wondercard_advances: pgf.get_advances(),
        };

        if !pgf.egg {
            new.base.tsv = pgf.tid ^ pgf.sid;
        }

        new
    }

    pub fn generate(&self, seed: u64) -> Vec<State5> {
        let info = personal_loader::get_personal_info(
            self.base.profile.get_version(),
            self.pgf.species,
            0,
        );

        let advances = util::utilities5::initial_advances(seed, self.base.profile);
        let mut rng = BWRNG::new_with_initial_advances(
            seed,
            advances
                .wrapping_add(self.base.initial_advances)
                .wrapping_add(self.base.delay),
        );

        let mut states = vec![];
        for cnt in 0..=self.base.max_advances {
            let mut go = rng;

            let mut ivs = [0; 6];
            for (i, iv) in ivs.iter_mut().enumerate() {
                let parameter_iv = self.pgf.get_iv(i);
                if parameter_iv == 255 {
                    *iv = go.next_u32_max(32) as u8;
                } else {
                    *iv = parameter_iv;
                }
            }

            go.advance(2);

            let mut pid = go.next_u32();
            if matches!(self.pgf.gender, 0 | 1) {
                pid = util::utilities5::force_gender(
                    pid,
                    &mut go,
                    self.pgf.gender,
                    info.get_gender(),
                );
            }

            if self.pgf.shiny == 0 {
                if ((((pid >> 16) ^ (pid & 0xFFFF)) as u16) ^ self.base.tsv) < 8 {
                    pid ^= 0x10000000;
                }
            } else if self.pgf.shiny == 2 {
                let low = pid & 0xFF;
                pid = ((low ^ (self.base.tsv as u32)) << 16) | low;
            }

            let ability = if self.pgf.ability < 3 {
                if self.pgf.ability == 1 {
                    pid |= 0x10000;
                } else {
                    pid &= !0x10000;
                }
                self.pgf.ability
            } else {
                pid ^= 0x10000;
                ((pid >> 16) & 1) as u8
            };

            let nature = if self.pgf.nature != 0xFF {
                self.pgf.nature
            } else {
                go.advance(1);
                go.next_u32_max(25) as u8
            };

            let state = State5::new(
                rng.next_u32_max(0x1FFF) as u16,
                advances
                    .wrapping_add(self.base.initial_advances)
                    .wrapping_add(cnt),
                pid,
                ivs,
                ability,
                get_gender(pid, info),
                self.pgf.level,
                nature,
                get_shiny(pid, self.base.tsv),
                info,
            );

            if self.base.filter.compare_generator_state(&state.base) {
                states.push(state);
            }
        }

        states
    }
}
