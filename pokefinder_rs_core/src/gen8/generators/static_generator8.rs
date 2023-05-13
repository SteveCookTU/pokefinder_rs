use crate::enums::{Lead, Method, Shiny};
use crate::gen8::filters::StateFilter8;
use crate::gen8::Profile8;
use crate::parents::generators::StaticGenerator;
use crate::parents::states::GeneratorState;
use crate::parents::{StaticTemplate, Template};
use crate::rng::{RNGList, XoroshiroBDSP, Xorshift};

fn gen(rng: &mut Xorshift) -> u32 {
    rng.next_range(0x80000000, 0x7fffffff)
}

#[derive(Clone)]
pub struct StaticGenerator8<'a, 'b> {
    pub base: StaticGenerator<'a, 'b, Profile8, StateFilter8>,
}

impl<'a, 'b> StaticGenerator8<'a, 'b> {
    pub fn new(
        initial_advances: u32,
        max_advances: u32,
        delay: u32,
        lead: Lead,
        profile: &'a Profile8,
        filter: &'b StateFilter8,
    ) -> Self {
        Self {
            base: StaticGenerator::new(
                initial_advances,
                max_advances,
                delay,
                Method::None,
                lead,
                profile,
                filter,
            ),
        }
    }

    pub fn generate(
        &self,
        seed0: u64,
        seed1: u64,
        static_template: &StaticTemplate,
    ) -> Vec<GeneratorState> {
        let info = static_template.get_info();
        let rng = Xorshift::new_with_initial_advances(
            seed0,
            seed1,
            self.base
                .base
                .initial_advances
                .wrapping_add(self.base.base.delay),
        );
        let mut rng_list = RNGList::<u32, Xorshift, 32>::new(rng, Some(gen));

        let mut states = vec![];
        let mut cnt = 0;
        while cnt <= self.base.base.max_advances {
            let ec = rng_list.next();
            let sidtid = rng_list.next();
            let mut pid = rng_list.next();

            let psv = ((pid >> 16) ^ (pid & 0xFFFF)) as u16;

            let shiny = if static_template.get_shiny() == Shiny::Never {
                if (psv ^ self.base.base.tsv) < 16 {
                    pid ^= 0x10000000;
                }
                0
            } else {
                let fake_xor = (((sidtid >> 16) ^ (sidtid & 0xFFFF)) as u16) ^ psv;
                if fake_xor < 16 {
                    let shiny = if fake_xor == 0 { 2 } else { 1 };
                    let real_xor = psv ^ self.base.base.tsv;
                    let real_shiny = if real_xor == 0 {
                        2
                    } else if real_xor < 16 {
                        1
                    } else {
                        0
                    };

                    if real_shiny != shiny {
                        let high = ((pid & 0xFFFF) as u16) ^ self.base.base.tsv ^ (2 - shiny);
                        pid = ((high as u32) << 16) | (pid & 0xFFFF);
                    }
                    shiny as u8
                } else {
                    if (psv ^ self.base.base.tsv) < 16 {
                        pid ^= 0x10000000;
                    }
                    0
                }
            };

            let mut ivs = [255; 6];
            let mut i = 0;
            while i < static_template.get_iv_count() {
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

            let ability = if static_template.get_ability() != 255 {
                static_template.get_ability()
            } else {
                (rng_list.next() % 2) as u8
            };

            let gender = match info.get_gender() {
                255 => 2,
                254 => 1,
                0 => 0,
                i => {
                    if (self.base.lead == Lead::CUTE_CHARM_F
                        || self.base.lead == Lead::CUTE_CHARM_M)
                        && (rng_list.next() % 3) > 0
                    {
                        if self.base.lead == Lead::CUTE_CHARM_F {
                            0
                        } else {
                            1
                        }
                    } else {
                        u8::from((rng_list.next() % 253) as u8 + 1 < i)
                    }
                }
            };

            let nature = if self.base.lead.bits() <= Lead::SYNCHRONIZE_END.bits() {
                self.base.lead.bits()
            } else {
                (rng_list.next() % 25) as u8
            };

            let state = GeneratorState::new_with_ec(
                self.base.base.initial_advances.wrapping_add(cnt),
                ec,
                pid,
                ivs,
                ability,
                gender,
                static_template.get_level(),
                nature,
                shiny,
                info,
            );
            if self.base.base.filter.compare_generator_state(&state) {
                states.push(state);
            }

            cnt += 1;
            rng_list.advance_state();
        }
        states
    }

    pub fn generate_roamer(
        &self,
        seed0: u64,
        seed1: u64,
        static_template: &StaticTemplate,
    ) -> Vec<GeneratorState> {
        let gender = if static_template.get_specie() == 488 {
            1
        } else {
            2
        };

        let mut roamer = Xorshift::new_with_initial_advances(
            seed0,
            seed1,
            self.base
                .base
                .initial_advances
                .wrapping_add(self.base.base.delay),
        );

        let mut states = vec![];
        for cnt in 0..=self.base.base.max_advances {
            let ec = roamer.next_range(0x80000000, 0x7fffffff);
            let mut rng = XoroshiroBDSP::new(ec as u64);

            let sidtid = rng.next_u32(0xffffffff);
            let mut pid = rng.next_u32(0xffffffff);

            let psv = ((pid >> 16) ^ (pid & 0xFFFF)) as u16;
            let fake_xor = (((sidtid >> 16) ^ (sidtid & 0xFFFF)) as u16) ^ psv;
            let shiny = if fake_xor < 16 {
                let shiny = if fake_xor == 0 { 2 } else { 1 };
                let real_xor = psv ^ self.base.base.tsv;
                let real_shiny = if real_xor == 0 {
                    2
                } else if real_xor < 16 {
                    1
                } else {
                    0
                };

                if real_shiny != shiny {
                    let high = ((pid & 0xFFFF) as u16) ^ self.base.base.tsv ^ (2 - shiny);
                    pid = ((high as u32) << 16) | (pid & 0xFFFF);
                }
                shiny as u8
            } else {
                if (psv ^ self.base.base.tsv) < 16 {
                    pid ^= 0x10000000;
                }
                0
            };

            let mut ivs = [255; 6];
            let mut i = 0;
            while i < 3 {
                let index = rng.next_u32(6) as u8;
                if ivs[index as usize] == 255 {
                    ivs[index as usize] = 31;
                    i += 1;
                }
            }

            ivs.iter_mut().for_each(|iv| {
                if *iv == 255 {
                    *iv = rng.next_u32(32) as u8;
                }
            });

            let ability = rng.next_u32(2) as u8;

            let nature = if self.base.lead.bits() <= Lead::SYNCHRONIZE_END.bits() {
                self.base.lead.bits()
            } else {
                rng.next_u32(25) as u8
            };

            let state = GeneratorState::new_with_ec(
                self.base.base.initial_advances.wrapping_add(cnt),
                ec,
                pid,
                ivs,
                ability,
                gender,
                static_template.get_level(),
                nature,
                shiny,
                static_template.get_info(),
            );
            if self.base.base.filter.compare_generator_state(&state) {
                states.push(state);
            }
        }
        states
    }
}
