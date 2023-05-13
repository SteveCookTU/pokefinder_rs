use crate::{assert_states, get_test_data};
use pokefinder_rs_core::enums::{Game, Lead};
use pokefinder_rs_core::gen8::filters::StateFilter8;
use pokefinder_rs_core::gen8::generators::StaticGenerator8;
use pokefinder_rs_core::gen8::{encounters8, Profile8};
use pokefinder_rs_core::parents::states::GeneratorState;
use serde::Deserialize;

#[derive(Deserialize)]
struct Static8Data<'a> {
    #[serde(borrow)]
    generate: Vec<GenerateData<'a>>,
    #[serde(borrow, rename = "generateRoamer")]
    generate_roamer: Vec<GenerateData<'a>>,
}

#[derive(Deserialize)]
struct GenerateData<'a> {
    name: &'a str,
    seed0: u64,
    seed1: u64,
    lead: u8,
    category: usize,
    pokemon: u16,
    results: Vec<GeneratorResult>,
}

#[derive(Deserialize, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct GeneratorResult {
    ability: u8,
    #[serde(rename = "abilityIndex")]
    ability_index: u16,
    advances: u32,
    characteristic: u8,
    ec: u32,
    gender: u8,
    ivs: [u8; 6],
    level: u8,
    nature: u8,
    pid: u32,
    shiny: u8,
    stats: [u16; 6],
}

impl From<GeneratorState> for GeneratorResult {
    fn from(value: GeneratorState) -> Self {
        Self {
            ability: value.base.ability,
            ability_index: value.base.ability_index,
            advances: value.advances,
            characteristic: value.base.characteristic,
            ec: value.base.ec,
            gender: value.base.gender,
            ivs: value.base.ivs,
            level: value.base.level,
            nature: value.base.nature,
            pid: value.base.pid,
            shiny: value.base.shiny,
            stats: value.base.stats,
        }
    }
}

const JSON_DATA: &str = include_str!("static8.json");

#[test]
fn generate() {
    let data = get_test_data::<'static, Static8Data>(JSON_DATA);
    for (
        num,
        GenerateData {
            name,
            seed0,
            seed1,
            lead,
            category,
            pokemon,
            results,
        },
    ) in data.generate.into_iter().enumerate()
    {
        let min = [0; 6];
        let max = [31; 6];
        let natures = [true; 25];
        let powers = [true; 16];

        let profile = Profile8::new(
            "-".to_string(),
            Game::BDSP,
            12345,
            54321,
            false,
            false,
            false,
        );

        let static_template = encounters8::get_static_encounter(category, pokemon as usize);
        let filter = StateFilter8::new(255, 255, 255, false, min, max, natures, powers);
        let generator =
            StaticGenerator8::new(0, 9, 0, Lead::from_bits_retain(lead), &profile, &filter);

        let states = generator.generate(seed0, seed1, static_template);

        assert_states(results, states, name, num);
    }
}

#[test]
fn generate_roamer() {
    let data = get_test_data::<'static, Static8Data>(JSON_DATA);
    for (
        num,
        GenerateData {
            name,
            seed0,
            seed1,
            lead,
            category,
            pokemon,
            results,
        },
    ) in data.generate_roamer.into_iter().enumerate()
    {
        let min = [0; 6];
        let max = [31; 6];
        let natures = [true; 25];
        let powers = [true; 16];

        let profile = Profile8::new(
            "-".to_string(),
            Game::BDSP,
            12345,
            54321,
            false,
            false,
            false,
        );

        let static_template = encounters8::get_static_encounter(category, pokemon as usize);
        let filter = StateFilter8::new(255, 255, 255, false, min, max, natures, powers);
        let generator =
            StaticGenerator8::new(0, 9, 0, Lead::from_bits_retain(lead), &profile, &filter);

        let states = generator.generate_roamer(seed0, seed1, static_template);

        assert_states(results, states, name, num);
    }
}
