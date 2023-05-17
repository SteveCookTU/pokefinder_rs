use crate::enums::{Game, Shiny};
use crate::parents::StaticTemplate;

/// Contains information about how to generate a Raid pokemon
#[derive(Copy, Clone)]
pub struct Raid {
    /// Base static template data
    pub base: StaticTemplate,
    /// If pokemon is gigantamax
    pub gigantamax: bool,
    /// Template available stars
    pub star: [bool; 5],
}

impl Raid {
    /// Construct a new [`Raid`] struct
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        species: u16,
        form: u8,
        shiny: Shiny,
        ability: u8,
        gender: u8,
        iv_count: u8,
        gigantamax: bool,
        star: [bool; 5],
    ) -> Self {
        Self {
            base: StaticTemplate::new(
                Game::SWSH,
                species,
                form,
                shiny,
                ability,
                gender,
                iv_count,
                0,
            ),
            gigantamax,
            star,
        }
    }

    /// Returns a string display fo the available stars
    pub fn get_star_display(&self) -> String {
        let mut low = 4;
        let mut high = 0;
        for (i, &star) in self.star.iter().enumerate() {
            if star {
                if i < low {
                    low = i;
                }
                if i > high {
                    high = i;
                }
            }
        }

        if low == high {
            format!("{}★", low + 1)
        } else {
            format!("{}-{}★", low + 1, high + 1)
        }
    }
}
