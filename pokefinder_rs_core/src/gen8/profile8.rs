use crate::enums::Game;
use crate::parents::Profile;
use serde::{Deserialize, Serialize};

/// Provides additional storage specific to Gen8
#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct Profile8 {
    name: String,
    version: Game,
    sid: u16,
    tid: u16,
    dex: bool,
    oval_charm: bool,
    shiny_charm: bool,
}

impl Profile for Profile8 {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_version(&self) -> Game {
        self.version
    }

    fn get_sid(&self) -> u16 {
        self.sid
    }

    fn get_tid(&self) -> u16 {
        self.tid
    }
}

impl Profile8 {
    /// Construct a new [`Profile8`] struct
    pub fn new(
        name: String,
        version: Game,
        tid: u16,
        sid: u16,
        dex: bool,
        oval_charm: bool,
        shiny_charm: bool,
    ) -> Self {
        Self {
            name,
            version,
            sid,
            tid,
            dex,
            oval_charm,
            shiny_charm,
        }
    }

    /// Get if the profile has the national dex unlocked
    pub fn get_national_dex(&self) -> bool {
        self.dex
    }

    /// Get if the profile has the oval charm unlocked
    pub fn get_oval_charm(&self) -> bool {
        self.oval_charm
    }

    /// Get if the profile has the shiny charm unlocked
    pub fn get_shiny_charm(&self) -> bool {
        self.shiny_charm
    }
}
