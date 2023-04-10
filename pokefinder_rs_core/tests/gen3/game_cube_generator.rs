use crate::gen3::GAME_CUBE_DATA;
use crate::{assert_states, get_test_data};
use pokefinder_rs_core::enums::{Game, Method};
use pokefinder_rs_core::gen3::filters::StateFilter3;
use pokefinder_rs_core::gen3::generators::GameCubeGenerator;
use pokefinder_rs_core::gen3::{encounters3, Profile3};
use pokefinder_rs_core::parents::states::GeneratorState;
use serde::Deserialize;

#[derive(Deserialize)]
struct GameCubeData<'a> {
    #[serde(borrow)]
    gamecubegenerator: GameCubeGeneratorData<'a>,
}

#[derive(Deserialize)]
struct GameCubeGeneratorData<'a> {
    #[serde(borrow, rename = "generateChannel")]
    generate_channel: Vec<GameCubeGeneratorChannelData<'a>>,
    #[serde(borrow, rename = "generateColoShadow")]
    generate_colo_shadow: Vec<GameCubeGeneratorColoShadowData<'a>>,
    #[serde(borrow, rename = "generateGalesShadow")]
    generate_gales_shadow: Vec<GameCubeGeneratorGalesShadowData<'a>>,
    #[serde(borrow, rename = "generateNonLock")]
    generate_non_lock: Vec<GameCubeGeneratorNonLockData<'a>>,
}

#[derive(Deserialize)]
struct GameCubeGeneratorChannelData<'a> {
    name: &'a str,
    seed: u32,
    results: Vec<GameCubeGeneratorResult>,
}

#[derive(Deserialize)]
struct GameCubeGeneratorColoShadowData<'a> {
    name: &'a str,
    seed: u32,
    pokemon: u16,
    results: Vec<GameCubeGeneratorResult>,
}

#[derive(Deserialize)]
struct GameCubeGeneratorGalesShadowData<'a> {
    name: &'a str,
    seed: u32,
    unset: bool,
    pokemon: u16,
    results: Vec<GameCubeGeneratorResult>,
}

#[derive(Deserialize)]
struct GameCubeGeneratorNonLockData<'a> {
    name: &'a str,
    seed: u32,
    version: u32,
    pokemon: u16,
    results: Vec<GameCubeGeneratorResult>,
}

#[derive(Deserialize, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct GameCubeGeneratorResult {
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

impl From<GeneratorState> for GameCubeGeneratorResult {
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
fn generate_channel() {
    let data = get_test_data::<'static, GameCubeData>(GAME_CUBE_DATA);

    for (num, generate_data) in data
        .gamecubegenerator
        .generate_channel
        .into_iter()
        .enumerate()
    {
        let GameCubeGeneratorChannelData {
            name,
            seed,
            results,
        } = generate_data;

        let min = [0; 6];
        let max = [31; 6];
        let natures = [true; 25];
        let powers = [true; 16];
        let profile = Profile3::new("-".to_string(), Game::GC, 12345, 54321, false);

        let static_template = encounters3::get_static_encounter(8, 0);
        let filter = StateFilter3::new(255, 255, 255, false, min, max, natures, powers);
        let generator = GameCubeGenerator::new(0, 9, 0, Method::Channel, false, &profile, &filter);

        let states = generator.generate(seed, static_template);

        assert_states(results, states, name, num);
    }
}

#[test]
fn generate_colo_shadow() {
    let data = get_test_data::<'static, GameCubeData>(GAME_CUBE_DATA);

    for (num, generate_data) in data
        .gamecubegenerator
        .generate_colo_shadow
        .into_iter()
        .enumerate()
    {
        let GameCubeGeneratorColoShadowData {
            name,
            seed,
            pokemon,
            results,
        } = generate_data;

        let min = [0; 6];
        let max = [31; 6];
        let natures = [true; 25];
        let powers = [true; 16];
        let profile = Profile3::new("-".to_string(), Game::COLOSSEUM, 12345, 54321, false);

        let shadow_template = encounters3::get_shadow_team(pokemon as usize);
        let filter = StateFilter3::new(255, 255, 255, false, min, max, natures, powers);
        let generator = GameCubeGenerator::new(0, 9, 0, Method::None, false, &profile, &filter);

        let states = generator.generate_shadow(seed, shadow_template);

        assert_states(results, states, name, num);
    }
}

#[test]
fn generate_gales_shadow() {
    let data = get_test_data::<'static, GameCubeData>(GAME_CUBE_DATA);

    for (num, generate_data) in data
        .gamecubegenerator
        .generate_gales_shadow
        .into_iter()
        .enumerate()
    {
        let GameCubeGeneratorGalesShadowData {
            name,
            seed,
            unset,
            pokemon,
            results,
        } = generate_data;

        let min = [0; 6];
        let max = [31; 6];
        let natures = [true; 25];
        let powers = [true; 16];
        let profile = Profile3::new("-".to_string(), Game::GALES, 12345, 54321, false);

        let shadow_template = encounters3::get_shadow_team(pokemon as usize);
        let filter = StateFilter3::new(255, 255, 255, false, min, max, natures, powers);
        let generator = GameCubeGenerator::new(0, 9, 0, Method::None, unset, &profile, &filter);

        let states = generator.generate_shadow(seed, shadow_template);

        assert_states(results, states, name, num);
    }
}

#[test]
fn generate_non_lock() {
    let data = get_test_data::<'static, GameCubeData>(GAME_CUBE_DATA);

    for (num, generate_data) in data
        .gamecubegenerator
        .generate_non_lock
        .into_iter()
        .enumerate()
    {
        let GameCubeGeneratorNonLockData {
            name,
            seed,
            version,
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

        let static_template = encounters3::get_static_encounter(7, pokemon as usize);
        let filter = StateFilter3::new(255, 255, 255, false, min, max, natures, powers);
        let generator = GameCubeGenerator::new(0, 9, 0, Method::None, false, &profile, &filter);

        let states = generator.generate(seed, static_template);

        assert_states(results, states, name, num);
    }
}
