/// Contains information about the parents in the daycare
#[derive(Clone)]
pub struct Daycare {
    species: u16,
    parent_ivs: [[u8; 6]; 2],
    parent_ability: [u8; 2],
    parent_gender: [u8; 2],
    parent_item: [u8; 2],
    parent_nature: [u8; 2],
    masuda: bool,
}

impl Daycare {
    /// Construct a new [`Daycare`] struct
    pub fn new(
        parent_ivs: [[u8; 6]; 2],
        parent_ability: [u8; 2],
        parent_gender: [u8; 2],
        parent_item: [u8; 2],
        parent_nature: [u8; 2],
        species: u16,
        masuda: bool,
    ) -> Self {
        Self {
            species,
            parent_ivs,
            parent_ability,
            parent_gender,
            parent_item,
            parent_nature,
            masuda,
        }
    }

    /// Determines if either of the parents are Ditto
    pub fn get_ditto(&self) -> bool {
        self.parent_nature.iter().any(|&g| g == 3)
    }

    /// Determines the species of the egg
    pub fn get_egg_species(&self) -> u16 {
        self.species
    }

    /// Determines how many everstones the parents are holding
    pub fn get_everstone_count(&self) -> u8 {
        self.parent_item.iter().filter(|&&i| i == 1).count() as u8
    }

    /// Determines if the parents form an international combo
    pub fn get_masuda(&self) -> bool {
        self.masuda
    }

    /// Determines the ability of the `parent`
    pub fn get_parent_ability(&self, parent: u8) -> u8 {
        self.parent_ability[parent as usize]
    }

    /// Determines the gender of the `parent`
    pub fn get_parent_gender(&self, parent: u8) -> u8 {
        self.parent_gender[parent as usize]
    }

    /// Determines the item of the `parent`
    pub fn get_parent_item(&self, parent: u8) -> u8 {
        self.parent_item[parent as usize]
    }

    /// Determines the iv of the `parent` at the specified `index`
    pub fn get_parent_iv(&self, parent: u8, index: usize) -> u8 {
        self.parent_ivs[parent as usize][index]
    }

    /// Determines the nature of the `parent`
    pub fn get_parent_nature(&self, parent: u8) -> u8 {
        self.parent_nature[parent as usize]
    }

    /// Determines how many power items the parents are holding
    pub fn get_power_item_count(&self) -> u8 {
        self.parent_item
            .iter()
            .filter(|&i| (2..=7).contains(i))
            .count() as u8
    }
}
