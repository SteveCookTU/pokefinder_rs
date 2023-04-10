use crate::enums::{DSType, Game, Language};
use crate::parents::Profile;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct Profile5 {
    name: String,
    version: Game,
    sid: u16,
    tid: u16,
    mac: u64,
    key_presses: [bool; 4],
    timer0_max: u16,
    timer0_min: u16,
    memory_link: bool,
    shiny_charm: bool,
    skip_lr: bool,
    soft_reset: bool,
    ds_type: DSType,
    language: Language,
    gx_stat: u8,
    vcount: u8,
    vframe: u8,
}

impl Profile for Profile5 {
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

impl Profile5 {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        version: Game,
        tid: u16,
        sid: u16,
        mac: u64,
        key_presses: [bool; 4],
        vcount: u8,
        gx_stat: u8,
        vframe: u8,
        skip_lr: bool,
        timer0_min: u16,
        timer0_max: u16,
        soft_reset: bool,
        memory_link: bool,
        shiny_charm: bool,
        ds_type: DSType,
        language: Language,
    ) -> Self {
        Self {
            name,
            version,
            sid,
            tid,
            mac,
            key_presses,
            timer0_max,
            timer0_min,
            memory_link,
            shiny_charm,
            skip_lr,
            soft_reset,
            ds_type,
            language,
            gx_stat,
            vcount,
            vframe,
        }
    }

    pub fn get_ds_type(&self) -> DSType {
        self.ds_type
    }

    pub fn get_ds_type_string(&self) -> String {
        String::new()
    }

    pub fn get_gx_stat(&self) -> u8 {
        self.gx_stat
    }

    pub fn get_key_presses(&self) -> [bool; 4] {
        self.key_presses
    }

    pub fn get_key_presses_string(&self) -> String {
        String::new()
    }

    pub fn get_language(&self) -> Language {
        self.language
    }

    pub fn get_language_string(&self) -> String {
        String::new()
    }

    pub fn get_mac(&self) -> u64 {
        self.mac
    }

    pub fn get_memory_link(&self) -> bool {
        self.memory_link
    }

    pub fn get_shiny_charm(&self) -> bool {
        self.shiny_charm
    }

    pub fn get_skip_lr(&self) -> bool {
        self.skip_lr
    }

    pub fn get_soft_reset(&self) -> bool {
        self.soft_reset
    }

    pub fn get_timer0_max(&self) -> u16 {
        self.timer0_max
    }

    pub fn get_timer0_min(&self) -> u16 {
        self.timer0_min
    }

    pub fn get_vcount(&self) -> u8 {
        self.vcount
    }

    pub fn get_vframe(&self) -> u8 {
        self.vframe
    }
}
