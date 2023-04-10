use crate::enums::{Game, Method, ShadowType, Shiny};
use crate::gen3::filters::StateFilter3;
use crate::gen3::{Profile3, ShadowTemplate};
use crate::parents::generators::Generator;
use crate::parents::states::GeneratorState;
use crate::parents::{PersonalInfo, Profile, StaticTemplate, Template};
use crate::rng::XDRNG;

fn get_gender(pid: u32, info: &PersonalInfo) -> u8 {
    match info.get_gender() {
        255 => 2,
        254 => 1,
        0 => 0,
        i => (((pid & 255) as u8) < i) as u8,
    }
}

fn get_shiny(pid: u32, tsv: u16) -> u8 {
    let psv = ((pid >> 16) as u16) ^ (pid & 0xFFFF) as u16;
    if tsv == psv {
        2
    } else if (tsv ^ psv) < 8 {
        1
    } else {
        0
    }
}

fn is_shiny(high: u16, low: u16, tsv: u16) -> bool {
    (high ^ low ^ tsv) < 8
}

#[derive(Clone)]
pub struct GameCubeGenerator<'a, 'b> {
    pub base: Generator<'a, 'b, Profile3, StateFilter3>,
    unset: bool,
}

impl<'a, 'b> GameCubeGenerator<'a, 'b> {
    pub fn new(
        initial_advances: u32,
        max_advances: u32,
        delay: u32,
        method: Method,
        unset: bool,
        profile: &'a Profile3,
        filter: &'b StateFilter3,
    ) -> Self {
        Self {
            base: Generator::new(
                initial_advances,
                max_advances,
                delay,
                method,
                profile,
                filter,
            ),
            unset,
        }
    }

    pub fn generate_shadow(
        &self,
        seed: u32,
        shadow_template: &ShadowTemplate,
    ) -> Vec<GeneratorState> {
        if (self.base.profile.get_version() & Game::COLOSSEUM) != Game::NONE {
            self.generate_colo_shadow(seed, shadow_template)
        } else {
            self.generate_gales_shadow(seed, shadow_template)
        }
    }

    pub fn generate(&self, seed: u32, static_template: &StaticTemplate) -> Vec<GeneratorState> {
        if self.base.method == Method::Channel {
            self.generate_channel(seed, static_template)
        } else {
            self.generate_non_lock(seed, static_template)
        }
    }

    fn generate_channel(&self, seed: u32, static_template: &StaticTemplate) -> Vec<GeneratorState> {
        let mut states = vec![];

        let info = static_template.get_info();
        const THRESHOLDS: [u16; 2] = [0x4000, 0x547a];

        let mut rng =
            XDRNG::new_with_initial_advances(seed, self.base.initial_advances + self.base.delay);
        let mut cnt = 0;
        while cnt <= self.base.max_advances {
            let mut go = rng;

            let mut mask = 0;
            while (mask & 14) != 14 {
                mask |= 1 << (go.next_u16() >> 14);
            }

            go.advance(4);
            if THRESHOLDS.into_iter().any(|thresh| go.next_u16() <= thresh) {
                go.advance(1);
            } else {
                go.advance(2);
            }

            const TID: u16 = 40122;
            let sid = go.next_u16();
            let mut high = go.next_u16();
            let low = go.next_u16();

            go.advance(3);

            if (TID ^ sid ^ high ^ u16::from(low < 8)) != 0 {
                high ^= 0x8000;
            }
            let pid = ((high as u32) << 16) | (low as u32);
            let mut ivs = [0; 6];
            ivs[0] = (go.next_u16() >> 11) as u8;
            ivs[1] = (go.next_u16() >> 11) as u8;
            ivs[2] = (go.next_u16() >> 11) as u8;
            ivs[5] = (go.next_u16() >> 11) as u8;
            ivs[3] = (go.next_u16() >> 11) as u8;
            ivs[4] = (go.next_u16() >> 11) as u8;

            let state = GeneratorState::new(
                self.base.initial_advances + cnt,
                pid,
                ivs,
                (pid & 1) as u8,
                2,
                static_template.get_level(),
                (pid % 25) as u8,
                get_shiny(pid, TID ^ sid),
                info,
            );

            if self.base.filter.compare_generator_state(&state) {
                states.push(state);
            }

            cnt += 1;
            rng.next();
        }

        states
    }

    fn generate_colo_shadow(
        &self,
        seed: u32,
        shadow_template: &ShadowTemplate,
    ) -> Vec<GeneratorState> {
        let mut states = vec![];
        let info = shadow_template.get_info();

        let mut rng =
            XDRNG::new_with_initial_advances(seed, self.base.initial_advances + self.base.delay);
        let mut cnt = 0;
        while cnt <= self.base.max_advances {
            let mut go = rng;

            let trainer_tsv = go.next_u16() ^ go.next_u16();
            let mut ability = 0;
            let mut high = 0;
            let mut low = 0;

            for i in 0..shadow_template.get_count() {
                let lock = shadow_template.get_lock(i);

                go.advance(4);
                ability = go.next_u16_max::<true>(2) as u8;

                let mut pid;
                'inner: while {
                    high = go.next_u16();
                    low = go.next_u16();
                    pid = ((high as u32) << 16) | (low as u32);
                    if is_shiny(high, low, trainer_tsv) {
                        continue 'inner;
                    };
                    !lock.compare(pid)
                } {}
            }

            let iv1;
            let iv2;
            if shadow_template.get_type() == ShadowType::EReader {
                iv1 = 0;
                iv2 = 0;
            } else {
                go.advance(2);

                iv1 = go.next_u16();
                iv2 = go.next_u16();
                ability = go.next_u16_max::<true>(2) as u8;
                high = go.next_u16();
                low = go.next_u16();

                while is_shiny(high, low, trainer_tsv) {
                    high = go.next_u16();
                    low = go.next_u16();
                }
            }

            let pid = ((high as u32) << 16) | (low as u32);
            let mut ivs = [0; 6];
            ivs[0] = (iv1 & 31) as u8;
            ivs[1] = ((iv1 >> 5) & 31) as u8;
            ivs[2] = ((iv1 >> 10) & 31) as u8;
            ivs[3] = ((iv2 >> 5) & 31) as u8;
            ivs[4] = ((iv2 >> 10) & 31) as u8;
            ivs[5] = (iv2 & 31) as u8;

            let state = GeneratorState::new(
                self.base.initial_advances + cnt,
                pid,
                ivs,
                ability,
                get_gender(pid, info),
                shadow_template.get_level(),
                (pid % 25) as u8,
                get_shiny(pid, self.base.tsv),
                info,
            );

            if self.base.filter.compare_generator_state(&state) {
                states.push(state);
            }

            cnt += 1;
            rng.next();
        }

        states
    }

    fn generate_gales_shadow(
        &self,
        seed: u32,
        shadow_template: &ShadowTemplate,
    ) -> Vec<GeneratorState> {
        let mut states = vec![];
        let info = shadow_template.get_info();

        let mut rng =
            XDRNG::new_with_initial_advances(seed, self.base.initial_advances + self.base.delay);
        let mut cnt = 0;
        while cnt <= self.base.max_advances {
            let mut go = rng;

            go.advance(2);

            for i in 0..shadow_template.get_count() {
                let lock = shadow_template.get_lock(i);

                go.advance(5);

                if !lock.get_ignore() {
                    let mut pid;
                    'inner: while {
                        let high = go.next_u16();
                        let low = go.next_u16();
                        pid = ((high as u32) << 16) | (low as u32);
                        if is_shiny(high, low, self.base.tsv) {
                            continue 'inner;
                        };
                        !lock.compare(pid)
                    } {}
                }
            }

            if matches!(
                shadow_template.get_type(),
                ShadowType::SecondShadow | ShadowType::Salamence
            ) {
                go.advance(5);

                if self.unset {
                    while is_shiny(go.next_u16(), go.next_u16(), self.base.tsv) {}
                }
            }

            go.advance(2);

            let iv1 = go.next_u16();
            let iv2 = go.next_u16();
            let ability = (go.next_u16_max::<true>(2) as u8)
                & u8::from(info.get_ability(0) != info.get_ability(1));
            let mut high = go.next_u16();
            let mut low = go.next_u16();

            while is_shiny(high, low, self.base.tsv) {
                high = go.next_u16();
                low = go.next_u16();
            }

            let pid = ((high as u32) << 16) | (low as u32);
            let mut ivs = [0; 6];
            ivs[0] = (iv1 & 31) as u8;
            ivs[1] = ((iv1 >> 5) & 31) as u8;
            ivs[2] = ((iv1 >> 10) & 31) as u8;
            ivs[3] = ((iv2 >> 5) & 31) as u8;
            ivs[4] = ((iv2 >> 10) & 31) as u8;
            ivs[5] = (iv2 & 31) as u8;

            let state = GeneratorState::new(
                self.base.initial_advances + cnt,
                pid,
                ivs,
                ability,
                get_gender(pid, info),
                shadow_template.get_level(),
                (pid % 25) as u8,
                get_shiny(pid, self.base.tsv),
                info,
            );

            if self.base.filter.compare_generator_state(&state) {
                states.push(state);
            }

            cnt += 1;
            rng.next();
        }

        states
    }

    fn generate_non_lock(
        &self,
        seed: u32,
        static_template: &StaticTemplate,
    ) -> Vec<GeneratorState> {
        let mut states = vec![];
        let info = static_template.get_info();

        let mut actual_tsv = self.base.tsv;

        if static_template.specie == 25 || static_template.specie == 251 {
            actual_tsv = 31121;
        } else if static_template.specie == 250 {
            actual_tsv = 10048;
        }

        let mut rng =
            XDRNG::new_with_initial_advances(seed, self.base.initial_advances + self.base.delay);
        let mut cnt = 0;
        while cnt <= self.base.max_advances {
            let mut go = rng;

            let mut iv1 = 0;
            let mut iv2 = 0;
            let mut ability = 0;
            let mut high = 0;
            let mut low = 0;

            if static_template.specie == 133 {
                actual_tsv = go.next_u16() ^ go.next_u16();
                go.advance(2);
            }

            if static_template.specie == 196 || static_template.specie == 197 {
                actual_tsv = go.next_u16() ^ go.next_u16();

                let rounds = if static_template.specie == 196 { 2 } else { 1 };
                for _ in 0..rounds {
                    go.advance(2);
                    iv1 = go.next_u16();
                    iv2 = go.next_u16();
                    ability = go.next_u16_max::<true>(2) as u8;
                    high = go.next_u16();
                    low = go.next_u16();

                    while is_shiny(high, low, actual_tsv) || ((low & 255) as u8) < info.get_gender()
                    {
                        high = go.next_u16();
                        low = go.next_u16();
                    }
                }
            } else {
                iv1 = go.next_u16();
                iv2 = go.next_u16();
                ability = go.next_u16_max::<true>(2) as u8;
                high = go.next_u16();
                low = go.next_u16();

                if static_template.get_shiny() == Shiny::Never {
                    while is_shiny(high, low, actual_tsv) {
                        high = go.next_u16();
                        low = go.next_u16();
                    }
                }
            }

            let pid = ((high as u32) << 16) | (low as u32);
            ability &= u8::from(info.get_ability(0) != info.get_ability(1));
            let mut ivs = [0; 6];
            ivs[0] = (iv1 & 31) as u8;
            ivs[1] = ((iv1 >> 5) & 31) as u8;
            ivs[2] = ((iv1 >> 10) & 31) as u8;
            ivs[3] = ((iv2 >> 5) & 31) as u8;
            ivs[4] = ((iv2 >> 10) & 31) as u8;
            ivs[5] = (iv2 & 31) as u8;

            let state = GeneratorState::new(
                self.base.initial_advances + cnt,
                pid,
                ivs,
                ability,
                get_gender(pid, info),
                static_template.get_level(),
                (pid % 25) as u8,
                get_shiny(pid, actual_tsv),
                info,
            );

            if self.base.filter.compare_generator_state(&state) {
                states.push(state);
            }

            cnt += 1;
            rng.next();
        }

        states
    }
}
