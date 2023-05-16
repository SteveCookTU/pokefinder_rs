use serde::Deserialize;

#[derive(Deserialize)]
pub struct UgEncountEntry {
    pub monsno: u16,
    pub version: u8,
    pub zukanflag: u8,
}

#[derive(Deserialize)]
pub struct UgEncount {
    pub table: Vec<UgEncountEntry>,
}
