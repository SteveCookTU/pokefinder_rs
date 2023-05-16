pub mod emerald;
pub mod frlg;
mod pack;
pub mod rs;
mod text;
pub mod xd;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct WildEncounters {
    pub wild_encounter_groups: Vec<WildEncounterGroup>,
}

#[derive(Deserialize)]
pub struct WildEncounterGroup {
    pub label: String,
    pub for_maps: bool,
    #[serde(default)]
    pub fields: Vec<WildEncounterField>,
    pub encounters: Vec<WildEncounter>,
}

#[derive(Deserialize, Default)]
pub struct WildEncounterField {
    #[serde(rename = "type")]
    pub ty: String,
    pub encounter_rates: Vec<u8>,
    pub groups: Option<WildEncounterFieldGroup>,
}

#[derive(Deserialize, Default)]
pub struct WildEncounterFieldGroup {
    pub old_rod: Vec<u8>,
    pub good_rod: Vec<u8>,
    pub super_rod: Vec<u8>,
}

#[derive(Deserialize)]
pub struct WildEncounter {
    #[serde(default)]
    pub map: String,
    pub base_label: String,
    pub land_mons: Option<WildEncounterSlots>,
    pub water_mons: Option<WildEncounterSlots>,
    pub rock_smash_mons: Option<WildEncounterSlots>,
    pub fishing_mons: Option<WildEncounterSlots>,
}

#[derive(Deserialize)]
pub struct WildEncounterSlots {
    pub encounter_rate: u8,
    pub mons: Vec<WildEncounterSlot>,
}

#[derive(Deserialize)]
pub struct WildEncounterSlot {
    pub min_level: u8,
    pub max_level: u8,
    pub species: String,
}
