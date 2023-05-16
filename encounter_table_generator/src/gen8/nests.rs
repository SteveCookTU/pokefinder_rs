use serde::Deserialize;

#[derive(Deserialize)]
pub struct DenSlot {
    #[serde(rename = "Ability")]
    pub ability: u8,
    #[serde(rename = "AltForm")]
    pub alt_form: u8,
    #[serde(rename = "FlawlessIVs")]
    pub flawless_ivs: u8,
    #[serde(rename = "Gender")]
    pub gender: u8,
    #[serde(rename = "IsGigantamax")]
    pub is_gigantamax: bool,
    #[serde(rename = "Species")]
    pub species: u16,
    #[serde(rename = "Stars")]
    pub stars: Vec<bool>,
}

#[derive(Deserialize)]
pub struct Den {
    #[serde(rename = "ShieldEntries")]
    pub shield_entries: Vec<DenSlot>,
    #[serde(rename = "SwordEntries")]
    pub sword_entries: Vec<DenSlot>,
    #[serde(rename = "TableID")]
    pub table_id: String,
}

#[derive(Deserialize)]
pub struct Nests {
    #[serde(rename = "Tables")]
    pub tables: Vec<Den>,
}