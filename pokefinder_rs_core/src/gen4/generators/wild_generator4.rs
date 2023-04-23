use crate::enums::{Encounter, Lead, Method};
use crate::gen4::filters::WildStateFilter4;
use crate::gen4::states::WildGeneratorState4;
use crate::gen4::{EncounterArea4, Profile4};
use crate::parents::filters::{Filter, WildFilter};
use crate::parents::generators::WildGenerator;
use crate::parents::{EncounterAreaT, PersonalInfo, Profile};
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

fn get_item(rand: u8, lead: Lead, info: &PersonalInfo) -> u16 {
    const ITEM_TABLE_RANGE: [[u8; 2]; 2] = [[45, 95], [20, 80]];

    if info.get_item(0) == info.get_item(1) && info.get_item(0) != 0 {
        info.get_item(0)
    } else if rand < ITEM_TABLE_RANGE[if lead == Lead::COMPOUND_EYES { 1 } else { 0 }][0] {
        0
    } else if rand < ITEM_TABLE_RANGE[if lead == Lead::COMPOUND_EYES { 1 } else { 0 }][1] {
        info.get_item(0)
    } else {
        info.get_item(1)
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

#[derive(Clone)]
pub struct WildGenerator4<'a, 'b> {
    pub base: WildGenerator<'a, 'b, Profile4, WildStateFilter4>,
    pub shiny: bool,
}

impl<'a, 'b> WildGenerator4<'a, 'b> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        initial_advances: u32,
        max_advances: u32,
        delay: u32,
        method: Method,
        encounter: Encounter,
        lead: Lead,
        shiny: bool,
        profile: &'a Profile4,
        filter: &'b WildStateFilter4,
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
            shiny,
        }
    }

    pub fn generate(
        &self,
        seed: u32,
        encounter_area: &EncounterArea4,
        index: u8,
    ) -> Vec<WildGeneratorState4> {
        match self.base.base.method {
            Method::MethodJ => self.generate_method_j(seed, encounter_area),
            Method::MethodK => self.generate_method_k(seed, encounter_area),
            Method::PokeRadar if self.shiny => {
                self.generate_poke_radar_shiny(seed, encounter_area, index)
            }
            Method::PokeRadar if !self.shiny => {
                self.generate_poke_radar(seed, encounter_area, index)
            }
            _ => vec![],
        }
    }

    fn generate_method_j(
        &self,
        seed: u32,
        encounter_area: &EncounterArea4,
    ) -> Vec<WildGeneratorState4> {
        let mut states = vec![];

        let thresh = encounter_area.get_rate();
        let modified_slots = encounter_area.get_slots_by_lead(self.base.lead);

        let mut rng = PokeRNG::new_with_initial_advances(
            seed,
            self.base
                .base
                .initial_advances
                .wrapping_add(self.base.base.delay),
        );
        let mut cnt = 0;
        while cnt <= self.base.base.max_advances {
            let mut occidentary = self.base.base.initial_advances.wrapping_add(cnt);
            let mut go = rng;

            if matches!(
                self.base.encounter,
                Encounter::OldRod | Encounter::GoodRod | Encounter::SuperRod
            ) && {
                occidentary = occidentary.wrapping_add(1);
                (go.next_u16_max::<false>(100) as u8) >= thresh
            } {
                rng.next();
                cnt += 1;
                continue;
            }

            let encounter_slot = if matches!(self.base.lead, Lead::MAGNET_PULL | Lead::STATIC)
                && {
                    occidentary = occidentary.wrapping_add(1);
                    (go.next_u16_max::<false>(2) as u8) == 0
                }
                && !modified_slots.is_empty()
            {
                occidentary = occidentary.wrapping_add(1);
                modified_slots[go.next_u16_max::<true>(modified_slots.len() as u16) as usize]
            } else {
                occidentary = occidentary.wrapping_add(1);
                encounter_slot::j_slot(go.next_u16_max::<false>(100) as u8, self.base.encounter)
            };

            if !self.base.base.filter.compare_encounter_slot(encounter_slot) {
                rng.next();
                cnt += 1;
                continue;
            }

            let level = if self.base.encounter == Encounter::Grass {
                encounter_area.calculate_level_with_rng::<false, false>(
                    encounter_slot,
                    &mut go,
                    &mut occidentary,
                    self.base.lead == Lead::PRESSURE,
                )
            } else {
                encounter_area.calculate_level_with_rng::<true, false>(
                    encounter_slot,
                    &mut go,
                    &mut occidentary,
                    self.base.lead == Lead::PRESSURE,
                )
            };

            let slot = encounter_area.get_specific_pokemon(encounter_slot as usize);
            let info = slot.get_info();

            let mut cute_charm_flag = false;
            let mut buffer = 0;
            if self.base.lead == Lead::CUTE_CHARM_M || self.base.lead == Lead::CUTE_CHARM_F {
                match info.get_gender() {
                    0 | 254 | 255 => cute_charm_flag = false,
                    _ => {
                        occidentary = occidentary.wrapping_add(1);
                        cute_charm_flag = go.next_u16_max::<false>(3) != 0;
                        buffer = 25u8.wrapping_mul((info.get_gender() / 25).wrapping_add(1));
                    }
                }
            }

            let nature = if self.base.lead <= Lead::SYNCHRONIZE_END {
                let sync = {
                    occidentary = occidentary.wrapping_add(1);
                    go.next_u16_max::<false>(2) == 0
                };
                if sync {
                    self.base.lead.bits()
                } else {
                    occidentary = occidentary.wrapping_add(1);
                    go.next_u16_max::<false>(25) as u8
                }
            } else {
                occidentary = occidentary.wrapping_add(1);
                go.next_u16_max::<false>(25) as u8
            };

            if !self.base.base.filter.compare_nature(nature) {
                rng.next();
                cnt += 1;
                continue;
            }

            let mut pid;
            if cute_charm_flag {
                pid = (buffer as u32).wrapping_add(nature as u32);
            } else {
                while {
                    occidentary = occidentary.wrapping_add(1);
                    let low = go.next_u16() as u32;
                    occidentary = occidentary.wrapping_add(1);
                    let high = go.next_u16() as u32;
                    pid = (high << 16) | low;
                    (pid % 25) as u8 != nature
                } {}
            }

            occidentary = occidentary.wrapping_add(1);
            let iv1 = go.next_u16();
            occidentary = occidentary.wrapping_add(1);
            let iv2 = go.next_u16();
            let mut ivs = [0; 6];
            ivs[0] = (iv1 & 31) as u8;
            ivs[1] = ((iv1 >> 5) & 31) as u8;
            ivs[2] = ((iv1 >> 10) & 31) as u8;
            ivs[3] = ((iv2 >> 5) & 31) as u8;
            ivs[4] = ((iv2 >> 10) & 31) as u8;
            ivs[5] = (iv2 & 31) as u8;

            occidentary = occidentary.wrapping_add(1);
            let item = get_item(go.next_u16_max::<true>(100) as u8, self.base.lead, info);

            let mut form = 0;
            if slot.get_specie() == 201 {
                form = encounter_area.unown_form(go.next_u16());
            }

            let state = WildGeneratorState4::new(
                rng.next_u16(),
                occidentary,
                self.base.base.initial_advances.wrapping_add(cnt),
                pid,
                ivs,
                (pid & 1) as u8,
                get_gender(pid, info),
                level,
                nature,
                get_shiny(pid, self.base.base.tsv),
                encounter_slot,
                item,
                slot.get_specie(),
                form,
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
        encounter_area: &EncounterArea4,
    ) -> Vec<WildGeneratorState4> {
        let mut states = vec![];

        let mut rate = encounter_area.get_rate() as u16;
        if (self.base.lead == Lead::SUCTION_CUPS
            && matches!(
                self.base.encounter,
                Encounter::OldRod | Encounter::GoodRod | Encounter::SuperRod
            ))
            || (self.base.lead == Lead::ARENA_TRAP && self.base.encounter == Encounter::RockSmash)
        {
            rate *= 2;
        }
        let modified_slots = encounter_area.get_slots_by_lead(self.base.lead);
        let safari = encounter_area.safari_zone(self.base.base.profile.get_version());

        let mut rng = PokeRNG::new_with_initial_advances(
            seed,
            self.base
                .base
                .initial_advances
                .wrapping_add(self.base.base.delay),
        );
        let mut cnt = 0;
        while cnt <= self.base.base.max_advances {
            let mut occidentary = self.base.base.initial_advances.wrapping_add(cnt);
            let mut go = rng;

            if matches!(
                self.base.encounter,
                Encounter::RockSmash | Encounter::OldRod | Encounter::GoodRod | Encounter::SuperRod
            ) && {
                occidentary = occidentary.wrapping_add(1);
                go.next_u16_max::<true>(100) >= rate
            } {
                rng.next();
                cnt += 1;
                continue;
            }

            let encounter_slot = if matches!(self.base.lead, Lead::MAGNET_PULL | Lead::STATIC)
                && {
                    occidentary = occidentary.wrapping_add(1);
                    (go.next_u16_max::<true>(2) as u8) == 0
                }
                && !modified_slots.is_empty()
            {
                occidentary = occidentary.wrapping_add(1);
                modified_slots[go.next_u16_max::<true>(modified_slots.len() as u16) as usize]
            } else if safari {
                occidentary = occidentary.wrapping_add(1);
                go.next_u16_max::<true>(10) as u8
            } else {
                occidentary = occidentary.wrapping_add(1);
                encounter_slot::k_slot(go.next_u16_max::<true>(100) as u8, self.base.encounter)
            };

            if !self.base.base.filter.compare_encounter_slot(encounter_slot) {
                rng.next();
                cnt += 1;
                continue;
            }

            let level = if self.base.encounter == Encounter::Grass || safari {
                encounter_area.calculate_level_with_rng::<false, true>(
                    encounter_slot,
                    &mut go,
                    &mut occidentary,
                    self.base.lead == Lead::PRESSURE,
                )
            } else {
                encounter_area.calculate_level_with_rng::<true, true>(
                    encounter_slot,
                    &mut go,
                    &mut occidentary,
                    self.base.lead == Lead::PRESSURE,
                )
            };

            let slot = encounter_area.get_specific_pokemon(encounter_slot as usize);
            let info = slot.get_info();

            let mut cute_charm_flag = false;
            let mut buffer = 0;
            if self.base.lead == Lead::CUTE_CHARM_M || self.base.lead == Lead::CUTE_CHARM_F {
                match info.get_gender() {
                    0 | 254 | 255 => cute_charm_flag = false,
                    _ => {
                        occidentary = occidentary.wrapping_add(1);
                        cute_charm_flag = go.next_u16_max::<true>(3) != 0;
                        buffer = 25u8.wrapping_mul((info.get_gender() / 25).wrapping_add(1));
                    }
                }
            }

            let mut pid = 0;
            let mut nature = 0;
            let mut iv1 = 0;
            let mut iv2 = 0;
            if cute_charm_flag {
                occidentary = occidentary.wrapping_add(1);
                nature = go.next_u16_max::<true>(25) as u8;
                if !self.base.base.filter.compare_nature(nature) {
                    rng.next();
                    cnt += 1;
                    continue;
                }

                pid = (buffer as u32).wrapping_add(nature as u32);
                occidentary = occidentary.wrapping_add(1);
                iv1 = go.next_u16();
                occidentary = occidentary.wrapping_add(1);
                iv2 = go.next_u16();
            } else if self.base.encounter == Encounter::BugCatchingContest || safari {
                'outer: for _ in 0..4 {
                    if self.base.lead <= Lead::SYNCHRONIZE_END {
                        let sync = {
                            occidentary = occidentary.wrapping_add(1);
                            (go.next_u16_max::<true>(2) as u8) == 0
                        };
                        nature = if sync {
                            self.base.lead.bits()
                        } else {
                            occidentary = occidentary.wrapping_add(1);
                            go.next_u16_max::<true>(25) as u8
                        };
                    } else {
                        occidentary = occidentary.wrapping_add(1);
                        nature = go.next_u16_max::<true>(25) as u8;
                    }

                    while {
                        occidentary = occidentary.wrapping_add(1);
                        let low = go.next_u16() as u32;
                        occidentary = occidentary.wrapping_add(1);
                        let high = go.next_u16() as u32;
                        pid = (high << 16) | low;
                        (pid % 25) as u8 != nature
                    } {}

                    occidentary = occidentary.wrapping_add(1);
                    iv1 = go.next_u16();
                    occidentary = occidentary.wrapping_add(1);
                    iv2 = go.next_u16();

                    for j in 0..3 {
                        if ((iv1 >> (5 * j)) & 31) == 31 {
                            break 'outer;
                        }
                        if ((iv2 >> (5 * j)) & 31) == 31 {
                            break 'outer;
                        }
                    }
                }

                if !self.base.base.filter.compare_nature(nature) {
                    rng.next();
                    cnt += 1;
                    continue;
                }
            } else {
                if self.base.lead <= Lead::SYNCHRONIZE_END {
                    let sync = {
                        occidentary = occidentary.wrapping_add(1);
                        (go.next_u16_max::<true>(2) as u8) == 0
                    };
                    nature = if sync {
                        self.base.lead.bits()
                    } else {
                        occidentary = occidentary.wrapping_add(1);
                        go.next_u16_max::<true>(25) as u8
                    };
                } else {
                    occidentary = occidentary.wrapping_add(1);
                    nature = go.next_u16_max::<true>(25) as u8;
                }

                if !self.base.base.filter.compare_nature(nature) {
                    rng.next();
                    cnt += 1;
                    continue;
                }

                while {
                    occidentary = occidentary.wrapping_add(1);
                    let low = go.next_u16() as u32;
                    occidentary = occidentary.wrapping_add(1);
                    let high = go.next_u16() as u32;
                    pid = (high << 16) | low;
                    (pid % 25) as u8 != nature
                } {}

                occidentary = occidentary.wrapping_add(1);
                iv1 = go.next_u16();
                occidentary = occidentary.wrapping_add(1);
                iv2 = go.next_u16();
            }

            let mut ivs = [0; 6];
            ivs[0] = (iv1 & 31) as u8;
            ivs[1] = ((iv1 >> 5) & 31) as u8;
            ivs[2] = ((iv1 >> 10) & 31) as u8;
            ivs[3] = ((iv2 >> 5) & 31) as u8;
            ivs[4] = ((iv2 >> 10) & 31) as u8;
            ivs[5] = (iv2 & 31) as u8;

            occidentary = occidentary.wrapping_add(1);
            let item = get_item(go.next_u16_max::<true>(100) as u8, self.base.lead, info);

            let state = WildGeneratorState4::new(
                rng.next_u16(),
                occidentary,
                self.base.base.initial_advances.wrapping_add(cnt),
                pid,
                ivs,
                (pid & 1) as u8,
                get_gender(pid, info),
                level,
                nature,
                get_shiny(pid, self.base.base.tsv),
                encounter_slot,
                item,
                slot.get_specie(),
                0,
                info,
            );

            if self.base.base.filter.compare_generator_state(&state) {
                states.push(state);
            }

            cnt += 1;
        }

        states
    }

    fn generate_poke_radar(
        &self,
        seed: u32,
        encounter_area: &EncounterArea4,
        index: u8,
    ) -> Vec<WildGeneratorState4> {
        let mut states = vec![];

        let slot = encounter_area.get_specific_pokemon(index as usize);
        let info = slot.get_info();

        let mut cute_charm = false;
        let mut buffer = 0;
        match info.get_gender() {
            0 | 254 | 255 => {}
            _ => {
                cute_charm = true;
                if self.base.lead == Lead::CUTE_CHARM_F {
                    buffer = 25u8.wrapping_mul((info.get_gender() / 25).wrapping_add(1));
                }
            }
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
            let mut occidentary = self.base.base.initial_advances.wrapping_add(cnt);
            let mut go = rng;

            let nature;
            let mut pid;

            let mut cute_charm_flag = false;
            if (self.base.lead == Lead::CUTE_CHARM_F || self.base.lead == Lead::CUTE_CHARM_M)
                && cute_charm
            {
                occidentary = occidentary.wrapping_add(1);
                cute_charm_flag = go.next_u16_max::<false>(3) != 0;
            }

            if self.base.lead <= Lead::SYNCHRONIZE_END {
                let sync = {
                    occidentary = occidentary.wrapping_add(1);
                    (go.next_u16_max::<false>(2) as u8) == 0
                };
                nature = if sync {
                    self.base.lead.bits()
                } else {
                    occidentary = occidentary.wrapping_add(1);
                    go.next_u16_max::<false>(25) as u8
                };
            } else {
                occidentary = occidentary.wrapping_add(1);
                nature = go.next_u16_max::<false>(25) as u8;
            }

            if !self.base.base.filter.compare_nature(nature) {
                rng.next();
                cnt += 1;
                continue;
            }

            if cute_charm_flag {
                pid = (buffer as u32).wrapping_add(nature as u32);
            } else {
                while {
                    occidentary = occidentary.wrapping_add(1);
                    let low = go.next_u16() as u32;
                    occidentary = occidentary.wrapping_add(1);
                    let high = go.next_u16() as u32;
                    pid = (high << 16) | low;
                    (pid % 25) as u8 != nature
                } {}
            }

            occidentary = occidentary.wrapping_add(1);
            let iv1 = go.next_u16();
            occidentary = occidentary.wrapping_add(1);
            let iv2 = go.next_u16();
            let mut ivs = [0; 6];
            ivs[0] = (iv1 & 31) as u8;
            ivs[1] = ((iv1 >> 5) & 31) as u8;
            ivs[2] = ((iv1 >> 10) & 31) as u8;
            ivs[3] = ((iv2 >> 5) & 31) as u8;
            ivs[4] = ((iv2 >> 10) & 31) as u8;
            ivs[5] = (iv2 & 31) as u8;

            occidentary = occidentary.wrapping_add(1);
            let item = get_item(go.next_u16_max::<true>(100) as u8, self.base.lead, info);

            let state = WildGeneratorState4::new(
                rng.next_u16(),
                occidentary,
                self.base.base.initial_advances.wrapping_add(cnt),
                pid,
                ivs,
                (pid & 1) as u8,
                get_gender(pid, info),
                slot.get_max_level(),
                nature,
                get_shiny(pid, self.base.base.tsv),
                index,
                item,
                slot.get_specie(),
                0,
                info,
            );

            if self.base.base.filter.compare_generator_state(&state) {
                states.push(state);
            }

            cnt += 1;
        }

        states
    }

    fn generate_poke_radar_shiny(
        &self,
        seed: u32,
        encounter_area: &EncounterArea4,
        index: u8,
    ) -> Vec<WildGeneratorState4> {
        let mut states = vec![];

        let slot = encounter_area.get_specific_pokemon(index as usize);
        let info = slot.get_info();

        let cute_charm = !matches!(info.get_gender(), 0 | 254 | 255);

        let cute_charm_check = |info: &PersonalInfo, pid: u32| {
            if self.base.lead == Lead::CUTE_CHARM_F {
                ((pid & 0xFF) as u8) >= info.get_gender()
            } else {
                ((pid & 0xFF) as u8) < info.get_gender()
            }
        };

        let mut rng = PokeRNG::new_with_initial_advances(
            seed,
            self.base
                .base
                .initial_advances
                .wrapping_add(self.base.base.delay),
        );
        let mut cnt = 0;
        while cnt <= self.base.base.max_advances {
            let mut occidentary = self.base.base.initial_advances.wrapping_add(cnt);
            let mut go = rng;

            let mut pid;

            let shiny_pid = |go: &mut PokeRNG, occidentary: &mut u32| {
                *occidentary = occidentary.wrapping_add(1);
                let mut low = go.next_u16_max::<true>(8);
                *occidentary = occidentary.wrapping_add(1);
                let mut high = go.next_u16_max::<true>(8);
                for i in 3..16 {
                    *occidentary = occidentary.wrapping_add(1);
                    low |= (go.next_u16() & 1) << i;
                }
                high |= (self.base.base.tsv ^ low) & 0xFFF8;
                ((high as u32) << 16) | (low as u32)
            };

            if (self.base.lead == Lead::CUTE_CHARM_F || self.base.lead == Lead::CUTE_CHARM_M)
                && cute_charm
                && {
                    occidentary = occidentary.wrapping_add(1);
                    go.next_u16_max::<false>(3) != 0
                }
            {
                while {
                    pid = shiny_pid(&mut go, &mut occidentary);
                    !cute_charm_check(info, pid)
                } {}
            } else if self.base.lead.bits() <= Lead::SYNCHRONIZE_END.bits() && {
                occidentary = occidentary.wrapping_add(1);
                go.next_u16_max::<false>(2) == 0
            } {
                while {
                    pid = shiny_pid(&mut go, &mut occidentary);
                    ((pid % 25) as u8) != self.base.lead.bits()
                } {}
            } else {
                pid = shiny_pid(&mut go, &mut occidentary);
            }

            let nature = (pid % 25) as u8;

            if !self.base.base.filter.compare_nature(nature) {
                rng.next();
                cnt += 1;
                continue;
            }

            occidentary = occidentary.wrapping_add(1);
            let iv1 = go.next_u16();
            occidentary = occidentary.wrapping_add(1);
            let iv2 = go.next_u16();
            let mut ivs = [0; 6];
            ivs[0] = (iv1 & 31) as u8;
            ivs[1] = ((iv1 >> 5) & 31) as u8;
            ivs[2] = ((iv1 >> 10) & 31) as u8;
            ivs[3] = ((iv2 >> 5) & 31) as u8;
            ivs[4] = ((iv2 >> 10) & 31) as u8;
            ivs[5] = (iv2 & 31) as u8;

            occidentary = occidentary.wrapping_add(1);
            let item = get_item(go.next_u16_max::<true>(100) as u8, self.base.lead, info);

            let state = WildGeneratorState4::new(
                rng.next_u16(),
                occidentary,
                self.base.base.initial_advances.wrapping_add(cnt),
                pid,
                ivs,
                (pid & 1) as u8,
                get_gender(pid, info),
                slot.get_max_level(),
                nature,
                get_shiny(pid, self.base.base.tsv),
                index,
                item,
                slot.get_specie(),
                0,
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
