use crate::rng::Rng;

/// Provides random numbers via the Mersenne Twister algorithm
#[derive(Clone)]
pub struct MT {
    state: [u32; 624],
    index: usize,
}

impl MT {
    /// Construct a new MT struct
    pub fn new(mut seed: u32) -> Self {
        let mut state = [0; 624];
        state[0] = seed;

        for (i, s) in state.iter_mut().enumerate().skip(1) {
            seed = 0x6c078965u32
                .wrapping_mul(seed ^ (seed >> 30))
                .wrapping_add(i as u32);
            *s = seed;
        }

        Self { state, index: 624 }
    }

    /// Construct a new MT struct with initial advances
    ///
    /// Internally this uses [`MT::advance()`] for the initial advances
    pub fn new_with_initial_advances(seed: u32, advances: u32) -> Self {
        let mut new = Self::new(seed);
        new.advance(advances);
        new
    }

    /// Gets the next 16bit PRNG state
    pub fn next_u16(&mut self) -> u16 {
        (self.next() >> 16) as u16
    }

    fn shuffle(&mut self) {
        let upper_mask = [0x80000000; 4];
        let lower_mask = [0x7fffffff; 4];
        let matrix = [0x9908b0df; 4];
        let one = [1; 4];

        let mm_recursion = |m0: [u32; 4], m1: [u32; 4], m2: [u32; 4]| {
            let y = or(and(m0, upper_mask), and(m1, lower_mask));
            let y1 = shr::<1>(y);
            let mag01 = and(cmpeq(and(y, one), one), matrix);
            xor(xor(y1, mag01), m2)
        };

        for i in (0..224).step_by(4) {
            let m0: [u32; 4] = self.state[i..(i + 4)].try_into().unwrap();
            let m1: [u32; 4] = self.state[(i + 1)..(i + 5)].try_into().unwrap();
            let m2: [u32; 4] = self.state[(i + 397)..(i + 401)].try_into().unwrap();

            self.state[i..(i + 4)].copy_from_slice(&mm_recursion(m0, m1, m2));
        }

        let last = [
            self.state[621],
            self.state[622],
            self.state[623],
            self.state[0],
        ];
        let m0: [u32; 4] = self.state[224..228].try_into().unwrap();
        let m1: [u32; 4] = self.state[225..229].try_into().unwrap();
        self.state[224..228].copy_from_slice(&mm_recursion(m0, m1, last));

        for i in (228..620).step_by(4) {
            let m0: [u32; 4] = self.state[i..(i + 4)].try_into().unwrap();
            let m1: [u32; 4] = self.state[(i + 1)..(i + 5)].try_into().unwrap();
            let m2: [u32; 4] = self.state[(i - 227)..(i - 223)].try_into().unwrap();

            self.state[i..(i + 4)].copy_from_slice(&mm_recursion(m0, m1, m2));
        }

        let m0: [u32; 4] = self.state[620..].try_into().unwrap();
        let m2: [u32; 4] = self.state[393..397].try_into().unwrap();
        self.state[620..].copy_from_slice(&mm_recursion(m0, last, m2));
    }
}

impl Rng for MT {
    type Output = u32;

    fn next(&mut self) -> Self::Output {
        if self.index == 624 {
            self.shuffle();
            self.index = 0;
        }

        let mut y = self.state[self.index];
        self.index += 1;

        y ^= y >> 11;
        y ^= (y << 7) & 0x9d2c5680;
        y ^= (y << 15) & 0xefc60000;
        y ^= y >> 18;

        y
    }

    fn advance(&mut self, advances: u32) {
        let mut advance = (advances as usize) + self.index;
        while advance >= 624 {
            self.shuffle();
            advance -= 624;
        }
        self.index = advance;
    }

    fn jump(&mut self, _: u32) {}
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

fn xor(mut x: [u32; 4], y: [u32; 4]) -> [u32; 4] {
    for (x, y) in x.iter_mut().zip(y.into_iter()) {
        *x ^= y;
    }
    x
}
