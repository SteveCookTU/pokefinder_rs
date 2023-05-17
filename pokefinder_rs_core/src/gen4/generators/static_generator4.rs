use crate::enums::{Lead, Method, Shiny};
use crate::gen4::filters::StateFilter4;
use crate::gen4::states::GeneratorState4;
use crate::gen4::{Profile4, StaticTemplate4};
use crate::parents::generators::StaticGenerator;
use crate::parents::{PersonalInfo, Template};
use crate::rng::{PokeRNG, ARNG};

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

fn is_shiny(pid: u32, tsv: u16) -> bool {
    let psv = ((pid >> 16) ^ (pid & 0xFFFF)) as u16;
    (psv ^ tsv) < 8
}

/// Static encounter generator for Gen4
#[derive(Clone)]
pub struct StaticGenerator4<'a, 'b> {
    /// Base static generator data
    pub base: StaticGenerator<'a, 'b, Profile4, StateFilter4>,
}

impl<'a, 'b> StaticGenerator4<'a, 'b> {
    /// Construct a new [`StaticGenerator4`] struct
    pub fn new(
        initial_advances: u32,
        max_advances: u32,
        delay: u32,
        method: Method,
        lead: Lead,
        profile: &'a Profile4,
        filter: &'b StateFilter4,
    ) -> Self {
        Self {
            base: StaticGenerator::new(
                initial_advances,
                max_advances,
                delay,
                method,
                lead,
                profile,
                filter,
            ),
        }
    }

    /// Generates states for the `static_template`
    pub fn generate(&self, seed: u32, static_template: &StaticTemplate4) -> Vec<GeneratorState4> {
        match self.base.base.method {
            Method::Method1 => self.generate_method_1(seed, static_template),
            Method::MethodJ => self.generate_method_j(seed, static_template),
            Method::MethodK => self.generate_method_k(seed, static_template),
            _ => vec![],
        }
    }

    fn generate_method_1(
        &self,
        seed: u32,
        static_template: &StaticTemplate4,
    ) -> Vec<GeneratorState4> {
        let mut states = vec![];
        let info = static_template.get_info();

        let mut rng = PokeRNG::new_with_initial_advances(
            seed,
            self.base
                .base
                .initial_advances
                .wrapping_add(self.base.base.delay),
        );
        let mut cnt = 0;
        while cnt <= self.base.base.max_advances {
            let mut go = rng;

            let mut pid;
            if static_template.get_shiny() == Shiny::Always {
                let mut low = go.next_u16_max::<true>(8);
                let mut high = go.next_u16_max::<true>(8);

                for i in 3..16 {
                    low |= go.next_u16_max::<true>(2) << i;
                }
                high |= (low ^ self.base.base.tsv) & 0xFFF8;
                pid = ((high as u32) << 16) | (low as u32);
            } else {
                pid = go.next_u16() as u32;
                pid |= (go.next_u16() as u32) << 16;

                if static_template.get_shiny() == Shiny::Never {
                    while is_shiny(pid, self.base.base.tsv) {
                        pid = ARNG::new(pid).next();
                    }
                }
            }

            let iv1 = go.next_u16();
            let iv2 = go.next_u16();
            let mut ivs = [0; 6];
            ivs[0] = (iv1 & 31) as u8;
            ivs[1] = ((iv1 >> 5) & 31) as u8;
            ivs[2] = ((iv1 >> 10) & 31) as u8;
            ivs[3] = ((iv2 >> 5) & 31) as u8;
            ivs[4] = ((iv2 >> 10) & 31) as u8;
            ivs[5] = (iv2 & 31) as u8;

            let state = GeneratorState4::new(
                rng.next_u16(),
                self.base.base.initial_advances.wrapping_add(cnt),
                pid,
                ivs,
                (pid & 1) as u8,
                get_gender(pid, info),
                static_template.get_level(),
                (pid % 25) as u8,
                get_shiny(pid, self.base.base.tsv),
                info,
            );
            if self.base.base.filter.compare_generator_state(&state) {
                states.push(state);
            }

            cnt += 1;
        }

        states
    }

    fn generate_method_j(
        &self,
        seed: u32,
        static_template: &StaticTemplate4,
    ) -> Vec<GeneratorState4> {
        let mut states = vec![];
        let info = static_template.get_info();

        let mut cute_charm_flag = false;
        let mut buffer = 0;
        if self.base.lead == Lead::CUTE_CHARM_F {
            buffer = 25u8.wrapping_mul((info.get_gender() / 25).wrapping_add(1));
        }

        let mut rng = PokeRNG::new_with_initial_advances(
            seed,
            self.base
                .base
                .initial_advances
                .wrapping_add(self.base.base.delay),
        );
        let mut cnt = 0;
        while cnt <= self.base.base.max_advances {
            let mut go = rng;

            if self.base.lead == Lead::CUTE_CHARM_M || self.base.lead == Lead::CUTE_CHARM_F {
                match info.get_gender() {
                    0 | 254 | 255 => cute_charm_flag = false,
                    _ => cute_charm_flag = go.next_u16_max::<false>(3) != 0,
                }
            }

            let nature = if self.base.lead <= Lead::SYNCHRONIZE_END {
                if go.next_u16_max::<false>(2) == 0 {
                    self.base.lead.bits()
                } else {
                    go.next_u16_max::<false>(25) as u8
                }
            } else {
                go.next_u16_max::<false>(25) as u8
            };

            let mut pid;
            if cute_charm_flag {
                pid = (buffer as u32).wrapping_add(nature as u32);
            } else {
                while {
                    let low = go.next_u16();
                    let high = go.next_u16();
                    pid = ((high as u32) << 16) | (low as u32);
                    (pid % 25) as u8 != nature
                } {}
            }

            let iv1 = go.next_u16();
            let iv2 = go.next_u16();
            let mut ivs = [0; 6];
            ivs[0] = (iv1 & 31) as u8;
            ivs[1] = ((iv1 >> 5) & 31) as u8;
            ivs[2] = ((iv1 >> 10) & 31) as u8;
            ivs[3] = ((iv2 >> 5) & 31) as u8;
            ivs[4] = ((iv2 >> 10) & 31) as u8;
            ivs[5] = (iv2 & 31) as u8;

            let state = GeneratorState4::new(
                rng.next_u16(),
                self.base.base.initial_advances.wrapping_add(cnt),
                pid,
                ivs,
                (pid & 1) as u8,
                get_gender(pid, info),
                static_template.get_level(),
                (pid % 25) as u8,
                get_shiny(pid, self.base.base.tsv),
                info,
            );

            if self.base.base.filter.compare_generator_state(&state) {
                states.push(state);
            }

            cnt += 1;
        }

        states
    }

    fn generate_method_k(
        &self,
        seed: u32,
        static_template: &StaticTemplate4,
    ) -> Vec<GeneratorState4> {
        let mut states = vec![];
        let info = static_template.get_info();

        let mut cute_charm_flag = false;
        let mut buffer = 0;
        if self.base.lead == Lead::CUTE_CHARM_F {
            buffer = 25u8.wrapping_mul((info.get_gender() / 25).wrapping_add(1));
        }

        let mut rng = PokeRNG::new_with_initial_advances(
            seed,
            self.base
                .base
                .initial_advances
                .wrapping_add(self.base.base.delay),
        );
        let mut cnt = 0;
        while cnt <= self.base.base.max_advances {
            let mut go = rng;

            if self.base.lead == Lead::CUTE_CHARM_M || self.base.lead == Lead::CUTE_CHARM_F {
                match info.get_gender() {
                    0 | 254 | 255 => cute_charm_flag = false,
                    _ => cute_charm_flag = go.next_u16_max::<true>(3) != 0,
                }
            }

            let nature = if self.base.lead <= Lead::SYNCHRONIZE_END {
                if go.next_u16_max::<true>(2) == 0 {
                    self.base.lead.bits()
                } else {
                    go.next_u16_max::<true>(25) as u8
                }
            } else {
                go.next_u16_max::<true>(25) as u8
            };

            let mut pid;
            if cute_charm_flag {
                pid = (buffer as u32).wrapping_add(nature as u32);
            } else {
                while {
                    let low = go.next_u16();
                    let high = go.next_u16();
                    pid = ((high as u32) << 16) | (low as u32);
                    (pid % 25) as u8 != nature
                } {}
            }

            let iv1 = go.next_u16();
            let iv2 = go.next_u16();
            let mut ivs = [0; 6];
            ivs[0] = (iv1 & 31) as u8;
            ivs[1] = ((iv1 >> 5) & 31) as u8;
            ivs[2] = ((iv1 >> 10) & 31) as u8;
            ivs[3] = ((iv2 >> 5) & 31) as u8;
            ivs[4] = ((iv2 >> 10) & 31) as u8;
            ivs[5] = (iv2 & 31) as u8;

            let state = GeneratorState4::new(
                rng.next_u16(),
                self.base.base.initial_advances.wrapping_add(cnt),
                pid,
                ivs,
                (pid & 1) as u8,
                get_gender(pid, info),
                static_template.get_level(),
                (pid % 25) as u8,
                get_shiny(pid, self.base.base.tsv),
                info,
            );

            if self.base.base.filter.compare_generator_state(&state) {
                states.push(state);
            }

            cnt += 1;
        }

        states
    }
}
