use crate::enums::Game;
use crate::parents::{personal_loader, PersonalInfo};

#[derive(Clone)]
pub struct WB8 {
    pub ec: u32,
    pub pid: u32,
    pub sid: u16,
    pub specie: u16,
    pub tid: u16,
    pub egg: bool,
    pub ability: u8,
    pub form: u8,
    pub gender: u8,
    pub iv_count: u8,
    pub level: u8,
    pub nature: u8,
    pub shiny: u8,
}

impl From<&[u8]> for WB8 {
    fn from(value: &[u8]) -> Self {
        Self {
            ec: u32::from_le_bytes(value[0x28..0x2C].try_into().unwrap()),
            pid: u32::from_le_bytes(value[0x2C..0x30].try_into().unwrap()),
            sid: u16::from_le_bytes(value[0x22..0x24].try_into().unwrap()),
            specie: u16::from_le_bytes(value[0x288..0x28A].try_into().unwrap()),
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
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        tid: u16,
        sid: u16,
        ec: u32,
        pid: u32,
        specie: u16,
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
            specie,
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

    pub fn get_info(&self, version: Game) -> &'static PersonalInfo {
        personal_loader::get_personal_info(version, self.specie, self.form)
    }
}
