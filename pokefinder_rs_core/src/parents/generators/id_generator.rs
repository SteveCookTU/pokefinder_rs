use crate::parents::filters::IDFilter;

/// Generator struct for TID/SID generation
#[derive(Copy, Clone)]
pub struct IDGenerator<'a> {
    /// State filter
    pub filter: &'a IDFilter,
    /// Initial number of advances
    pub initial_advances: u32,
    /// Maximum number of advances
    pub max_advances: u32,
}

impl<'a> IDGenerator<'a> {
    /// Construct a new [`IDGenerator`] struct
    pub fn new(initial_advances: u32, max_advances: u32, filter: &'a IDFilter) -> Self {
        Self {
            filter,
            initial_advances,
            max_advances,
        }
    }
}
