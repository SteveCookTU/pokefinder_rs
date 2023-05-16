use serde::Deserialize;

#[derive(Deserialize)]
pub struct UgSpecialPokemonEntry {
    pub id: u8,
    pub monsno: u16,
    pub version: u8,
    #[serde(rename = "Dspecialrate")]
    pub dspecialrate: u16,
    #[serde(rename = "Pspecialrate")]
    pub pspecialrate: u16,
}

#[derive(Deserialize)]
pub struct UgSpecialPokemon {
    #[serde(rename = "Sheet1")]
    pub sheet1: Vec<UgSpecialPokemonEntry>,
}
