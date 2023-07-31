use crate::enums::{Game, Shiny};
use crate::parents::StaticTemplate;

#[derive(Copy, Clone)]
pub struct DreamRadarTemplate {
    pub base: StaticTemplate,
    pub genie: bool,
    pub legend: bool,
}

impl DreamRadarTemplate {
    pub const fn new(species: u16, form: u8, ability: u8) -> Self {
        let genie = species == 641 || species == 642 || species == 645;
        Self {
            base: StaticTemplate::new(Game::BW2, species, form, Shiny::Never, ability, 255, 0, 0),
            genie: species == 641 || species == 642 || species == 645,
            legend: genie
                || species == 249
                || species == 250
                || species == 483
                || species == 484
                || species == 487,
        }
    }
}
