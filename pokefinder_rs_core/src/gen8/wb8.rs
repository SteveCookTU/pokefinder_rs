use crate::enums::Game;
use crate::parents::{personal_loader, PersonalInfo};

/// Represents BDSP Wondercard data
#[derive(Clone)]
pub struct WB8 {
    /// Template EC
    pub ec: u32,
    /// Template PID
    pub pid: u32,
    /// Template SID
    pub sid: u16,
    /// Template species
    pub species: u16,
    /// Template TID
    pub tid: u16,
    /// Template is egg
    pub egg: bool,
    /// Template ability
    ///
    /// 0: 0, 1: 1, 2: H, 3: 1/3, 4: 1/2/H
    pub ability: u8,
    /// Template form
    pub form: u8,
    /// Template gender
    pub gender: u8,
    /// Template flawless IV count
    pub iv_count: u8,
    /// Template level
    pub level: u8,
    /// Template nature
    ///
    /// 0xFF => unset
    pub nature: u8,
    /// Template shininess
    ///
    /// 0: never, 1: random, 2: star, 3: square, 4: static
    pub shiny: u8,
}

/// Converts a u8 slice into a [`WB8`]
impl From<&[u8]> for WB8 {
    fn from(value: &[u8]) -> Self {
        Self {
            ec: u32::from_le_bytes(value[0x28..0x2C].try_into().unwrap()),
            pid: u32::from_le_bytes(value[0x2C..0x30].try_into().unwrap()),
            sid: u16::from_le_bytes(value[0x22..0x24].try_into().unwrap()),
            species: u16::from_le_bytes(value[0x288..0x28A].try_into().unwrap()),
            tid: u16::from_le_bytes(value[0x20..0x22].try_into().unwrap()),
            egg: value[0x28D] == 1,
            ability: value[0x28F],
            form: value[0x28A],
            gender: value[0x28B],
            iv_count: if value[0x2b2].wrapping_sub(0xFC) < 3 {
                value[0x2b2].wrapping_sub(0xFB)
            } else {
                0
            },
            level: value[0x28C],
            nature: value[0x28E],
            shiny: value[0x290],
        }
    }
}

impl WB8 {
    /// Construct a new [`WB8`] struct
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        tid: u16,
        sid: u16,
        ec: u32,
        pid: u32,
        species: u16,
        form: u8,
        gender: u8,
        egg: bool,
        nature: u8,
        ability: u8,
        shiny: u8,
        iv_count: u8,
        level: u8,
    ) -> Self {
        Self {
            ec,
            pid,
            sid,
            species,
            tid,
            egg,
            ability,
            form,
            gender,
            iv_count,
            level,
            nature,
            shiny,
        }
    }

    /// Returns the pokemon information
    pub fn get_info(&self, version: Game) -> &'static PersonalInfo {
        personal_loader::get_personal_info(version, self.species, self.form)
    }
}
