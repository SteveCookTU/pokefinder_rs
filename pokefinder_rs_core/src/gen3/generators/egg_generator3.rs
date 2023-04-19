use crate::enums::{Game, Method};
use crate::gen3::filters::StateFilter3;
use crate::gen3::states::EggState3;
use crate::gen3::Profile3;
use crate::parents::filters::{Filter, StateFilter};
use crate::parents::generators::EggGenerator;
use crate::parents::personal_loader;
use crate::parents::{Daycare, PersonalInfo, Profile};
use crate::rng::PokeRNG;
use std::cmp::Ordering;

fn compare(left: &EggState3, right: &EggState3) -> Ordering {
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
        i => ((pid as u8) < i) as u8,
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

fn set_inheritance<const EMERALD: bool>(
    daycare: &Daycare,
    ivs: &mut [u8; 6],
    inheritance: &mut [u8; 6],
    inh: &[u8],
    par: &[u8],
) {
    const ORDER: [usize; 6] = [0, 1, 2, 5, 3, 4];

    if EMERALD {
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
fn get_male(version: Game, specie: u16) -> Option<&'static PersonalInfo> {
    match specie {
        29 => Some(personal_loader::get_personal_info(version, 32, 0)),
        314 => Some(personal_loader::get_personal_info(version, 313, 0)),
        _ => None,
    }
}

#[derive(Clone)]
pub struct EggGenerator3<'a, 'b, 'c> {
    pub base: EggGenerator<'a, 'b, 'c, Profile3, StateFilter3>,
    delay_pickup: u32,
    initial_advances_pickup: u32,
    max_advances_pickup: u32,
    calibration: u8,
    inh: u8,
    iv1: u8,
    iv2: u8,
    max_redraw: u8,
    min_redraw: u8,
}

impl<'a, 'b, 'c> EggGenerator3<'a, 'b, 'c> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        initial_advances: u32,
        max_advances: u32,
        delay: u32,
        initial_advances_pickup: u32,
        max_advances_pickup: u32,
        delay_pickup: u32,
        calibration: u8,
        min_redraw: u8,
        max_redraw: u8,
        method: Method,
        compatability: u8,
        daycare: &'c Daycare,
        profile: &'a Profile3,
        filter: &'b StateFilter3,
    ) -> Self {
        let (iv1, iv2, inh) = match method {
            Method::EBred => (0, 0, 1),
            Method::EBredSplit => (0, 1, 1),
            Method::EBredAlternate => (0, 0, 2),
            Method::RSFRLGBred => (1, 0, 1),
            Method::RSFRLGBredSplit => (0, 1, 1),
            Method::RSFRLGBredAlternate => (1, 0, 2),
            Method::RSFRLGBredMixed => (0, 0, 2),
            _ => (0, 0, 0),
        };

        Self {
            base: EggGenerator::new(
                initial_advances,
                max_advances,
                delay,
                method,
                compatability,
                daycare,
                profile,
                filter,
            ),
            delay_pickup,
            initial_advances_pickup,
            max_advances_pickup,
            calibration,
            inh,
            iv1,
            iv2,
            max_redraw,
            min_redraw,
        }
    }

    pub fn generate(&self, seed_held: u32, seed_pickup: u32) -> Vec<EggState3> {
        match self.base.base.method {
            Method::EBred | Method::EBredSplit | Method::EBredAlternate => {
                let held = self.generate_emerald_held();
                if held.is_empty() {
                    held
                } else {
                    self.generate_emerald_pickup(held)
                }
            }
            Method::RSFRLGBredSplit
            | Method::RSFRLGBred
            | Method::RSFRLGBredAlternate
            | Method::RSFRLGBredMixed => {
                let held = self.generate_rsfrlg_held(seed_held);
                println!("{}", held.len());
                if held.is_empty() {
                    held
                } else {
                    self.generate_rsfrlg_pickup(seed_pickup, held)
                }
            }
            _ => Vec::new(),
        }
    }

    pub fn generate_emerald_held(&self) -> Vec<EggState3> {
        let base = personal_loader::get_personal_info(
            self.base.base.profile.get_version(),
            self.base.daycare.get_egg_specie(),
            0,
        );
        let male = get_male(
            self.base.base.profile.get_version(),
            self.base.daycare.get_egg_specie(),
        );

        let mut parent = 0;
        for i in 0..2 {
            if self.base.daycare.get_parent_gender(i) == 1 {
                parent = i;
            }
        }

        for i in 0..2 {
            if self.base.daycare.get_parent_gender(i) == 3 {
                parent = i;
            }
        }

        let everstone = self.base.daycare.get_parent_item(parent) == 1;

        let mut rng = PokeRNG::new_with_initial_advances(
            0,
            self.base.base.initial_advances + self.base.base.delay,
        );
        let mut val = self.base.base.initial_advances + self.base.base.delay + 1;

        let mut states = Vec::new();
        let mut cnt = 0;
        while cnt <= self.base.base.max_advances {
            if ((rng.next_u16() as u32).wrapping_mul(100) / 0xFFFF) as u8 >= self.base.compatability
            {
                cnt += 1;
                val += 1;
                continue;
            }

            'redraw: for redraw in self.min_redraw..=self.max_redraw {
                let mut go = rng;

                let offset =
                    (self.calibration as u32).wrapping_add(3u32.wrapping_mul(redraw as u32));
                let flag = if everstone {
                    (go.next_u16() >> 15) == 0
                } else {
                    false
                };
                let mut trng = PokeRNG::new(val.wrapping_sub(offset) & 0xFFFF);
                let pid = if !flag {
                    (go.next_u16_max::<true>(0xFFFE) as u32 + 1) | (trng.next() & 0xFFFF0000)
                } else {
                    let mut pid = go.next_u16() as u32 | (trng.next() & 0xFFFF0000);
                    let mut i = 1;
                    while (pid % 25) as u8 != self.base.daycare.get_parent_nature(parent) {
                        if i == 17 {
                            break;
                        }
                        i += 1;
                        pid = go.next_u16() as u32 | (trng.next() & 0xFFFF0000);
                    }

                    if i == 17 {
                        continue 'redraw;
                    }
                    pid
                };

                let info = if let Some(male) = male {
                    if pid & 0x8000 > 0 {
                        male
                    } else {
                        base
                    }
                } else {
                    base
                };

                let state = EggState3::new_from_pid(
                    self.base
                        .base
                        .initial_advances
                        .wrapping_add(cnt)
                        .wrapping_sub(offset),
                    redraw,
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
                {
                    states.push(state);
                }
            }

            cnt += 1;
            val += 1;
        }

        states
    }

    pub fn generate_emerald_pickup(&self, held: Vec<EggState3>) -> Vec<EggState3> {
        let base = personal_loader::get_personal_info(
            self.base.base.profile.get_version(),
            self.base.daycare.get_egg_specie(),
            0,
        );
        let male = get_male(
            self.base.base.profile.get_version(),
            self.base.daycare.get_egg_specie(),
        );

        let mut rng =
            PokeRNG::new_with_initial_advances(0, self.initial_advances_pickup + self.delay_pickup);

        let mut states = Vec::new();
        let mut cnt = 0;
        while cnt <= self.max_advances_pickup {
            let mut go = rng;

            go.advance(self.iv1 as u32);
            let iv1 = go.next_u16();
            go.advance(self.iv2 as u32);
            let iv2 = go.next_u16();

            let mut ivs = [0; 6];
            ivs[0] = (iv1 & 31) as u8;
            ivs[1] = ((iv1 >> 5) & 31) as u8;
            ivs[2] = ((iv1 >> 10) & 31) as u8;
            ivs[3] = ((iv2 >> 5) & 31) as u8;
            ivs[4] = ((iv2 >> 10) & 31) as u8;
            ivs[5] = (iv2 & 31) as u8;

            go.advance(self.inh as u32);
            let mut inh = [0; 3];
            inh[0] = go.next_u16_max::<true>(6) as u8;
            inh[1] = go.next_u16_max::<true>(5) as u8;
            inh[2] = go.next_u16_max::<true>(4) as u8;

            let mut par = [0; 3];
            par[0] = go.next_u16_max::<true>(2) as u8;
            par[1] = go.next_u16_max::<true>(2) as u8;
            par[2] = go.next_u16_max::<true>(2) as u8;

            let mut inheritance = [0; 6];
            set_inheritance::<true>(self.base.daycare, &mut ivs, &mut inheritance, &inh, &par);

            for state in held.iter() {
                let info = if let Some(male) = male {
                    if state.base.base.base.pid & 0x8000 > 0 {
                        male
                    } else {
                        base
                    }
                } else {
                    base
                };
                let mut state = *state;
                state.update(self.initial_advances_pickup + cnt, ivs, inheritance, info);
                if self.base.base.filter.compare_egg_state(&state) {
                    states.push(state);
                }
            }

            cnt += 1;
            rng.next();
        }

        states.sort_by(compare);

        states
    }

    pub fn generate_rsfrlg_held(&self, seed: u32) -> Vec<EggState3> {
        let base = personal_loader::get_personal_info(
            self.base.base.profile.get_version(),
            self.base.daycare.get_egg_specie(),
            0,
        );
        let male = get_male(
            self.base.base.profile.get_version(),
            self.base.daycare.get_egg_specie(),
        );

        let mut rng = PokeRNG::new_with_initial_advances(seed, self.base.base.initial_advances);

        let mut states = Vec::new();
        let mut cnt = 0;
        while cnt <= self.base.base.max_advances {
            let mut go = rng;
            if (((go.next_u16() as u32).wrapping_mul(100) / 0xFFFF) as u8) < self.base.compatability
            {
                let pid = go.next_u16_max::<true>(0xFFFE) + 1;
                let info = if let Some(male) = male {
                    if pid & 0x8000 > 0 {
                        male
                    } else {
                        base
                    }
                } else {
                    base
                };

                let state = EggState3::new_from_low(
                    self.base.base.initial_advances + cnt,
                    pid,
                    get_gender(pid as u32, info),
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
                {
                    states.push(state);
                }
            }
            cnt += 1;
            rng.next();
        }

        states
    }

    pub fn generate_rsfrlg_pickup(&self, seed: u32, held: Vec<EggState3>) -> Vec<EggState3> {
        let base = personal_loader::get_personal_info(
            self.base.base.profile.get_version(),
            self.base.daycare.get_egg_specie(),
            0,
        );
        let male = get_male(
            self.base.base.profile.get_version(),
            self.base.daycare.get_egg_specie(),
        );

        let mut rng = PokeRNG::new_with_initial_advances(
            seed,
            self.initial_advances_pickup + self.delay_pickup,
        );

        let mut states = Vec::new();
        let mut cnt = 0;
        while cnt <= self.max_advances_pickup {
            let mut go = rng;

            let high = (go.next_u16() as u32) << 16;

            go.advance(self.iv1 as u32);
            let iv1 = go.next_u16();
            go.advance(self.iv2 as u32);
            let iv2 = go.next_u16();

            let mut ivs = [0; 6];
            ivs[0] = (iv1 & 31) as u8;
            ivs[1] = ((iv1 >> 5) & 31) as u8;
            ivs[2] = ((iv1 >> 10) & 31) as u8;
            ivs[3] = ((iv2 >> 5) & 31) as u8;
            ivs[4] = ((iv2 >> 10) & 31) as u8;
            ivs[5] = (iv2 & 31) as u8;

            go.advance(self.inh as u32);
            let mut inh = [0; 3];
            inh[0] = go.next_u16_max::<true>(6) as u8;
            inh[1] = go.next_u16_max::<true>(5) as u8;
            inh[2] = go.next_u16_max::<true>(4) as u8;

            let mut par = [0; 3];
            par[0] = go.next_u16_max::<true>(2) as u8;
            par[1] = go.next_u16_max::<true>(2) as u8;
            par[2] = go.next_u16_max::<true>(2) as u8;

            let mut inheritance = [0; 6];
            set_inheritance::<false>(self.base.daycare, &mut ivs, &mut inheritance, &inh, &par);

            for state in held.iter() {
                let pid = high | state.base.base.base.pid;
                let info = if let Some(male) = male {
                    if pid & 0x8000 > 0 {
                        male
                    } else {
                        base
                    }
                } else {
                    base
                };
                let mut state = *state;
                state.update_pid(
                    self.initial_advances_pickup + cnt,
                    pid,
                    get_shiny(pid, self.base.base.tsv),
                    ivs,
                    inheritance,
                    info,
                );
                if self.base.base.filter.compare_egg_state(&state) {
                    states.push(state);
                }
            }

            cnt += 1;
            rng.next();
        }

        states.sort_by(compare);

        states
    }
}
