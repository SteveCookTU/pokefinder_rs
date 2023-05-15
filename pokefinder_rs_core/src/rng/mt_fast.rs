/// Provides random numbers via the Mersenne Twister algorithm
///
/// The assumptions of MTFast allow some simplifications to be made from normal MT
/// 1. Computing less of the internal MT array
/// 2. Storing less of the internal MT array
/// 3. Skipping the shuffle check when generating numbers for use
/// 4. If the fast parameter is true, skip the last bit shift operation and shift by 27 during shuffle (only in Gen 5)
/// 5. Temper the results in the initial shuffle to take advantage of SIMD
///
/// SIMD is not currently used in this implementation
#[derive(Copy, Clone)]
pub struct MTFast<const SIZE: usize, const ALIGNED_SIZE: usize, const FAST: bool> {
    state: [u32; ALIGNED_SIZE],
    index: u16,
}

impl<const SIZE: usize, const ALIGNED_SIZE: usize, const FAST: bool>
    MTFast<SIZE, ALIGNED_SIZE, FAST>
{
    /// Construct a new MTFast struct
    pub fn new(mut seed: u32, advances: u32) -> Self {
        assert!(SIZE < 227, "Size exceeds range of MTFast");
        let mut ptr = [0; ALIGNED_SIZE];

        let mut i = 1;
        while i < SIZE + 2 {
            ptr[i - 1] = seed;
            seed = 0x6c078965u32
                .wrapping_mul(seed ^ (seed >> 30))
                .wrapping_add(i as u32);
            i += 1;
        }

        while {
            seed = 0x6c078965u32
                .wrapping_mul(seed ^ (seed >> 30))
                .wrapping_add(i as u32);
            i += 1;
            i < 397
        } {}

        if SIZE >= 4 {
            let upper_mask = [0x80000000; 4];
            let lower_mask = [0x7fffffff; 4];
            let matrix = [0x9908b0df; 4];
            let one = [1; 4];
            let mask1 = [0x9d2c5680; 4];
            let mask2 = if FAST {
                [0xe8000000; 4]
            } else {
                [0xefc60000; 4]
            };

            for j in (0..(SIZE - (SIZE % 4))).step_by(4) {
                let m0: [u32; 4] = ptr[j..(j + 4)].try_into().unwrap();
                let m1: [u32; 4] = ptr[(i + 1)..(i + 5)].try_into().unwrap();

                let x0 = 0x6c078965u32
                    .wrapping_mul(seed ^ (seed >> 30))
                    .wrapping_add((j as u32).wrapping_add(397));
                let x1 = 0x6c078965u32
                    .wrapping_mul(x0 ^ (x0 >> 30))
                    .wrapping_add((j as u32).wrapping_add(398));
                let x2 = 0x6c078965u32
                    .wrapping_mul(x1 ^ (x1 >> 30))
                    .wrapping_add((j as u32).wrapping_add(399));
                seed = 0x6c078965u32
                    .wrapping_mul(x2 ^ (x2 >> 30))
                    .wrapping_add((j as u32).wrapping_add(400));

                let m2 = [x0, x1, x2, seed];

                let mut y = or(and(m0, upper_mask), and(m1, lower_mask));
                let y1 = shr::<1>(y);
                let mag01 = and(cmpeq(and(y, one), one), matrix);

                y = xor(xor(y1, mag01), m2);
                y = xor(y, shr::<11>(y));
                y = xor(y, and(shl::<7>(y), mask1));
                y = xor(y, and(shl::<15>(y), mask2));

                if FAST {
                    y = shr::<27>(y);
                } else {
                    y = xor(y, shr::<18>(y));
                }
                ptr[i..(i + 4)].copy_from_slice(&y);
            }
        }

        if SIZE % 4 != 0 {
            for j in (SIZE - (SIZE % 4))..SIZE {
                let m0 = ptr[j];
                let m1 = ptr[j + 1];
                seed = 0x6c078965u32
                    .wrapping_mul(seed ^ (seed >> 30))
                    .wrapping_add((j as u32).wrapping_add(397));

                let mut y = (m0 & 0x80000000) | (m1 & 0x7fffffff);
                let mut y1 = y >> 1;
                if (y & 1) != 0 {
                    y1 ^= 0x9908b0df;
                }

                y = y1 ^ seed;
                y ^= y >> 11;
                y ^= (y << 7) & 0x9d2c5680;
                if FAST {
                    y ^= (y << 15) & 0xe8000000;
                    y >>= 27;
                } else {
                    y ^= (y << 15) & 0xefc60000;
                    y ^= y >> 18;
                }

                ptr[j] = y;
            }
        }

        Self {
            state: ptr,
            index: advances as u16,
        }
    }

    /// Gets the next 32bit PRNG state
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> u32 {
        let state = self.state[self.index as usize];
        self.index += 1;
        state
    }
}

fn or(mut x: [u32; 4], y: [u32; 4]) -> [u32; 4] {
    for (x, y) in x.iter_mut().zip(y.into_iter()) {
        *x |= y;
    }
    x
}

fn and(mut x: [u32; 4], y: [u32; 4]) -> [u32; 4] {
    for (x, y) in x.iter_mut().zip(y.into_iter()) {
        *x &= y;
    }
    x
}

fn cmpeq(mut x: [u32; 4], y: [u32; 4]) -> [u32; 4] {
    for (x, y) in x.iter_mut().zip(y.into_iter()) {
        *x = if *x == y { 0xffffffff } else { 0 };
    }
    x
}

fn shr<const SHIFT: u32>(mut value: [u32; 4]) -> [u32; 4] {
    value.iter_mut().for_each(|v| *v >>= SHIFT);
    value
}

fn shl<const SHIFT: u32>(mut value: [u32; 4]) -> [u32; 4] {
    value.iter_mut().for_each(|v| *v <<= SHIFT);
    value
}

fn xor(mut x: [u32; 4], y: [u32; 4]) -> [u32; 4] {
    for (x, y) in x.iter_mut().zip(y.into_iter()) {
        *x ^= y;
    }
    x
}
