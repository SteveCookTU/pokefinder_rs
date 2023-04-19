use crate::enums::{Game, Method, Shiny};
use crate::parents::StaticTemplate;

#[derive(Copy, Clone)]
pub struct StaticTemplate4 {
    pub base: StaticTemplate,
    pub method: Method,
}

impl StaticTemplate4 {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        version: Game,
        specie: u16,
        form: u8,
        shiny: Shiny,
        level: u8,
        method: Method,
    ) -> Self {
        Self {
            base: StaticTemplate::new(version, specie, form, shiny, 255, 255, 0, level),
            method,
        }
    }
}
