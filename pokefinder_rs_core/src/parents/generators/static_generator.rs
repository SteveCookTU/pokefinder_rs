use crate::enums::{Lead, Method};
use crate::parents::filters::Filter;
use crate::parents::generators::Generator;
use crate::parents::Profile;

#[derive(Copy, Clone)]
pub struct StaticGenerator<'a, 'b, P: Profile, F: Filter> {
    pub base: Generator<'a, 'b, P, F>,
    pub lead: Lead,
}

impl<'a, 'b, P: Profile, F: Filter> StaticGenerator<'a, 'b, P, F> {
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
