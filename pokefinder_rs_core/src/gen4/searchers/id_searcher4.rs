use crate::gen4::states::IDState4;
use crate::parents::filters::IDFilter;
use crate::rng::MTFast;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex};

/// TID/SID searcher for Gen4
#[derive(Clone)]
pub struct IDSearcher4 {
    filter: Arc<IDFilter>,
    results: Arc<Mutex<Vec<IDState4>>>,
    progress: Arc<AtomicU32>,
    searching: Arc<AtomicBool>,
}

impl IDSearcher4 {
    /// Constructs a new [`IDSearcher4`] struct
    pub fn new(filter: &IDFilter) -> Self {
        Self {
            filter: Arc::new(filter.clone()),
            results: Arc::new(Mutex::new(vec![])),
            progress: Arc::new(AtomicU32::new(0)),
            searching: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Cancels the running search
    pub fn cancel_search(&self) {
        self.searching.store(false, Ordering::SeqCst);
    }

    /// Returns the progress of the running search
    pub fn get_progress(&self) -> u32 {
        self.progress.load(Ordering::SeqCst)
    }

    /// Returns the states of the running search
    pub fn get_results(&self) -> Vec<IDState4> {
        std::mem::take(self.results.lock().unwrap().as_mut())
    }

    /// Starts the search
    pub fn start_search(&self, infinite: bool, year: u16, min_delay: u32, mut max_delay: u32) {
        self.searching.store(true, Ordering::SeqCst);
        max_delay = if infinite { 0xe8ffff } else { max_delay };

        for efgh in min_delay..=max_delay {
            for ab in 0u32..256 {
                for cd in 0u32..24 {
                    if !self.searching.load(Ordering::SeqCst) {
                        return;
                    }

                    let seed = ((ab << 24) | (cd << 16)).wrapping_add(efgh);
                    let mut mt = MTFast::<2, 8, false>::new(seed, 1);

                    let sid_tid = mt.next();
                    let tid = (sid_tid & 0xFFFF) as u16;
                    let sid = (sid_tid >> 16) as u16;

                    let state = IDState4::new(
                        seed,
                        efgh.wrapping_add(2000).wrapping_sub(year as u32),
                        tid,
                        sid,
                    );
                    if self.filter.compare(&state) {
                        self.results.lock().unwrap().push(state);
                    }

                    self.progress.fetch_add(1, Ordering::SeqCst);
                }
            }
        }
    }
}
