use crate::enums::{Encounter, Game};
use crate::parents::{EncounterArea, EncounterAreaT, Slot};
use crate::rng::{RNGList, Xorshift};

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

#[derive(Clone)]
pub struct EncounterArea8 {
    pub base: EncounterArea,
}

impl EncounterArea8 {
    pub fn new(location: u8, rate: u8, ty: Encounter, pokemon: Vec<Slot>) -> Self {
        Self {
            base: EncounterArea::new(location, rate, ty, pokemon),
        }
    }

    pub fn calculate_level_with_list<const DIFF: bool>(
        &self,
        encounter_slot: u8,
        rng_list: &mut RNGList<u32, Xorshift, 128>,
        force: bool,
    ) -> u8 {
        if DIFF {
            let slot = &self.base.pokemon[encounter_slot as usize];

            let min = slot.get_min_level();
            let max = slot.get_max_level();
            let range = (max - min + 1) as u32;

            let rand = (rng_list.next() % range) as u8;
            if force && (rng_list.next() % 2) != 0 {
                max
            } else {
                min + rand
            }
        } else {
            let slot = &self.base.pokemon[encounter_slot as usize];
            let mut level = slot.get_max_level();
            if force && (rng_list.next() % 2) != 0 {
                for s in &self.base.pokemon {
                    if s.species == slot.species {
                        level = level.max(s.get_max_level());
                    }
                }
            }
            level
        }
    }

    pub fn great_marsh(&self, version: Game) -> bool {
        if (version & Game::BDSP) != Game::NONE {
            matches!(self.base.location, 23 | 24 | 25 | 26 | 27 | 28)
        } else {
            false
        }
    }

    pub fn trophy_garden(&self, version: Game) -> bool {
        if (version & Game::BDSP) != Game::NONE {
            self.base.location == 117
        } else {
            false
        }
    }

    pub fn unown_form(&self, prng: u32) -> u8 {
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
}

impl EncounterAreaT for EncounterArea8 {
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
