use crate::enums::{Encounter, Lead};
use crate::parents::Slot;

pub trait EncounterAreaBase {
    fn calculate_level_with_prng_state(&self, index: u8, prng: u16) -> u8 {
        let pokemon = self.get_specific_pokemon(index as usize);
        let range = pokemon.get_max_level() - pokemon.get_min_level() + 1;
        (prng % (range as u16)) as u8 + pokemon.get_min_level()
    }
    fn calculate_level(&self, index: u8) -> u8 {
        self.get_specific_pokemon(index as usize).get_max_level()
    }
    fn get_level_range(&self, specie: u16) -> (u8, u8) {
        let mut range = (100, 0);

        for slot in self.get_pokemon() {
            if slot.get_specie() == specie {
                range.0 = range.0.min(slot.get_min_level());
                range.1 = range.1.max(slot.get_max_level());
            }
        }

        range
    }
    fn get_specific_pokemon(&self, index: usize) -> &Slot {
        &self.get_pokemon()[index]
    }
    fn get_slots_by_specie(&self, specie: u16) -> Vec<bool> {
        self.get_pokemon()
            .iter()
            .map(|mon| mon.get_specie() == specie)
            .collect()
    }
    fn get_slots_by_lead(&self, lead: Lead) -> Vec<u8> {
        let mut encounters = Vec::new();
        let ty: u8 = match lead.bits() {
            i if i == Lead::MAGNET_PULL.bits() => 8,
            i if i == Lead::STATIC.bits() => 12,
            i if i == Lead::HARVEST.bits() => 11,
            i if i == Lead::FLASH_FIRE.bits() => 9,
            i if i == Lead::STORM_DRAIN.bits() => 10,
            _ => return encounters,
        };

        for (i, info) in self.get_pokemon().iter().map(|s| s.get_info()).enumerate() {
            if info.get_type(0) == ty || info.get_type(1) == ty {
                encounters.push(i as u8);
            }
        }

        encounters
    }

    fn get_specie_names(&self) -> Vec<String> {
        Vec::new()
    }

    fn get_unique_species(&self) -> Vec<u16> {
        let mut nums = Vec::new();

        for mon in self.get_pokemon() {
            let specie = mon.get_specie();
            if !nums.contains(&specie) {
                nums.push(specie);
            }
        }

        nums
    }

    fn get_rate(&self) -> u8;
    fn get_location(&self) -> u8;
    fn get_encounter(&self) -> Encounter;
    fn get_pokemon(&self) -> &[Slot];
}

#[derive(Clone)]
pub struct EncounterArea {
    pub rate: u8,
    pub location: u8,
    pub encounter: Encounter,
    pub pokemon: Vec<Slot>,
}

impl EncounterArea {
    pub fn new(location: u8, rate: u8, encounter: Encounter, pokemon: Vec<Slot>) -> Self {
        Self {
            rate,
            location,
            encounter,
            pokemon,
        }
    }
}

impl EncounterAreaBase for EncounterArea {
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
