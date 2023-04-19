use crate::enums::{Encounter, Game, Lead, Method};
use crate::gen3::filters::WildStateFilter3;
use crate::gen3::states::WildSearcherState3;
use crate::gen3::{EncounterArea3, Profile3};
use crate::parents::filters::{Filter, StateFilter, WildFilter, WildStateFilter};
use crate::parents::searchers::WildSearcher;
use crate::parents::{EncounterAreaBase, PersonalInfo, Profile};
use crate::rng::{lcrng_reverse, PokeRNGR};
use crate::util::encounter_slot;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex};

fn cute_charm_gender(info: &PersonalInfo, pid: u32, lead: Lead) -> bool {
    match info.get_gender() {
        0 | 254 | 255 => false,
        i if lead == Lead::CUTE_CHARM_F => ((pid & 255) as u8) >= i,
        i => ((pid & 255) as u8) < i,
    }
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
pub struct WildSearcher3 {
    pub base: WildSearcher<EncounterArea3, Profile3, WildStateFilter3>,
    pub results: Arc<Mutex<Vec<WildSearcherState3>>>,
    pub searching: Arc<AtomicBool>,
    pub progress: Arc<AtomicU32>,
    pub iv_advance: bool,
    pub modified_slots: Arc<Vec<u8>>,
}

impl WildSearcher3 {
    pub fn new(
        method: Method,
        encounter: Encounter,
        lead: Lead,
        encounter_area: &EncounterArea3,
        profile: &Profile3,
        filter: &WildStateFilter3,
    ) -> Self {
        Self {
            base: WildSearcher::new(method, encounter, lead, encounter_area, profile, filter),
            results: Arc::new(Mutex::new(vec![])),
            searching: Arc::new(AtomicBool::new(false)),
            progress: Arc::new(AtomicU32::new(0)),
            iv_advance: method == Method::Method2,
            modified_slots: Arc::new(encounter_area.get_slots_by_lead(lead)),
        }
    }

    pub fn cancel_search(&self) {
        self.searching.store(false, Ordering::SeqCst);
    }

    pub fn get_progress(&self) -> u32 {
        self.progress.load(Ordering::SeqCst)
    }

    pub fn get_results(&self) -> Vec<WildSearcherState3> {
        let mut lock = self.results.lock().unwrap();
        std::mem::take(lock.as_mut())
    }

    pub fn start_search(&self, min: [u8; 6], max: [u8; 6]) {
        self.searching.store(true, Ordering::SeqCst);

        let safari = self
            .base
            .encounter_area
            .safari_zone(self.base.base.profile.get_version());
        let tanoby = self
            .base
            .encounter_area
            .tanoby_chamber(self.base.base.profile.get_version());

        for hp in min[0]..=max[0] {
            for atk in min[1]..=max[1] {
                for def in min[2]..=max[2] {
                    for spa in min[3]..=max[3] {
                        for spd in min[4]..=max[4] {
                            for spe in min[5]..=max[5] {
                                if !self.searching.load(Ordering::SeqCst) {
                                    return;
                                }

                                let mut states =
                                    self.search(hp, atk, def, spa, spd, spe, safari, tanoby);
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
    pub fn search(
        &self,
        hp: u8,
        atk: u8,
        def: u8,
        spa: u8,
        spd: u8,
        spe: u8,
        safari: bool,
        tanoby: bool,
    ) -> Vec<WildSearcherState3> {
        let mut states = vec![];

        let ivs = [hp, atk, def, spa, spd, spe];
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

        for seed in seeds.into_iter().take(size) {
            let mut rng = PokeRNGR::new(seed);
            if self.iv_advance {
                rng.next();
            }

            let mut pid;

            if tanoby {
                pid = rng.next_u16() as u32;
                pid |= (rng.next_u16() as u32) << 16;
            } else {
                pid = (rng.next_u16() as u32) << 16;
                pid |= rng.next_u16() as u32;
            }

            let nature = (pid % 25) as u8;
            if !self.base.base.filter.compare_nature(nature) {
                continue;
            }

            let mut hunt_nature;
            let mut next_rng = rng.next_u16() as u32;
            let mut next_rng_2 = rng.next_u16() as u32;

            while {
                let mut test = rng;

                let mut encounter_slot = 0;
                let mut level = 0;
                let mut cute_charm_flag = false;
                let mut valid = false;

                match self.base.lead.bits() {
                    i if i == Lead::NONE.bits() => {
                        if ((next_rng % 25) as u8) == nature {
                            let prng = if safari {
                                test.next_u16()
                            } else {
                                next_rng_2 as u16
                            };
                            encounter_slot = encounter_slot::h_slot(
                                test.next_u16_max::<true>(100) as u8,
                                self.base.encounter,
                            );
                            level = self
                                .base
                                .encounter_area
                                .calculate_level_with_prng_state(encounter_slot, prng);
                            valid = self.base.base.filter.compare_encounter_slot(encounter_slot);
                        }
                    }
                    i if i == Lead::CUTE_CHARM_F.bits() || i == Lead::CUTE_CHARM_M.bits() => {
                        if ((next_rng % 25) as u8) == nature {
                            if safari {
                                test.next();
                            }
                            let prng = test.next_u16();
                            encounter_slot = encounter_slot::h_slot(
                                test.next_u16_max::<true>(100) as u8,
                                self.base.encounter,
                            );
                            level = self
                                .base
                                .encounter_area
                                .calculate_level_with_prng_state(encounter_slot, prng);
                            cute_charm_flag = (next_rng_2 % 3) > 0;
                            valid = self.base.base.filter.compare_encounter_slot(encounter_slot);
                        }
                    }
                    i if i <= Lead::SYNCHRONIZE_END.bits() => {
                        if (next_rng & 1) == 0 {
                            let prng = if safari {
                                test.next_u16()
                            } else {
                                next_rng_2 as u16
                            };
                            encounter_slot = encounter_slot::h_slot(
                                test.next_u16_max::<true>(100) as u8,
                                self.base.encounter,
                            );
                            level = self
                                .base
                                .encounter_area
                                .calculate_level_with_prng_state(encounter_slot, prng);
                            valid = self.base.base.filter.compare_encounter_slot(encounter_slot);
                        } else if (next_rng_2 & 1) == 1 && ((next_rng % 25) as u8) == nature {
                            if safari {
                                test.next();
                            }
                            let prng = test.next_u16();
                            encounter_slot = encounter_slot::h_slot(
                                test.next_u16_max::<true>(100) as u8,
                                self.base.encounter,
                            );
                            level = self
                                .base
                                .encounter_area
                                .calculate_level_with_prng_state(encounter_slot, prng);
                            valid = self.base.base.filter.compare_encounter_slot(encounter_slot);
                        }
                    }
                    i if i == Lead::MAGNET_PULL.bits() || i == Lead::STATIC.bits() => {
                        if ((next_rng % 25) as u8) == nature {
                            let level_rand = if safari {
                                test.next_u16()
                            } else {
                                next_rng_2 as u16
                            };
                            let encounter_rand = test.next_u16();
                            if test.next_u16_max::<true>(2) == 0 && !self.modified_slots.is_empty()
                            {
                                encounter_slot = self.modified_slots
                                    [(encounter_rand as usize) % self.modified_slots.len()];
                            } else {
                                encounter_slot = encounter_slot::h_slot(
                                    (encounter_rand % 100) as u8,
                                    self.base.encounter,
                                );
                            }
                            level = self
                                .base
                                .encounter_area
                                .calculate_level_with_prng_state(encounter_slot, level_rand);
                            valid = self.base.base.filter.compare_encounter_slot(encounter_slot);
                        }
                    }
                    i if i == Lead::PRESSURE.bits() => {
                        if ((next_rng % 25) as u8) == nature {
                            let rand = if safari {
                                test.next_u16()
                            } else {
                                next_rng_2 as u16
                            };
                            let level_rand = test.next_u16();
                            encounter_slot = encounter_slot::h_slot(
                                test.next_u16_max::<true>(100) as u8,
                                self.base.encounter,
                            );
                            level = self.base.encounter_area.calculate_level_with_rand(
                                encounter_slot,
                                level_rand,
                                (rand & 1) == 0,
                            );
                            valid = self.base.base.filter.compare_encounter_slot(encounter_slot);
                        }
                    }
                    _ => {}
                }

                if valid {
                    let slot = self
                        .base
                        .encounter_area
                        .get_specific_pokemon(encounter_slot as usize);
                    let info = slot.get_info();
                    if (!cute_charm_flag || cute_charm_gender(info, pid, self.base.lead))
                        && (slot.specie != 201 || unown_check(pid, slot.form))
                    {
                        let state = WildSearcherState3::new(
                            test.next(),
                            pid,
                            ivs,
                            (pid & 1) as u8,
                            get_gender(pid, info),
                            level,
                            nature,
                            get_shiny(pid, self.base.base.tsv),
                            encounter_slot,
                            slot.specie,
                            slot.form,
                            info,
                        );

                        if self.base.base.filter.compare_searcher_state(&state.base) {
                            states.push(state);
                        }
                    }
                }

                hunt_nature = (((next_rng << 16) | next_rng_2) % 25) as u8;
                next_rng = rng.next_u16() as u32;
                next_rng_2 = rng.next_u16() as u32;
                hunt_nature != nature
            } {}
        }

        if (self.base.base.profile.get_version() & Game::RSE) != Game::NONE
            && self.base.encounter == Encounter::RockSmash
        {
            let rate = self.base.encounter_area.get_rate() as u16 * 16;
            let mut i = 0;
            while i < states.len() {
                let check = (states[i].base.seed >> 16) as u16;
                if (check % 2880) >= rate {
                    states.remove(i);
                } else {
                    states[i].base.seed = PokeRNGR::new(states[i].base.seed).next();
                    i += 1;
                }
            }
        }

        states
    }
}
