use crate::rng::Rng;

/// Provides a storage container to re-use RNG calculations and cycle out old states with new states
///
/// `SIZE` must be a perfect multiple of two
pub struct RNGList<T: Default + Copy, RNG: Rng<Output = T>, const SIZE: usize> {
    rng: RNG,
    list: [T; SIZE],
    head: usize,
    pointer: usize,
    generate: Option<fn(&mut RNG) -> T>,
}

impl<T: Default + Copy, RNG: Rng<Output = T>, const SIZE: usize> RNGList<T, RNG, SIZE> {
    /// Construct a new RNGList struct
    ///
    /// `generate` is an optional function to modify PRNG states
    pub fn new(rng: RNG, generate: Option<fn(&mut RNG) -> T>) -> Self {
        let mut new = Self {
            rng,
            list: [T::default(); SIZE],
            head: 0,
            pointer: 0,
            generate,
        };

        new.init();
        new
    }

    /// Advances the RNG by `advances` amount
    pub fn advance_states(&mut self, advances: u32) {
        for _ in 0..advances {
            self.advance_state();
        }
    }

    /// Advances the RNG by 1
    pub fn advance_state(&mut self) {
        if let Some(generate) = self.generate {
            self.list[self.head] = generate(&mut self.rng);
        } else {
            self.list[self.head] = self.rng.next();
        }
        self.head += 1;
        self.head %= SIZE;
        self.pointer = self.head;
    }

    /// Advances the internal state by `advances` amount
    pub fn advance(&mut self, advances: u32) {
        self.pointer = (self.pointer + advances as usize) % SIZE;
    }

    /// Gets the next PRNG state
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> T {
        let result = self.list[self.pointer];
        self.pointer += 1;
        self.pointer %= SIZE;

        if SIZE > 8 {
            debug_assert_ne!(self.pointer, self.head);
        }

        result
    }

    /// Gets the next PRNG state with a function that modifies the output
    pub fn next_alt<R>(&mut self, value: fn(T) -> R) -> R {
        value(self.next())
    }

    /// Resets the current internal state
    pub fn reset_rate(&mut self) {
        self.pointer = self.head;
    }

    fn init(&mut self) {
        for x in &mut self.list {
            if let Some(generate) = self.generate {
                *x = generate(&mut self.rng);
            } else {
                *x = self.rng.next();
            }
        }
    }
}
