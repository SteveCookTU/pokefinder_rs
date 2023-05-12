use crate::rng::Rng;

pub struct RNGList<T: Default + Copy, RNG: Rng<Output = T>, const SIZE: usize> {
    rng: RNG,
    list: [T; SIZE],
    head: usize,
    pointer: usize,
    generate: Option<fn(&mut RNG) -> T>,
}

impl<T: Default + Copy, RNG: Rng<Output = T>, const SIZE: usize> RNGList<T, RNG, SIZE> {
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

    pub fn advance_states(&mut self, advances: u32) {
        for _ in 0..advances {
            self.advance_state();
        }
    }

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

    pub fn advance(&mut self, advances: u32) {
        self.pointer = (self.pointer + advances as usize) % SIZE;
    }

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

    pub fn next_alt<R>(&mut self, value: fn(T) -> R) -> R {
        value(self.next())
    }

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
