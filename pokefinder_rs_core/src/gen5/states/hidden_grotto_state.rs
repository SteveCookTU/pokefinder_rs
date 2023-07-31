#[derive(Copy, Clone)]
pub struct HiddenGrottoState {
    pub advances: u32,
    pub data: u16,
    pub item: bool,
    pub chatot: u8,
    pub gender: u8,
    pub group: u8,
    pub slot: u8,
}

impl HiddenGrottoState {
    pub fn new_item(prng: u16, advances: u32, group: u8, slot: u8, item: u16) -> Self {
        Self {
            advances,
            data: item,
            item: true,
            chatot: (prng / 82) as u8,
            gender: 0,
            group,
            slot,
        }
    }

    pub fn new_species(
        prng: u16,
        advances: u32,
        group: u8,
        slot: u8,
        species: u16,
        gender: u8,
    ) -> Self {
        Self {
            advances,
            data: species,
            item: false,
            chatot: (prng / 82) as u8,
            gender,
            group,
            slot,
        }
    }
}
