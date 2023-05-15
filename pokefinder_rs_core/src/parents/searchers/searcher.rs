use crate::enums::Method;
use crate::parents::filters::Filter;
use crate::parents::Profile;
use std::sync::Arc;

/// Searcher struct that stores common attributes
#[derive(Clone)]
pub struct Searcher<P: Profile + Send + Sync + Clone, F: Filter + Send + Sync + Clone> {
    /// Profile information
    pub profile: Arc<P>,
    /// Trainer TSV
    pub tsv: u16,
    /// Encounter method
    pub method: Method,
    /// State filter
    pub filter: Arc<F>,
}

impl<P: Profile + Send + Sync + Clone, F: Filter + Send + Sync + Clone> Searcher<P, F> {
    /// Construct a new [`Searcher`] struct
    pub fn new(method: Method, profile: &P, filter: &F) -> Self {
        Self {
            profile: Arc::new(profile.clone()),
            tsv: profile.get_tid() ^ profile.get_sid(),
            method,
            filter: Arc::new(filter.clone()),
        }
    }
}
