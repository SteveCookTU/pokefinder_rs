use crate::enums::Method;
use crate::gen8::filters::StateFilter8;
use crate::gen8::{Profile8, WB8};
use crate::parents::generators::Generator;
use crate::parents::states::GeneratorState;
use crate::parents::Profile;
use crate::rng::{RNGList, Xorshift};

fn gen(rng: &mut Xorshift) -> u32 {
    rng.next_range(0x80000000, 0x7fffffff)
}

#[derive(Clone)]
pub struct EventGenerator8<'a, 'b, 'c> {
    pub base: Generator<'a, 'b, Profile8, StateFilter8>,
    pub wb8: &'c WB8,
}

impl<'a, 'b, 'c> EventGenerator8<'a, 'b, 'c> {
    pub fn new(
        initial_advances: u32,
        max_advances: u32,
        delay: u32,
        wb8: &'c WB8,
        profile: &'a Profile8,
        filter: &'b StateFilter8,
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
            wb8,
        };
        if !wb8.egg {
            new.base.tsv = wb8.tid ^ wb8.sid;
        } else {
            new.base.tsv = (profile.get_tid() & 0xFFF0) ^ profile.get_sid();
        }
        new
    }

    pub fn generate(&self, seed0: u64, seed1: u64) -> Vec<GeneratorState> {
        let info = self.wb8.get_info(self.base.profile.get_version());
        let rng = Xorshift::new_with_initial_advances(
            seed0,
            seed1,
            self.base.initial_advances.wrapping_add(self.base.delay),
        );
        let mut rng_list = RNGList::<u32, Xorshift, 32>::new(rng, Some(gen));

        let mut states = vec![];
        let mut cnt = 0;
        while cnt <= self.base.max_advances {
            let ec = if self.wb8.ec == 0 {
                rng_list.next()
            } else {
                self.wb8.ec
            };

            let mut pid;
            let shiny = match self.wb8.shiny {
                0 => {
                    pid = rng_list.next();
                    let psv = ((pid >> 16) ^ (pid & 0xFFF0)) as u16;

                    if (psv ^ self.base.tsv) < 16 {
                        pid ^= 0x10000000;
                    }
                    0
                }
                1 | 2 => {
                    pid = rng_list.next();
                    let psv = ((pid >> 16) ^ (pid & 0xFFF0)) as u16;

                    let real_xor = psv ^ self.base.tsv;
                    let shiny_type = if real_xor == 0 {
                        2
                    } else if real_xor < 16 {
                        1
                    } else {
                        0
                    };

                    if shiny_type != self.wb8.shiny {
                        let high =
                            ((pid & 0xFFFF) as u16) ^ self.base.tsv ^ (2 - self.wb8.shiny as u16);
                        pid = ((high as u32) << 16) | (pid & 0xFFFF);
                    }
                    self.wb8.shiny
                }
                4 => {
                    pid = self.wb8.pid;
                    let psv = ((pid >> 16) ^ (pid & 0xFFF0)) as u16;

                    let real_xor = psv ^ self.base.tsv;
                    if real_xor == 0 {
                        2
                    } else if real_xor < 16 {
                        1
                    } else {
                        0
                    }
                }
                _ => {
                    pid = 0;
                    0
                }
            };

            let mut ivs = [255; 6];
            let mut i = 0;
            while i < self.wb8.iv_count {
                let index = (rng_list.next() % 6) as u8;
                if ivs[index as usize] == 255 {
                    ivs[index as usize] = 31;
                    i += 1;
                }
            }

            ivs.iter_mut().for_each(|iv| {
                if *iv == 255 {
                    *iv = (rng_list.next() % 32) as u8;
                }
            });

            let ability = match self.wb8.ability {
                0 | 1 | 2 => self.wb8.ability,
                3 => (rng_list.next() % 2) as u8,
                4 => (rng_list.next() % 3) as u8,
                _ => 0,
            };

            let gender = match self.wb8.gender {
                0 | 1 | 2 => self.wb8.gender,
                i => u8::from(((rng_list.next() % 252) as u8 + 1) < i),
            };

            let nature = if self.wb8.nature != 255 {
                self.wb8.nature
            } else {
                (rng_list.next() % 25) as u8
            };

            let state = GeneratorState::new_with_ec(
                self.base.initial_advances.wrapping_add(cnt),
                ec,
                pid,
                ivs,
                ability,
                gender,
                self.wb8.level,
                nature,
                shiny,
                info,
            );

            if self.base.filter.compare_generator_state(&state) {
                states.push(state);
            }

            cnt += 1;
            rng_list.advance_state();
        }
        states
    }
}
