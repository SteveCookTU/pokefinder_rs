use crate::enums::Game;
use crate::parents::Profile;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct Profile3 {
    name: String,
    version: Game,
    sid: u16,
    tid: u16,
    dead_battery: bool,
}

impl Profile3 {
    pub fn new(name: String, version: Game, tid: u16, sid: u16, dead_battery: bool) -> Self {
        Self {
            name,
            version,
            sid,
            tid,
            dead_battery,
        }
    }

    pub fn get_dead_battery(&self) -> bool {
        self.dead_battery
    }
}

impl Profile for Profile3 {
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
