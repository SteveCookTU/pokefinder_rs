use crate::enums::{Lead, Method};
use crate::parents::filters::Filter;
use crate::parents::searchers::Searcher;
use crate::parents::Profile;

/// Searcher struct for static encounters
#[derive(Clone)]
pub struct StaticSearcher<P: Profile + Send + Sync + Clone, F: Filter + Send + Sync + Clone> {
    /// Base searcher data
    pub base: Searcher<P, F>,
    /// Encounter lead
    pub lead: Lead,
}

impl<P: Profile + Send + Sync + Clone, F: Filter + Send + Sync + Clone> StaticSearcher<P, F> {
    /// Construct a new [`StaticSearcher`] struct
    pub fn new(method: Method, lead: Lead, profile: &P, filter: &F) -> Self {
        Self {
            base: Searcher::new(method, profile, filter),
            lead,
        }
    }
}
