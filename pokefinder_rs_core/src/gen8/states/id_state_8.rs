use crate::parents::states::IDStateT;

pub struct IDState8 {
    advances: u32,
    sid: u16,
    tid: u16,
    tsv: u16,
    display_tid: u32,
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
    pub fn new(advances: u32, tid: u16, sid: u16, display_tid: u32) -> Self {
        Self {
            advances,
            sid,
            tid,
            display_tid,
            tsv: (tid ^ sid) >> 4,
        }
    }

    pub fn get_display_tid(&self) -> u32 {
        self.display_tid
    }
}
