use crate::parents::states::IDStateT;

/// Contains additional information for Gen 8 TID/SID
#[derive(Copy, Clone)]
pub struct IDState8 {
    /// Advances of the state
    pub advances: u32,
    /// Secret ID
    pub sid: u16,
    /// Trainer ID
    pub tid: u16,
    /// Trainer TSV
    pub tsv: u16,
    /// Displayed trainer ID
    pub display_tid: u32,
}

impl IDStateT for IDState8 {
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

impl IDState8 {
    /// Construct a new [`IDState8`] struct
    pub fn new(advances: u32, tid: u16, sid: u16, display_tid: u32) -> Self {
        Self {
            advances,
            sid,
            tid,
            display_tid,
            tsv: (tid ^ sid) >> 4,
        }
    }

    /// Returns the display TID of the state
    pub fn get_display_tid(&self) -> u32 {
        self.display_tid
    }
}
