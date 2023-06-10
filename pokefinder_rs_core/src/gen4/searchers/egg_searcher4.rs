use crate::gen4::generators::EggGenerator4;
use crate::gen4::states::EggSearcherState4;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex};

/// Egg encounter searcher for Gen4
#[derive(Clone)]
pub struct EggSearcher4 {
    results: Arc<Mutex<Vec<EggSearcherState4>>>,
    progress: Arc<AtomicU32>,
    max_delay: u32,
    min_delay: u32,
    searching: Arc<AtomicBool>,
}

impl EggSearcher4 {
    /// Construct a new ['EggSearcher4'] struct
    pub fn new(min_delay: u32, max_delay: u32) -> Self {
        Self {
            results: Arc::new(Mutex::new(vec![])),
            progress: Arc::new(AtomicU32::new(0)),
            max_delay,
            min_delay,
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
    pub fn get_results(&self) -> Vec<EggSearcherState4> {
        std::mem::take(self.results.lock().unwrap().as_mut())
    }

    /// Starts the search
    pub fn start_search(&self, generator: &EggGenerator4) {
        self.searching.store(true, Ordering::SeqCst);
        let mut total = 0;

        for ab in 0u32..256 {
            for cd in 0u32..24 {
                for efgh in self.min_delay..=self.max_delay {
                    if !self.searching.load(Ordering::SeqCst) {
                        return;
                    }

                    if total > 10000 {
                        self.progress.store(
                            256u32.wrapping_mul(24).wrapping_mul(
                                self.max_delay.wrapping_sub(self.min_delay).wrapping_add(1),
                            ),
                            Ordering::SeqCst,
                        );
                        return;
                    }

                    let seed = ((ab << 24) | (cd << 16)).wrapping_add(efgh);

                    let states = generator.generate(seed, seed);
                    if !states.is_empty() {
                        total += states.len();
                        self.results
                            .lock()
                            .unwrap()
                            .extend(states.into_iter().map(|s| EggSearcherState4::new(seed, s)));
                    }

                    self.progress.fetch_add(1, Ordering::SeqCst);
                }
            }
        }
    }
}
