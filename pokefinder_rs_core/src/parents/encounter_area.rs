use crate::enums::{Encounter, Lead};
use crate::parents::Slot;
use crate::util::translator;

/// Trait that contains common functions for EncounterArea structs
pub trait EncounterAreaT {
    /// Calculates the level of a pokemon that has a range
    fn calculate_level_with_prng_state(&self, index: u8, prng: u16) -> u8 {
        let pokemon = self.get_specific_pokemon(index as usize);
        let range = pokemon.get_max_level() - pokemon.get_min_level() + 1;
        (prng % (range as u16)) as u8 + pokemon.get_min_level()
    }
    /// Calculates the level of a pokemon that has no range
    fn calculate_level(&self, index: u8) -> u8 {
        self.get_specific_pokemon(index as usize).get_max_level()
    }
    /// Calculates the level range of a `species` across all possible slots it can be encountered
    fn get_level_range(&self, species: u16) -> (u8, u8) {
        let mut range = (100, 0);

        for slot in self.get_pokemon() {
            if slot.get_species() == species {
                range.0 = range.0.min(slot.get_min_level());
                range.1 = range.1.max(slot.get_max_level());
            }
        }

        range
    }
    /// Returns the pokemon that the specified `index`
    fn get_specific_pokemon(&self, index: usize) -> &Slot {
        &self.get_pokemon()[index]
    }
    /// Returns a vec of true/false which indicate slots that match the `specie`
    fn get_slots_by_specie(&self, species: u16) -> Vec<bool> {
        self.get_pokemon()
            .iter()
            .map(|mon| {
                mon.get_species() == (species & 0x7ff) && (mon.get_form() as u16) == (species >> 11)
            })
            .collect()
    }
    /// Returns the indexes of the slots that match the type of `lead`
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

    /// Returns a vec of names of all the pokemon slots
    fn get_species_names(&self) -> Vec<String> {
        translator::get_species_list(&self.get_unique_species())
    }

    /// Returns the species numbers of unique pokemon of the area
    fn get_unique_species(&self) -> Vec<u16> {
        let mut nums = Vec::new();

        for mon in self.get_pokemon() {
            let num = ((mon.get_form() as u16) << 11) | mon.get_species();
            if !nums.contains(&num) {
                nums.push(num);
            }
        }

        nums
    }

    /// Returns the encounter rate of the area
    fn get_rate(&self) -> u8;
    /// Returns the location of the area
    fn get_location(&self) -> u8;
    /// Returns the encounter type of the area
    fn get_encounter(&self) -> Encounter;
    /// Returns the list of pokemon
    fn get_pokemon(&self) -> &[Slot];
}

/// Contains information about the encounters for an area.
///
/// This includes location, rate, and the slots.
#[derive(Clone)]
pub struct EncounterArea {
    /// Encounter rate of the area
    pub rate: u8,
    /// Location number
    pub location: u8,
    /// Encounter type of the area
    pub encounter: Encounter,
    /// Available pokemon of the area
    pub pokemon: Vec<Slot>,
}

impl EncounterArea {
    /// Construct a new [`EncounterArea`] struct
    pub fn new(location: u8, rate: u8, encounter: Encounter, pokemon: Vec<Slot>) -> Self {
        Self {
            rate,
            location,
            encounter,
            pokemon,
        }
    }
}

impl EncounterAreaT for EncounterArea {
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
