use crate::enums::{Encounter, Game, Lead, Method};
use crate::gen3::filters::WildStateFilter3;
use crate::gen3::{EncounterArea3, Profile3};
use crate::parents::filters::{Filter, WildFilter};
use crate::parents::generators::WildGenerator;
use crate::parents::states::WildGeneratorState;
use crate::parents::{EncounterAreaBase, PersonalInfo, Profile};
use crate::rng::PokeRNG;
use crate::util::encounter_slot;

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

fn unown_check(pid: u32, form: u8) -> bool {
    let letter =
        ((((pid & 0x3000000) >> 18) | ((pid & 0x30000) >> 12) | ((pid & 0x300) >> 6) | (pid & 0x3))
            % 0x1C) as u8;
    letter == form
}

#[derive(Clone)]
pub struct WildGenerator3<'a, 'b> {
    pub base: WildGenerator<'a, 'b, Profile3, WildStateFilter3>,
}

impl<'a, 'b> WildGenerator3<'a, 'b> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        initial_advances: u32,
        max_advances: u32,
        delay: u32,
        method: Method,
        encounter: Encounter,
        lead: Lead,
        profile: &'a Profile3,
        filter: &'b WildStateFilter3,
    ) -> Self {
        Self {
            base: WildGenerator::new(
                initial_advances,
                max_advances,
                delay,
                method,
                encounter,
                lead,
                profile,
                filter,
            ),
        }
    }

    pub fn generate(&self, seed: u32, encounter_area: &EncounterArea3) -> Vec<WildGeneratorState> {
        let mut states = vec![];

        let modified_slots = encounter_area.get_slots_by_lead(self.base.lead);
        let rate = encounter_area.get_rate() as u16 * 16;
        let safari = encounter_area.safari_zone(self.base.base.profile.get_version());
        let tanoby = encounter_area.tanoby_chamber(self.base.base.profile.get_version());
        let rse = (self.base.base.profile.get_version() & Game::RSE) != Game::NONE;

        let mut cute_charm = false;
        let cute_charm_check = |info: &PersonalInfo, pid: u32| {
            if self.base.lead == Lead::CUTE_CHARM_F {
                ((pid & 0xFF) as u8) >= info.get_gender()
            } else {
                ((pid & 0xFF) as u8) < info.get_gender()
            }
        };
        let mut rng = PokeRNG::new_with_initial_advances(
            seed,
            self.base.base.initial_advances + self.base.base.delay,
        );
        let mut cnt = 0;
        while cnt <= self.base.base.max_advances {
            let mut go = rng;

            if rse
                && self.base.encounter == Encounter::RockSmash
                && go.next_u16_max::<true>(2880) >= rate
            {
                cnt += 1;
                rng.next();
                continue;
            }

            let encounter_slot = if matches!(self.base.lead, Lead::MAGNET_PULL | Lead::STATIC)
                && go.next_u16_max::<true>(2) == 0
                && !modified_slots.is_empty()
            {
                modified_slots[go.next_u16_max::<true>(modified_slots.len() as u16) as usize]
            } else {
                encounter_slot::h_slot(go.next_u16_max::<true>(100) as u8, self.base.encounter)
            };

            if !self.base.base.filter.compare_encounter_slot(encounter_slot) {
                cnt += 1;
                rng.next();
                continue;
            }

            let level = encounter_area.calculate_level_with_rng(
                encounter_slot,
                &mut go,
                self.base.lead == Lead::PRESSURE,
            );

            let slot = encounter_area.get_specific_pokemon(encounter_slot as usize);
            let info = slot.get_info();
            if self.base.lead == Lead::CUTE_CHARM_F || self.base.lead == Lead::CUTE_CHARM_M {
                match info.get_gender() {
                    0 | 254 | 255 => cute_charm = false,
                    _ => cute_charm = go.next_u16_max::<true>(3) != 0,
                }
            }

            if safari {
                go.next();
            }

            let nature = if self.base.lead.bits() <= Lead::SYNCHRONIZE_END.bits() {
                if go.next_u16_max::<true>(2) == 0 {
                    self.base.lead.bits()
                } else {
                    go.next_u16_max::<true>(25) as u8
                }
            } else {
                go.next_u16_max::<true>(25) as u8
            };

            if !self.base.base.filter.compare_nature(nature) {
                cnt += 1;
                rng.next();
                continue;
            }

            let mut pid;
            while {
                let low = go.next_u16() as u32;
                let high = go.next_u16() as u32;
                if tanoby {
                    pid = (low << 16) | high;
                } else {
                    pid = (high << 16) | low;
                }
                ((pid % 25) as u8) != nature
                    || (cute_charm && !cute_charm_check(info, pid))
                    || (slot.specie == 201 && !unown_check(pid, slot.get_form()))
            } {}

            if self.base.base.method == Method::Method2 {
                go.next();
            }
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

            let state = WildGeneratorState::new(
                self.base.base.initial_advances + cnt,
                pid,
                ivs,
                (pid & 1) as u8,
                get_gender(pid, info),
                level,
                nature,
                get_shiny(pid, self.base.base.tsv),
                encounter_slot,
                0,
                slot.get_specie(),
                slot.get_form(),
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
