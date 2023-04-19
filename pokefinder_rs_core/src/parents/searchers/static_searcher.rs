use crate::enums::{Lead, Method};
use crate::parents::filters::{Filter, StateFilter};
use crate::parents::searchers::Searcher;
use crate::parents::Profile;

#[derive(Clone)]
pub struct StaticSearcher<P: Profile + Send + Sync + Clone, F: Filter + Send + Sync + Clone> {
    pub base: Searcher<P, F>,
    pub lead: Lead,
}

impl<P: Profile + Send + Sync + Clone, F: Filter + Send + Sync + Clone> StaticSearcher<P, F> {
    pub fn new(method: Method, lead: Lead, profile: &P, filter: &F) -> Self {
        Self {
            base: Searcher::new(method, profile, filter),
            lead,
        }
    }
}
