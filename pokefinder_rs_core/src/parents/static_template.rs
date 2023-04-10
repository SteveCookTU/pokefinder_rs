use crate::enums::{Game, Shiny};
use crate::parents::{personal_loader, PersonalInfo};

#[derive(Copy, Clone)]
pub struct StaticTemplate {
    pub ability: u8,
    pub form: u8,
    pub gender: u8,
    pub iv_count: u8,
    pub level: u8,
    pub shiny: Shiny,
    pub specie: u16,
    pub version: Game,
}

impl StaticTemplate {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        version: Game,
        specie: u16,
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
            specie,
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

    fn get_specie(&self) -> u16 {
        self.specie
    }

    fn get_version(&self) -> Game {
        self.version
    }
}

pub trait Template {
    fn get_ability(&self) -> u8;

    fn get_form(&self) -> u8;

    fn get_gender(&self) -> u8;

    fn get_info(&self) -> &'static PersonalInfo {
        personal_loader::get_personal_info(self.get_version(), self.get_specie(), self.get_form())
    }

    fn get_iv_count(&self) -> u8;

    fn get_level(&self) -> u8;

    fn get_shiny(&self) -> Shiny;

    fn get_specie(&self) -> u16;

    fn get_version(&self) -> Game;
}
