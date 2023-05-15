#[derive(Copy, Clone)]
struct JumpTable {
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

static ARNG_TABLE: JumpTable = compute_jump_table(0x01, 0x6C078965);
static ARNGR_TABLE: JumpTable = compute_jump_table(0x69C77F93, 0x9638806D);
static POKE_RNG_TABLE: JumpTable = compute_jump_table(0x6073, 0x41C64E6D);
static POKE_RNGR_TABLE: JumpTable = compute_jump_table(0xA3561A1, 0xEEB9EB65);
static XD_RNG_TABLE: JumpTable = compute_jump_table(0x269EC3, 0x343FD);
static XD_RNGR_TABLE: JumpTable = compute_jump_table(0xA170F641, 0xB9B33155);

/// LCRNG used for Gen 4
pub type ARNG = LCRNG<0x01, 0x6C078965>;
/// Reverse LCRNG used for Gen 4
pub type ARNGR = LCRNG<0x69C77F93, 0x9638806D>;
/// LCRNG used for Gen 3 and 4
pub type PokeRNG = LCRNG<0x6073, 0x41C64E6D>;
/// Reverse LCRNG used for Gen 3 and 4
pub type PokeRNGR = LCRNG<0xA3561A1, 0xEEB9EB65>;
/// LCRNG used for GameCube games
pub type XDRNG = LCRNG<0x269EC3, 0x343FD>;
/// Reverse LCRNG used for GameCube games
pub type XDRNGR = LCRNG<0xA170F641, 0xB9B33155>;

/// Provides random numbers via the LCRNG algorithm
///
/// Most commonly used ones are:
/// - [`ARNG`]
/// - [`ARNGR`]
/// - [`PokeRNG`]
/// - [`PokeRNGR`]
/// - [`XDRNG`]
/// - [`XDRNGR`]
#[derive(Copy, Clone)]
pub struct LCRNG<const ADD: u32, const MULT: u32> {
    /// Current PRNG state
    pub seed: u32,
}

impl<const ADD: u32, const MULT: u32> LCRNG<ADD, MULT> {
    /// Construct a new LCRNG struct
    pub fn new(seed: u32) -> Self {
        Self { seed }
    }

    /// Construct a new LCRNG struct with initial advances
    ///
    /// Internally this uses [`LCRNG::jump()`] for the initial advances
    pub fn new_with_initial_advances(seed: u32, advances: u32) -> Self {
        let mut table = Self { seed };
        table.jump(advances);
        table
    }

    /// Advances the RNG by `advances` amount
    pub fn advance(&mut self, advances: u32) -> u32 {
        for _ in 0..advances {
            self.next();
        }
        self.seed
    }

    /// Jumps the RNG by `advances` amount
    ///
    /// This function uses a jump ahead table to advance any amount in just O(32)
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

    /// Gets the next 32bit PRNG state
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> u32 {
        self.seed = self.seed.wrapping_mul(MULT).wrapping_add(ADD);
        self.seed
    }

    /// Gets the next 16bit PRNG state
    pub fn next_u16(&mut self) -> u16 {
        (self.next() >> 16) as u16
    }

    /// Gets the next 16bit PRNG state bounded by the `max` value
    ///
    /// `MOD` determines whether the calculation is done with or without the modulo operator
    pub fn next_u16_max<const MOD: bool>(&mut self, max: u16) -> u16 {
        let rand = (self.next() >> 16) as u16;
        if MOD {
            rand % max
        } else {
            rand / ((0xFFFF / max) + 1)
        }
    }
}
