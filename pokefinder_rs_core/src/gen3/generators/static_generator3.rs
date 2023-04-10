use crate::enums::{Lead, Method};
use crate::gen3::filters::StateFilter3;
use crate::gen3::Profile3;
use crate::parents::generators::StaticGenerator;
use crate::parents::states::GeneratorState;
use crate::parents::{PersonalInfo, StaticTemplate, Template};
use crate::rng::PokeRNG;

fn get_gender(pid: u32, info: &PersonalInfo) -> u8 {
    match info.get_gender() {
        255 => 2,
        254 => 1,
        0 => 0,
        i => (((pid & 255) as u8) < i) as u8,
    }
}

fn get_shiny(pid: u32, tsv: u16) -> u8 {
    let psv = ((pid >> 16) as u16) ^ ((pid & 0xFFFF) as u16);
    if tsv == psv {
        2
    } else if (tsv ^ psv) < 8 {
        1
    } else {
        0
    }
}

#[derive(Clone)]
pub struct StaticGenerator3<'a, 'b> {
    pub base: StaticGenerator<'a, 'b, Profile3, StateFilter3>,
}

impl<'a, 'b> StaticGenerator3<'a, 'b> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        initial_advances: u32,
        max_advances: u32,
        delay: u32,
        method: Method,
        profile: &'a Profile3,
        filter: &'b StateFilter3,
    ) -> Self {
        Self {
            base: StaticGenerator::new(
                initial_advances,
                max_advances,
                delay,
                method,
                Lead::NONE,
                profile,
                filter,
            ),
        }
    }

    pub fn generate(&self, seed: u32, static_template: &StaticTemplate) -> Vec<GeneratorState> {
        let mut states = vec![];
        let info = static_template.get_info();

        let mut rng = PokeRNG::new_with_initial_advances(
            seed,
            self.base.base.initial_advances + self.base.base.delay,
        );
        let mut cnt = 0;
        while cnt <= self.base.base.max_advances {
            let mut go = rng;

            let mut pid = go.next_u16() as u32;
            pid |= (go.next_u16() as u32) << 16;

            let iv1 = go.next_u16();
            if self.base.base.method == Method::Method4 {
                go.next();
            }
            let iv2 = go.next_u16();

            let mut ivs = [0; 6];
            ivs[0] = (iv1 & 31) as u8;
            ivs[1] = ((iv1 >> 5) & 31) as u8;
            ivs[2] = ((iv1 >> 10) & 31) as u8;
            ivs[3] = ((iv2 >> 5) & 31) as u8;
            ivs[4] = ((iv2 >> 10) & 31) as u8;
            ivs[5] = (iv2 & 31) as u8;

            let state = GeneratorState::new(
                self.base.base.initial_advances + cnt,
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
            rng.next();
        }

        states
    }
}
