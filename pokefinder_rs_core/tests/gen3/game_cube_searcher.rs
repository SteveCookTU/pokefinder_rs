use crate::gen3::GAME_CUBE_DATA;
use crate::{assert_states, get_test_data};
use pokefinder_rs_core::enums::{Game, Method};
use pokefinder_rs_core::gen3::filters::StateFilter3;
use pokefinder_rs_core::gen3::searchers::GameCubeSearcher;
use pokefinder_rs_core::gen3::{encounters3, Profile3};
use pokefinder_rs_core::parents::states::SearcherState;
use serde::Deserialize;

#[derive(Deserialize)]
struct GameCubeData<'a> {
    #[serde(borrow)]
    gamecubesearcher: GameCubeSearchData<'a>,
}

#[derive(Deserialize)]
struct GameCubeSearchData<'a> {
    #[serde(rename = "searchChannel", borrow)]
    search_channel: Vec<SearchChannelData<'a>>,
    #[serde(rename = "searchColoShadow", borrow)]
    search_colo_shadow: Vec<SearchColoShadowData<'a>>,
    #[serde(rename = "searchGalesShadow", borrow)]
    search_gales_shadow: Vec<SearchGalesShadowData<'a>>,
    #[serde(rename = "searchNonLock", borrow)]
    search_non_lock: Vec<SearchNonLockData<'a>>
}

#[derive(Deserialize)]
struct SearchChannelData<'a> {
    name: &'a str,
    min: [u8; 6],
    max: [u8; 6],
    results: Vec<SearcherResult>,
}

#[derive(Deserialize)]
struct SearchColoShadowData<'a> {
    name: &'a str,
    min: [u8; 6],
    max: [u8; 6],
    pokemon: usize,
    results: Vec<SearcherResult>,
}

#[derive(Deserialize)]
struct SearchGalesShadowData<'a> {
    name: &'a str,
    min: [u8; 6],
    max: [u8; 6],
    unset: bool,
    pokemon: usize,
    results: Vec<SearcherResult>,
}

#[derive(Deserialize)]
struct SearchNonLockData<'a> {
    name: &'a str,
    min: [u8; 6],
    max: [u8; 6],
    version: u32,
    pokemon: usize,
    results: Vec<SearcherResult>,
}

#[derive(Deserialize, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct SearcherResult {
    ability: u8,
    #[serde(rename = "abilityIndex")]
    ability_index: u16,
    gender: u8,
    #[serde(rename = "hiddenPower")]
    hidden_power: u8,
    #[serde(rename = "hiddenPowerStrength")]
    hidden_power_strength: u8,
    ivs: [u8; 6],
    level: u8,
    nature: u8,
    pid: u32,
    seed: u32,
    shiny: u8,
    stats: [u16; 6],
}

impl From<SearcherState> for SearcherResult {
    fn from(value: SearcherState) -> Self {
        Self {
            ability: value.base.ability,
            ability_index: value.base.ability_index,
            gender: value.base.gender,
            hidden_power: value.base.hidden_power,
            hidden_power_strength: value.base.hidden_power_strength,
            ivs: value.base.ivs,
            level: value.base.level,
            nature: value.base.nature,
            pid: value.base.pid,
            seed: value.seed,
            shiny: value.base.shiny,
            stats: value.base.stats,
        }
    }
}

#[test]
fn search_channel() {
    let data = get_test_data::<'static, GameCubeData>(GAME_CUBE_DATA);

    for (num, search_data) in data.gamecubesearcher.search_channel.into_iter().enumerate() {
        let SearchChannelData {
            name,
            min,
            max,
            results,
        } = search_data;

        let natures = [true; 25];
        let powers = [true; 16];
        let profile = Profile3::new("-".to_string(), Game::GC, 12345, 54321, false);

        let static_template = encounters3::get_static_encounter(8, 0);
        let filter = StateFilter3::new(255, 255, 255, false, min, max, natures, powers);
        let searcher = GameCubeSearcher::new(Method::Channel, false, &profile, &filter);
        searcher.start_search_static(min, max, static_template);
        let states = searcher.get_results();

        assert_states(results, states, name, num);
    }
}

#[test]
fn search_colo_shadow() {
    let data = get_test_data::<'static, GameCubeData>(GAME_CUBE_DATA);

    for (num, search_data) in data
        .gamecubesearcher
        .search_colo_shadow
        .into_iter()
        .enumerate()
    {
        let SearchColoShadowData {
            name,
            min,
            max,
            pokemon,
            results,
        } = search_data;

        let natures = [true; 25];
        let powers = [true; 16];
        let profile = Profile3::new("-".to_string(), Game::COLOSSEUM, 12345, 54321, false);

        let shadow_template = encounters3::get_shadow_team(pokemon);
        let filter = StateFilter3::new(255, 255, 255, false, min, max, natures, powers);
        let searcher = GameCubeSearcher::new(Method::None, false, &profile, &filter);
        searcher.start_search_shadow(min, max, shadow_template);
        let states = searcher.get_results();

        assert_states(results, states, name, num);
    }
}

#[test]
fn search_non_lock() {
    let data = get_test_data::<'static, GameCubeData>(GAME_CUBE_DATA);

    for (num, search_data) in data
        .gamecubesearcher
        .search_non_lock
        .into_iter()
        .enumerate()
    {
        let SearchNonLockData {
            name,
            min,
            max,
            version, pokemon,
            results,
        } = search_data;

        let natures = [true; 25];
        let powers = [true; 16];
        let profile = Profile3::new("-".to_string(), Game::from_bits_truncate(version), 12345, 54321, false);

        let static_template = encounters3::get_static_encounter(7, pokemon);
        let filter = StateFilter3::new(255, 255, 255, false, min, max, natures, powers);
        let searcher = GameCubeSearcher::new(Method::None, false, &profile, &filter);
        searcher.start_search_static(min, max, static_template);
        let states = searcher.get_results();

        assert_states(results, states, name, num);
    }
}

#[test]
fn search_gales_shadow() {
    let data = get_test_data::<'static, GameCubeData>(GAME_CUBE_DATA);

    for (num, search_data) in data
        .gamecubesearcher
        .search_gales_shadow
        .into_iter()
        .enumerate()
    {
        let SearchGalesShadowData {
            name,
            min,
            max,
            unset, pokemon,
            results,
        } = search_data;

        let natures = [true; 25];
        let powers = [true; 16];
        let profile = Profile3::new("-".to_string(), Game::GALES, 12345, 54321, false);

        let shadow_template = encounters3::get_shadow_team(pokemon);
        let filter = StateFilter3::new(255, 255, 255, false, min, max, natures, powers);
        let searcher = GameCubeSearcher::new(Method::None, unset, &profile, &filter);
        searcher.start_search_shadow(min, max, shadow_template);
        let states = searcher.get_results();

        assert_states(results, states, name, num);
    }
}
