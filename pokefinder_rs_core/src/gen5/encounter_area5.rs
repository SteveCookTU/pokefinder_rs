use crate::enums::Encounter;
use crate::parents::{EncounterArea, EncounterAreaT, Slot};

pub struct EncounterArea5 {
    pub base: EncounterArea,
}

impl EncounterArea5 {
    pub fn new(location: u8, rate: u8, ty: Encounter, pokemon: Vec<Slot>) -> Self {
        Self {
            base: EncounterArea::new(location, rate, ty, pokemon),
        }
    }
}

impl EncounterAreaT for EncounterArea5 {
    fn get_rate(&self) -> u8 {
        self.base.rate
    }

    fn get_location(&self) -> u8 {
        self.base.location
    }

    fn get_encounter(&self) -> Encounter {
        self.base.encounter
    }

    fn get_pokemon(&self) -> &[Slot] {
        &self.base.pokemon
    }
}
