use crate::enums::{Encounter, Lead, Method};
use crate::gen4::filters::WildStateFilter4;
use crate::gen4::states::WildSearcherState4;
use crate::gen4::{EncounterArea4, Profile4};
use crate::parents::filters::{Filter, WildFilter};
use crate::parents::searchers::WildSearcher;
use crate::parents::{EncounterAreaT, PersonalInfo, Profile};
use crate::rng::{lcrng_reverse, PokeRNG, PokeRNGR};
use crate::util::encounter_slot;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex};

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
pub struct WildSearcher4 {
    pub base: WildSearcher<EncounterArea4, Profile4, WildStateFilter4>,
    pub modified_slots: Arc<Vec<u8>>,
    pub results: Arc<Mutex<Vec<WildSearcherState4>>>,
    pub progress: Arc<AtomicU32>,
    pub max_advance: u32,
    pub min_advance: u32,
    pub max_delay: u32,
    pub min_delay: u32,
    pub thresh: u16,
    pub safari: bool,
    pub searching: Arc<AtomicBool>,
    pub shiny: bool,
}

impl WildSearcher4 {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        min_advance: u32,
        max_advance: u32,
        min_delay: u32,
        max_delay: u32,
        method: Method,
        encounter: Encounter,
        lead: Lead,
        shiny: bool,
        encounter_area: &EncounterArea4,
        profile: &Profile4,
        filter: &WildStateFilter4,
    ) -> Self {
        let mut new = Self {
            base: WildSearcher::new(method, encounter, lead, encounter_area, profile, filter),
            modified_slots: Arc::new(encounter_area.get_slots_by_lead(lead)),
            results: Arc::new(Mutex::new(vec![])),
            progress: Arc::new(AtomicU32::new(0)),
            max_advance,
            min_advance,
            max_delay,
            min_delay,
            thresh: encounter_area.get_rate() as u16,
            safari: encounter_area.safari_zone(profile.get_version()),
            searching: Arc::new(AtomicBool::new(false)),
            shiny,
        };

        if (lead == Lead::SUCTION_CUPS
            && matches!(
                encounter,
                Encounter::OldRod | Encounter::GoodRod | Encounter::SuperRod
            ))
            || (lead == Lead::ARENA_TRAP && encounter == Encounter::RockSmash)
        {
            new.thresh *= 2;
        }

        new
    }

    pub fn cancel_search(&self) {
        self.searching.store(false, Ordering::SeqCst);
    }

    pub fn get_progress(&self) -> u32 {
        self.progress.load(Ordering::SeqCst)
    }

    pub fn get_results(&self) -> Vec<WildSearcherState4> {
        std::mem::take(self.results.lock().unwrap().as_mut())
    }

    pub fn start_search(&self, min: [u8; 6], max: [u8; 6], index: u8) {
        self.searching.store(true, Ordering::SeqCst);

        for hp in min[0]..=max[0] {
            for atk in min[1]..=max[1] {
                for def in min[2]..=max[2] {
                    for spa in min[3]..=max[3] {
                        for spd in min[4]..=max[4] {
                            for spe in min[5]..=max[5] {
                                if !self.searching.load(Ordering::SeqCst) {
                                    return;
                                }

                                let mut states = self.search(hp, atk, def, spa, spd, spe, index);
                                self.results.lock().unwrap().append(&mut states);
                                self.progress.fetch_add(1, Ordering::SeqCst);
                            }
                        }
                    }
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn search(
        &self,
        hp: u8,
        atk: u8,
        def: u8,
        spa: u8,
        spd: u8,
        spe: u8,
        index: u8,
    ) -> Vec<WildSearcherState4> {
        let states = match self.base.base.method {
            Method::MethodJ => self.search_method_j(hp, atk, def, spa, spd, spe),
            Method::MethodK => self.search_method_k(hp, atk, def, spa, spd, spe),
            Method::PokeRadar if self.shiny => {
                self.search_poke_radar_shiny(hp, atk, def, spa, spd, spe, index)
            }
            Method::PokeRadar => self.search_poke_radar(hp, atk, def, spa, spd, spe, index),
            _ => vec![],
        };

        self.search_initial_seeds(states)
    }

    fn search_initial_seeds(&self, results: Vec<WildSearcherState4>) -> Vec<WildSearcherState4> {
        let mut states = vec![];

        for mut result in results.into_iter() {
            let mut rng = PokeRNGR::new_with_initial_advances(result.base.seed, self.min_advance);
            let mut seed = rng.seed;
            for cnt in self.min_advance..=self.max_advance {
                let hour = (seed >> 16) & 0xFF;
                let delay = seed & 0xFFFF;

                if hour < 24 && (self.min_delay..=self.max_delay).contains(&delay) {
                    result.base.seed = seed;
                    result.advances = cnt;
                    states.push(result);
                }
                seed = rng.next();
            }
        }

        states
    }

    fn search_method_j(
        &self,
        hp: u8,
        atk: u8,
        def: u8,
        spa: u8,
        spd: u8,
        spe: u8,
    ) -> Vec<WildSearcherState4> {
        let mut states = vec![];

        let ivs = [hp, atk, def, spa, spd, spe];
        let grass = self.base.encounter == Encounter::Grass;
        let nibble = matches!(
            self.base.encounter,
            Encounter::OldRod | Encounter::GoodRod | Encounter::SuperRod
        );

        let mut seeds = [0; 6];
        let size = lcrng_reverse::recover_poke_rng_iv(
            hp,
            atk,
            def,
            spa,
            spd,
            spe,
            &mut seeds,
            Method::Method1,
        );
        for rev_seed in seeds.into_iter().take(size) {
            let mut rng = PokeRNGR::new(rev_seed);

            let mut forward = PokeRNG::new(rev_seed);
            forward.advance(1);
            let item_rand = forward.next_u16_max::<true>(100);
            let unown_form = self.base.encounter_area.unown_form(forward.next_u16());

            if self.base.lead == Lead::CUTE_CHARM_F || self.base.lead == Lead::CUTE_CHARM_M {
                let nature = rng.next_u16_max::<false>(25) as u8;
                if !self.base.base.filter.compare_nature(nature) {
                    continue;
                }

                if rng.next_u16_max::<false>(3) != 0 {
                    let level_rand = if grass { 0 } else { rng.next_u16() };
                    let encounter_slot = encounter_slot::j_slot(
                        rng.next_u16_max::<false>(100) as u8,
                        self.base.encounter,
                    );
                    if !self.base.base.filter.compare_encounter_slot(encounter_slot) {
                        continue;
                    }

                    if !nibble || rng.next_u16_max::<true>(100) < self.thresh {
                        let slot = self
                            .base
                            .encounter_area
                            .get_specific_pokemon(encounter_slot as usize);
                        let info = slot.get_info();

                        let mut buffer = 0;
                        match info.get_gender() {
                            0 | 254 | 255 => {
                                continue;
                            }
                            _ => {
                                if self.base.lead == Lead::CUTE_CHARM_F {
                                    buffer =
                                        25u8.wrapping_mul((info.get_gender() / 25).wrapping_add(1));
                                }
                            }
                        }

                        let level = if grass {
                            self.base.encounter_area.calculate_level(encounter_slot)
                        } else {
                            self.base
                                .encounter_area
                                .calculate_level_with_prng_state(encounter_slot, level_rand)
                        };

                        let item = get_item(item_rand as u8, self.base.lead, info);

                        let form = if slot.get_species() == 201 {
                            unown_form
                        } else {
                            0
                        };

                        let pid = (nature as u32).wrapping_add(buffer as u32);
                        let state = WildSearcherState4::new(
                            rng.next(),
                            pid,
                            ivs,
                            (pid & 1) as u8,
                            get_gender(pid, info),
                            level,
                            nature,
                            get_shiny(pid, self.base.base.tsv),
                            encounter_slot,
                            item,
                            slot.get_species(),
                            form,
                            info,
                        );

                        if self.base.base.filter.compare_searcher_state(&state) {
                            states.push(state);
                        }
                    }
                }
            } else {
                let mut pid = (rng.next_u16() as u32) << 16;
                pid |= rng.next_u16() as u32;

                let nature = (pid % 25) as u8;
                if !self.base.base.filter.compare_nature(nature) {
                    continue;
                }

                let mut hunt_nature;
                let mut next_rng = rng.next_u16();
                let mut next_rng2 = rng.next_u16();

                while {
                    let mut test = PokeRNGR::new(rng.seed);

                    let mut encounter_slot = 0;
                    let mut level_rand = 0;
                    let mut force = false;
                    let mut valid = false;
                    if matches!(self.base.lead, Lead::NONE | Lead::COMPOUND_EYES) {
                        if ((next_rng / 0xa3e) as u8) == nature {
                            if grass {
                                encounter_slot = encounter_slot::j_slot(
                                    (next_rng2 / 0x290) as u8,
                                    self.base.encounter,
                                );
                            } else {
                                level_rand = next_rng2;
                                encounter_slot = encounter_slot::j_slot(
                                    test.next_u16_max::<false>(100) as u8,
                                    self.base.encounter,
                                );
                            }
                            valid = self.base.base.filter.compare_encounter_slot(encounter_slot);
                        }
                    } else if self.base.lead == Lead::SYNCHRONIZE {
                        if (next_rng / 0x8000) == 0 {
                            if grass {
                                encounter_slot = encounter_slot::j_slot(
                                    (next_rng2 / 0x290) as u8,
                                    self.base.encounter,
                                );
                            } else {
                                level_rand = next_rng2;
                                encounter_slot = encounter_slot::j_slot(
                                    test.next_u16_max::<false>(100) as u8,
                                    self.base.encounter,
                                );
                            }
                            valid = self.base.base.filter.compare_encounter_slot(encounter_slot);
                        } else if (next_rng2 / 0x8000) == 1 && ((next_rng / 0xa3e) as u8) == nature
                        {
                            if grass {
                                encounter_slot = encounter_slot::j_slot(
                                    test.next_u16_max::<false>(100) as u8,
                                    self.base.encounter,
                                );
                            } else {
                                level_rand = test.next_u16();
                                encounter_slot = encounter_slot::j_slot(
                                    test.next_u16_max::<false>(100) as u8,
                                    self.base.encounter,
                                );
                            }
                            valid = self.base.base.filter.compare_encounter_slot(encounter_slot);
                        }
                    } else if matches!(self.base.lead, Lead::MAGNET_PULL | Lead::STATIC) {
                        if ((next_rng / 0xa3e) as u8) == nature {
                            let encounter_rand = if grass {
                                next_rng2
                            } else {
                                level_rand = next_rng2;
                                test.next_u16()
                            };

                            if test.next_u16_max::<false>(2) == 0 && !self.modified_slots.is_empty()
                            {
                                encounter_slot = self.modified_slots
                                    [(encounter_rand as usize) % self.modified_slots.len()];
                            } else {
                                encounter_slot = encounter_slot::j_slot(
                                    (encounter_rand / 0x290) as u8,
                                    self.base.encounter,
                                );
                            }
                            valid = self.base.base.filter.compare_encounter_slot(encounter_slot);
                        }
                    } else if self.base.lead == Lead::PRESSURE
                        && ((next_rng / 0xa3e) as u8) == nature
                    {
                        force = (next_rng2 / 0x8000) != 0;

                        if grass {
                            encounter_slot = encounter_slot::j_slot(
                                test.next_u16_max::<false>(100) as u8,
                                self.base.encounter,
                            );
                        } else {
                            level_rand = test.next_u16();
                            encounter_slot = encounter_slot::j_slot(
                                test.next_u16_max::<false>(100) as u8,
                                self.base.encounter,
                            );
                        }
                        valid = self.base.base.filter.compare_encounter_slot(encounter_slot);
                    }

                    if valid && (!nibble || test.next_u16_max::<false>(100) < self.thresh) {
                        let level = if self.base.encounter == Encounter::Grass {
                            self.base.encounter_area.calculate_level_with_rand::<false>(
                                encounter_slot,
                                level_rand,
                                force,
                            )
                        } else {
                            self.base.encounter_area.calculate_level_with_rand::<true>(
                                encounter_slot,
                                level_rand,
                                force,
                            )
                        };

                        let slot = self
                            .base
                            .encounter_area
                            .get_specific_pokemon(encounter_slot as usize);
                        let info = slot.info;
                        let item = get_item(item_rand as u8, self.base.lead, info);

                        let form = if slot.get_species() == 201 {
                            unown_form
                        } else {
                            0
                        };

                        let state = WildSearcherState4::new(
                            test.next(),
                            pid,
                            ivs,
                            (pid & 1) as u8,
                            get_gender(pid, info),
                            level,
                            nature,
                            get_shiny(pid, self.base.base.tsv),
                            encounter_slot,
                            item,
                            slot.get_species(),
                            form,
                            info,
                        );

                        if self.base.base.filter.compare_searcher_state(&state) {
                            states.push(state);
                        }
                    }
                    hunt_nature = ((((next_rng as u32) << 16) | (next_rng2 as u32)) % 25) as u8;
                    next_rng = rng.next_u16();
                    next_rng2 = rng.next_u16();
                    hunt_nature != nature
                } {}
            }
        }

        states
    }

    fn search_method_k(
        &self,
        hp: u8,
        atk: u8,
        def: u8,
        spa: u8,
        spd: u8,
        spe: u8,
    ) -> Vec<WildSearcherState4> {
        let mut states = vec![];

        let ivs = [hp, atk, def, spa, spd, spe];
        let grass = self.base.encounter == Encounter::Grass;
        let nibble = matches!(
            self.base.encounter,
            Encounter::OldRod | Encounter::GoodRod | Encounter::SuperRod | Encounter::RockSmash
        );

        let mut seeds = [0; 6];
        let size = lcrng_reverse::recover_poke_rng_iv(
            hp,
            atk,
            def,
            spa,
            spd,
            spe,
            &mut seeds,
            Method::Method1,
        );
        for rev_seed in seeds.into_iter().take(size) {
            let mut rng = PokeRNGR::new(rev_seed);

            let item_rand = ((PokeRNG::new(rev_seed).advance(2) >> 16) % 100) as u8;

            if self.base.lead == Lead::CUTE_CHARM_F || self.base.lead == Lead::CUTE_CHARM_M {
                let nature = rng.next_u16_max::<true>(25) as u8;
                if !self.base.base.filter.compare_nature(nature) {
                    continue;
                }

                if rng.next_u16_max::<true>(3) != 0 {
                    let mut level_rand = 0;
                    let encounter_slot = if self.safari {
                        rng.next_u16_max::<true>(10) as u8
                    } else if grass {
                        encounter_slot::k_slot(
                            rng.next_u16_max::<true>(100) as u8,
                            self.base.encounter,
                        )
                    } else {
                        level_rand = rng.next_u16();
                        encounter_slot::k_slot(
                            rng.next_u16_max::<true>(100) as u8,
                            self.base.encounter,
                        )
                    };

                    if !self.base.base.filter.compare_encounter_slot(encounter_slot) {
                        continue;
                    }

                    if !nibble || rng.next_u16_max::<true>(100) < self.thresh {
                        let slot = self
                            .base
                            .encounter_area
                            .get_specific_pokemon(encounter_slot as usize);
                        let info = slot.get_info();

                        let mut buffer = 0;
                        match info.get_gender() {
                            0 | 254 | 255 => {
                                continue;
                            }
                            _ => {
                                if self.base.lead == Lead::CUTE_CHARM_F {
                                    buffer =
                                        25u8.wrapping_mul((info.get_gender() / 25).wrapping_add(1));
                                }
                            }
                        }

                        let level = if grass || self.safari {
                            self.base.encounter_area.calculate_level(encounter_slot)
                        } else {
                            self.base
                                .encounter_area
                                .calculate_level_with_prng_state(encounter_slot, level_rand)
                        };

                        let item = get_item(item_rand, self.base.lead, info);

                        let pid = (nature as u32).wrapping_add(buffer as u32);
                        let state = WildSearcherState4::new(
                            rng.next(),
                            pid,
                            ivs,
                            (pid & 1) as u8,
                            get_gender(pid, info),
                            level,
                            nature,
                            get_shiny(pid, self.base.base.tsv),
                            encounter_slot,
                            item,
                            slot.get_species(),
                            0,
                            info,
                        );

                        if self.base.base.filter.compare_searcher_state(&state) {
                            states.push(state);
                        }
                    }
                }
            } else {
                let mut pid = (rng.next_u16() as u32) << 16;
                pid |= rng.next_u16() as u32;

                let nature = (pid % 25) as u8;
                if !self.base.base.filter.compare_nature(nature) {
                    continue;
                }

                let mut hunt_nature;
                let mut next_rng = rng.next_u16();
                let mut next_rng2 = rng.next_u16();

                while {
                    let mut test = PokeRNGR::new(rng.seed);

                    let mut encounter_slot = 0;
                    let mut level_rand = 0;
                    let mut force = false;
                    let mut valid = false;
                    if matches!(
                        self.base.lead,
                        Lead::NONE | Lead::COMPOUND_EYES | Lead::ARENA_TRAP | Lead::SUCTION_CUPS
                    ) && ((next_rng % 25) as u8) == nature
                    {
                        if self.safari {
                            encounter_slot = (next_rng2 % 10) as u8;
                        } else if grass {
                            encounter_slot = encounter_slot::k_slot(
                                (next_rng2 % 100) as u8,
                                self.base.encounter,
                            );
                        } else {
                            level_rand = next_rng2;
                            encounter_slot = encounter_slot::k_slot(
                                test.next_u16_max::<true>(100) as u8,
                                self.base.encounter,
                            );
                        }
                        valid = self.base.base.filter.compare_encounter_slot(encounter_slot);
                    } else if self.base.lead == Lead::SYNCHRONIZE {
                        if (next_rng % 2) == 0 {
                            if self.safari {
                                encounter_slot = (next_rng2 % 10) as u8;
                            } else if grass {
                                encounter_slot = encounter_slot::k_slot(
                                    (next_rng2 % 100) as u8,
                                    self.base.encounter,
                                );
                            } else {
                                level_rand = next_rng2;
                                encounter_slot = encounter_slot::k_slot(
                                    test.next_u16_max::<true>(100) as u8,
                                    self.base.encounter,
                                );
                            }
                            valid = self.base.base.filter.compare_encounter_slot(encounter_slot);
                        } else if (next_rng2 % 2) == 1 && ((next_rng % 25) as u8) == nature {
                            if self.safari {
                                encounter_slot = test.next_u16_max::<true>(10) as u8;
                            } else if grass {
                                encounter_slot = encounter_slot::k_slot(
                                    test.next_u16_max::<true>(100) as u8,
                                    self.base.encounter,
                                );
                            } else {
                                level_rand = test.next_u16();
                                encounter_slot = encounter_slot::k_slot(
                                    test.next_u16_max::<true>(100) as u8,
                                    self.base.encounter,
                                );
                            }
                            valid = self.base.base.filter.compare_encounter_slot(encounter_slot);
                        }
                    } else if matches!(self.base.lead, Lead::MAGNET_PULL | Lead::STATIC) {
                        if ((next_rng % 25) as u8) == nature {
                            let encounter_rand = if grass || self.safari {
                                next_rng2
                            } else {
                                level_rand = next_rng2;
                                test.next_u16()
                            };

                            if test.next_u16_max::<true>(2) == 0 && !self.modified_slots.is_empty()
                            {
                                encounter_slot = self.modified_slots
                                    [(encounter_rand as usize) % self.modified_slots.len()];
                            } else if self.safari {
                                encounter_slot = (encounter_rand % 10) as u8;
                            } else {
                                encounter_slot = encounter_slot::k_slot(
                                    (encounter_rand % 100) as u8,
                                    self.base.encounter,
                                );
                            }
                            valid = self.base.base.filter.compare_encounter_slot(encounter_slot);
                        }
                    } else if self.base.lead == Lead::PRESSURE && ((next_rng % 25) as u8) == nature
                    {
                        force = (next_rng2 % 2) != 0;

                        if self.safari {
                            encounter_slot = test.next_u16_max::<true>(10) as u8;
                        } else if grass {
                            encounter_slot = encounter_slot::k_slot(
                                test.next_u16_max::<true>(100) as u8,
                                self.base.encounter,
                            );
                        } else {
                            level_rand = test.next_u16();
                            encounter_slot = encounter_slot::k_slot(
                                test.next_u16_max::<true>(100) as u8,
                                self.base.encounter,
                            );
                        }
                        valid = self.base.base.filter.compare_encounter_slot(encounter_slot);
                    }

                    if valid && (!nibble || test.next_u16_max::<true>(100) < self.thresh) {
                        let level = if self.base.encounter == Encounter::Grass || self.safari {
                            self.base.encounter_area.calculate_level_with_rand::<false>(
                                encounter_slot,
                                level_rand,
                                force,
                            )
                        } else {
                            self.base.encounter_area.calculate_level_with_rand::<true>(
                                encounter_slot,
                                level_rand,
                                force,
                            )
                        };

                        let slot = self
                            .base
                            .encounter_area
                            .get_specific_pokemon(encounter_slot as usize);
                        let info = slot.info;
                        let item = get_item(item_rand, self.base.lead, info);

                        let state = WildSearcherState4::new(
                            test.next(),
                            pid,
                            ivs,
                            (pid & 1) as u8,
                            get_gender(pid, info),
                            level,
                            nature,
                            get_shiny(pid, self.base.base.tsv),
                            encounter_slot,
                            item,
                            slot.get_species(),
                            0,
                            info,
                        );

                        if self.base.base.filter.compare_searcher_state(&state) {
                            states.push(state);
                        }
                    }
                    hunt_nature = ((((next_rng as u32) << 16) | (next_rng2 as u32)) % 25) as u8;
                    next_rng = rng.next_u16();
                    next_rng2 = rng.next_u16();
                    hunt_nature != nature
                } {}
            }
        }

        states
    }

    #[allow(clippy::too_many_arguments)]
    fn search_poke_radar(
        &self,
        hp: u8,
        atk: u8,
        def: u8,
        spa: u8,
        spd: u8,
        spe: u8,
        index: u8,
    ) -> Vec<WildSearcherState4> {
        let mut states = vec![];

        let ivs = [hp, atk, def, spa, spd, spe];
        let slot = self
            .base
            .encounter_area
            .get_specific_pokemon(index as usize);
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

        let mut seeds = [0; 6];
        let size = lcrng_reverse::recover_poke_rng_iv(
            hp,
            atk,
            def,
            spa,
            spd,
            spe,
            &mut seeds,
            self.base.base.method,
        );
        for rev_seed in seeds.into_iter().take(size) {
            let mut rng = PokeRNGR::new(rev_seed);
            let item = get_item(
                ((PokeRNG::new(rev_seed).advance(2) >> 16) % 100) as u8,
                self.base.lead,
                info,
            );

            if matches!(self.base.lead, Lead::CUTE_CHARM_F | Lead::CUTE_CHARM_M) && cute_charm {
                let nature = rng.next_u16_max::<false>(25) as u8;
                if !self.base.base.filter.compare_nature(nature) {
                    continue;
                }

                if rng.next_u16_max::<false>(3) != 0 {
                    let pid = (nature as u32).wrapping_add(buffer as u32);
                    let state = WildSearcherState4::new(
                        rng.next(),
                        pid,
                        ivs,
                        (pid & 1) as u8,
                        get_gender(pid, info),
                        slot.max_level,
                        nature,
                        get_shiny(pid, self.base.base.tsv),
                        index,
                        item,
                        slot.get_species(),
                        0,
                        info,
                    );

                    if self.base.base.filter.compare_searcher_state(&state) {
                        states.push(state);
                    }
                }
            } else {
                let mut pid = (rng.next_u16() as u32) << 16;
                pid |= rng.next_u16() as u32;

                let nature = (pid % 25) as u8;
                if !self.base.base.filter.compare_nature(nature) {
                    continue;
                }

                let mut hunt_nature;
                let mut next_rng = rng.next_u16();
                let mut next_rng2 = rng.next_u16();

                while {
                    let mut test = PokeRNGR::new(rng.seed);
                    let mut seed = 0;
                    let valid = if matches!(self.base.lead, Lead::NONE | Lead::COMPOUND_EYES)
                        && ((next_rng / 0xa3e) as u8) == nature
                    {
                        seed = test.seed;
                        true
                    } else if self.base.lead == Lead::SYNCHRONIZE {
                        if next_rng / 0x8000 == 0 {
                            seed = test.seed;
                            true
                        } else if next_rng2 / 0x8000 == 1 && ((next_rng / 0xa3e) as u8) == nature {
                            seed = test.next();
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    };

                    if valid {
                        let state = WildSearcherState4::new(
                            seed,
                            pid,
                            ivs,
                            (pid & 1) as u8,
                            get_gender(pid, info),
                            slot.max_level,
                            nature,
                            get_shiny(pid, self.base.base.tsv),
                            index,
                            item,
                            slot.species,
                            0,
                            info,
                        );

                        if self.base.base.filter.compare_searcher_state(&state) {
                            states.push(state);
                        }
                    }

                    hunt_nature = ((((next_rng as u32) << 16) | (next_rng2 as u32)) % 25) as u8;
                    next_rng = rng.next_u16();
                    next_rng2 = rng.next_u16();
                    hunt_nature != nature
                } {}
            }
        }
        states
    }

    #[allow(clippy::too_many_arguments)]
    fn search_poke_radar_shiny(
        &self,
        hp: u8,
        atk: u8,
        def: u8,
        spa: u8,
        spd: u8,
        spe: u8,
        index: u8,
    ) -> Vec<WildSearcherState4> {
        let mut states = vec![];

        let ivs = [hp, atk, def, spa, spd, spe];
        let slot = self
            .base
            .encounter_area
            .get_specific_pokemon(index as usize);
        let info = slot.get_info();

        let mut cute_charm = false;
        match info.get_gender() {
            0 | 254 | 255 => {}
            _ => {
                cute_charm = true;
            }
        }

        let cute_charm_check = |info: &PersonalInfo, pid: u32| {
            if self.base.lead == Lead::CUTE_CHARM_F {
                ((pid & 0xFF) as u8) >= info.get_gender()
            } else {
                ((pid & 0xFF) as u8) < info.get_gender()
            }
        };

        let shiny_pid = |rng: &mut PokeRNGR| {
            let mut low = 0;
            for j in (3..=15).rev() {
                low |= rng.next_u16_max::<true>(2) << j;
            }
            let mut high = rng.next_u16_max::<true>(8);
            low |= rng.next_u16_max::<true>(8);
            high |= (self.base.base.tsv ^ low) & 0xFFF8;
            ((high as u32) << 16) | (low as u32)
        };

        let mut seeds = [0; 6];
        let size = lcrng_reverse::recover_poke_rng_iv(
            hp,
            atk,
            def,
            spa,
            spd,
            spe,
            &mut seeds,
            self.base.base.method,
        );
        for rev_seed in seeds.into_iter().take(size) {
            let mut rng = PokeRNGR::new(rev_seed);
            let item = get_item(
                ((PokeRNG::new(rev_seed).advance(2) >> 16) % 100) as u8,
                self.base.lead,
                info,
            );

            let pid = shiny_pid(&mut rng);
            let nature = (pid % 25) as u8;
            if !self.base.base.filter.compare_nature(nature) {
                continue;
            }

            if self.base.lead == Lead::SYNCHRONIZE
                || (cute_charm
                    && (matches!(self.base.lead, Lead::CUTE_CHARM_F | Lead::CUTE_CHARM_M)))
            {
                let mut hunt_nature;
                let gender = (((pid & 0xFF) as u8) < info.get_gender()) as u8;
                'valid: while {
                    let mut test = PokeRNGR::new(rng.seed);
                    let valid = if self.base.lead == Lead::SYNCHRONIZE {
                        test.next_u16_max::<false>(2) == 0
                    } else {
                        test.next_u16_max::<false>(3) != 0 && cute_charm_check(info, pid)
                    };

                    if valid {
                        let state = WildSearcherState4::new(
                            test.next(),
                            pid,
                            ivs,
                            (pid & 1) as u8,
                            get_gender(pid, info),
                            slot.max_level,
                            nature,
                            get_shiny(pid, self.base.base.tsv),
                            index,
                            item,
                            slot.species,
                            0,
                            info,
                        );

                        if self.base.base.filter.compare_searcher_state(&state) {
                            states.push(state);
                        }
                    }

                    let hunt_pid = shiny_pid(&mut rng);
                    hunt_nature = (hunt_pid % 25) as u8;
                    if matches!(self.base.lead, Lead::CUTE_CHARM_F | Lead::CUTE_CHARM_M)
                        && gender == (((hunt_pid & 0xFF) as u8) < info.get_gender()) as u8
                    {
                        break 'valid;
                    }
                    hunt_nature != nature
                } {}
            } else {
                let state = WildSearcherState4::new(
                    rng.next(),
                    pid,
                    ivs,
                    (pid & 1) as u8,
                    get_gender(pid, info),
                    slot.max_level,
                    nature,
                    get_shiny(pid, self.base.base.tsv),
                    index,
                    item,
                    slot.species,
                    0,
                    info,
                );

                if self.base.base.filter.compare_searcher_state(&state) {
                    states.push(state);
                }
            }
        }

        states
    }
}
