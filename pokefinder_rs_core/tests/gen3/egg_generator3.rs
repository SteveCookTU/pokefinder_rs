use crate::{assert_states, get_test_data};
use pokefinder_rs_core::enums::{Game, Method};
use pokefinder_rs_core::gen3::filters::StateFilter3;
use pokefinder_rs_core::gen3::generators::EggGenerator3;
use pokefinder_rs_core::gen3::states::EggState3;
use pokefinder_rs_core::gen3::Profile3;
use pokefinder_rs_core::parents::Daycare;
use serde::Deserialize;

#[derive(Deserialize)]
struct EggGenerator3Data<'a> {
    #[serde(borrow)]
    generate: Vec<EggGenerator3GenerateData<'a>>,
}

#[derive(Deserialize)]
struct EggGenerator3GenerateData<'a> {
    name: &'a str,
    version: u32,
    seed: u32,
    #[serde(rename = "seedPickup")]
    seed_pickup: u32,
    calibration: u8,
    #[serde(rename = "minRedraw")]
    min_redraw: u8,
    #[serde(rename = "maxRedraw")]
    max_redraw: u8,
    method: u8,
    compatability: u8,
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
    results: Vec<EggGenerator3Result>,
}

#[derive(Deserialize, PartialEq, Ord, PartialOrd, Eq, Debug)]
struct EggGenerator3Result {
    ability: u8,
    #[serde(rename = "abilityIndex")]
    ability_index: u16,
    advances: u32,
    gender: u8,
    #[serde(rename = "hiddenPower")]
    hidden_power: u8,
    #[serde(rename = "hiddenPowerStrength")]
    hidden_power_strength: u8,
    inheritance: [u8; 6],
    ivs: [u8; 6],
    level: u8,
    nature: u8,
    #[serde(rename = "pickupAdvances")]
    pickup_advances: u32,
    pid: u32,
    redraws: u8,
    shiny: u8,
    stats: [u16; 6],
}

impl From<EggState3> for EggGenerator3Result {
    fn from(value: EggState3) -> Self {
        Self {
            ability: value.base.base.base.ability,
            ability_index: value.base.base.base.ability_index,
            advances: value.base.advances,
            gender: value.base.base.base.gender,
            hidden_power: value.base.base.base.hidden_power,
            hidden_power_strength: value.base.base.base.hidden_power_strength,
            inheritance: value.base.base.inheritance,
            ivs: value.base.base.base.ivs,
            level: value.base.base.base.level,
            nature: value.base.base.base.nature,
            pickup_advances: value.pickup_advances,
            pid: value.base.base.base.pid,
            redraws: value.redraws,
            shiny: value.base.base.base.shiny,
            stats: value.base.base.base.stats,
        }
    }
}

const JSON_DATA: &str = include_str!("egg3.json");

#[test]
fn generate() {
    let data = get_test_data::<'static, EggGenerator3Data>(JSON_DATA);

    for (num, generate_data) in data.generate.into_iter().enumerate() {
        let EggGenerator3GenerateData {
            name,
            version,
            seed,
            seed_pickup,
            calibration,
            min_redraw,
            max_redraw,
            method,
            compatability,
            pokemon,
            parent_ivs,
            parent_ability,
            parent_gender,
            parent_item,
            parent_nature,
            results,
        } = generate_data;

        let min = [0; 6];
        let max = [31; 6];
        let natures = [true; 25];
        let powers = [true; 16];
        let profile = Profile3::new(
            "-".to_string(),
            Game::from_bits(version).unwrap(),
            12345,
            54321,
            false,
        );

        let daycare = Daycare::new(
            parent_ivs,
            parent_ability,
            parent_gender,
            parent_item,
            parent_nature,
            pokemon,
            false,
        );
        let filter = StateFilter3::new(255, 255, 255, false, min, max, natures, powers);
        let generator = EggGenerator3::new(
            0,
            9,
            0,
            0,
            9,
            0,
            calibration,
            min_redraw,
            max_redraw,
            Method::from(method),
            compatability,
            &daycare,
            &profile,
            &filter,
        );

        let states = generator.generate(seed, seed_pickup);
        assert_states(results, states, name, num);
    }
}
