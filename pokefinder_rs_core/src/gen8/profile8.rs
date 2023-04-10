use crate::enums::Game;
use crate::parents::Profile;
use serde::{Deserialize, Serialize};

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

    pub fn get_national_dex(&self) -> bool {
        self.dex
    }

    pub fn get_oval_charm(&self) -> bool {
        self.oval_charm
    }

    pub fn get_shiny_charm(&self) -> bool {
        self.shiny_charm
    }
}
