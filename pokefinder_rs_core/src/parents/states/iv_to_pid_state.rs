use crate::enums::Method;

/// State struct for PID to IV information
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct IVtoPIDState {
    pid: u32,
    seed: u32,
    sid: u16,
    method: Method,
}

impl IVtoPIDState {
    /// Construct a new [`IVtoPIDState`] struct
    pub fn new(seed: u32, pid: u32, sid: u16, method: Method) -> Self {
        Self {
            pid,
            seed,
            sid,
            method,
        }
    }

    /// Returns the generation method of the state
    pub fn get_method(&self) -> Method {
        self.method
    }

    /// Returns the PID of the state
    pub fn get_pid(&self) -> u32 {
        self.pid
    }

    /// Returns the seed of the state
    pub fn get_seed(&self) -> u32 {
        self.seed
    }

    /// Returns the SID of the state
    pub fn get_sid(&self) -> u16 {
        self.sid
    }
}
