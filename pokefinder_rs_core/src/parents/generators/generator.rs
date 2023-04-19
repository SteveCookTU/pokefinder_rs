use crate::enums::Method;
use crate::parents::filters::Filter;
use crate::parents::Profile;

#[derive(Copy, Clone)]
pub struct Generator<'a, 'b, P: Profile, F: Filter> {
    pub profile: &'a P,
    pub initial_advances: u32,
    pub max_advances: u32,
    pub delay: u32,
    pub tsv: u16,
    pub filter: &'b F,
    pub method: Method,
}

impl<'a, 'b, P: Profile, F: Filter> Generator<'a, 'b, P, F> {
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
