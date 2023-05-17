use crate::enums::Game;
use crate::parents::Profile;
use serde::{Deserialize, Serialize};

/// Provides additional storage specific to Gen4
#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Profile4 {
    /// Profile name
    pub name: String,
    /// Profile version
    pub version: Game,
    /// Profile TID
    pub tid: u16,
    /// Profile SID
    pub sid: u16,
    /// Whether nation pokedex is obtained
    pub dex: bool,
}

impl Default for Profile4 {
    fn default() -> Self {
        Self {
            name: "-".to_string(),
            version: Game::DIAMOND,
            tid: 12345,
            sid: 54321,
            dex: false,
        }
    }
}

impl Profile for Profile4 {
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

impl Profile4 {
    /// Construct a new [`Profile4`] struct
    pub fn new(name: String, version: Game, tid: u16, sid: u16, dex: bool) -> Self {
        Self {
            name,
            version,
            tid,
            sid,
            dex,
        }
    }

    /// Gets whether the national dex is unlocked
    pub fn get_national_dex(&self) -> bool {
        self.dex
    }
}
