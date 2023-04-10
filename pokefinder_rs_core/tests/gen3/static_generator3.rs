use crate::gen3::STATIC_DATA;
use crate::{assert_states, get_test_data};
use pokefinder_rs_core::enums::{Game, Method};
use pokefinder_rs_core::gen3::filters::StateFilter3;
use pokefinder_rs_core::gen3::generators::StaticGenerator3;
use pokefinder_rs_core::gen3::{encounters3, Profile3};
use pokefinder_rs_core::parents::states::GeneratorState;
use serde::Deserialize;

#[derive(Deserialize)]
struct Static3Data<'a> {
    #[serde(borrow)]
    staticgenerator3: StaticGenerator3Data<'a>,
}

#[derive(Deserialize)]
struct StaticGenerator3Data<'a> {
    #[serde(borrow)]
    generate: Vec<StaticGenerator3GenerateData<'a>>,
}

#[derive(Deserialize)]
struct StaticGenerator3GenerateData<'a> {
    name: &'a str,
    seed: u32,
    version: u32,
    method: u8,
    category: usize,
    pokemon: usize,
    results: Vec<StaticGenerator3Result>,
}

#[derive(Deserialize, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct StaticGenerator3Result {
    ability: u8,
    #[serde(rename = "abilityIndex")]
    ability_index: u16,
    advances: u32,
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
    stats: [u16; 6],
}

impl From<GeneratorState> for StaticGenerator3Result {
    fn from(value: GeneratorState) -> Self {
        Self {
            ability: value.base.ability,
            ability_index: value.base.ability_index,
            advances: value.advances,
            gender: value.base.gender,
            hidden_power: value.base.hidden_power,
            hidden_power_strength: value.base.hidden_power_strength,
            ivs: value.base.ivs,
            level: value.base.level,
            nature: value.base.nature,
            pid: value.base.pid,
            shiny: value.base.shiny,
            stats: value.base.stats,
        }
    }
}

#[test]
fn generate() {
    let data = get_test_data::<'static, Static3Data>(STATIC_DATA);

    for (num, generate_data) in data.staticgenerator3.generate.into_iter().enumerate() {
        let StaticGenerator3GenerateData {
            name,
            seed,
            version,
            method,
            category,
            pokemon,
            results,
        } = generate_data;

        let min = [0; 6];
        let max = [31; 6];
        let natures = [true; 25];
        let powers = [true; 16];
        let profile = Profile3::new(
            "-".to_string(),
            Game::from_bits_truncate(version),
            12345,
            54321,
            false,
        );

        let static_template = encounters3::get_static_encounter(category, pokemon);
        let filter = StateFilter3::new(255, 255, 255, false, min, max, natures, powers);
        let generator = StaticGenerator3::new(0, 9, 0, Method::from(method), &profile, &filter);

        let states = generator.generate(seed, &static_template);

        assert_states(results, states, name, num);
    }
}
