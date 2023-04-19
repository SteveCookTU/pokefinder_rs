use crate::enums::Method;
use crate::parents::filters::{Filter, StateFilter};
use crate::parents::Profile;
use std::sync::Arc;

#[derive(Clone)]
pub struct Searcher<P: Profile + Send + Sync + Clone, F: Filter + Send + Sync + Clone> {
    pub profile: Arc<P>,
    pub tsv: u16,
    pub method: Method,
    pub filter: Arc<F>,
}

impl<P: Profile + Send + Sync + Clone, F: Filter + Send + Sync + Clone> Searcher<P, F> {
    pub fn new(method: Method, profile: &P, filter: &F) -> Self {
        Self {
            profile: Arc::new(profile.clone()),
            tsv: profile.get_tid() ^ profile.get_sid(),
            method,
            filter: Arc::new(filter.clone()),
        }
    }
}
