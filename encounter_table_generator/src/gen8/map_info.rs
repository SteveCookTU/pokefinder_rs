use serde::Deserialize;

#[derive(Deserialize)]
pub struct ZoneData {
    #[serde(rename = "PokePlaceName")]
    pub poke_place_name: String,
    #[serde(rename = "ZoneID")]
    pub zone_id: isize,
}

#[derive(Deserialize)]
pub struct MapInfo {
    #[serde(rename = "ZoneData")]
    pub zone_data: Vec<ZoneData>,
}
