use crate::parents::PersonalInfo;
use crate::util::translator;

#[derive(Copy, Clone)]
pub struct HiddenGrottoSlot {
    pub info: &'static PersonalInfo,
    pub species: u16,
    pub gender: u8,
    pub max_level: u8,
    pub min_level: u8,
}

impl HiddenGrottoSlot {
    pub fn new(
        species: u16,
        gender: u8,
        min_level: u8,
        max_level: u8,
        info: &'static PersonalInfo,
    ) -> Self {
        Self {
            info,
            species,
            gender,
            max_level,
            min_level,
        }
    }
}

#[derive(Copy, Clone)]
pub struct HiddenGrottoArea {
    pub pokemon: [HiddenGrottoSlot; 12],
    pub item: [u16; 16],
    pub hidden_item: [u16; 16],
    pub location: u8,
}

impl HiddenGrottoArea {
    pub fn new(
        location: u8,
        pokemon: [HiddenGrottoSlot; 12],
        item: [u16; 16],
        hidden_item: [u16; 16],
    ) -> Self {
        Self {
            pokemon,
            item,
            hidden_item,
            location,
        }
    }

    pub fn get_item(&self, group: u8, index: u8) -> u16 {
        self.item[(group as usize) * 4 + (index as usize)]
    }

    pub fn get_item_names(&self) -> Vec<&'static str> {
        translator::get_items(&self.get_unique_items())
    }

    pub fn get_hidden_item(&self, group: u8, index: u8) -> u16 {
        self.hidden_item[(group as usize) * 4 + (index as usize)]
    }

    pub fn get_pokemon(&self, group: u8, index: u8) -> HiddenGrottoSlot {
        self.pokemon[(group as usize) * 4 + (index as usize)]
    }

    pub fn get_species_names(&self) -> Vec<String> {
        translator::get_species_list(&self.get_unique_species())
    }

    pub fn get_unique_items(&self) -> Vec<u16> {
        let mut nums = vec![];

        for i in self.item.into_iter() {
            if !nums.contains(&i) {
                nums.push(i);
            }
        }

        for i in self.hidden_item.into_iter() {
            if !nums.contains(&i) {
                nums.push(i);
            }
        }

        nums
    }

    pub fn get_unique_species(&self) -> Vec<u16> {
        let mut nums = vec![];

        for i in self.pokemon.iter() {
            if !nums.contains(&i.species) {
                nums.push(i.species);
            }
        }

        nums
    }
}
