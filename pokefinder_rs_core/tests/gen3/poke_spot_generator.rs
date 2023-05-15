use crate::{assert_states, get_test_data};
use pokefinder_rs_core::enums::Game;
use pokefinder_rs_core::gen3::filters::WildStateFilter3;
use pokefinder_rs_core::gen3::generators::PokeSpotGenerator;
use pokefinder_rs_core::gen3::states::PokeSpotState;
use pokefinder_rs_core::gen3::{encounters3, Profile3};
use serde::Deserialize;

#[derive(Deserialize)]
struct PokeSpotGeneratorData<'a> {
    #[serde(borrow)]
    generate: Vec<PokeSpotGeneratorGenerateData<'a>>,
}

#[derive(Deserialize)]
struct PokeSpotGeneratorGenerateData<'a> {
    name: &'a str,
    seed: u32,
    location: u8,
    results: Vec<PokeSpotGeneratorResult>,
}

#[derive(Deserialize, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct PokeSpotGeneratorResult {
    ability: u8,
    #[serde(rename = "abilityIndex")]
    ability_index: u16,
    advances: u32,
    #[serde(rename = "encounterAdvances")]
    encounter_advances: u32,
    #[serde(rename = "encounterSlot")]
    encounter_slot: u8,
    gender: u8,
    #[serde(rename = "hiddenPower")]
    hidden_power: u8,
    #[serde(rename = "hiddenPowerStrength")]
    hidden_power_strength: u8,
    ivs: [u8; 6],
    level: u8,
    nature: u8,
    pid: u32,
    species: u16,
    stats: [u16; 6],
}

impl From<PokeSpotState> for PokeSpotGeneratorResult {
    fn from(value: PokeSpotState) -> Self {
        Self {
            ability: value.base.base.ability,
            ability_index: value.base.base.ability_index,
            advances: value.base.advances,
            encounter_advances: value.encounter_advances,
            encounter_slot: value.encounter_slot,
            gender: value.base.base.gender,
            hidden_power: value.base.base.hidden_power,
            hidden_power_strength: value.base.base.hidden_power_strength,
            ivs: value.base.base.ivs,
            level: value.base.base.level,
            nature: value.base.base.nature,
            pid: value.base.base.pid,
            species: value.species,
            stats: value.base.base.stats,
        }
    }
}

const JSON_DATA: &str = include_str!("pokespot.json");

#[test]
fn generate() {
    let data = get_test_data::<'static, PokeSpotGeneratorData>(JSON_DATA);

    for (num, generate_data) in data.generate.into_iter().enumerate() {
        let PokeSpotGeneratorGenerateData {
            name,
            seed,
            location,
            results,
        } = generate_data;

        let min = [0; 6];
        let max = [31; 6];
        let natures = [true; 25];
        let powers = [true; 16];
        let encounter_slots = [true; 12];
        let profile = Profile3::new("-".to_string(), Game::GALES, 12345, 54321, false);

        let encounter_areas = encounters3::get_poke_spot_encounters();

        let encounter_area = encounter_areas
            .into_iter()
            .find(|encounter_area| encounter_area.location == location)
            .unwrap();

        let filter = WildStateFilter3::new(
            255,
            255,
            255,
            false,
            min,
            max,
            natures,
            powers,
            encounter_slots,
        );
        let generator = PokeSpotGenerator::new(0, 9, 0, 0, 9, 0, &profile, &filter);

        let states = generator.generate(seed, seed, &encounter_area);

        assert_states(results, states, name, num);
    }
}
