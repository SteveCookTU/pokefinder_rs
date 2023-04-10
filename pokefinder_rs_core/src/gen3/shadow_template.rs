use crate::enums::{Game, ShadowType, Shiny};
use crate::gen3::LockInfo;
use crate::parents::Template;

#[derive(Copy, Clone)]
pub struct ShadowTemplate {
    version: Game,
    specie: u16,
    shiny: Shiny,
    level: u8,
    locks: [LockInfo; 5],
    count: i8,
    ty: ShadowType,
}

impl ShadowTemplate {
    pub const fn new(
        version: Game,
        specie: u16,
        shiny: Shiny,
        level: u8,
        locks: [LockInfo; 5],
        count: i8,
        ty: ShadowType,
    ) -> Self {
        Self {
            version,
            specie,
            shiny,
            level,
            locks,
            count,
            ty,
        }
    }

    pub const fn get_count(&self) -> i8 {
        self.count
    }

    pub const fn get_lock(&self, index: i8) -> LockInfo {
        self.locks[index as usize]
    }

    pub const fn get_type(&self) -> ShadowType {
        self.ty
    }
}

impl Template for ShadowTemplate {
    fn get_ability(&self) -> u8 {
        255
    }

    fn get_form(&self) -> u8 {
        0
    }

    fn get_gender(&self) -> u8 {
        255
    }

    fn get_iv_count(&self) -> u8 {
        0
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
