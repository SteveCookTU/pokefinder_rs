use serde::Deserialize;

#[derive(Deserialize)]
pub struct UgPokemonEntry {
    pub monsno: u16,
    pub size: u8,
    pub flagrate: Vec<u8>,
    pub rateup: u8,
}

#[derive(Deserialize)]
pub struct UgPokemonData {
    pub table: Vec<UgPokemonEntry>,
}
