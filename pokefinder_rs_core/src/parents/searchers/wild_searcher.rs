use crate::enums::{Encounter, Lead, Method};
use crate::parents::filters::Filter;
use crate::parents::searchers::Searcher;
use crate::parents::{EncounterAreaT, Profile};
use std::sync::Arc;

/// Searcher struct for wild encounters
#[derive(Clone)]
pub struct WildSearcher<
    E: EncounterAreaT + Send + Sync + Clone,
    P: Profile + Send + Sync + Clone,
    F: Filter + Send + Sync + Clone,
> {
    /// Base searcher data
    pub base: Searcher<P, F>,
    /// Encounter area used by the searcher
    pub encounter_area: Arc<E>,
    /// Encounter type
    pub encounter: Encounter,
    /// Encounter lead
    pub lead: Lead,
}

impl<
        E: EncounterAreaT + Send + Sync + Clone,
        P: Profile + Send + Sync + Clone,
        F: Filter + Send + Sync + Clone,
    > WildSearcher<E, P, F>
{
    /// Construct a new [`WildSearcher`] struct
    pub fn new(
        method: Method,
        encounter: Encounter,
        lead: Lead,
        encounter_area: &E,
        profile: &P,
        filter: &F,
    ) -> Self {
        Self {
            base: Searcher::new(method, profile, filter),
            encounter_area: Arc::new(encounter_area.clone()),
            encounter,
            lead,
        }
    }
}
