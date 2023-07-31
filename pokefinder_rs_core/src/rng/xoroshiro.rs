use crate::rng::Rng;

const JUMP_TABLE: [[u64; 2]; 25] = [
    [0x8828e513b43d5, 0x95b8f76579aa001],
    [0x7a8ff5b1c465a931, 0x162ad6ec01b26eae],
    [0xb18b0d36cd81a8f5, 0xb4fbaa5c54ee8b8f],
    [0x23ac5e0ba1cecb29, 0x1207a1706bebb202],
    [0xbb18e9c8d463bb1b, 0x2c88ef71166bc53d],
    [0xe3fbe606ef4e8e09, 0xc3865bb154e9be10],
    [0x28faaaebb31ee2db, 0x1a9fc99fa7818274],
    [0x30a7c4eef203c7eb, 0x588abd4c2ce2ba80],
    [0xa425003f3220a91d, 0x9c90debc053e8cef],
    [0x81e1dd96586cf985, 0xb82ca99a09a4e71e],
    [0x4f7fd3dfbb820bfb, 0x35d69e118698a31d],
    [0xfee2760ef3a900b3, 0x49613606c466efd3],
    [0xf0df0531f434c57d, 0xbd031d011900a9e5],
    [0x442576715266740c, 0x235e761b3b378590],
    [0x1e8bae8f680d2b35, 0x3710a7ae7945df77],
    [0xfd7027fe6d2f6764, 0x75d8e7dbceda609c],
    [0x28eff231ad438124, 0xde2cba60cd3332b5],
    [0x1808760d0a0909a1, 0x377e64c4e80a06fa],
    [0xb9a362fafedfe9d2, 0xcf0a2225da7fb95],
    [0xf57881ab117349fd, 0x2bab58a3cadfc0a3],
    [0x849272241425c996, 0x8d51ecdb9ed82455],
    [0xf1ccb8898cbc07cd, 0x521b29d0a57326c1],
    [0x61179e44214caafa, 0xfbe65017abec72dd],
    [0xd9aa6b1e93fbb6e4, 0x6c446b9bc95c267b],
    [0x86e3772194563f6d, 0x64f80248d23655c6],
];

#[inline]
fn split_mix(mut seed: u64, state: u64) -> u64 {
    seed = seed.wrapping_add(state);
    seed = 0xBF58476D1CE4E5B9u64.wrapping_mul(seed ^ (seed >> 30));
    seed = 0x94D049BB133111EBu64.wrapping_mul(seed ^ (seed >> 27));
    seed ^ (seed >> 31)
}

/// Provides random numbers via the Xoroshiro algorithm
#[derive(Copy, Clone, Default)]
pub struct Xoroshiro {
    state: [u64; 2],
}

/// Converts a 64bit seed into a [`Xoroshiro`] RNG
impl From<u64> for Xoroshiro {
    fn from(value: u64) -> Self {
        Xoroshiro::new(value, 0x82A2B175229D6A5B)
    }
}

impl Xoroshiro {
    /// Constructs a new [`Xoroshiro`] struct
    pub fn new(seed0: u64, seed1: u64) -> Self {
        Self {
            state: [seed0, seed1],
        }
    }

    /// Gets the next 32bit PRNG state bounded by the `MAX` value
    pub fn next_u32<const MAX: u32>(&mut self) -> u32 {
        const BIT_MASK: fn(u32) -> u32 = |mut x: u32| {
            x -= 1;
            x |= x >> 1;
            x |= x >> 2;
            x |= x >> 4;
            x |= x >> 8;
            x |= x >> 16;
            x
        };
        let mask = BIT_MASK(MAX);
        if MAX - 1 == mask {
            (self.next() & mask as u64) as u32
        } else {
            let mut result;
            while {
                result = (self.next() & mask as u64) as u32;
                result >= MAX
            } {}
            result
        }
    }
}

impl Rng for Xoroshiro {
    type Output = u64;

    /// Gets the next 64bit PRNG state
    fn next(&mut self) -> Self::Output {
        let s0 = self.state[0];
        let mut s1 = self.state[1];
        let result = s0.wrapping_add(s1);

        s1 ^= s0;
        self.state[0] = s0.rotate_left(24) ^ s1 ^ (s1 << 16);
        self.state[1] = s1.rotate_right(37);

        result
    }

    fn advance(&mut self, advances: u32) {
        for _ in 0..advances {
            self.next();
        }
    }

    /// Jumps the RNG by `advances` amount
    ///
    /// Uses a precomputed jump table to complete in O(4096)
    fn jump(&mut self, mut advances: u32) {
        self.advance(advances & 0x7F);
        advances >>= 7;
        let mut i = 0;
        while advances > 0 {
            if (advances & 1) != 0 {
                let mut jump = [0u32; 4];
                for j in (0..=1).rev() {
                    let mut val = JUMP_TABLE[i][j];
                    for _ in 0..64 {
                        if val & 1 != 0 {
                            jump = xor(jump, self.state);
                        }
                        self.next();
                        val >>= 1;
                    }
                }
                self.state = [
                    (jump[0] as u64) << 32 | (jump[1] as u64),
                    (jump[2] as u64) << 32 | (jump[3] as u64),
                ];
            }
            advances >>= 1;
            i += 1;
        }
    }
}

/// Provides random numbers via the Xoroshiro algorithm with modified construction for BDSP
#[derive(Copy, Clone, Default)]
pub struct XoroshiroBDSP {
    base: Xoroshiro,
}

/// Converts a 64bit seed into a [`XoroshiroBDSP`] RNG
impl From<u64> for XoroshiroBDSP {
    fn from(value: u64) -> Self {
        XoroshiroBDSP::new(value)
    }
}

impl XoroshiroBDSP {
    /// Construct a new [`XoroshiroBDSP`] struct
    pub fn new(seed: u64) -> Self {
        Self {
            base: Xoroshiro::new(
                split_mix(seed, 0x9E3779B97F4A7C15),
                split_mix(seed, 0x3C6EF372FE94F82A),
            ),
        }
    }

    /// Gets the next 32bit PRNG state bounded by the `max` value
    pub fn next_u32(&mut self, max: u32) -> u32 {
        (self.base.next() >> 32) as u32 % max
    }
}

impl Rng for XoroshiroBDSP {
    type Output = u64;

    /// Gets the next 64bit PRNG state
    fn next(&mut self) -> Self::Output {
        self.base.next()
    }

    fn advance(&mut self, advances: u32) {
        self.base.advance(advances);
    }

    /// Jumps the RNG by `advances` amount
    ///
    /// Uses a precomputed jump table to complete in O(4096)
    fn jump(&mut self, advances: u32) {
        self.base.jump(advances);
    }
}

#[inline]
fn xor(mut x: [u32; 4], y: [u64; 2]) -> [u32; 4] {
    x[0] ^= (y[0] >> 32) as u32;
    x[1] ^= (y[0] & 0xFFFFFFFF) as u32;
    x[2] ^= (y[1] >> 32) as u32;
    x[3] ^= (y[1] & 0xFFFFFFFF) as u32;
    x
}
