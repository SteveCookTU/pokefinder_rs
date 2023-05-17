use crate::enums::{Game, Method};
use crate::gen4::filters::StateFilter4;
use crate::gen4::states::EggGeneratorState4;
use crate::gen4::Profile4;
use crate::parents::filters::Filter;
use crate::parents::generators::EggGenerator;
use crate::parents::personal_loader::get_personal_info;
use crate::parents::{Daycare, PersonalInfo, Profile};
use crate::rng::{PokeRNG, ARNG, MT};
use std::cmp::Ordering;

fn compare(left: &EggGeneratorState4, right: &EggGeneratorState4) -> Ordering {
    match left.base.advances.cmp(&right.base.advances) {
        Ordering::Less => return Ordering::Less,
        Ordering::Greater => return Ordering::Greater,
        _ => {}
    }

    match left.pickup_advances.cmp(&right.pickup_advances) {
        Ordering::Less => return Ordering::Less,
        Ordering::Greater => return Ordering::Greater,
        _ => {}
    }

    Ordering::Equal
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

fn set_inheritance<const DPPT: bool>(
    daycare: &Daycare,
    ivs: &mut [u8; 6],
    inheritance: &mut [u8; 6],
    inh: &[u8],
    par: &[u8],
) {
    const ORDER: [usize; 6] = [0, 1, 2, 5, 3, 4];

    if DPPT {
        const AVAILABLE1: [usize; 6] = [0, 1, 2, 3, 4, 5];
        const AVAILABLE2: [usize; 5] = [1, 2, 3, 4, 5];
        const AVAILABLE3: [usize; 4] = [1, 3, 4, 5];

        let mut stat = AVAILABLE1[inh[0] as usize];
        ivs[stat] = daycare.get_parent_iv(par[0], ORDER[stat]);
        inheritance[stat] = par[0] + 1;

        stat = AVAILABLE2[inh[1] as usize];
        ivs[stat] = daycare.get_parent_iv(par[1], ORDER[stat]);
        inheritance[stat] = par[1] + 1;

        stat = AVAILABLE3[inh[2] as usize];
        ivs[stat] = daycare.get_parent_iv(par[2], ORDER[stat]);
        inheritance[stat] = par[2] + 1;
    } else {
        let mut available: [usize; 6] = [0, 1, 2, 3, 4, 5];
        let avoid = |available: &mut [usize; 6], stat: usize, i: usize| {
            for j in stat..(5 - i) {
                available[j] = available[j + 1];
            }
        };

        let mut stat = available[inh[0] as usize];
        ivs[stat] = daycare.get_parent_iv(par[0], ORDER[stat]);
        inheritance[stat] = par[1] + 1;

        avoid(&mut available, stat, 0);

        stat = available[inh[1] as usize];
        ivs[stat] = daycare.get_parent_iv(par[1], ORDER[stat]);
        inheritance[stat] = par[1] + 1;

        avoid(&mut available, stat, 1);

        stat = available[inh[2] as usize];
        ivs[stat] = daycare.get_parent_iv(par[2], ORDER[stat]);
        inheritance[stat] = par[2] + 1;
    }
}

#[inline]
fn get_male(version: Game, species: u16) -> Option<&'static PersonalInfo> {
    match species {
        29 => Some(get_personal_info(version, 32, 0)),
        314 => Some(get_personal_info(version, 313, 0)),
        _ => None,
    }
}

/// Egg encounter generator for Gen4
#[derive(Clone)]
pub struct EggGenerator4<'a, 'b, 'c> {
    /// Base egg generator data
    pub base: EggGenerator<'a, 'b, 'c, Profile4, StateFilter4>,
    /// Number of pickup advances to offset
    pub delay_pickup: u32,
    /// Initial number of pickup advances
    pub initial_advances_pickup: u32,
    /// Maximum number of pickup advances
    pub max_advances_pickup: u32,
}

impl<'a, 'b, 'c> EggGenerator4<'a, 'b, 'c> {
    /// Construct a new [`EggGenerator4`] struct
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        initial_advances: u32,
        max_advances: u32,
        delay: u32,
        initial_advances_pickup: u32,
        max_advances_pickup: u32,
        delay_pickup: u32,
        daycare: &'c Daycare,
        profile: &'a Profile4,
        filter: &'b StateFilter4,
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
            delay_pickup,
            initial_advances_pickup,
            max_advances_pickup,
        }
    }

    /// Generates [`EggGeneratorState4`] states
    pub fn generate(&self, seed_held: u32, seed_pickup: u32) -> Vec<EggGeneratorState4> {
        let held = self.generate_held(seed_held);
        if !held.is_empty() {
            self.generate_pickup(seed_pickup, held)
        } else {
            held
        }
    }

    fn generate_held(&self, seed: u32) -> Vec<EggGeneratorState4> {
        let base = get_personal_info(
            self.base.base.profile.get_version(),
            self.base.daycare.get_egg_species(),
            0,
        );
        let male = get_male(
            self.base.base.profile.get_version(),
            self.base.daycare.get_egg_species(),
        );

        let mut mt = MT::new_with_initial_advances(
            seed,
            self.base
                .base
                .initial_advances
                .wrapping_add(self.base.base.delay),
        );

        let mut states = vec![];
        let mut cnt = 0;
        while cnt <= self.base.base.max_advances {
            let mut pid = mt.next();
            if self.base.daycare.get_masuda() {
                let mut rng = ARNG::new(pid);
                for _ in 0..4 {
                    if get_shiny(pid, self.base.base.tsv) != 0 {
                        break;
                    }
                    pid = rng.next();
                }
            }

            let mut info = base;

            if let Some(male) = male {
                if (pid & 0x8000) != 0 {
                    info = male;
                }
            }

            let state = EggGeneratorState4::new(
                self.base.base.initial_advances.wrapping_add(cnt),
                pid,
                get_gender(pid, info),
                get_shiny(pid, self.base.base.tsv),
                info,
            );
            if self
                .base
                .base
                .filter
                .compare_ability(state.base.base.base.ability)
                && self
                    .base
                    .base
                    .filter
                    .compare_gender(state.base.base.base.gender)
                && self
                    .base
                    .base
                    .filter
                    .compare_nature(state.base.base.base.nature)
                && self
                    .base
                    .base
                    .filter
                    .compare_shiny(state.base.base.base.shiny)
            {
                states.push(state);
            }

            cnt += 1;
        }

        states
    }

    fn generate_pickup(&self, seed: u32, held: Vec<EggGeneratorState4>) -> Vec<EggGeneratorState4> {
        let base = get_personal_info(
            self.base.base.profile.get_version(),
            self.base.daycare.get_egg_species(),
            0,
        );
        let male = get_male(
            self.base.base.profile.get_version(),
            self.base.daycare.get_egg_species(),
        );

        let mut rng = PokeRNG::new_with_initial_advances(
            seed,
            self.base
                .base
                .initial_advances
                .wrapping_add(self.base.base.delay),
        );

        let mut states = vec![];
        let mut cnt = 0;
        while cnt <= self.base.base.max_advances {
            let mut go = PokeRNG::new(rng.seed);

            let iv1 = go.next_u16();
            let iv2 = go.next_u16();

            let mut ivs = [0; 6];
            ivs[0] = (iv1 & 31) as u8;
            ivs[1] = ((iv1 >> 5) & 31) as u8;
            ivs[2] = ((iv1 >> 10) & 31) as u8;
            ivs[3] = ((iv2 >> 5) & 31) as u8;
            ivs[4] = ((iv2 >> 10) & 31) as u8;
            ivs[5] = (iv2 & 31) as u8;

            let mut inh = [0; 3];
            inh[0] = go.next_u16_max::<true>(6) as u8;
            inh[1] = go.next_u16_max::<true>(5) as u8;
            inh[2] = go.next_u16_max::<true>(4) as u8;

            let mut par = [0; 3];
            par[0] = go.next_u16_max::<true>(2) as u8;
            par[1] = go.next_u16_max::<true>(2) as u8;
            par[2] = go.next_u16_max::<true>(2) as u8;

            let mut inheritance = [0; 6];
            if (self.base.base.profile.get_version() & Game::DPPT) != Game::NONE {
                set_inheritance::<true>(self.base.daycare, &mut ivs, &mut inheritance, &inh, &par);
            } else {
                set_inheritance::<false>(self.base.daycare, &mut ivs, &mut inheritance, &inh, &par);
            }

            let prng = rng.next_u16();

            for mut state in held.iter().copied() {
                let mut info = base;
                if let Some(male) = male {
                    if (state.base.base.base.pid & 0x8000) != 0 {
                        info = male;
                    }
                }
                state.update(
                    prng,
                    self.base.base.initial_advances.wrapping_add(cnt),
                    ivs,
                    inheritance,
                    info,
                );
                if self.base.base.filter.compare_egg_state(&state) {
                    states.push(state);
                }
            }

            cnt += 1;
        }

        states.sort_by(compare);

        states
    }
}
