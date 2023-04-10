use crate::enums::{Encounter, Lead, Method};
use crate::parents::filters::StateFilter;
use crate::parents::searchers::Searcher;
use crate::parents::{EncounterAreaBase, Profile};
use std::sync::Arc;

#[derive(Clone)]
pub struct WildSearcher<
    E: EncounterAreaBase + Send + Sync + Clone,
    P: Profile + Send + Sync + Clone,
    F: StateFilter + Send + Sync + Clone,
> {
    pub base: Searcher<P, F>,
    pub encounter_area: Arc<E>,
    pub encounter: Encounter,
    pub lead: Lead,
}

impl<
        E: EncounterAreaBase + Send + Sync + Clone,
        P: Profile + Send + Sync + Clone,
        F: StateFilter + Send + Sync + Clone,
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
