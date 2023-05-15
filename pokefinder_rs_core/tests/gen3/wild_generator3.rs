use crate::gen3::WILD_DATA;
use crate::{assert_states, get_test_data};
use pokefinder_rs_core::enums::{Encounter, Game, Lead, Method};
use pokefinder_rs_core::gen3::filters::WildStateFilter3;
use pokefinder_rs_core::gen3::generators::WildGenerator3;
use pokefinder_rs_core::gen3::{encounters3, Profile3};
use pokefinder_rs_core::parents::states::WildGeneratorState;
use serde::Deserialize;

#[derive(Deserialize)]
struct Wild3Data<'a> {
    #[serde(borrow)]
    wildgenerator3: Wild3GeneratorData<'a>,
}

#[derive(Deserialize)]
struct Wild3GeneratorData<'a> {
    #[serde(borrow)]
    generate: Vec<Wild3GeneratorGenerateData<'a>>,
}

#[derive(Deserialize)]
struct Wild3GeneratorGenerateData<'a> {
    name: &'a str,
    seed: u32,
    version: u32,
    method: u8,
    encounter: u8,
    lead: u8,
    location: u8,
    results: Vec<Wild3GeneratorResult>,
}

#[derive(Deserialize, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Wild3GeneratorResult {
    ability: u8,
    #[serde(rename = "abilityIndex")]
    ability_index: u16,
    advances: u32,
    #[serde(rename = "encounterSlot")]
    encounter_slot: u8,
    form: u8,
    gender: u8,
    #[serde(rename = "hiddenPower")]
    hidden_power: u8,
    #[serde(rename = "hiddenPowerStrength")]
    hidden_power_strength: u8,
    ivs: [u8; 6],
    level: u8,
    nature: u8,
    pid: u32,
    shiny: u8,
    species: u16,
    stats: [u16; 6],
}

impl From<WildGeneratorState> for Wild3GeneratorResult {
    fn from(value: WildGeneratorState) -> Self {
        Self {
            ability: value.base.base.ability,
            ability_index: value.base.base.ability_index,
            advances: value.advances,
            encounter_slot: value.base.encounter_slot,
            form: value.base.form,
            gender: value.base.base.gender,
            hidden_power: value.base.base.hidden_power,
            hidden_power_strength: value.base.base.hidden_power_strength,
            ivs: value.base.base.ivs,
            level: value.base.base.level,
            nature: value.base.base.nature,
            pid: value.base.base.pid,
            shiny: value.base.base.shiny,
            species: value.base.species,
            stats: value.base.base.stats,
        }
    }
}

#[test]
fn generate() {
    let data = get_test_data::<'static, Wild3Data>(WILD_DATA);

    for (num, generate_data) in data.wildgenerator3.generate.into_iter().enumerate() {
        let Wild3GeneratorGenerateData {
            name,
            seed,
            version,
            method,
            encounter,
            lead,
            location,
            results,
        } = generate_data;

        let min = [0; 6];
        let max = [31; 6];
        let natures = [true; 25];
        let powers = [true; 16];
        let encounter_slots = [true; 12];
        let profile = Profile3::new(
            "-".to_string(),
            Game::from_bits_truncate(version),
            12345,
            54321,
            false,
        );

        let encounter_areas = encounters3::get_encounters(
            Encounter::from(encounter),
            Game::from_bits_truncate(version),
        );

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
        let generator = WildGenerator3::new(
            0,
            9,
            0,
            Method::from(method),
            Encounter::from(encounter),
            Lead::from_bits_retain(lead),
            &profile,
            &filter,
        );

        let states = generator.generate(seed, &encounter_area);

        assert_states(results, states, name, num);
    }
}
