use serde::Deserialize;

#[derive(Deserialize)]
pub struct FieldEncountSlot {
    pub maxlv: u8,
    pub minlv: u8,
    #[serde(rename = "monsNo")]
    pub mons_no: u16,
}

#[derive(Deserialize)]
pub struct FieldEncountTableEntry {
    #[serde(rename = "zoneID")]
    pub zone_id: isize,
    #[serde(rename = "encRate_gr")]
    pub enc_rate_gr: u8,
    pub ground_mons: Vec<FieldEncountSlot>,
    pub tairyo: Vec<FieldEncountSlot>,
    pub day: Vec<FieldEncountSlot>,
    pub night: Vec<FieldEncountSlot>,
    #[serde(rename = "swayGrass")]
    pub sway_grass: Vec<FieldEncountSlot>,
    #[serde(rename = "encRate_wat")]
    pub enc_rate_wat: u8,
    pub water_mons: Vec<FieldEncountSlot>,
    #[serde(rename = "encRate_turi_boro")]
    pub enc_rate_turi_boro: u8,
    pub boro_mons: Vec<FieldEncountSlot>,
    #[serde(rename = "encRate_turi_ii")]
    pub enc_rate_turi_ii: u8,
    pub ii_mons: Vec<FieldEncountSlot>,
    #[serde(rename = "encRate_sugoi")]
    pub enc_rate_sugoi: u8,
    pub sugoi_mons: Vec<FieldEncountSlot>,
}

#[derive(Deserialize)]
pub struct FieldEncountTable {
    pub table: Vec<FieldEncountTableEntry>,
}
