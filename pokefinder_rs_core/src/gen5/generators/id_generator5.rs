use crate::gen5::Profile5;
use crate::parents::filters::IDFilter;
use crate::parents::generators::IDGenerator;
use crate::parents::states::IDState;
use crate::parents::Profile;
use crate::rng::BWRNG;
use crate::util;

#[derive(Copy, Clone)]
pub struct IDGenerator5<'a, 'b> {
    pub base: IDGenerator<'a>,
    pub profile: &'b Profile5,
}

impl<'a, 'b> IDGenerator5<'a, 'b> {
    pub fn new(
        initial_advances: u32,
        max_advances: u32,
        profile: &'b Profile5,
        filter: &'a IDFilter,
    ) -> Self {
        Self {
            base: IDGenerator::new(initial_advances, max_advances, filter),
            profile,
        }
    }

    pub fn generate(&self, seed: u64, pid: u32, check_pid: bool, check_xor: bool) -> Vec<IDState> {
        let pid_bit = ((pid >> 31) ^ (pid & 1)) as u8;
        let psv = ((pid >> 16) ^ (pid & 0xFFFF)) as u16;

        let advances = util::utilities5::initial_advances_id(seed, self.profile.get_version());
        let mut rng = BWRNG::new_with_initial_advances(
            seed,
            advances.wrapping_add(self.base.initial_advances),
        );

        let mut states = vec![];
        for cnt in 0..=self.base.max_advances {
            let rand = rng.next_u32_max(0xFFFFFFFF);
            let tid = (rand & 0xFFFF) as u16;
            let sid = (rand >> 16) as u16;
            let tsv = (tid ^ sid) >> 3;

            let state = IDState::new_with_advances(
                advances
                    .wrapping_add(self.base.initial_advances)
                    .wrapping_add(cnt)
                    .wrapping_add(1),
                tid,
                sid,
                tsv,
            );
            if self.base.filter.compare(&state) {
                let mut shiny = (psv >> 3) == state.tsv;

                if shiny && check_xor {
                    let id_bit = ((tid & 1) ^ (sid & 1)) as u8;
                    shiny = id_bit == pid_bit;
                }

                if !check_pid || shiny {
                    states.push(state);
                }
            }
        }

        states
    }
}
