use crate::enums::Game;
use crate::parents::Profile;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Profile4 {
    name: String,
    version: Game,
    tid: u16,
    sid: u16,
    dex: bool,
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
