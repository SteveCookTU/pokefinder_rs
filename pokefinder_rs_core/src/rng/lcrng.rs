#[derive(Copy, Clone)]
pub struct JumpTable {
    pub add: [u32; 32],
    pub mult: [u32; 32],
}

impl JumpTable {
    const fn new() -> Self {
        Self {
            add: [0; 32],
            mult: [0; 32],
        }
    }
}

const fn compute_jump_table(add: u32, mult: u32) -> JumpTable {
    let mut table = JumpTable::new();
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

pub static ARNG_TABLE: JumpTable = compute_jump_table(0x01, 0x6C078965);
pub static ARNGR_TABLE: JumpTable = compute_jump_table(0x69C77F93, 0x9638806D);
pub static POKE_RNG_TABLE: JumpTable = compute_jump_table(0x6073, 0x41C64E6D);
pub static POKE_RNGR_TABLE: JumpTable = compute_jump_table(0xA3561A1, 0xEEB9EB65);
pub static XD_RNG_TABLE: JumpTable = compute_jump_table(0x269EC3, 0x343FD);
pub static XD_RNGR_TABLE: JumpTable = compute_jump_table(0xA170F641, 0xB9B33155);

pub type ARNG = LCRNG<0x01, 0x6C078965>;
pub type ARNGR = LCRNG<0x69C77F93, 0x9638806D>;
pub type PokeRNG = LCRNG<0x6073, 0x41C64E6D>;
pub type PokeRNGR = LCRNG<0xA3561A1, 0xEEB9EB65>;
pub type XDRNG = LCRNG<0x269EC3, 0x343FD>;
pub type XDRNGR = LCRNG<0xA170F641, 0xB9B33155>;

#[derive(Copy, Clone)]
pub struct LCRNG<const ADD: u32, const MULT: u32> {
    pub seed: u32,
}

impl<const ADD: u32, const MULT: u32> LCRNG<ADD, MULT> {
    pub fn new(seed: u32) -> Self {
        Self { seed }
    }

    pub fn new_with_initial_advances(seed: u32, advances: u32) -> Self {
        let mut table = Self { seed };
        table.jump(advances);
        table
    }

    pub fn advance(&mut self, advances: u32) -> u32 {
        for _ in 0..advances {
            self.next();
        }
        self.seed
    }

    pub fn jump(&mut self, mut advances: u32) -> u32 {
        let table = match ADD {
            0x01 => &ARNG_TABLE,
            0x69C77F93 => &ARNGR_TABLE,
            0x6073 => &POKE_RNG_TABLE,
            0xA3561A1 => &POKE_RNGR_TABLE,
            0x269EC3 => &XD_RNG_TABLE,
            _ => {
                assert_eq!(ADD, 0xA170F641, "Unsupported LCRNG");
                &XD_RNGR_TABLE
            }
        };
        let mut i = 0;
        while advances > 0 {
            if (advances & 1) == 1 {
                self.seed = self
                    .seed
                    .wrapping_mul(table.mult[i])
                    .wrapping_add(table.add[i]);
            }
            i += 1;
            advances >>= 1;
        }
        self.seed
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> u32 {
        self.seed = self.seed.wrapping_mul(MULT).wrapping_add(ADD);
        self.seed
    }

    pub fn next_u16(&mut self) -> u16 {
        (self.next() >> 16) as u16
    }

    pub fn next_u16_max<const MOD: bool>(&mut self, max: u16) -> u16 {
        let rand = (self.next() >> 16) as u16;
        if MOD {
            rand % max
        } else {
            rand / ((0xFFFF / max) + 1)
        }
    }
}
