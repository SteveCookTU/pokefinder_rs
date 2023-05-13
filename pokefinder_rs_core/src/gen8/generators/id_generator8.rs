use crate::gen8::states::IDState8;
use crate::parents::filters::IDFilter;
use crate::parents::generators::IDGenerator;
use crate::rng::Xorshift;

pub struct IDGenerator8<'a> {
    pub base: IDGenerator<'a>,
}

impl<'a> IDGenerator8<'a> {
    pub fn new(initial_advances: u32, max_advances: u32, filter: &'a IDFilter) -> Self {
        Self {
            base: IDGenerator::new(initial_advances, max_advances, filter),
        }
    }

    pub fn generate(&self, seed0: u64, seed1: u64) -> Vec<IDState8> {
        let mut rng = Xorshift::new_with_initial_advances(seed0, seed1, self.base.initial_advances);

        let mut states = vec![];
        for cnt in 0..self.base.max_advances {
            let mut sidtid = rng.next_range(0x80000000, 0x7fffffff);
            if sidtid == 0 {
                let mut gen = rng;
                while sidtid == 0 {
                    sidtid = gen.next_range(0x80000000, 0x7fffffff);
                }
            }

            let tid = (sidtid & 0xFFFF) as u16;
            let sid = (sidtid >> 16) as u16;
            let display_tid = sidtid % 1000000;

            let state = IDState8::new(
                self.base.initial_advances.wrapping_add(cnt),
                tid,
                sid,
                display_tid,
            );
            if self.base.filter.compare_8(&state) {
                states.push(state);
            }
        }

        states
    }
}
