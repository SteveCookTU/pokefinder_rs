use crate::enums::{Game, Method, Shiny};
use crate::parents::{StaticTemplate, Template};

/// Contains additional information for Gen4 static encounters
#[derive(Copy, Clone)]
pub struct StaticTemplate4 {
    /// Base static template data
    pub base: StaticTemplate,
    /// Template generation method
    pub method: Method,
}

impl StaticTemplate4 {
    /// Construct a new [`StaticTemplate4`] struct
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        version: Game,
        species: u16,
        form: u8,
        shiny: Shiny,
        level: u8,
        method: Method,
    ) -> Self {
        Self {
            base: StaticTemplate::new(version, species, form, shiny, 255, 255, 0, level),
            method,
        }
    }
}

impl Template for StaticTemplate4 {
    fn get_ability(&self) -> u8 {
        self.base.ability
    }

    fn get_form(&self) -> u8 {
        self.base.form
    }

    fn get_gender(&self) -> u8 {
        self.base.gender
    }

    fn get_iv_count(&self) -> u8 {
        self.base.iv_count
    }

    fn get_level(&self) -> u8 {
        self.base.level
    }

    fn get_shiny(&self) -> Shiny {
        self.base.shiny
    }

    fn get_species(&self) -> u16 {
        self.base.species
    }

    fn get_version(&self) -> Game {
        self.base.version
    }
}
