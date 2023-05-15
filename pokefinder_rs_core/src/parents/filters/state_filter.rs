/// Provides a way to determine if the given [`State`] meets the given criteria
///
/// [`State`]: ../states/struct.State.html
#[derive(Copy, Clone, Default)]
pub struct StateFilter {
    /// Natures to filter by
    pub natures: [bool; 25],
    /// Hidden powers to filter by
    pub powers: [bool; 16],
    /// Maximum IV thresholds
    pub max: [u8; 6],
    /// Minimum IV thresholds
    pub min: [u8; 6],
    /// If filters should be skipped
    pub skip: bool,
    /// Ability value to filter by
    pub ability: u8,
    /// Gender value to filter by
    pub gender: u8,
    /// Shiny value to filter by
    pub shiny: u8,
}

impl StateFilter {
    /// Construct a new ['StateFilter'] struct
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

/// Trait that contains common functions for Filter structs
pub trait Filter {
    /// Returns the natures to filter by
    fn get_natures(&self) -> [bool; 25];
    /// Returns the hidden powers to filter by
    fn get_powers(&self) -> [bool; 16];
    /// Returns the maximum IV thresholds to filter by
    fn get_max(&self) -> [u8; 6];
    /// Returns the minimum IV thresholds to filter by
    fn get_min(&self) -> [u8; 6];
    /// Determines if the filter should skip checks
    fn skip(&self) -> bool;
    /// Returns the ability value to filter by
    fn get_ability(&self) -> u8;
    /// Returns the gender value to filter by
    fn get_gender(&self) -> u8;
    /// Returns the shiny value to filter by
    fn get_shiny(&self) -> u8;

    /// Determines if the `ability` meets the filter criteria
    fn compare_ability(&self, ability: u8) -> bool {
        self.skip() || self.get_ability() == 255 || self.get_ability() == ability
    }

    /// Determines if the `gender` meets the filter criteria
    fn compare_gender(&self, gender: u8) -> bool {
        self.skip() || self.get_gender() == 255 || self.get_gender() == gender
    }

    /// Determines if the `hidden_power` meets the filter criteria
    fn compare_hidden_power(&self, hidden_power: u8) -> bool {
        self.skip() || self.get_powers()[hidden_power as usize]
    }

    /// Determines if the `ivs` meets the filter criteria
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

    /// Determines if the `natures` meets the filter criteria
    fn compare_nature(&self, nature: u8) -> bool {
        self.skip() || self.get_natures()[nature as usize]
    }

    /// Determines if the `shiny` value meets the filter criteria
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

/// Provides a way to determine if the given [`WildState`] meets the given criteria
///
/// [`WildState`]: ../states/struct.WildState.html
#[derive(Copy, Clone, Default)]
pub struct WildStateFilter {
    /// Base filter criteria
    pub base: StateFilter,
    /// Encounter slots to filter by
    pub encounter_slots: [bool; 12],
}

impl WildStateFilter {
    /// Construct a new [`WildStateFilter`] struct
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

/// Trait that holds common functions for wild filters
pub trait WildFilter: Filter {
    /// Returns the encounter slots to filter by
    fn get_encounter_slots(&self) -> [bool; 12];

    /// Determines if the `encounter_slot` meets the filter criteria
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
