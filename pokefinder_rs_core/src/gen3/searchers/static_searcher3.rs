use crate::enums::{Lead, Method};
use crate::gen3::filters::StateFilter3;
use crate::gen3::Profile3;
use crate::parents::filters::{Filter, StateFilter};
use crate::parents::searchers::StaticSearcher;
use crate::parents::states::SearcherState;
use crate::parents::{PersonalInfo, StaticTemplate, Template};
use crate::rng::{lcrng_reverse, PokeRNGR};
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
pub struct StaticSearcher3 {
    pub base: StaticSearcher<Profile3, StateFilter3>,
    pub results: Arc<Mutex<Vec<SearcherState>>>,
    pub searching: Arc<AtomicBool>,
    pub progress: Arc<AtomicU32>,
    pub iv_advance: bool,
}

impl StaticSearcher3 {
    pub fn new(method: Method, profile: &Profile3, filter: &StateFilter3) -> Self {
        Self {
            base: StaticSearcher::new(method, Lead::NONE, profile, filter),
            results: Arc::new(Mutex::new(vec![])),
            searching: Arc::new(AtomicBool::new(false)),
            progress: Arc::new(AtomicU32::new(0)),
            iv_advance: false,
        }
    }

    pub fn start_search(
        &self,
        min: [u8; 6],
        max: [u8; 6],
        static_template: &'static StaticTemplate,
    ) {
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

                                let mut states =
                                    self.search(hp, atk, def, spa, spd, spe, static_template);
                                self.results.lock().unwrap().append(&mut states);
                                self.progress.fetch_add(1, Ordering::SeqCst);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn cancel_search(&self) {
        self.searching.store(false, Ordering::SeqCst);
    }

    pub fn progress(&self) -> u32 {
        self.progress.load(Ordering::SeqCst)
    }

    pub fn get_results(&self) -> Vec<SearcherState> {
        let mut lock = self.results.lock().unwrap();
        std::mem::take(lock.as_mut())
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
        static_template: &'static StaticTemplate,
    ) -> Vec<SearcherState> {
        let mut states = vec![];
        let ivs = [hp, atk, def, spa, spd, spe];
        let info = static_template.get_info();

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

            let mut pid = (rng.next_u16() as u32) << 16;
            pid |= rng.next_u16() as u32;

            let nature = (pid % 25) as u8;
            if !self.base.base.filter.compare_nature(nature) {
                continue;
            }

            let state = SearcherState::new(
                rng.next(),
                pid,
                ivs,
                (pid & 1) as u8,
                get_gender(pid, info),
                static_template.get_level(),
                nature,
                get_shiny(pid, self.base.base.tsv),
                info,
            );

            if self.base.base.filter.compare_searcher_state(&state) {
                states.push(state);
            }
        }

        states
    }
}
