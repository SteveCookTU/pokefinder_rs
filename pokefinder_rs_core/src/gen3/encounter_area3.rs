use crate::enums::{Encounter, Game};
use crate::parents::{EncounterAreaT, Slot};
use crate::rng::PokeRNG;

#[derive(Clone)]
pub struct EncounterArea3 {
    pub rate: u8,
    pub location: u8,
    pub encounter: Encounter,
    pub pokemon: Vec<Slot>,
}

impl EncounterArea3 {
    pub const fn new(location: u8, rate: u8, encounter: Encounter, pokemon: Vec<Slot>) -> Self {
        Self {
            rate,
            location,
            encounter,
            pokemon,
        }
    }

    pub fn calculate_level_with_rng(
        &self,
        encounter_slot: u8,
        rng: &mut PokeRNG,
        force: bool,
    ) -> u8 {
        let slot = &self.pokemon[encounter_slot as usize];
        let min = slot.get_min_level();
        let max = slot.get_max_level();
        let range = max - min + 1;

        let mut rand = rng.next_u16_max::<true>(range as u16) as u8;
        if force {
            if rng.next_u16_max::<true>(2) == 0 {
                return max;
            }
            rand = rand.saturating_sub(1);
        }
        min + rand
    }

    pub fn calculate_level_with_rand(
        &self,
        encounter_slot: u8,
        level_rand: u16,
        force: bool,
    ) -> u8 {
        let slot = &self.pokemon[encounter_slot as usize];
        let min = slot.get_min_level();
        let max = slot.get_max_level();
        let range = (max - min + 1) as u16;

        let mut rand = (level_rand % range) as u8;
        if force {
            return max;
        } else {
            rand = rand.saturating_sub(1);
        }

        min + rand
    }

    pub fn safari_zone(&self, version: Game) -> bool {
        ((version & Game::RS) != Game::NONE
            && matches!(self.location, 90 | 187 | 89 | 186 | 92 | 189 | 91 | 188))
            || ((version & Game::EMERALD) != Game::NONE
                && matches!(self.location, 73 | 98 | 74 | 20 | 97 | 72))
    }

    pub fn tanoby_chamber(&self, version: Game) -> bool {
        ((version & Game::FRLG) != Game::NONE) && matches!(self.location, 0 | 1 | 2 | 3 | 4 | 5 | 6)
    }
}

impl EncounterAreaT for EncounterArea3 {
    fn get_rate(&self) -> u8 {
        self.rate
    }

    fn get_location(&self) -> u8 {
        self.location
    }

    fn get_encounter(&self) -> Encounter {
        self.encounter
    }

    fn get_pokemon(&self) -> &[Slot] {
        &self.pokemon
    }
}
