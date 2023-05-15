use crate::enums::{Game, Shiny};
use crate::parents::{personal_loader, PersonalInfo};

/// Contains common information that impacts the generation of static encounters
#[derive(Copy, Clone)]
pub struct StaticTemplate {
    /// Template ability
    pub ability: u8,
    /// Template form
    pub form: u8,
    /// Template gender
    pub gender: u8,
    /// Template IV count
    pub iv_count: u8,
    /// Template level
    pub level: u8,
    /// Template [`Shiny`]
    pub shiny: Shiny,
    /// Template species
    pub species: u16,
    /// Template [`Game`]
    pub version: Game,
}

impl StaticTemplate {
    /// Construct a new [`StaticTemplate`] struct
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        version: Game,
        species: u16,
        form: u8,
        shiny: Shiny,
        ability: u8,
        gender: u8,
        iv_count: u8,
        level: u8,
    ) -> Self {
        Self {
            ability,
            form,
            gender,
            iv_count,
            level,
            shiny,
            species,
            version,
        }
    }
}

impl Template for StaticTemplate {
    fn get_ability(&self) -> u8 {
        self.ability
    }

    fn get_form(&self) -> u8 {
        self.form
    }

    fn get_gender(&self) -> u8 {
        self.gender
    }

    fn get_iv_count(&self) -> u8 {
        self.iv_count
    }

    fn get_level(&self) -> u8 {
        self.level
    }

    fn get_shiny(&self) -> Shiny {
        self.shiny
    }

    fn get_species(&self) -> u16 {
        self.species
    }

    fn get_version(&self) -> Game {
        self.version
    }
}

/// Trait that contains common functions for templates that impact the generation of static encounters
pub trait Template {
    /// Determines what ability the template should have
    fn get_ability(&self) -> u8;

    /// Determines what form the template should have
    fn get_form(&self) -> u8;

    /// Determines what gender the template should have
    fn get_gender(&self) -> u8;

    /// Returns the pokemon information
    fn get_info(&self) -> &'static PersonalInfo {
        personal_loader::get_personal_info(self.get_version(), self.get_species(), self.get_form())
    }

    /// Determines what IV count the template should have
    fn get_iv_count(&self) -> u8;

    /// Determines what level the template should have
    fn get_level(&self) -> u8;

    /// Determines what [`Shiny`] the template should have
    fn get_shiny(&self) -> Shiny;

    /// Determines what species the template should have
    fn get_species(&self) -> u16;

    /// Determines what [`Game`] the template should appear in
    fn get_version(&self) -> Game;
}
