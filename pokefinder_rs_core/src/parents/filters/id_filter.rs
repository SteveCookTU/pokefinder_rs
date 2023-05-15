use crate::gen8::states::IDState8;
use crate::parents::states::IDStateT;

/// Provides a way to determine if the given [`IDStateT`] meets the given criteria
#[derive(Clone)]
pub struct IDFilter {
    display_filter: Vec<u32>,
    sid_filter: Vec<u16>,
    tid_filter: Vec<u16>,
    tsv_filter: Vec<u16>,
}

impl IDFilter {
    /// Construct a new [`IDFilter`] struct
    pub fn new(
        tid_filter: Vec<u16>,
        sid_filter: Vec<u16>,
        tsv_filter: Vec<u16>,
        display_filter: Vec<u32>,
    ) -> Self {
        Self {
            display_filter,
            sid_filter,
            tid_filter,
            tsv_filter,
        }
    }

    /// Determines if the `state` meets the filter criteria
    pub fn compare(&self, state: &impl IDStateT) -> bool {
        if !self.tid_filter.is_empty() && !self.tid_filter.iter().any(|&id| id == state.get_tid()) {
            return false;
        }
        if !self.sid_filter.is_empty() && !self.sid_filter.iter().any(|&id| id == state.get_sid()) {
            return false;
        }
        if !self.tsv_filter.is_empty() && !self.tsv_filter.iter().any(|&id| id == state.get_tsv()) {
            return false;
        }
        true
    }

    /// Determines if the `state` meets the filter criteria.
    ///
    /// This also runs a comparison for how new generation games display the TID
    pub fn compare_8(&self, state: &IDState8) -> bool {
        !(!self.compare(state)
            || (!self.display_filter.is_empty()
                && !self
                    .display_filter
                    .iter()
                    .any(|&id| id == state.get_display_tid())))
    }
}
