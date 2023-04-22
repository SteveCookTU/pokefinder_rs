use crate::{assert_states, get_test_data};
use pokefinder_rs_core::enums::Game;
use pokefinder_rs_core::gen4::filters::StateFilter4;
use pokefinder_rs_core::gen4::generators::EggGenerator4;
use pokefinder_rs_core::gen4::states::EggGeneratorState4;
use pokefinder_rs_core::gen4::Profile4;
use pokefinder_rs_core::parents::Daycare;
use serde::Deserialize;

#[derive(Deserialize)]
struct EggGenerator4Data<'a> {
    #[serde(borrow)]
    generate: Vec<GeneratorData<'a>>,
}

#[derive(Deserialize)]
struct GeneratorData<'a> {
    name: &'a str,
    version: u32,
    seed: u32,
    #[serde(rename = "seedPickup")]
    seed_pickup: u32,
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
    masuda: bool,
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
    inheritance: [u8; 6],
    ivs: [u8; 6],
    level: u8,
    nature: u8,
    #[serde(rename = "pickupAdvances")]
    pickup_advances: u32,
    pid: u32,
    shiny: u8,
    stats: [u16; 6],
}

impl From<EggGeneratorState4> for GeneratorResult {
    fn from(value: EggGeneratorState4) -> Self {
        Self {
            ability: value.base.base.base.ability,
            ability_index: value.base.base.base.ability_index,
            advances: value.base.advances,
            call: value.call,
            characteristic: value.base.base.base.characteristic,
            chatot: value.chatot,
            gender: value.base.base.base.gender,
            hidden_power: value.base.base.base.hidden_power,
            hidden_power_strength: value.base.base.base.hidden_power_strength,
            inheritance: value.base.base.inheritance,
            ivs: value.base.base.base.ivs,
            level: value.base.base.base.level,
            nature: value.base.base.base.nature,
            pickup_advances: value.pickup_advances,
            pid: value.base.base.base.pid,
            shiny: value.base.base.base.shiny,
            stats: value.base.base.base.stats,
        }
    }
}

const JSON_DATA: &str = include_str!("egg4.json");

#[test]
fn generate() {
    let data = get_test_data::<'static, EggGenerator4Data>(JSON_DATA);

    for (
        num,
        GeneratorData {
            name,
            version,
            seed,
            seed_pickup,
            pokemon,
            parent_ivs,
            parent_ability,
            parent_gender,
            parent_item,
            parent_nature,
            masuda,
            results,
        },
    ) in data.generate.into_iter().enumerate()
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

        let daycare = Daycare::new(
            parent_ivs,
            parent_ability,
            parent_gender,
            parent_item,
            parent_nature,
            pokemon,
            masuda,
        );
        let filter = StateFilter4::new(255, 255, 255, false, min, max, natures, powers);
        let generator = EggGenerator4::new(0, 9, 0, 0, 9, 0, &daycare, &profile, &filter);

        let states = generator.generate(seed, seed_pickup);

        assert_states(results, states, name, num);
    }
}
