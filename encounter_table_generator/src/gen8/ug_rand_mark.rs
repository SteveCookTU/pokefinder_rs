use serde::Deserialize;

#[derive(Deserialize)]
pub struct UgRandMarkEntry {
    pub id: u8,
    #[serde(rename = "FileName")]
    pub file_name: String,
    pub size: u8,
    pub min: u8,
    pub max: u8,
    pub typerate: Vec<u8>,
}

#[derive(Deserialize)]
pub struct UgRandMark {
    pub table: Vec<UgRandMarkEntry>,
}
