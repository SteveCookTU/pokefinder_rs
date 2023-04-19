use crate::parents::states::{IDState, IDStateT};

#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct IDState4 {
    pub base: IDState,
    pub delay: u32,
    pub seed: u32,
    pub seconds: u8,
}

impl IDState4 {
    pub fn new_with_seconds(seed: u32, delay: u32, tid: u16, sid: u16, seconds: u8) -> Self {
        Self {
            base: IDState::new(tid, sid, (tid ^ sid) >> 3),
            delay,
            seed,
            seconds,
        }
    }

    pub fn new(seed: u32, delay: u32, tid: u16, sid: u16) -> Self {
        Self {
            base: IDState::new(tid, sid, (tid ^ sid) >> 3),
            delay,
            seed,
            seconds: 0,
        }
    }
}

impl IDStateT for IDState4 {
    fn get_advances(&self) -> u32 {
        self.base.advances
    }

    fn get_sid(&self) -> u16 {
        self.base.sid
    }

    fn get_tid(&self) -> u16 {
        self.base.tid
    }

    fn get_tsv(&self) -> u16 {
        self.base.tsv
    }
}
