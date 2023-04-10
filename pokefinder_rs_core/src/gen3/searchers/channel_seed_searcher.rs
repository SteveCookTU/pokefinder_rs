use crate::gen3::searchers::SeedSearcher;
use crate::rng::XDRNG;
use std::sync::atomic::Ordering;
use std::thread;

#[derive(Clone)]
pub struct ChannelSeedSearcher {
    pub base: SeedSearcher<Vec<u8>>,
}

impl ChannelSeedSearcher {
    pub fn new(criteria: Vec<u8>) -> Self {
        Self {
            base: SeedSearcher::new(criteria),
        }
    }

    pub fn get_progress(&self) -> u32 {
        self.base.progress.load(Ordering::SeqCst)
    }

    pub fn start_search(&self, threads: usize) {
        self.base.searching.store(true, Ordering::SeqCst);

        let mut thread_container = vec![];
        let split = (0xBFFFFFFE / threads) as u32;
        let mut start = 0x40000001u32;
        for i in 0..threads {
            let clone = self.clone();
            if i == threads - 1 {
                thread_container.push(thread::spawn(move || {
                    clone.search(start, 0xffffffff);
                }))
            } else {
                thread_container.push(thread::spawn(move || {
                    clone.search(start, start + split);
                }))
            }
            start = start.wrapping_add(split);
        }
    }

    pub fn search(&self, start: u32, end: u32) {
        let mut seeds = vec![];

        let mut seed = start;
        while seed < end {
            if !self.base.searching.load(Ordering::SeqCst) {
                return;
            }

            let mut rng = XDRNG::new(seed);
            if self.search_seed(&mut rng) {
                seeds.push(rng.seed);
            }

            seed += 1;
            self.base.progress.fetch_add(1, Ordering::SeqCst);
        }

        let mut lock = self.base.results.lock().unwrap();
        lock.append(&mut seeds);
    }

    pub fn search_seed(&self, rng: &mut XDRNG) -> bool {
        for &compare in self.base.criteria.iter() {
            let mut mask = 0;
            let mut shift = 0;
            let mut pattern = 0;

            while mask != 7 {
                let num = (rng.next() >> 30) as u8;
                if shift == 0 {
                    if (compare > 20 && num != 0) || (compare < 20 && num == 0) {
                        return false;
                    }

                    if num == 0 {
                        shift += 1;
                    }
                }

                if (mask & 1) == 0 && num == 1 {
                    pattern += num << shift;
                    shift += 1;
                    mask |= 1;
                } else if (mask & 2) == 0 && num == 2 {
                    pattern += num << shift;
                    shift += 1;
                    mask |= 2;
                } else if (mask & 4) == 0 && num == 3 {
                    pattern += num << shift;
                    shift += 1;
                    mask |= 4;
                }
            }

            if pattern != compare {
                return false;
            }
        }

        true
    }
}
