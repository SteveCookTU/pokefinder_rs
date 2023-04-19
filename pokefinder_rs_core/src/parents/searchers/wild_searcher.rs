use crate::enums::{Encounter, Lead, Method};
use crate::parents::filters::Filter;
use crate::parents::searchers::Searcher;
use crate::parents::{EncounterAreaT, Profile};
use std::sync::Arc;

#[derive(Clone)]
pub struct WildSearcher<
    E: EncounterAreaT + Send + Sync + Clone,
    P: Profile + Send + Sync + Clone,
    F: Filter + Send + Sync + Clone,
> {
    pub base: Searcher<P, F>,
    pub encounter_area: Arc<E>,
    pub encounter: Encounter,
    pub lead: Lead,
}

impl<
        E: EncounterAreaT + Send + Sync + Clone,
        P: Profile + Send + Sync + Clone,
        F: Filter + Send + Sync + Clone,
    > WildSearcher<E, P, F>
{
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
