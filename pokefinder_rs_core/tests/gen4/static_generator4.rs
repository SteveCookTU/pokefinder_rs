use crate::gen4::STATIC_DATA;
use crate::{assert_states, get_test_data};
use pokefinder_rs_core::enums::{Game, Lead, Method};
use pokefinder_rs_core::gen4::filters::StateFilter4;
use pokefinder_rs_core::gen4::generators::StaticGenerator4;
use pokefinder_rs_core::gen4::states::GeneratorState4;
use pokefinder_rs_core::gen4::{encounters4, Profile4};
use serde::Deserialize;

#[derive(Deserialize)]
struct Static4Data<'a> {
    #[serde(borrow)]
    staticgenerator4: StaticGeneratorData<'a>,
}

#[derive(Deserialize)]
struct StaticGeneratorData<'a> {
    #[serde(rename = "generateMethod1", borrow)]
    generate_method_1: Vec<GeneratorMethod1Data<'a>>,
    #[serde(rename = "generateMethodJ", borrow)]
    generate_method_j: Vec<GeneratorMethodJData<'a>>,
    #[serde(rename = "generateMethodK", borrow)]
    generate_method_k: Vec<GeneratorMethodKData<'a>>,
}

#[derive(Deserialize)]
struct GeneratorMethod1Data<'a> {
    name: &'a str,
    seed: u32,
    version: u32,
    category: u8,
    pokemon: u16,
    results: Vec<GeneratorResult>,
}

#[derive(Deserialize)]
struct GeneratorMethodJData<'a> {
    name: &'a str,
    seed: u32,
    version: u32,
    category: u8,
    pokemon: u16,
    lead: u8,
    results: Vec<GeneratorResult>,
}

#[derive(Deserialize)]
struct GeneratorMethodKData<'a> {
    name: &'a str,
    seed: u32,
    version: u32,
    category: u8,
    pokemon: u16,
    lead: u8,
    results: Vec<GeneratorResult>,
}

#[derive(Deserialize, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct GeneratorResult {
    ability: u8,
    #[serde(rename = "abilityIndex")]
    ability_index: u16,
    advances: u32,
    call: u8,
    characteristic: u8,
    chatot: u8,
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

impl From<GeneratorState4> for GeneratorResult {
    fn from(value: GeneratorState4) -> Self {
        Self {
            ability: value.base.base.ability,
            ability_index: value.base.base.ability_index,
            advances: value.base.advances,
            call: value.call,
            characteristic: value.base.base.characteristic,
            chatot: value.chatot,
            gender: value.base.base.gender,
            hidden_power: value.base.base.hidden_power,
            hidden_power_strength: value.base.base.hidden_power_strength,
            ivs: value.base.base.ivs,
            level: value.base.base.level,
            nature: value.base.base.nature,
            pid: value.base.base.pid,
            shiny: value.base.base.shiny,
            stats: value.base.base.stats,
        }
    }
}

#[test]
fn generate_method_1() {
    let data = get_test_data::<'static, Static4Data>(STATIC_DATA);

    for (
        num,
        GeneratorMethod1Data {
            name,
            seed,
            version,
            category,
            pokemon,
            results,
        },
    ) in data
        .staticgenerator4
        .generate_method_1
        .into_iter()
        .enumerate()
    {
        let min = [0; 6];
        let max = [31; 6];
        let natures = [true; 25];
        let powers = [true; 16];
        let profile = Profile4::new(
            "-".to_string(),
            Game::from_bits_retain(version),
            12345,
            54321,
            false,
        );

        let static_template =
            encounters4::get_static_encounter(category as usize, pokemon as usize);
        let filter = StateFilter4::new(255, 255, 255, false, min, max, natures, powers);
        let generator =
            StaticGenerator4::new(0, 9, 0, Method::Method1, Lead::NONE, &profile, &filter);

        let states = generator.generate(seed, static_template);

        assert_states(results, states, name, num);
    }
}

#[test]
fn generate_method_j() {
    let data = get_test_data::<'static, Static4Data>(STATIC_DATA);

    for (
        num,
        GeneratorMethodJData {
            name,
            seed,
            version,
            category,
            pokemon,
            lead,
            results,
        },
    ) in data
        .staticgenerator4
        .generate_method_j
        .into_iter()
        .enumerate()
    {
        let min = [0; 6];
        let max = [31; 6];
        let natures = [true; 25];
        let powers = [true; 16];
        let profile = Profile4::new(
            "-".to_string(),
            Game::from_bits_retain(version),
            12345,
            54321,
            false,
        );

        let static_template =
            encounters4::get_static_encounter(category as usize, pokemon as usize);
        let filter = StateFilter4::new(255, 255, 255, false, min, max, natures, powers);
        let generator = StaticGenerator4::new(
            0,
            9,
            0,
            Method::MethodJ,
            Lead::from_bits_retain(lead),
            &profile,
            &filter,
        );

        let states = generator.generate(seed, static_template);

        assert_states(results, states, name, num);
    }
}

#[test]
fn generate_method_k() {
    let data = get_test_data::<'static, Static4Data>(STATIC_DATA);

    for (
        num,
        GeneratorMethodKData {
            name,
            seed,
            version,
            category,
            pokemon,
            lead,
            results,
        },
    ) in data
        .staticgenerator4
        .generate_method_k
        .into_iter()
        .enumerate()
    {
        let min = [0; 6];
        let max = [31; 6];
        let natures = [true; 25];
        let powers = [true; 16];
        let profile = Profile4::new(
            "-".to_string(),
            Game::from_bits_retain(version),
            12345,
            54321,
            false,
        );

        let static_template =
            encounters4::get_static_encounter(category as usize, pokemon as usize);
        let filter = StateFilter4::new(255, 255, 255, false, min, max, natures, powers);
        let generator = StaticGenerator4::new(
            0,
            9,
            0,
            Method::MethodK,
            Lead::from_bits_retain(lead),
            &profile,
            &filter,
        );

        let states = generator.generate(seed, static_template);

        assert_states(results, states, name, num);
    }
}
