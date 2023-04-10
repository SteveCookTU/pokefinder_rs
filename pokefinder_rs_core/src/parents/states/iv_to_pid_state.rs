use crate::enums::Method;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct IVtoPIDState {
    pid: u32,
    seed: u32,
    sid: u16,
    method: Method,
}

impl IVtoPIDState {
    pub fn new(seed: u32, pid: u32, sid: u16, method: Method) -> Self {
        Self {
            pid,
            seed,
            sid,
            method,
        }
    }

    pub fn get_method(&self) -> Method {
        self.method
    }

    pub fn get_pid(&self) -> u32 {
        self.pid
    }

    pub fn get_seed(&self) -> u32 {
        self.seed
    }

    pub fn get_sid(&self) -> u16 {
        self.sid
    }
}
