use crate::enums::{Lead, Method, Shiny};
use crate::gen4::filters::StateFilter4;
use crate::gen4::states::SearcherState4;
use crate::gen4::{Profile4, StaticTemplate4};
use crate::parents::filters::Filter;
use crate::parents::searchers::StaticSearcher;
use crate::parents::{PersonalInfo, Template};
use crate::rng::{lcrng_reverse, PokeRNGR, ARNG};
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

#[derive(Clone)]
pub struct StaticSearcher4 {
    pub base: StaticSearcher<Profile4, StateFilter4>,
    pub results: Arc<Mutex<Vec<SearcherState4>>>,
    pub progress: Arc<AtomicU32>,
    pub max_advance: u32,
    pub min_advance: u32,
    pub max_delay: u32,
    pub min_delay: u32,
    pub searching: Arc<AtomicBool>,
}

impl StaticSearcher4 {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        min_advance: u32,
        max_advance: u32,
        min_delay: u32,
        max_delay: u32,
        method: Method,
        lead: Lead,
        profile: &Profile4,
        filter: &StateFilter4,
    ) -> Self {
        Self {
            base: StaticSearcher::new(method, lead, profile, filter),
            results: Arc::new(Mutex::new(vec![])),
            progress: Arc::new(AtomicU32::new(0)),
            max_advance,
            min_advance,
            max_delay,
            min_delay,
            searching: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn cancel_search(&self) {
        self.searching.store(false, Ordering::SeqCst);
    }

    pub fn get_progress(&self) -> u32 {
        self.progress.load(Ordering::SeqCst)
    }

    pub fn get_results(&self) -> Vec<SearcherState4> {
        std::mem::take(self.results.lock().unwrap().as_mut())
    }

    pub fn start_search(
        &self,
        min: [u8; 6],
        max: [u8; 6],
        static_template: &'static StaticTemplate4,
    ) {
        self.searching.store(true, Ordering::SeqCst);
        let buffer = if self.base.lead == Lead::CUTE_CHARM_F {
            25u16
                .wrapping_mul((static_template.get_info().get_gender() as u16 / 25).wrapping_add(1))
                as u8
        } else {
            0
        };

        for hp in min[0]..=max[0] {
            for atk in min[1]..=max[1] {
                for def in min[2]..=max[2] {
                    for spa in min[3]..=max[3] {
                        for spd in min[4]..=max[4] {
                            for spe in min[5]..=max[5] {
                                if !self.searching.load(Ordering::SeqCst) {
                                    return;
                                }

                                let mut states = self.search(
                                    hp,
                                    atk,
                                    def,
                                    spa,
                                    spd,
                                    spe,
                                    buffer,
                                    static_template,
                                );
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
        buffer: u8,
        static_template: &'static StaticTemplate4,
    ) -> Vec<SearcherState4> {
        let states = match self.base.base.method {
            Method::Method1 => self.search_method_1(hp, atk, def, spa, spd, spe, static_template),
            Method::MethodJ => {
                self.search_method_j(hp, atk, def, spa, spd, spe, buffer, static_template)
            }
            Method::MethodK => {
                self.search_method_k(hp, atk, def, spa, spd, spe, buffer, static_template)
            }
            _ => vec![],
        };

        self.search_initial_seeds(states)
    }

    fn search_initial_seeds(&self, results: Vec<SearcherState4>) -> Vec<SearcherState4> {
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

    #[allow(clippy::too_many_arguments)]
    fn search_method_1(
        &self,
        hp: u8,
        atk: u8,
        def: u8,
        spa: u8,
        spd: u8,
        spe: u8,
        static_template: &'static StaticTemplate4,
    ) -> Vec<SearcherState4> {
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
            Method::Method1,
        );
        for rev_seed in seeds.into_iter().take(size) {
            let mut rng = PokeRNGR::new(rev_seed);
            let mut pid;

            if static_template.get_shiny() == Shiny::Always {
                let mut low = 0;
                for j in (3..=15).rev() {
                    low |= rng.next_u16_max::<true>(2) << j;
                }
                let mut high = rng.next_u16_max::<true>(8);
                low |= rng.next_u16_max::<true>(8);
                high |= (low ^ self.base.base.tsv) & 0xFFF8;

                pid = ((high as u32) << 16) | (low as u32);
            } else {
                pid = (rng.next_u16() as u32) << 16;
                pid |= rng.next_u16() as u32;

                if static_template.get_shiny() == Shiny::Never {
                    while is_shiny(pid, self.base.base.tsv) {
                        pid = ARNG::new(pid).next();
                    }
                }
            }

            let nature = (pid % 25) as u8;
            if !self.base.base.filter.compare_nature(nature) {
                continue;
            }

            let state = SearcherState4::new(
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

    #[allow(clippy::too_many_arguments)]
    fn search_method_j(
        &self,
        hp: u8,
        atk: u8,
        def: u8,
        spa: u8,
        spd: u8,
        spe: u8,
        buffer: u8,
        static_template: &'static StaticTemplate4,
    ) -> Vec<SearcherState4> {
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
            Method::Method1,
        );
        for rev_seed in seeds.into_iter().take(size) {
            let mut rng = PokeRNGR::new(rev_seed);
            if matches!(self.base.lead, Lead::CUTE_CHARM_F | Lead::CUTE_CHARM_M) {
                let nature = rng.next_u16_max::<false>(25) as u8;
                if rng.next_u16_max::<false>(3) != 0 {
                    if !self.base.base.filter.compare_nature(nature) {
                        continue;
                    }
                    let pid = nature as u32 + buffer as u32;
                    let state = SearcherState4::new(
                        rng.next(),
                        pid,
                        ivs,
                        (pid & 1) as u8,
                        get_gender(pid, info),
                        static_template.get_level(),
                        nature,
                        get_shiny(pid, self.base.base.tsv),
                        static_template.get_info(),
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
                    let mut seed = 0;
                    let mut valid = false;
                    if self.base.lead == Lead::NONE && ((next_rng / 0xa3e) as u8) == nature {
                        seed = rng.seed;
                        valid = true;
                    } else if self.base.lead == Lead::SYNCHRONIZE {
                        if (next_rng >> 15) == 0 {
                            seed = rng.seed;
                            valid = true;
                        } else if (next_rng2 >> 15) == 1 && ((next_rng / 0xa3e) as u8) == nature {
                            seed = PokeRNGR::new(rng.seed).next();
                            valid = true;
                        }
                    }

                    if valid {
                        let state = SearcherState4::new(
                            seed,
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
    fn search_method_k(
        &self,
        hp: u8,
        atk: u8,
        def: u8,
        spa: u8,
        spd: u8,
        spe: u8,
        buffer: u8,
        static_template: &'static StaticTemplate4,
    ) -> Vec<SearcherState4> {
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
            Method::Method1,
        );
        for rev_seed in seeds.into_iter().take(size) {
            let mut rng = PokeRNGR::new(rev_seed);
            if matches!(self.base.lead, Lead::CUTE_CHARM_F | Lead::CUTE_CHARM_M) {
                let nature = rng.next_u16_max::<true>(25) as u8;
                if rng.next_u16_max::<true>(3) != 0 {
                    if !self.base.base.filter.compare_nature(nature) {
                        continue;
                    }
                    let pid = nature as u32 + buffer as u32;
                    let state = SearcherState4::new(
                        rng.next(),
                        pid,
                        ivs,
                        (pid & 1) as u8,
                        get_gender(pid, info),
                        static_template.get_level(),
                        nature,
                        get_shiny(pid, self.base.base.tsv),
                        static_template.get_info(),
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
                    let mut seed = 0;
                    let mut valid = false;
                    if self.base.lead == Lead::NONE && ((next_rng % 25) as u8) == nature {
                        seed = rng.seed;
                        valid = true;
                    } else if self.base.lead == Lead::SYNCHRONIZE {
                        if (next_rng % 2) == 0 {
                            seed = rng.seed;
                            valid = true;
                        } else if (next_rng2 % 2) == 1 && ((next_rng % 25) as u8) == nature {
                            seed = PokeRNGR::new(rng.seed).next();
                            valid = true;
                        }
                    }

                    if valid {
                        let state = SearcherState4::new(
                            seed,
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

                    hunt_nature = ((((next_rng as u32) << 16) | (next_rng2 as u32)) % 25) as u8;
                    next_rng = rng.next_u16();
                    next_rng2 = rng.next_u16();
                    hunt_nature != nature
                } {}
            }
        }

        states
    }
}
