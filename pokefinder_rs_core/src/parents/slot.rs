use crate::parents::PersonalInfo;

/// Contains information that impacts the generation of wild encounters
#[derive(Copy, Clone)]
pub struct Slot {
    pub(crate) info: &'static PersonalInfo,
    pub(crate) species: u16,
    pub(crate) form: u8,
    pub(crate) min_level: u8,
    pub(crate) max_level: u8,
}

impl Slot {
    /// Construct a new [`Slot`] struct
    pub fn new(species: u16, min_level: u8, max_level: u8, info: &'static PersonalInfo) -> Self {
        Self {
            info,
            species,
            form: 0,
            min_level,
            max_level,
        }
    }

    /// Construct a new [`Slot`] struct with a specific form
    pub fn new_with_form(
        species: u16,
        form: u8,
        min_level: u8,
        max_level: u8,
        info: &'static PersonalInfo,
    ) -> Self {
        Self {
            info,
            species,
            form,
            min_level,
            max_level,
        }
    }

    /// Determines the form of the pokemon
    pub fn get_form(&self) -> u8 {
        self.form
    }

    /// Determines the information of the pokemon
    pub fn get_info(&self) -> &'static PersonalInfo {
        self.info
    }

    /// Determines the max level of the pokemon
    pub fn get_max_level(&self) -> u8 {
        self.max_level
    }

    /// Determines the min level of the pokemon
    pub fn get_min_level(&self) -> u8 {
        self.min_level
    }

    /// Determines the species of the pokemon
    pub fn get_species(&self) -> u16 {
        self.species
    }

    /// Allows the species to be changed.
    ///
    /// Typically due to some modifier (radar, swarms, etc.)
    pub fn set_species(&mut self, species: u16, info: &'static PersonalInfo) {
        self.species = species;
        self.info = info;
    }
}
