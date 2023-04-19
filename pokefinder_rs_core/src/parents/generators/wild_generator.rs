use crate::enums::{Encounter, Lead, Method};
use crate::parents::filters::{Filter, StateFilter};
use crate::parents::generators::Generator;
use crate::parents::Profile;

#[derive(Copy, Clone)]
pub struct WildGenerator<'a, 'b, P: Profile, F: Filter> {
    pub base: Generator<'a, 'b, P, F>,
    pub encounter: Encounter,
    pub lead: Lead,
}

impl<'a, 'b, P: Profile, F: Filter> WildGenerator<'a, 'b, P, F> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        initial_advances: u32,
        max_advances: u32,
        delay: u32,
        method: Method,
        encounter: Encounter,
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
            encounter,
            lead,
        }
    }
}
