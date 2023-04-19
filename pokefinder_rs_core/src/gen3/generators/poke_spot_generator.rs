use crate::enums::Method;
use crate::gen3::filters::WildStateFilter3;
use crate::gen3::states::PokeSpotState;
use crate::gen3::Profile3;
use crate::parents::filters::{Filter, WildFilter};
use crate::parents::generators::Generator;
use crate::parents::{EncounterArea, EncounterAreaBase, PersonalInfo};
use crate::rng::XDRNG;
use std::cmp::Ordering;

const ENCOUNTER_TABLE: [u8; 100] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2,
];

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
pub struct PokeSpotGenerator<'a, 'b> {
    pub base: Generator<'a, 'b, Profile3, WildStateFilter3>,
    pub delay_encounter: u32,
    pub initial_advances_encounter: u32,
    pub max_advances_encounter: u32,
}

impl<'a, 'b> PokeSpotGenerator<'a, 'b> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        initial_advances: u32,
        max_advances: u32,
        delay: u32,
        initial_advances_encounter: u32,
        max_advances_encounter: u32,
        delay_encounter: u32,
        profile: &'a Profile3,
        filter: &'b WildStateFilter3,
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
            delay_encounter,
            initial_advances_encounter,
            max_advances_encounter,
        }
    }

    pub fn generate(
        &self,
        seed_food: u32,
        seed_encounter: u32,
        encounter_area: &EncounterArea,
    ) -> Vec<PokeSpotState> {
        let food = self.generate_food(seed_food, encounter_area);
        if !food.is_empty() {
            self.generate_encounter(seed_encounter, food, encounter_area)
        } else {
            vec![]
        }
    }

    pub fn generate_encounter(
        &self,
        seed: u32,
        food: Vec<PokeSpotState>,
        encounter_area: &EncounterArea,
    ) -> Vec<PokeSpotState> {
        let mut rng = XDRNG::new_with_initial_advances(
            seed,
            self.initial_advances_encounter + self.delay_encounter,
        );

        let mut states = vec![];
        let mut cnt = 0;
        while cnt <= self.max_advances_encounter {
            let mut go = rng;

            let level_rand = go.next_u16();

            go.advance(2);

            let iv1 = go.next_u16();
            let iv2 = go.next_u16();

            let mut ivs = [0; 6];
            ivs[0] = (iv1 & 31) as u8;
            ivs[1] = ((iv1 >> 5) & 31) as u8;
            ivs[2] = ((iv1 >> 10) & 31) as u8;
            ivs[3] = ((iv2 >> 5) & 31) as u8;
            ivs[4] = ((iv2 >> 10) & 31) as u8;
            ivs[5] = (iv2 & 31) as u8;

            let ability = go.next_u16_max::<true>(2);

            if !self.base.filter.compare_iv(ivs) {
                cnt += 1;
                rng.next();
                continue;
            }

            for mut state in food.clone() {
                let slot = &encounter_area.pokemon[state.encounter_slot as usize];
                let info = slot.info;

                let level = encounter_area
                    .calculate_level_with_prng_state(state.get_encounter_slot(), level_rand);

                state.update(
                    self.initial_advances_encounter + cnt,
                    (ability == 1 && info.get_ability(0) != info.get_ability(1)) as u8,
                    level,
                    ivs,
                    info,
                );

                if self.base.filter.compare_ability(state.base.base.ability)
                    && self
                        .base
                        .filter
                        .compare_hidden_power(state.base.base.hidden_power)
                {
                    states.push(state);
                }
            }

            states.sort_by(|left, right| {
                match left.base.advances.cmp(&right.base.advances) {
                    Ordering::Less => return Ordering::Less,
                    Ordering::Greater => return Ordering::Greater,
                    _ => {}
                }

                match left.encounter_advances.cmp(&right.encounter_advances) {
                    Ordering::Less => return Ordering::Less,
                    Ordering::Greater => return Ordering::Greater,
                    _ => {}
                }

                Ordering::Greater
            });

            cnt += 1;
            rng.next();
        }

        states
    }

    pub fn generate_food(&self, seed: u32, encounter_area: &EncounterArea) -> Vec<PokeSpotState> {
        let mut rng =
            XDRNG::new_with_initial_advances(seed, self.base.initial_advances + self.base.delay);

        let mut states = vec![];
        let mut cnt = 0;
        while cnt <= self.base.max_advances {
            let mut go = rng;

            if go.next_u16_max::<true>(3) != 0 {
                cnt += 1;
                rng.next();
                continue;
            }

            if go.next_u16_max::<true>(100) < 10 {
                cnt += 1;
                rng.next();
                continue;
            }

            let encounter_slot = ENCOUNTER_TABLE[go.next_u16_max::<true>(100) as usize];
            if !self.base.filter.compare_encounter_slot(encounter_slot) {
                cnt += 1;
                rng.next();
                continue;
            }

            let mut pid = (go.next_u16() as u32) << 16;
            pid |= go.next_u16() as u32;

            if self.base.filter.compare_nature((pid % 25) as u8) {
                let slot = encounter_area.pokemon[encounter_slot as usize];
                let info = slot.get_info();

                let state = PokeSpotState::new(
                    self.base.initial_advances + cnt,
                    pid,
                    get_gender(pid, info),
                    get_shiny(pid, self.base.tsv),
                    encounter_slot,
                    slot.get_specie(),
                    info,
                );
                if self.base.filter.compare_gender(state.base.base.gender)
                    && self.base.filter.compare_shiny(state.base.base.shiny)
                {
                    states.push(state);
                }
            }

            cnt += 1;
            rng.next();
        }

        states
    }
}
