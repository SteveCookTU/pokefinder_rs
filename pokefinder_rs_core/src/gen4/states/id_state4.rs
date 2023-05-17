use crate::parents::states::{IDState, IDStateT};

/// State struct for Gen4 TID/SID
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct IDState4 {
    /// Base ID state data
    pub base: IDState,
    /// State delay
    pub delay: u32,
    /// State seed
    pub seed: u32,
    /// State seconds
    pub seconds: u8,
}

impl IDState4 {
    /// Construct a new [`IDState4`] struct including seconds
    pub fn new_with_seconds(seed: u32, delay: u32, tid: u16, sid: u16, seconds: u8) -> Self {
        Self {
            base: IDState::new(tid, sid, (tid ^ sid) >> 3),
            delay,
            seed,
            seconds,
        }
    }

    /// Construct a new [`IDState4`] struct
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
