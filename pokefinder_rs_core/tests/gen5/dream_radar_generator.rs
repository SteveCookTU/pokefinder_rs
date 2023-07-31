use crate::{assert_states, get_test_data};
use pokefinder_rs_core::enums::{DSType, Game, Language};
use pokefinder_rs_core::gen5::filters::StateFilter5;
use pokefinder_rs_core::gen5::generators::DreamRadarGenerator;
use pokefinder_rs_core::gen5::states::DreamRadarState;
use pokefinder_rs_core::gen5::{encounters5, Profile5};
use serde::Deserialize;

#[derive(Deserialize)]
struct DreamRadarData<'a> {
    #[serde(borrow)]
    generate: Vec<DreamRadarGenerateData<'a>>,
}

#[derive(Deserialize)]
struct DreamRadarGenerateData<'a> {
    name: &'a str,
    seed: u64,
    pokemon: Vec<u16>,
    gender: Vec<u8>,
    results: Vec<DreamRadarGenerateResult>,
}

#[derive(Deserialize, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct DreamRadarGenerateResult {
    ability: u8,
    #[serde(rename = "abilityIndex")]
    ability_index: u16,
    advances: u32,
    characteristic: u8,
    gender: u8,
    #[serde(rename = "hiddenPower")]
    hidden_power: u8,
    #[serde(rename = "hiddenPowerStrength")]
    hidden_power_strength: u8,
    ivs: [u8; 6],
    level: u8,
    nature: u8,
    needle: u8,
    pid: u32,
    shiny: u8,
    stats: [u16; 6],
}

impl From<DreamRadarState> for DreamRadarGenerateResult {
    fn from(value: DreamRadarState) -> Self {
        Self {
            ability: value.base.base.ability,
            ability_index: value.base.base.ability_index,
            advances: value.base.advances,
            characteristic: value.base.base.characteristic,
            gender: value.base.base.gender,
            hidden_power: value.base.base.hidden_power,
            hidden_power_strength: value.base.base.hidden_power_strength,
            ivs: value.base.base.ivs,
            level: value.base.base.level,
            nature: value.base.base.nature,
            needle: value.needle,
            pid: value.base.base.pid,
            shiny: value.base.base.shiny,
            stats: value.base.base.stats,
        }
    }
}

const JSON_DATA: &str = include_str!("dreamradar.json");

#[test]
fn generate() {
    let data = get_test_data::<'static, DreamRadarData>(JSON_DATA);

    for (
        num,
        DreamRadarGenerateData {
            name,
            seed,
            pokemon,
            gender,
            results,
        },
    ) in data.generate.into_iter().enumerate()
    {
        let min = [0; 6];
        let max = [31; 6];
        let natures = [true; 25];
        let powers = [true; 16];

        let profile = Profile5::new(
            "-".to_string(),
            Game::BW2,
            12345,
            54321,
            0,
            [false, false, false, false],
            0,
            0,
            0,
            false,
            0,
            0,
            false,
            false,
            false,
            DSType::DS,
            Language::English,
        );

        let dream_radar_templates = encounters5::get_dream_radar_encounters();

        let mut radar_templates = vec![];

        for (p, g) in pokemon.into_iter().zip(gender.into_iter()) {
            radar_templates.push(dream_radar_templates[p as usize]);
            radar_templates.last_mut().unwrap().base.gender = g;
        }

        let filter = StateFilter5::new(255, 255, 255, false, min, max, natures, powers);
        let generator = DreamRadarGenerator::new(0, 9, 0, &radar_templates, &profile, &filter);

        let states = generator.generate(seed);

        assert_states(results, states, name, num);
    }
}
