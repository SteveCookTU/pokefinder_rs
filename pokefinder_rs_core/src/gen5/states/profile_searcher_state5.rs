#[derive(Copy, Clone)]
pub struct ProfileSearcherState5 {
    pub seed: u64,
    pub timer0: u16,
    pub gxstat: u8,
    pub second: u8,
    pub vcount: u8,
    pub vframe: u8,
}

impl ProfileSearcherState5 {
    pub fn new(seed: u64, timer0: u16, vcount: u8, vframe: u8, gxstat: u8, second: u8) -> Self {
        Self {
            seed,
            timer0,
            gxstat,
            second,
            vcount,
            vframe,
        }
    }
}
