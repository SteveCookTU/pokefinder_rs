use crate::{assert_states, get_test_data};
use pokefinder_rs_core::enums::Game;
use pokefinder_rs_core::gen8::filters::StateFilter8;
use pokefinder_rs_core::gen8::generators::EventGenerator8;
use pokefinder_rs_core::gen8::{Profile8, WB8};
use pokefinder_rs_core::parents::states::GeneratorState;
use serde::Deserialize;

#[derive(Deserialize)]
struct Event8Data<'a> {
    #[serde(borrow)]
    generate: Vec<GenerateData<'a>>,
}

#[derive(Deserialize)]
struct GenerateData<'a> {
    name: &'a str,
    seed0: u64,
    seed1: u64,
    tid: u16,
    sid: u16,
    ec: u32,
    pid: u32,
    species: u16,
    form: u8,
    gender: u8,
    egg: bool,
    nature: u8,
    ability: u8,
    shiny: u8,
    #[serde(rename = "ivCount")]
    iv_count: u8,
    level: u8,
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

const JSON_DATA: &str = include_str!("event8.json");

#[test]
fn generate() {
    let data = get_test_data::<'static, Event8Data>(JSON_DATA);
    for (
        num,
        GenerateData {
            name,
            seed0,
            seed1,
            tid,
            sid,
            ec,
            pid,
            species,
            form,
            gender,
            egg,
            nature,
            ability,
            shiny,
            iv_count,
            level,
            results,
        },
    ) in data.generate.into_iter().enumerate()
    {
        let min = [0; 6];
        let max = [31; 6];
        let natures = [true; 25];
        let powers = [true; 16];

        let profile = Profile8::new("-".to_string(), Game::BD, 12345, 54321, false, false, false);

        let wb8 = WB8::new(
            tid, sid, ec, pid, species, form, gender, egg, nature, ability, shiny, iv_count, level,
        );
        let filter = StateFilter8::new(255, 255, 255, false, min, max, natures, powers);
        let generator = EventGenerator8::new(0, 9, 0, &wb8, &profile, &filter);

        let states = generator.generate(seed0, seed1);

        assert_states(results, states, name, num);
    }
}
