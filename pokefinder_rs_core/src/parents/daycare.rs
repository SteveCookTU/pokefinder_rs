#[derive(Clone)]
pub struct Daycare {
    specie: u16,
    parent_ivs: [[u8; 6]; 2],
    parent_ability: [u8; 2],
    parent_gender: [u8; 2],
    parent_item: [u8; 2],
    parent_nature: [u8; 2],
    masuda: bool,
}

impl Daycare {
    pub fn new(
        parent_ivs: [[u8; 6]; 2],
        parent_ability: [u8; 2],
        parent_gender: [u8; 2],
        parent_item: [u8; 2],
        parent_nature: [u8; 2],
        specie: u16,
        masuda: bool,
    ) -> Self {
        Self {
            specie,
            parent_ivs,
            parent_ability,
            parent_gender,
            parent_item,
            parent_nature,
            masuda,
        }
    }

    pub fn get_ditto(&self) -> bool {
        self.parent_nature.iter().any(|&g| g == 3)
    }

    pub fn get_egg_specie(&self) -> u16 {
        self.specie
    }

    pub fn get_everstone_count(&self) -> u8 {
        self.parent_item.iter().filter(|&&i| i == 1).count() as u8
    }

    pub fn get_masuda(&self) -> bool {
        self.masuda
    }

    pub fn get_parent_ability(&self, parent: u8) -> u8 {
        self.parent_ability[parent as usize]
    }

    pub fn get_parent_gender(&self, parent: u8) -> u8 {
        self.parent_gender[parent as usize]
    }

    pub fn get_parent_item(&self, parent: u8) -> u8 {
        self.parent_item[parent as usize]
    }

    pub fn get_parent_iv(&self, parent: u8, index: usize) -> u8 {
        self.parent_ivs[parent as usize][index]
    }

    pub fn get_parent_nature(&self, parent: u8) -> u8 {
        self.parent_nature[parent as usize]
    }

    pub fn get_power_item_count(&self) -> u8 {
        self.parent_item
            .iter()
            .filter(|&i| (2..=7).contains(i))
            .count() as u8
    }
}
