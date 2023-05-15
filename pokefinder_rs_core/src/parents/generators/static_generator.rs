use crate::enums::{Lead, Method};
use crate::parents::filters::Filter;
use crate::parents::generators::Generator;
use crate::parents::Profile;

/// Generator struct for static encounters
#[derive(Copy, Clone)]
pub struct StaticGenerator<'a, 'b, P: Profile, F: Filter> {
    /// Base generator data
    pub base: Generator<'a, 'b, P, F>,
    /// Encounter lead
    pub lead: Lead,
}

impl<'a, 'b, P: Profile, F: Filter> StaticGenerator<'a, 'b, P, F> {
    /// Construct a new [`StaticGenerator`] struct
    pub fn new(
        initial_advances: u32,
        max_advances: u32,
        delay: u32,
        method: Method,
        lead: Lead,
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
            lead,
        }
    }
}
