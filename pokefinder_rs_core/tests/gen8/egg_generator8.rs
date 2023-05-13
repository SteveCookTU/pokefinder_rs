use crate::{assert_states, get_test_data};
use pokefinder_rs_core::enums::Game;
use pokefinder_rs_core::gen8::filters::StateFilter8;
use pokefinder_rs_core::gen8::generators::EggGenerator8;
use pokefinder_rs_core::gen8::Profile8;
use pokefinder_rs_core::parents::states::EggGeneratorState;
use pokefinder_rs_core::parents::Daycare;
use serde::Deserialize;

#[derive(Deserialize)]
struct Egg8Data<'a> {
    #[serde(borrow)]
    generate: Vec<EggGenerator8Data<'a>>,
}

#[derive(Deserialize)]
struct EggGenerator8Data<'a> {
    name: &'a str,
    seed0: u64,
    seed1: u64,
    pokemon: u16,
    #[serde(rename = "parentIVs")]
    parent_ivs: [[u8; 6]; 2],
    #[serde(rename = "parentAbility")]
    parent_ability: [u8; 2],
    #[serde(rename = "parentGender")]
    parent_gender: [u8; 2],
    #[serde(rename = "parentItem")]
    parent_item: [u8; 2],
    #[serde(rename = "parentNature")]
    parent_nature: [u8; 2],
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
    inheritance: [u8; 6],
    ivs: [u8; 6],
    level: u8,
    nature: u8,
    pid: u32,
    shiny: u8,
    stats: [u16; 6],
}

impl From<EggGeneratorState> for GeneratorResult {
    fn from(value: EggGeneratorState) -> Self {
        Self {
            ability: value.base.base.ability,
            ability_index: value.base.base.ability_index,
            advances: value.advances,
            characteristic: value.base.base.characteristic,
            ec: value.base.base.ec,
            gender: value.base.base.gender,
            inheritance: value.base.inheritance,
            ivs: value.base.base.ivs,
            level: value.base.base.level,
            nature: value.base.base.nature,
            pid: value.base.base.pid,
            shiny: value.base.base.shiny,
            stats: value.base.base.stats,
        }
    }
}

const JSON_DATA: &str = include_str!("egg8.json");

#[test]
fn generate() {
    let data = get_test_data::<'static, Egg8Data>(JSON_DATA);

    for (
        num,
        EggGenerator8Data {
            name,
            seed0,
            seed1,
            pokemon,
            parent_ivs,
            parent_ability,
            parent_gender,
            parent_item,
            parent_nature,
            results,
        },
    ) in data.generate.into_iter().enumerate()
    {
        let min = [0; 6];
        let max = [31; 6];
        let natures = [true; 25];
        let powers = [true; 16];
        let profile = Profile8::new("-".to_string(), Game::BD, 12345, 54321, false, true, true);

        let daycare = Daycare::new(
            parent_ivs,
            parent_ability,
            parent_gender,
            parent_item,
            parent_nature,
            pokemon,
            true,
        );
        let filter = StateFilter8::new(255, 255, 255, false, min, max, natures, powers);
        let generator = EggGenerator8::new(0, 9, 0, 88, &daycare, &profile, &filter);

        let states = generator.generate(seed0, seed1);

        assert_states(results, states, name, num);
    }
}
