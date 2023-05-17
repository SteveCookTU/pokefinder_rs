use crate::enums::{DSType, Game, Language};
use crate::parents::Profile;
use serde::{Deserialize, Serialize};

/// Provides additional storage specific to Gen5
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
    /// Construct a new [`Profile5`] struct
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

    /// Returns the [`DSType`] of the profile
    pub fn get_ds_type(&self) -> DSType {
        self.ds_type
    }

    /// Returns the string representation of the DS type
    pub fn get_ds_type_string(&self) -> &'static str {
        match self.ds_type {
            DSType::DS => "DS Lite",
            DSType::DSi => "DSi",
            DSType::DS3 => "3DS",
        }
    }

    /// Returns the GxStat value of the profile
    pub fn get_gx_stat(&self) -> u8 {
        self.gx_stat
    }

    /// Returns the key presses selected on the profile
    pub fn get_key_presses(&self) -> [bool; 4] {
        self.key_presses
    }

    /// Returns the string representation of the key presses
    pub fn get_key_presses_string(&self) -> String {
        let mut keys = String::new();
        for (i, key_press) in self.key_presses.into_iter().enumerate() {
            if i == 0 && key_press {
                keys += "None";
            } else if key_press {
                if !keys.is_empty() {
                    keys += ", ";
                }
                keys += &i.to_string();
            }
        }
        keys
    }

    /// Returns the language of the profile
    pub fn get_language(&self) -> Language {
        self.language
    }

    /// Returns the string representation of the language
    pub fn get_language_string(&self) -> &'static str {
        match self.language {
            Language::English => "ENG",
            Language::French => "FRE",
            Language::German => "DEU",
            Language::Italian => "ITA",
            Language::Japanese => "JPN",
            Language::Korean => "KOR",
            Language::Spanish => "SPA",
        }
    }

    /// Returns the MAC address of the profile
    pub fn get_mac(&self) -> u64 {
        self.mac
    }

    /// Returns if memory link is activated
    pub fn get_memory_link(&self) -> bool {
        self.memory_link
    }

    /// Returns if the shiny charm is obtained
    pub fn get_shiny_charm(&self) -> bool {
        self.shiny_charm
    }

    /// Returns if the LR buttons should be skipped
    pub fn get_skip_lr(&self) -> bool {
        self.skip_lr
    }

    /// Returns if soft resetting is used
    pub fn get_soft_reset(&self) -> bool {
        self.soft_reset
    }

    /// Returns the maximum Timer0 value
    pub fn get_timer0_max(&self) -> u16 {
        self.timer0_max
    }

    /// Returns the minimum Timer0 value
    pub fn get_timer0_min(&self) -> u16 {
        self.timer0_min
    }

    /// Returns the VCount value
    pub fn get_vcount(&self) -> u8 {
        self.vcount
    }

    /// Returns the VFrame value
    pub fn get_vframe(&self) -> u8 {
        self.vframe
    }
}
