pub trait IDStateT {
    fn get_advances(&self) -> u32;
    fn get_sid(&self) -> u16;
    fn get_tid(&self) -> u16;
    fn get_tsv(&self) -> u16;
}

#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct IDState {
    advances: u32,
    sid: u16,
    tid: u16,
    tsv: u16,
}

impl IDState {
    pub fn new(advances: u32, tid: u16, sid: u16, tsv: u16) -> Self {
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
