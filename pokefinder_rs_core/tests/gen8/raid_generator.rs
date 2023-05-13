use crate::{assert_states, get_test_data};
use pokefinder_rs_core::enums::Game;
use pokefinder_rs_core::gen8::filters::StateFilter8;
use pokefinder_rs_core::gen8::generators::RaidGenerator;
use pokefinder_rs_core::gen8::{den_loader, Profile8};
use pokefinder_rs_core::parents::states::GeneratorState;
use serde::Deserialize;

#[derive(Deserialize)]
struct RaidData<'a> {
    #[serde(borrow)]
    generate: Vec<GeneratorData<'a>>,
}

#[derive(Deserialize)]
struct GeneratorData<'a> {
    name: &'a str,
    seed: u64,
    version: u32,
    #[serde(rename = "denIndex")]
    den_index: usize,
    rarity: usize,
    #[serde(rename = "raidIndex")]
    raid_index: usize,
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

const JSON_DATA: &str = include_str!("raid.json");

#[test]
fn generate() {
    let data = get_test_data::<'static, RaidData>(JSON_DATA);

    for (
        num,
        GeneratorData {
            name,
            seed,
            version,
            den_index,
            rarity,
            raid_index,
            level,
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
            Game::from_bits_retain(version),
            12345,
            54321,
            false,
            false,
            false,
        );

        let den = den_loader::get_den(den_index, rarity);
        let raid = den.get_raid(raid_index, Game::from_bits_retain(version));

        let filter = StateFilter8::new(255, 255, 255, false, min, max, natures, powers);
        let generator = RaidGenerator::new(0, 9, 0, &profile, &filter);

        let states = generator.generate(seed, level, &raid);

        assert_states(results, states, name, num);
    }
}
