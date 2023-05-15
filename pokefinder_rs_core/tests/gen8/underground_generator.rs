use crate::{assert_states, get_test_data};
use pokefinder_rs_core::enums::{Game, Lead};
use pokefinder_rs_core::gen8::filters::UndergroundStateFilter;
use pokefinder_rs_core::gen8::generators::UndergroundGenerator;
use pokefinder_rs_core::gen8::states::UndergroundState;
use pokefinder_rs_core::gen8::{encounters8, Profile8};
use serde::Deserialize;

#[derive(Deserialize)]
struct UndergroundData<'a> {
    #[serde(borrow)]
    generate: Vec<GeneratorData<'a>>,
}

#[derive(Deserialize)]
struct GeneratorData<'a> {
    name: &'a str,
    seed0: u64,
    seed1: u64,
    lead: u8,
    diglett: bool,
    #[serde(rename = "levelFlag")]
    level_flag: u8,
    #[serde(rename = "storyFlag")]
    story_flag: u8,
    location: u8,
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
    #[serde(rename = "eggMove")]
    egg_move: u16,
    gender: u8,
    item: u16,
    ivs: [u8; 6],
    level: u8,
    nature: u8,
    pid: u32,
    shiny: u8,
    species: u16,
    stats: [u16; 6],
}

impl From<UndergroundState> for GeneratorResult {
    fn from(value: UndergroundState) -> Self {
        Self {
            ability: value.base.base.ability,
            ability_index: value.base.base.ability_index,
            advances: value.base.advances,
            characteristic: value.base.base.characteristic,
            ec: value.base.base.ec,
            gender: value.base.base.gender,
            item: value.item,
            ivs: value.base.base.ivs,
            level: value.base.base.level,
            nature: value.base.base.nature,
            pid: value.base.base.pid,
            shiny: value.base.base.shiny,
            species: value.species,
            stats: value.base.base.stats,
            egg_move: value.egg_move,
        }
    }
}

const JSON_DATA: &str = include_str!("underground.json");

#[test]
fn generate() {
    let data = get_test_data::<'static, UndergroundData>(JSON_DATA);

    for (
        num,
        GeneratorData {
            name,
            seed0,
            seed1,
            lead,
            diglett,
            level_flag,
            story_flag,
            location,
            results,
        },
    ) in data.generate.into_iter().enumerate()
    {
        let min = [0; 6];
        let max = [31; 6];
        let natures = [true; 25];
        let powers = [true; 16];

        let profile = Profile8::new("-".to_string(), Game::BD, 12345, 54321, false, false, false);

        let encounter_areas =
            encounters8::get_underground_encounters(story_flag, diglett, &profile);
        let encounter_area = encounter_areas
            .into_iter()
            .find(|ea| ea.location == location)
            .unwrap();

        let species = encounter_area.get_species();
        let filter =
            UndergroundStateFilter::new(255, 255, 255, false, min, max, natures, powers, &species);
        let generator = UndergroundGenerator::new(
            0,
            9,
            0,
            Lead::from_bits_retain(lead),
            diglett,
            level_flag,
            &profile,
            &filter,
        );

        let states = generator.generate(seed0, seed1, &encounter_area);

        assert_states(results, states, name, num);
    }
}
