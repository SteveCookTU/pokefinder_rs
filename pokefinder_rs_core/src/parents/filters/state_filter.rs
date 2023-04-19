#[derive(Copy, Clone, Default)]
pub struct StateFilter {
    pub natures: [bool; 25],
    pub powers: [bool; 16],
    pub max: [u8; 6],
    pub min: [u8; 6],
    pub skip: bool,
    pub ability: u8,
    pub gender: u8,
    pub shiny: u8,
}

impl StateFilter {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        gender: u8,
        ability: u8,
        shiny: u8,
        skip: bool,
        min: [u8; 6],
        max: [u8; 6],
        natures: [bool; 25],
        powers: [bool; 16],
    ) -> Self {
        Self {
            natures,
            powers,
            max,
            min,
            skip,
            ability,
            gender,
            shiny,
        }
    }
}

pub trait Filter {
    fn get_natures(&self) -> [bool; 25];
    fn get_powers(&self) -> [bool; 16];
    fn get_max(&self) -> [u8; 6];
    fn get_min(&self) -> [u8; 6];
    fn skip(&self) -> bool;
    fn get_ability(&self) -> u8;
    fn get_gender(&self) -> u8;
    fn get_shiny(&self) -> u8;

    fn compare_ability(&self, ability: u8) -> bool {
        self.skip() || self.get_ability() == 255 || self.get_ability() == ability
    }

    fn compare_gender(&self, gender: u8) -> bool {
        self.skip() || self.get_gender() == 255 || self.get_gender() == gender
    }

    fn compare_hidden_power(&self, hidden_power: u8) -> bool {
        self.skip() || self.get_powers()[hidden_power as usize]
    }

    fn compare_iv(&self, ivs: [u8; 6]) -> bool {
        if self.skip() {
            true
        } else {
            let min = self.get_min();
            let max = self.get_max();
            for (i, &iv) in ivs.iter().enumerate() {
                if iv < min[i] || iv > max[i] {
                    return false;
                }
            }
            true
        }
    }

    fn compare_nature(&self, nature: u8) -> bool {
        self.skip() || self.get_natures()[nature as usize]
    }

    fn compare_shiny(&self, shiny: u8) -> bool {
        self.skip() || self.get_shiny() == 255 || (self.get_shiny() & shiny) >= 1
    }
}

impl Filter for StateFilter {
    #[inline]
    fn get_natures(&self) -> [bool; 25] {
        self.natures
    }
    #[inline]
    fn get_powers(&self) -> [bool; 16] {
        self.powers
    }
    #[inline]
    fn get_max(&self) -> [u8; 6] {
        self.max
    }
    #[inline]
    fn get_min(&self) -> [u8; 6] {
        self.min
    }
    #[inline]
    fn skip(&self) -> bool {
        self.skip
    }
    #[inline]
    fn get_ability(&self) -> u8 {
        self.ability
    }
    #[inline]
    fn get_gender(&self) -> u8 {
        self.gender
    }
    #[inline]
    fn get_shiny(&self) -> u8 {
        self.shiny
    }
}

#[derive(Copy, Clone, Default)]
pub struct WildStateFilter {
    pub base: StateFilter,
    pub encounter_slots: [bool; 12],
}

impl WildStateFilter {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        gender: u8,
        ability: u8,
        shiny: u8,
        skip: bool,
        min: [u8; 6],
        max: [u8; 6],
        natures: [bool; 25],
        powers: [bool; 16],
        encounter_slots: [bool; 12],
    ) -> Self {
        Self {
            base: StateFilter::new(gender, ability, shiny, skip, min, max, natures, powers),
            encounter_slots,
        }
    }
}

pub trait WildFilter: Filter {
    fn get_encounter_slots(&self) -> [bool; 12];

    fn compare_encounter_slot(&self, encounter_slot: u8) -> bool {
        self.skip() || self.get_encounter_slots()[encounter_slot as usize]
    }
}

impl Filter for WildStateFilter {
    #[inline]
    fn get_natures(&self) -> [bool; 25] {
        self.base.natures
    }
    #[inline]
    fn get_powers(&self) -> [bool; 16] {
        self.base.powers
    }
    #[inline]
    fn get_max(&self) -> [u8; 6] {
        self.base.max
    }
    #[inline]
    fn get_min(&self) -> [u8; 6] {
        self.base.min
    }
    #[inline]
    fn skip(&self) -> bool {
        self.base.skip
    }
    #[inline]
    fn get_ability(&self) -> u8 {
        self.base.ability
    }
    #[inline]
    fn get_gender(&self) -> u8 {
        self.base.gender
    }

    #[inline]
    fn get_shiny(&self) -> u8 {
        self.base.shiny
    }
}

impl WildFilter for WildStateFilter {
    #[inline]
    fn get_encounter_slots(&self) -> [bool; 12] {
        self.encounter_slots
    }
}
