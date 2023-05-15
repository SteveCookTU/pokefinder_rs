use crate::enums::Method;
use crate::parents::filters::Filter;
use crate::parents::Profile;

/// Generator struct that stores common attributes
#[derive(Copy, Clone)]
pub struct Generator<'a, 'b, P: Profile, F: Filter> {
    /// Profile information
    pub profile: &'a P,
    /// Initial number of advances
    pub initial_advances: u32,
    /// Maximum number of advances
    pub max_advances: u32,
    /// Number of advances to offset
    pub delay: u32,
    /// Trainer TSV
    pub tsv: u16,
    /// State filter
    pub filter: &'b F,
    /// Encounter method
    pub method: Method,
}

impl<'a, 'b, P: Profile, F: Filter> Generator<'a, 'b, P, F> {
    /// Construct a new [`Generator`] struct
    pub fn new(
        initial_advances: u32,
        max_advances: u32,
        delay: u32,
        method: Method,
        profile: &'a P,
        filter: &'b F,
    ) -> Self {
        Self {
            profile,
            initial_advances,
            max_advances,
            delay,
            tsv: profile.get_tid() ^ profile.get_sid(),
            filter,
            method,
        }
    }
}
