use crate::enums::{Game, Method, ShadowType, Shiny};
use crate::gen3::filters::StateFilter3;
use crate::gen3::{shadow_lock, Profile3, ShadowTemplate};
use crate::parents::filters::Filter;
use crate::parents::searchers::Searcher;
use crate::parents::states::SearcherState;
use crate::parents::{PersonalInfo, Profile, StaticTemplate, Template};
use crate::rng::{lcrng_reverse, XDRNG, XDRNGR};
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

fn is_shiny(high: u16, low: u16, tsv: u16) -> bool {
    (high ^ low ^ tsv) < 8
}

fn validate_menu(mut rng: XDRNGR) -> bool {
    let target = (rng.seed >> 30) as u8;

    if target == 0 {
        return false;
    }

    let mut mask = 1 << target;
    while (mask & 14) != 14 {
        let num = (rng.next() >> 30) as u8;
        if num == target {
            return false;
        }
        mask |= 1 << num;
    }

    true
}

fn validate_jirachi(seed: u32) -> bool {
    let mut rng = XDRNGR::new(seed);

    let num1 = rng.next_u16();
    let num2 = rng.next_u16();
    let num3 = rng.next_u16();

    rng.advance(3);

    if num1 <= 0x4000 && validate_menu(rng) {
        return true;
    }

    rng.advance(1);
    if num2 > 0x4000 && num1 <= 0x547a && validate_menu(rng) {
        return true;
    }

    rng.advance(1);
    if num3 > 0x4000 && num2 > 0x547a && validate_menu(rng) {
        return true;
    }

    false
}

#[derive(Clone)]
pub struct GameCubeSearcher {
    pub base: Searcher<Profile3, StateFilter3>,
    pub results: Arc<Mutex<Vec<SearcherState>>>,
    pub searching: Arc<AtomicBool>,
    pub progress: Arc<AtomicU32>,
    pub unset: bool,
}

impl GameCubeSearcher {
    pub fn new(method: Method, unset: bool, profile: &Profile3, filter: &StateFilter3) -> Self {
        Self {
            base: Searcher::new(method, profile, filter),
            results: Arc::new(Mutex::new(vec![])),
            searching: Arc::new(AtomicBool::new(false)),
            progress: Arc::new(AtomicU32::new(0)),
            unset,
        }
    }

    pub fn cancel_search(&self) {
        self.searching.store(false, Ordering::SeqCst);
    }

    pub fn get_progress(&self) -> u32 {
        self.progress.load(Ordering::SeqCst)
    }

    pub fn get_results(&self) -> Vec<SearcherState> {
        std::mem::take(self.results.lock().unwrap().as_mut())
    }

    pub fn start_search_shadow(
        &self,
        min: [u8; 6],
        max: [u8; 6],
        shadow_template: &'static ShadowTemplate,
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

                                let mut states;
                                if (self.base.profile.get_version() & Game::COLOSSEUM) != Game::NONE
                                {
                                    states = self.search_colo_shadow(
                                        hp,
                                        atk,
                                        def,
                                        spa,
                                        spd,
                                        spe,
                                        shadow_template,
                                    );
                                } else {
                                    states = self.search_gales_shadow(
                                        hp,
                                        atk,
                                        def,
                                        spa,
                                        spd,
                                        spe,
                                        shadow_template,
                                    );
                                }

                                self.results.lock().unwrap().append(&mut states);
                                self.progress.fetch_add(1, Ordering::SeqCst);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn start_search_static(
        &self,
        min: [u8; 6],
        max: [u8; 6],
        static_template: &'static StaticTemplate,
    ) {
        self.searching.store(true, Ordering::SeqCst);

        if self.base.method == Method::Channel {
            self.search_channel(min[4], max[4], static_template);
            return;
        }

        let tsv = match static_template.species {
            25 | 251 => 31121,
            250 => 10048,
            _ => self.base.tsv,
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

                                let mut states = self.search_non_lock(
                                    hp,
                                    atk,
                                    def,
                                    spa,
                                    spd,
                                    spe,
                                    tsv,
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

    pub fn search_channel(
        &self,
        min_spd: u8,
        max_spd: u8,
        static_template: &'static StaticTemplate,
    ) {
        let info = static_template.get_info();

        for spd in min_spd..=max_spd {
            let lower = (spd as u32) << 27;
            let upper = lower | 0x7ffffff;

            let mut seed = lower as u64;
            while seed <= upper as u64 {
                if !self.searching.load(Ordering::SeqCst) {
                    return;
                }

                let mut rng = XDRNGR::new(seed as u32);

                let mut ivs = [0; 6];
                ivs[4] = spd;
                ivs[3] = (rng.next_u16() >> 11) as u8;
                ivs[5] = (rng.next_u16() >> 11) as u8;
                ivs[2] = (rng.next_u16() >> 11) as u8;
                ivs[1] = (rng.next_u16() >> 11) as u8;
                ivs[0] = (rng.next_u16() >> 11) as u8;

                if !self.base.filter.compare_iv(ivs) {
                    seed = seed.wrapping_add(1);
                    self.progress.fetch_add(1, Ordering::SeqCst);
                    continue;
                }

                rng.advance(3);
                let low = rng.next_u16();
                let mut high = rng.next_u16();
                let sid = rng.next_u16();
                const TID: u16 = 40122;

                if (TID ^ sid ^ high ^ u16::from(low < 8)) != 0 {
                    high ^= 0x8000;
                }

                let pid = ((high as u32) << 16) | (low as u32);
                let nature = (pid % 25) as u8;
                if !self.base.filter.compare_nature(nature) || !validate_jirachi(rng.next()) {
                    seed = seed.wrapping_add(seed);
                    self.progress.fetch_add(1, Ordering::SeqCst);
                    continue;
                }

                let state = SearcherState::new(
                    rng.seed,
                    pid,
                    ivs,
                    (pid & 1) as u8,
                    2,
                    static_template.level,
                    nature,
                    get_shiny(pid, TID ^ sid),
                    info,
                );

                if self.base.filter.compare_searcher_state(&state) {
                    self.results.lock().unwrap().push(state);
                }

                seed += 1;
                self.progress.fetch_add(1, Ordering::SeqCst);
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn search_colo_shadow(
        &self,
        hp: u8,
        atk: u8,
        def: u8,
        spa: u8,
        spd: u8,
        spe: u8,
        shadow_template: &'static ShadowTemplate,
    ) -> Vec<SearcherState> {
        let mut states = vec![];

        let info = shadow_template.get_info();
        let ivs = [hp, atk, def, spa, spd, spe];

        let mut seeds = [0; 6];
        let size = lcrng_reverse::recover_xdrng_iv(hp, atk, def, spa, spd, spe, &mut seeds);
        let mut i = 0;
        while i < size {
            let mut rng = XDRNG::new(seeds[i]);

            rng.advance(1);
            let ability = (rng.next_u16_max::<true>(2) as u8)
                & ((info.get_ability(0) != info.get_ability(1)) as u8);
            let mut pid = (rng.next_u16() as u32) << 16;
            pid |= rng.next_u16() as u32;

            let nature = (pid % 25) as u8;
            if !self.base.filter.compare_nature(nature) {
                i += 1;
                continue;
            }

            let mut valid = false;
            let seed = XDRNGR::new(seeds[i]).next();
            match shadow_template.get_type() {
                ShadowType::FirstShadow => valid = shadow_lock::colo_shadow(seed, shadow_template),
                ShadowType::EReader => valid = shadow_lock::ereader(seed, pid, shadow_template),
                _ => {}
            }

            if valid {
                if i & 2 == 0 {
                    i += 1;
                }

                let state = SearcherState::new(
                    seed,
                    pid,
                    ivs,
                    ability,
                    get_gender(pid, info),
                    shadow_template.get_level(),
                    nature,
                    get_shiny(pid, self.base.tsv),
                    info,
                );

                if self.base.filter.compare_searcher_state(&state) {
                    states.push(state);
                }
            }
            i += 1;
        }

        states
    }

    #[allow(clippy::too_many_arguments)]
    pub fn search_gales_shadow(
        &self,
        hp: u8,
        atk: u8,
        def: u8,
        spa: u8,
        spd: u8,
        spe: u8,
        shadow_template: &'static ShadowTemplate,
    ) -> Vec<SearcherState> {
        let mut states = vec![];

        let info = shadow_template.get_info();
        let ivs = [hp, atk, def, spa, spd, spe];

        let mut seeds = [0; 6];
        let size = lcrng_reverse::recover_xdrng_iv(hp, atk, def, spa, spd, spe, &mut seeds);
        let mut i = 0;
        while i < size {
            let mut rng = XDRNG::new(seeds[i]);

            rng.advance(1);
            let ability = (rng.next_u16_max::<true>(2) as u8)
                & ((info.get_ability(0) != info.get_ability(1)) as u8);
            let mut high = rng.next_u16();
            let mut low = rng.next_u16();
            while is_shiny(high, low, self.base.tsv) {
                high = rng.next_u16();
                low = rng.next_u16();
            }

            let pid = ((high as u32) << 16) | (low as u32);
            let nature = (pid % 25) as u8;
            if !self.base.filter.compare_nature(nature) {
                i += 1;
                continue;
            }

            let mut valid = false;
            let seed = XDRNGR::new(seeds[i]).next();
            match shadow_template.get_type() {
                ShadowType::SingleLock => {
                    valid = shadow_lock::single_nl(seed, self.base.tsv, shadow_template)
                }
                ShadowType::FirstShadow => {
                    valid = shadow_lock::first_shadow_normal(seed, self.base.tsv, shadow_template)
                }
                ShadowType::SecondShadow => {
                    if self.unset {
                        valid =
                            shadow_lock::first_shadow_unset(seed, self.base.tsv, shadow_template);
                    } else {
                        valid = shadow_lock::first_shadow_set(seed, self.base.tsv, shadow_template);
                    }
                }
                ShadowType::Salamence => {
                    if self.unset {
                        valid = shadow_lock::salamence_unset(seed, self.base.tsv, shadow_template);
                    } else {
                        valid = shadow_lock::salamence_set(seed, self.base.tsv, shadow_template);
                    }
                }
                _ => {}
            }

            if valid {
                if i & 2 == 0 {
                    i += 1;
                }

                let state = SearcherState::new(
                    seed,
                    pid,
                    ivs,
                    ability,
                    get_gender(pid, info),
                    shadow_template.get_level(),
                    nature,
                    0,
                    info,
                );

                if self.base.filter.compare_searcher_state(&state) {
                    states.push(state);
                }
            }
            i += 1;
        }

        states
    }

    #[allow(clippy::too_many_arguments)]
    pub fn search_non_lock(
        &self,
        hp: u8,
        atk: u8,
        def: u8,
        spa: u8,
        spd: u8,
        spe: u8,
        mut tsv: u16,
        static_template: &'static StaticTemplate,
    ) -> Vec<SearcherState> {
        let mut states = vec![];
        let info = static_template.get_info();
        let ivs = [hp, atk, def, spa, spd, spe];

        let mut seeds = [0; 6];
        let size = lcrng_reverse::recover_xdrng_iv(hp, atk, def, spa, spd, spe, &mut seeds);
        let mut i = 0;
        while i < size {
            let mut rng = XDRNG::new(seeds[i]);

            let mut seed = XDRNGR::new(seeds[i]).next();
            let mut ability;
            let mut high;
            let mut low;

            if static_template.species == 133 {
                let mut temp = XDRNGR::new(rng.seed);
                temp.advance(2);
                tsv = temp.next_u16() ^ temp.next_u16();
                seed = temp.next();
            }

            match static_template.species {
                197 => {
                    let mut temp = XDRNGR::new(rng.seed);
                    temp.advance(2);
                    tsv = temp.next_u16() ^ temp.next_u16();
                    seed = temp.next();

                    rng.advance(1);
                    ability = rng.next_u16_max::<true>(2) as u8;
                    high = rng.next_u16();
                    low = rng.next_u16();

                    while is_shiny(high, low, tsv) || ((low & 255) as u8) < info.get_gender() {
                        high = rng.next_u16();
                        low = rng.next_u16();
                    }
                }
                196 => {
                    let mut temp = XDRNGR::new(rng.seed);
                    temp.advance(2);
                    let mut original_umbreon_pid = temp.next_u16() as u32;
                    original_umbreon_pid |= (temp.next_u16() as u32) << 16;

                    if ((original_umbreon_pid & 255) as u8) < info.get_gender() {
                        i += 1;
                        continue;
                    }

                    while (temp.next_u16_max::<true>(256) as u8) < info.get_gender() {
                        temp.advance(1);
                    }

                    let mut test = XDRNG::new(temp.seed);
                    temp.advance(2);
                    tsv = temp.next_u16() ^ temp.next_u16();
                    seed = temp.next();

                    let mut umbreon_high = test.next_u16();
                    let mut umbreon_low = test.next_u16();

                    while is_shiny(umbreon_high, umbreon_low, tsv)
                        || ((umbreon_low & 255) as u8) < info.get_gender()
                    {
                        umbreon_high = test.next_u16();
                        umbreon_low = test.next_u16();
                    }
                    let umbreon_pid = ((umbreon_high as u32) << 16) | (umbreon_low as u32);
                    if original_umbreon_pid != umbreon_pid {
                        i += 1;
                        continue;
                    }

                    rng.advance(1);
                    ability = rng.next_u16_max::<true>(2) as u8;
                    high = rng.next_u16();
                    low = rng.next_u16();

                    while is_shiny(high, low, tsv) || ((low & 255) as u8) < info.get_gender() {
                        high = rng.next_u16();
                        low = rng.next_u16();
                    }
                }
                _ => {
                    rng.advance(1);
                    ability = rng.next_u16_max::<true>(2) as u8;
                    high = rng.next_u16();
                    low = rng.next_u16();
                    if static_template.shiny == Shiny::Never {
                        while is_shiny(high, low, tsv) {
                            high = rng.next_u16();
                            low = rng.next_u16();
                        }
                    }
                }
            }

            let pid = ((high as u32) << 16) | (low as u32);

            let nature = (pid % 25) as u8;
            if !self.base.filter.compare_nature(nature) {
                i += 1;
                continue;
            }

            ability &= (info.get_ability(0) != info.get_ability(1)) as u8;

            let state = SearcherState::new(
                seed,
                pid,
                ivs,
                ability,
                get_gender(pid, info),
                static_template.level,
                nature,
                get_shiny(pid, tsv),
                info,
            );

            if self.base.filter.compare_searcher_state(&state) {
                states.push(state);
            }

            i += 1;
        }

        states
    }
}
