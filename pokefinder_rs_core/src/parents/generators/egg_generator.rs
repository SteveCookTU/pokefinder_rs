use crate::enums::Method;
use crate::parents::filters::Filter;
use crate::parents::generators::Generator;
use crate::parents::{Daycare, Profile};

/// Generator struct for egg encounters
#[derive(Copy, Clone)]
pub struct EggGenerator<'a, 'b, 'c, P: Profile, F: Filter> {
    /// Base generator data
    pub base: Generator<'a, 'b, P, F>,
    /// Daycare parent information
    pub daycare: &'c Daycare,
    /// Parent compatability
    pub compatability: u8,
}

impl<'a, 'b, 'c, P: Profile, F: Filter> EggGenerator<'a, 'b, 'c, P, F> {
    /// Construct a new [`EggGenerator`] struct
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        initial_advances: u32,
        max_advances: u32,
        delay: u32,
        method: Method,
        compatability: u8,
        daycare: &'c Daycare,
        profile: &'a P,
        filter: &'b F,
    ) -> Self {
        Self {
            base: Generator::new(
                initial_advances,
                max_advances,
                delay,
                method,
                profile,
                filter,
            ),
            daycare,
            compatability,
        }
    }
}
