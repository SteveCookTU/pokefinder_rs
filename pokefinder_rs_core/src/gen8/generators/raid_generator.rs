use crate::enums::{Method, Shiny};
use crate::gen8::filters::StateFilter8;
use crate::gen8::{Profile8, Raid};
use crate::parents::filters::Filter;
use crate::parents::generators::Generator;
use crate::parents::states::GeneratorState;
use crate::parents::Template;
use crate::rng::Xoroshiro;

const TOXTRICITY_AMPED_NATURES: [u8; 13] = [3, 4, 2, 8, 9, 19, 22, 11, 13, 14, 0, 6, 24];
const TOXTRICITY_LOWKEY_NATURES: [u8; 12] = [1, 5, 7, 10, 12, 15, 16, 17, 18, 20, 21, 23];

#[derive(Clone)]
pub struct RaidGenerator<'a, 'b> {
    pub base: Generator<'a, 'b, Profile8, StateFilter8>,
}

impl<'a, 'b> RaidGenerator<'a, 'b> {
    pub fn new(
        initial_advances: u32,
        max_advances: u32,
        delay: u32,
        profile: &'a Profile8,
        filter: &'b StateFilter8,
    ) -> Self {
        Self {
            base: Generator::new(
                initial_advances,
                max_advances,
                delay,
                Method::None,
                profile,
                filter,
            ),
        }
    }

    pub fn generate(&self, mut seed: u64, level: u8, raid: &Raid) -> Vec<GeneratorState> {
        let info = raid.base.get_info();
        seed = seed.wrapping_add(
            0x82A2B175229D6A5Bu64
                .wrapping_mul(self.base.initial_advances.wrapping_add(self.base.delay) as u64),
        );

        let mut states = vec![];
        let mut cnt = 0;
        while cnt <= self.base.max_advances {
            let mut rng: Xoroshiro = seed.into();

            let ec = rng.next_u32::<0xffffffff>();
            let sidtid = rng.next_u32::<0xffffffff>();
            let mut pid = rng.next_u32::<0xffffffff>();
            let psv = ((pid >> 16) ^ (pid & 0xFFFF)) as u16;
            let shiny = if raid.base.shiny == Shiny::Random {
                let fake_xor = (((sidtid >> 16) ^ (sidtid & 0xFFFF)) as u16) ^ psv;

                if fake_xor < 16 {
                    let shiny = if fake_xor == 0 { 2 } else { 1 };
                    let real_xor = psv ^ self.base.tsv;
                    let real_shiny = if real_xor == 0 {
                        2
                    } else if real_xor < 16 {
                        1
                    } else {
                        0
                    };
                    if real_shiny != shiny {
                        let high = ((pid & 0xFFFF) as u16) ^ self.base.tsv ^ (2 - shiny);
                        pid = ((high as u32) << 16) | (pid & 0xFFFF);
                    }
                    shiny as u8
                } else {
                    if (psv ^ self.base.tsv) < 16 {
                        pid ^= 0x10000000;
                    }
                    0
                }
            } else if raid.base.shiny == Shiny::Never {
                if (psv ^ self.base.tsv) < 16 {
                    pid ^= 0x10000000;
                }
                0
            } else {
                let real_xor = psv ^ self.base.tsv;
                if real_xor != 0 {
                    let high = ((pid & 0xFFFF) as u16) ^ self.base.tsv;
                    pid = ((high as u32) << 16) | (pid & 0xFFFF);
                }
                2
            };

            if !self.base.filter.compare_shiny(shiny) {
                cnt += 1;
                seed = seed.wrapping_add(0x82A2B175229D6A5B);
                continue;
            }

            let mut ivs = [255; 6];
            let mut i = 0;
            while i < raid.base.iv_count {
                let index = rng.next_u32::<6>() as u8;
                if ivs[index as usize] == 255 {
                    ivs[index as usize] = 31;
                    i += 1;
                }
            }

            ivs.iter_mut().for_each(|iv| {
                if *iv == 255 {
                    *iv = rng.next_u32::<32>() as u8;
                }
            });

            let ability = match raid.base.ability {
                4 => rng.next_u32::<3>() as u8,
                3 => rng.next_u32::<2>() as u8,
                _ => raid.base.ability,
            };

            let gender = match raid.base.gender {
                1 => 0,
                2 => 1,
                3 => 2,
                _ => match info.get_gender() {
                    255 => 2,
                    254 => 1,
                    0 => 0,
                    i => u8::from((rng.next_u32::<253>() as u8 + 1) < i),
                },
            };

            let nature = if raid.base.species != 849 {
                rng.next_u32::<25>() as u8
            } else if raid.base.form == 0 {
                TOXTRICITY_AMPED_NATURES[rng.next_u32::<13>() as usize]
            } else {
                TOXTRICITY_LOWKEY_NATURES[rng.next_u32::<12>() as usize]
            };

            let state = GeneratorState::new_with_ec(
                self.base.initial_advances.wrapping_add(cnt),
                ec,
                pid,
                ivs,
                ability,
                gender,
                level,
                nature,
                shiny,
                info,
            );
            if self.base.filter.compare_generator_state(&state) {
                states.push(state);
            }

            cnt += 1;
            seed = seed.wrapping_add(0x82A2B175229D6A5B);
        }
        states
    }
}
