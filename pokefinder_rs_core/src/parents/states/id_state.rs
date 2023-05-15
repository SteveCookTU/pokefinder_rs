/// Trait containing common functions for ID states
pub trait IDStateT {
    /// Returns the advances of the state
    fn get_advances(&self) -> u32;
    /// Returns the SID of the state
    fn get_sid(&self) -> u16;
    /// Returns the TID of the state
    fn get_tid(&self) -> u16;
    /// Returns the TSV of the state
    fn get_tsv(&self) -> u16;
}

/// State struct that contains all the common information for TID/SID across each game
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct IDState {
    /// State advances
    pub advances: u32,
    /// State SID
    pub sid: u16,
    /// State TID
    pub tid: u16,
    /// State TSV
    pub tsv: u16,
}

impl IDState {
    /// Construct a new [`IDState`] struct with 0 advances
    pub fn new(tid: u16, sid: u16, tsv: u16) -> Self {
        Self {
            advances: 0,
            sid,
            tid,
            tsv,
        }
    }

    /// Construct a new [`IDState`] struct with advances
    pub fn new_with_advances(advances: u32, tid: u16, sid: u16, tsv: u16) -> Self {
        Self {
            advances,
            sid,
            tid,
            tsv,
        }
    }
}

impl IDStateT for IDState {
    fn get_advances(&self) -> u32 {
        self.advances
    }

    fn get_sid(&self) -> u16 {
        self.sid
    }

    fn get_tid(&self) -> u16 {
        self.tid
    }

    fn get_tsv(&self) -> u16 {
        self.tsv
    }
}
