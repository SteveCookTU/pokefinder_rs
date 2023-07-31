use crate::enums::Method;
use crate::gen5::filters::StateFilter5;
use crate::gen5::states::DreamRadarState;
use crate::gen5::{DreamRadarTemplate, Profile5};
use crate::parents::generators::Generator;
use crate::parents::Template;
use crate::rng::{RNGList, Rng, BWRNG, MT};
use crate::util;

const LEVEL_TABLE: [u8; 9] = [5, 10, 10, 20, 20, 30, 30, 40, 40];

fn gen(rng: &mut MT) -> u32 {
    rng.next() >> 27
}

#[derive(Clone)]
pub struct DreamRadarGenerator<'a, 'b, 'c> {
    pub base: Generator<'a, 'b, Profile5, StateFilter5>,
    pub radar_template: &'c DreamRadarTemplate,
    pub iv_advances: u8,
    pub level: u8,
    pub pid_advances: u8,
}

impl<'a, 'b, 'c> DreamRadarGenerator<'a, 'b, 'c> {
    pub fn new(
        initial_advances: u32,
        max_advances: u32,
        badge_count: u8,
        radar_templates: &'c [DreamRadarTemplate],
        profile: &'a Profile5,
        filter: &'b StateFilter5,
    ) -> Self {
        let base = Generator::new(
            initial_advances,
            max_advances,
            0,
            Method::None,
            profile,
            filter,
        );
        let radar_template = radar_templates.last().unwrap();
        let level = LEVEL_TABLE[badge_count as usize];
        let mut pid_advances: u8 = 0;
        let mut iv_advances: u8 = 0;

        for (i, slot) in radar_templates.iter().enumerate() {
            if slot.genie {
                pid_advances = pid_advances.wrapping_add(5);
                iv_advances = iv_advances.wrapping_add(13);
            }

            let info = slot.base.get_info();
            if i != radar_templates.len() - 1 {
                pid_advances =
                    pid_advances.wrapping_add(if slot.legend || info.get_gender() != 255 {
                        5
                    } else {
                        4
                    });
                iv_advances = iv_advances.wrapping_add(13);
            }
        }

        Self {
            base,
            radar_template,
            iv_advances,
            level,
            pid_advances,
        }
    }

    pub fn generate(&self, seed: u64) -> Vec<DreamRadarState> {
        let info = self.radar_template.base.get_info();

        let mut rng = BWRNG::new_with_initial_advances(
            seed,
            self.base.initial_advances.wrapping_mul(2).wrapping_add(
                util::utilities5::initial_advances_bw2(seed, self.base.profile.get_memory_link()),
            ),
        );
        if !self.base.profile.get_memory_link() {
            rng.next();
        }
        let rng2 = MT::new_with_initial_advances(
            (seed >> 32) as u32,
            self.base
                .initial_advances
                .wrapping_mul(2)
                .wrapping_add(self.iv_advances as u32)
                .wrapping_add(9),
        );
        let mut rng_list = RNGList::<u32, MT, 8>::new(rng2, Some(gen));

        let mut states = vec![];
        let mut cnt = 0;
        while cnt <= self.base.max_advances {
            let mut go = BWRNG::new_with_initial_advances(rng.seed, self.pid_advances as u32);

            let mut ivs = [0; 6];
            ivs.iter_mut().for_each(|iv| *iv = rng_list.next() as u8);

            go.next();
            let mut pid = go.next_u32();

            if self.radar_template.legend {
                pid = util::utilities5::force_gender(pid, &mut go, 0, 0);
            } else if self.radar_template.base.gender == 0 || self.radar_template.base.gender == 1 {
                pid = util::utilities5::force_gender(
                    pid,
                    &mut go,
                    self.radar_template.base.gender,
                    info.get_gender(),
                );
            }

            pid ^= 0x10000;

            if (((pid >> 16) ^ (pid & 0xFFFF)) as u16 ^ self.base.tsv) < 8 {
                pid ^= 0x10000000;
            }

            let ability = if self.radar_template.base.ability == 255 {
                ((pid >> 16) & 1) as u8
            } else {
                2
            };

            go.advance(2);

            let nature = go.next_u32_max(25) as u8;

            let state = DreamRadarState::new(
                rng.next_u32_max(8) as u8,
                self.base.initial_advances.wrapping_add(cnt),
                pid,
                ivs,
                ability,
                self.radar_template.base.gender,
                self.level,
                nature,
                0,
                info,
            );

            if self.base.filter.compare_dream_radar_state(&state) {
                states.push(state);
            }

            cnt += 1;
            rng_list.advance_states(2);
            rng.next();
        }

        states
    }
}
