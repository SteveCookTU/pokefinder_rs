use crate::enums::{Encounter, Game, Lead};
use crate::parents::{EncounterArea, EncounterAreaT, Slot};
use crate::rng::PokeRNG;

/// Contains information about the encounters for an area in Gen4
///
/// This includes location, rate, and the slot
#[derive(Clone)]
pub struct EncounterArea4 {
    /// Base encounter area data
    pub base: EncounterArea,
}

const UNOWN0: [u8; 20] = [
    0, 1, 2, 6, 7, 9, 10, 11, 12, 14, 15, 16, 18, 19, 20, 21, 22, 23, 24, 25,
];
const UNOWN1: [u8; 1] = [5];
const UNOWN2: [u8; 1] = [17];
const UNOWN3: [u8; 1] = [8];
const UNOWN4: [u8; 1] = [13];
const UNOWN5: [u8; 1] = [4];
const UNOWN6: [u8; 1] = [3];
const UNOWN7: [u8; 2] = [26, 27];

impl EncounterArea4 {
    /// Construct a new [`EncounterArea4`] struct
    pub fn new(location: u8, rate: u8, encounter: Encounter, pokemon: Vec<Slot>) -> Self {
        Self {
            base: EncounterArea::new(location, rate, encounter, pokemon),
        }
    }

    /// Checks if the location is in the Great Marsh
    pub fn great_marsh(&self, version: Game) -> bool {
        if (version & Game::DPPT) != Game::NONE {
            (23..=28).contains(&self.base.location)
        } else {
            false
        }
    }

    /// Checks if the location is in the Safari Zone
    pub fn safari_zone(&self, version: Game) -> bool {
        if (version & Game::HGSS) != Game::NONE {
            (148..=160).contains(&self.base.location)
        } else {
            false
        }
    }

    /// Checks if the location is in the Trophy Garden
    pub fn trophy_garden(&self, version: Game) -> bool {
        if (version & Game::DPPT) != Game::NONE {
            self.base.location == 117
        } else {
            false
        }
    }

    /// Returns the Unown form to use for Solaceon Ruins
    pub fn unown_form(&self, prng: u16) -> u8 {
        match self.base.location {
            29 => UNOWN7[(prng as usize) % UNOWN7.len()],
            30 => UNOWN0[(prng as usize) % UNOWN0.len()],
            32 => UNOWN1[(prng as usize) % UNOWN1.len()],
            34 => UNOWN2[(prng as usize) % UNOWN2.len()],
            40 => UNOWN3[(prng as usize) % UNOWN3.len()],
            41 => UNOWN4[(prng as usize) % UNOWN4.len()],
            42 => UNOWN5[(prng as usize) % UNOWN5.len()],
            43 => UNOWN6[(prng as usize) % UNOWN6.len()],
            _ => 0,
        }
    }

    /// Calculates the level of a pokemon
    ///
    /// Takes into account any modification from [`Lead::PRESSURE`]
    ///
    /// `DIFF`: Whether min and max levels are different
    ///
    /// `MOD`: Whether the max calculation is done without modulo
    ///
    /// [`Lead::PRESSURE`]: ../enums/struct.Lead.html
    pub fn calculate_level_with_rng<const DIFF: bool, const MOD: bool>(
        &self,
        encounter_slot: u8,
        rng: &mut PokeRNG,
        occidentary: &mut u32,
        force: bool,
    ) -> u8 {
        if DIFF {
            let slot = &self.base.pokemon[encounter_slot as usize];

            let min = slot.min_level;
            let max = slot.max_level;
            let range = max - min + 1;

            let rand = rng.next_u16_max::<true>(range as u16) as u8;
            *occidentary += 1;
            if force && {
                *occidentary += 1;
                rng.next_u16_max::<MOD>(2) != 0
            } {
                max
            } else {
                min + rand
            }
        } else {
            let slot = &self.base.pokemon[encounter_slot as usize];
            let mut level = slot.max_level;
            if force && {
                *occidentary += 1;
                rng.next_u16_max::<MOD>(2) != 0
            } {
                for s in &self.base.pokemon {
                    if s.species == slot.species {
                        level = level.max(s.max_level);
                    }
                }
            }
            level
        }
    }

    /// Calculates the level of a pokemon.
    ///
    /// Used by [`WildSearcher4`] and assume [`Lead::PRESSURE`] is being used.
    ///
    /// [`WildSearcher4`]: searchers/struct.WildSearcher4.html
    /// [`Lead::PRESSURE`]: ../enums/struct.Lead.html
    pub fn calculate_level_with_rand<const DIFF: bool>(
        &self,
        encounter_slot: u8,
        level_rand: u16,
        force: bool,
    ) -> u8 {
        if DIFF {
            let slot = &self.base.pokemon[encounter_slot as usize];
            let min = slot.min_level;
            let max = slot.max_level;
            let range = max - min + 1;

            let rand = (level_rand % (range as u16)) as u8;
            if force {
                max
            } else {
                min + rand
            }
        } else {
            let slot = &self.base.pokemon[encounter_slot as usize];
            let mut level = slot.max_level;
            if force {
                for s in &self.base.pokemon {
                    if s.species == slot.species {
                        level = level.max(s.max_level);
                    }
                }
            }
            level
        }
    }
}

impl EncounterAreaT for EncounterArea4 {
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
