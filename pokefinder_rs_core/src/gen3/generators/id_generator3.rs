use crate::parents::filters::IDFilter;
use crate::parents::generators::IDGenerator;
use crate::parents::states::IDState;
use crate::rng::{PokeRNG, XDRNG};

#[derive(Copy, Clone)]
pub struct IDGenerator3<'a> {
    pub base: IDGenerator<'a>,
}

impl<'a> IDGenerator3<'a> {
    pub fn new(initial_advances: u32, max_advances: u32, filter: &'a IDFilter) -> Self {
        Self {
            base: IDGenerator::new(initial_advances, max_advances, filter),
        }
    }

    pub fn generate_xd_colo(&self, seed: u32) -> Vec<IDState> {
        let mut states = vec![];

        let mut rng = XDRNG::new_with_initial_advances(seed, self.base.initial_advances);
        let mut cnt = 0;
        while cnt <= self.base.max_advances {
            let mut go = rng;

            let tid = go.next_u16();
            let sid = go.next_u16();
            let tsv = (tid ^ sid) >> 3;

            let state = IDState::new(self.base.initial_advances + cnt, tid, sid, tsv);
            if self.base.filter.compare(&state) {
                states.push(state);
            }

            cnt += 1;
            rng.next();
        }

        states
    }

    pub fn generate_frlge(&self, tid: u16) -> Vec<IDState> {
        let mut states = vec![];

        let mut rng = PokeRNG::new_with_initial_advances(tid as u32, self.base.initial_advances);
        let mut cnt = 0;
        while cnt <= self.base.max_advances {
            let mut go = rng;

            let sid = go.next_u16();
            let tsv = (tid ^ sid) >> 3;

            let state = IDState::new(self.base.initial_advances + cnt, tid, sid, tsv);
            if self.base.filter.compare(&state) {
                states.push(state);
            }

            cnt += 1;
            rng.next();
        }

        states
    }

    pub fn generate_rs(&self, seed: u16) -> Vec<IDState> {
        let mut states = vec![];

        let mut rng = PokeRNG::new_with_initial_advances(seed as u32, self.base.initial_advances);
        let mut cnt = 0;
        while cnt <= self.base.max_advances {
            let mut go = rng;

            let tid = go.next_u16();
            let sid = go.next_u16();
            let tsv = (tid ^ sid) >> 3;

            let state = IDState::new(self.base.initial_advances + cnt, tid, sid, tsv);
            if self.base.filter.compare(&state) {
                states.push(state);
            }

            cnt += 1;
            rng.next();
        }

        states
    }
}
