use crate::parents::filters::IDFilter;

#[derive(Copy, Clone)]
pub struct IDGenerator<'a> {
    pub filter: &'a IDFilter,
    pub initial_advances: u32,
    pub max_advances: u32,
}

impl<'a> IDGenerator<'a> {
    pub fn new(initial_advances: u32, max_advances: u32, filter: &'a IDFilter) -> Self {
        Self {
            filter,
            initial_advances,
            max_advances,
        }
    }
}
