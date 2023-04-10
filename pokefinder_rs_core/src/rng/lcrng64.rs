#[derive(Copy, Clone)]
pub struct JumpTable64 {
    pub add: [u64; 32],
    pub mult: [u64; 32],
}

impl JumpTable64 {
    pub const fn new() -> Self {
        Self {
            add: [0; 32],
            mult: [0; 32],
        }
    }
}

const fn compute_jump_table(add: u64, mult: u64) -> JumpTable64 {
    let mut table = JumpTable64::new();
    table.add[0] = add;
    table.mult[0] = mult;

    let mut i = 1;
    while i < 32 {
        table.add[i] = table.add[i - 1].wrapping_mul(table.mult[i - 1].wrapping_add(1));
        table.mult[i] = table.mult[i - 1].wrapping_mul(table.mult[i - 1]);
        i += 1;
    }

    table
}

pub static BW_RNG_TABLE: JumpTable64 = compute_jump_table(0x269ec3, 0x5d588b656c078965);
pub static BW_RNGR_TABLE: JumpTable64 = compute_jump_table(0x9b1ae6e9a384e6f9, 0xdedcedae9638806d);

#[derive(Copy, Clone, Default)]
pub struct LCRNG64<const ADD: u64, const MULT: u64> {
    pub seed: u64,
}

impl<const ADD: u64, const MULT: u64> LCRNG64<ADD, MULT> {
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }

    pub fn new_with_initial_advances(seed: u64, advances: u32) -> Self {
        let mut rng = Self { seed };
        rng.jump(advances);
        rng
    }

    pub fn copy_and_advance(&self, advances: u32) -> Self {
        let mut new = *self;
        new.jump(advances);
        new
    }

    pub fn advance(&mut self, advances: u32) -> u64 {
        for _ in 0..advances {
            self.next();
        }
        self.seed
    }

    pub fn jump(&mut self, mut advances: u32) -> u64 {
        let table = match ADD {
            0x269ec3 => &BW_RNG_TABLE,
            _ => {
                assert_eq!(0x9b1ae6e9a384e6f9, ADD, "Unsupported LCRNG64");
                &BW_RNGR_TABLE
            }
        };

        let mut i = 0;
        while advances > 0 {
            if advances & 1 == 1 {
                self.seed = self
                    .seed
                    .wrapping_mul(table.mult[i])
                    .wrapping_add(table.add[i]);
            }
            advances >>= 1;
            i += 1;
        }
        self.seed
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> u64 {
        self.seed = self.seed.wrapping_mul(MULT).wrapping_add(ADD);
        self.seed
    }

    pub fn next_u32(&mut self) -> u32 {
        (self.next() >> 32) as u32
    }

    pub fn next_u32_max(&mut self, max: u32) -> u32 {
        ((self.next() >> 32).wrapping_mul(max as u64) >> 32) as u32
    }
}

pub type BWRNG = LCRNG64<0x269ec3, 0x5d588b656c078965>;
pub type BWRNGR = LCRNG64<0x9b1ae6e9a384e6f9, 0xdedcedae9638806d>;
