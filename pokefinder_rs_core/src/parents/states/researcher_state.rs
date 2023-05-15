/// Contains the information to display custom calculations for PRNG states
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct ResearcherState {
    custom: [u64; 10],
    prng: u64,
    advances: u32,
    rng_64bit: bool,
}

impl ResearcherState {
    /// Construct a new [`ResearcherState`] struct
    pub fn new(advances: u32, prng: u64, rng_64bit: bool) -> Self {
        Self {
            custom: [0; 10],
            prng,
            advances,
            rng_64bit,
        }
    }

    /// Returns the advances of the state
    pub fn get_advances(&self) -> u32 {
        self.advances
    }

    /// Returns the custom value at `index`
    pub fn get_custom(&self, index: usize) -> u64 {
        self.custom[index]
    }

    /// Sets the custom value a `index`
    pub fn set_custom(&mut self, index: usize, custom: u64) {
        self.custom[index] = custom;
    }

    /// Computes the high 32bits of the PRNG value
    pub fn get_high32(&self) -> u32 {
        (self.prng >> 32) as u32
    }

    /// Computes the high 16bits of the PRNG value
    pub fn get_high16(&self) -> u32 {
        if self.rng_64bit {
            self.get_high32() >> 16
        } else {
            (self.prng >> 16) as u32
        }
    }

    /// Computes the low 32bits of the PRNG value
    pub fn get_low32(&self) -> u32 {
        self.prng as u32
    }

    /// Computes the low 16bits of the PRNG value
    pub fn get_low16(&self) -> u32 {
        if self.rng_64bit {
            self.get_high32() & 0xFFFF
        } else {
            (self.prng & 0xFFFF) as u32
        }
    }

    /// Returns the PRNG value of the state
    pub fn get_prng(&self) -> u64 {
        self.prng
    }
}
