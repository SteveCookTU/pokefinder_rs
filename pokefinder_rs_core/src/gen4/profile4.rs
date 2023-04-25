use crate::enums::Game;
use crate::parents::Profile;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Profile4 {
    pub name: String,
    pub version: Game,
    pub tid: u16,
    pub sid: u16,
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
    pub fn new(name: String, version: Game, tid: u16, sid: u16, dex: bool) -> Self {
        Self {
            name,
            version,
            tid,
            sid,
            dex,
        }
    }

    pub fn get_national_dex(&self) -> bool {
        self.dex
    }
}
