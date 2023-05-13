use crate::{assert_states, get_test_data};
use pokefinder_rs_core::enums::{Encounter, Game, Lead};
use pokefinder_rs_core::gen8::filters::WildStateFilter8;
use pokefinder_rs_core::gen8::generators::WildGenerator8;
use pokefinder_rs_core::gen8::{encounters8, Profile8};
use pokefinder_rs_core::parents::states::WildGeneratorState;
use serde::Deserialize;

#[derive(Deserialize)]
struct Wild8Data<'a> {
    #[serde(borrow)]
    generate: Vec<GeneratorData<'a>>,
}

#[derive(Deserialize)]
struct GeneratorData<'a> {
    name: &'a str,
    seed0: u64,
    seed1: u64,
    encounter: u8,
    lead: u8,
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
    #[serde(rename = "encounterSlot")]
    encounter_slot: u8,
    form: u8,
    gender: u8,
    #[serde(rename = "hiddenPower")]
    hidden_power: u8,
    item: u16,
    ivs: [u8; 6],
    level: u8,
    nature: u8,
    pid: u32,
    shiny: u8,
    specie: u16,
    stats: [u16; 6],
}

impl From<WildGeneratorState> for GeneratorResult {
    fn from(value: WildGeneratorState) -> Self {
        Self {
            ability: value.base.base.ability,
            ability_index: value.base.base.ability_index,
            advances: value.advances,
            characteristic: value.base.base.characteristic,
            ec: value.base.base.ec,
            encounter_slot: value.base.encounter_slot,
            form: value.base.form,
            gender: value.base.base.gender,
            hidden_power: value.base.base.hidden_power,
            item: value.base.item,
            ivs: value.base.base.ivs,
            level: value.base.base.level,
            nature: value.base.base.nature,
            pid: value.base.base.pid,
            shiny: value.base.base.shiny,
            specie: value.base.specie,
            stats: value.base.base.stats,
        }
    }
}

const JSON_DATA: &str = include_str!("wild8.json");

#[test]
fn generate() {
    let data = get_test_data::<'static, Wild8Data>(JSON_DATA);
    for (
        num,
        GeneratorData {
            name,
            seed0,
            seed1,
            encounter,
            lead,
            location,
            results,
        },
    ) in data.generate.into_iter().enumerate()
    {
        let min = [0; 6];
        let max = [31; 6];
        let natures = [true; 25];
        let powers = [true; 16];
        let encounter_slots = [true; 12];
        let replacement = [0; 2];

        let encounter = Encounter::from(encounter);
        let lead = Lead::from_bits_retain(lead);

        let profile = Profile8::new("-".to_string(), Game::BD, 12345, 54321, false, false, false);

        let encounter_areas =
            encounters8::get_encounters(encounter, 0, false, false, replacement, &profile);
        let encounter_area = encounter_areas
            .into_iter()
            .find(|ea| ea.base.location == location)
            .unwrap();

        let filter = WildStateFilter8::new(
            255,
            255,
            255,
            false,
            min,
            max,
            natures,
            powers,
            encounter_slots,
        );
        let generator = WildGenerator8::new(0, 9, 0, encounter, lead, &profile, &filter);

        let states = generator.generate(seed0, seed1, &encounter_area);

        assert_states(results, states, name, num);
    }
}
