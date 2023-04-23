use crate::gen4::WILD_DATA;
use crate::{assert_states, get_test_data};
use pokefinder_rs_core::enums::{Encounter, Game, Lead, Method};
use pokefinder_rs_core::gen4::filters::WildStateFilter4;
use pokefinder_rs_core::gen4::generators::WildGenerator4;
use pokefinder_rs_core::gen4::states::WildGeneratorState4;
use pokefinder_rs_core::gen4::{encounters4, Profile4};
use pokefinder_rs_core::parents::EncounterAreaT;
use serde::Deserialize;

#[derive(Deserialize)]
struct Wild4Data<'a> {
    #[serde(borrow)]
    wildgenerator4: GeneratorData<'a>,
}

#[derive(Deserialize)]
struct GeneratorData<'a> {
    #[serde(rename = "generateMethodJ", borrow)]
    generate_method_j: Vec<GenerateMethodJData<'a>>,
    #[serde(rename = "generateMethodK", borrow)]
    generate_method_k: Vec<GenerateMethodKData<'a>>,
    #[serde(rename = "generatePokeRadar", borrow)]
    generate_poke_radar: Vec<GeneratePokeRadarData<'a>>,
}

#[derive(Deserialize)]
struct GenerateMethodJData<'a> {
    name: &'a str,
    seed: u32,
    version: u32,
    encounter: u8,
    lead: u8,
    location: u8,
    results: Vec<GeneratorResult>,
}

#[derive(Deserialize)]
struct GenerateMethodKData<'a> {
    name: &'a str,
    seed: u32,
    version: u32,
    encounter: u8,
    lead: u8,
    location: u8,
    results: Vec<GeneratorResult>,
}

#[derive(Deserialize)]
struct GeneratePokeRadarData<'a> {
    name: &'a str,
    seed: u32,
    version: u32,
    encounter: u8,
    lead: u8,
    location: u8,
    shiny: bool,
    index: u8,
    results: Vec<GeneratorResult>,
}

#[derive(Deserialize, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct GeneratorResult {
    ability: u8,
    #[serde(rename = "abilityIndex")]
    ability_index: u16,
    advances: u32,
    #[serde(rename = "battleAdvances")]
    battle_advances: u32,
    call: u8,
    characteristic: u8,
    chatot: u8,
    #[serde(rename = "encounterSlot")]
    encounter_slot: u8,
    form: u8,
    gender: u8,
    #[serde(rename = "hiddenPower")]
    hidden_power: u8,
    #[serde(rename = "hiddenPowerStrength")]
    hidden_power_strength: u8,
    item: u16,
    ivs: [u8; 6],
    level: u8,
    nature: u8,
    pid: u32,
    shiny: u8,
    specie: u16,
    stats: [u16; 6],
}

impl From<WildGeneratorState4> for GeneratorResult {
    fn from(value: WildGeneratorState4) -> Self {
        Self {
            ability: value.base.base.base.ability,
            ability_index: value.base.base.base.ability_index,
            advances: value.base.advances,
            battle_advances: value.battle_advances,
            call: value.call,
            characteristic: value.base.base.base.characteristic,
            chatot: value.chatot,
            encounter_slot: value.base.base.encounter_slot,
            form: value.base.base.form,
            gender: value.base.base.base.gender,
            hidden_power: value.base.base.base.hidden_power,
            hidden_power_strength: value.base.base.base.hidden_power_strength,
            item: value.base.base.item,
            ivs: value.base.base.base.ivs,
            level: value.base.base.base.level,
            nature: value.base.base.base.nature,
            pid: value.base.base.base.pid,
            shiny: value.base.base.base.shiny,
            specie: value.base.base.specie,
            stats: value.base.base.base.stats,
        }
    }
}

#[test]
fn generate_method_j() {
    let data = get_test_data::<'static, Wild4Data>(WILD_DATA);
    for (
        num,
        GenerateMethodJData {
            name,
            seed,
            version,
            encounter,
            lead,
            location,
            results,
        },
    ) in data
        .wildgenerator4
        .generate_method_j
        .into_iter()
        .enumerate()
    {
        let min = [0; 6];
        let max = [31; 6];
        let natures = [true; 25];
        let powers = [true; 16];
        let encounter_slots = [true; 12];
        let replacement = [0; 2];
        let blocks = [0; 5];
        let profile = Profile4::new(
            "".to_string(),
            Game::from_bits_retain(version),
            12345,
            54321,
            false,
        );

        let encounter_areas = encounters4::get_encounters(
            Encounter::from(encounter),
            0,
            Game::NONE,
            false,
            0,
            false,
            replacement,
            blocks,
            &profile,
        );
        let encounter_area = encounter_areas
            .into_iter()
            .find(|ea| ea.get_location() == location)
            .unwrap();

        let filter = WildStateFilter4::new(
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
        let generator = WildGenerator4::new(
            0,
            9,
            0,
            Method::MethodJ,
            Encounter::from(encounter),
            Lead::from_bits_retain(lead),
            false,
            &profile,
            &filter,
        );

        let states = generator.generate(seed, &encounter_area, 0);

        assert_states(results, states, name, num);
    }
}

#[test]
fn generate_method_k() {
    let data = get_test_data::<'static, Wild4Data>(WILD_DATA);
    for (
        num,
        GenerateMethodKData {
            name,
            seed,
            version,
            encounter,
            lead,
            location,
            results,
        },
    ) in data
        .wildgenerator4
        .generate_method_k
        .into_iter()
        .enumerate()
    {
        let min = [0; 6];
        let max = [31; 6];
        let natures = [true; 25];
        let powers = [true; 16];
        let encounter_slots = [true; 12];
        let replacement = [0; 2];
        let blocks = [0; 5];
        let profile = Profile4::new(
            "".to_string(),
            Game::from_bits_retain(version),
            12345,
            54321,
            false,
        );

        let encounter_areas = encounters4::get_encounters(
            Encounter::from(encounter),
            0,
            Game::NONE,
            false,
            0,
            false,
            replacement,
            blocks,
            &profile,
        );
        let encounter_area = encounter_areas
            .into_iter()
            .find(|ea| ea.get_location() == location)
            .unwrap();

        let filter = WildStateFilter4::new(
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
        let generator = WildGenerator4::new(
            0,
            9,
            0,
            Method::MethodK,
            Encounter::from(encounter),
            Lead::from_bits_retain(lead),
            false,
            &profile,
            &filter,
        );

        let states = generator.generate(seed, &encounter_area, 0);

        assert_states(results, states, name, num);
    }
}

#[test]
fn generate_poke_radar() {
    let data = get_test_data::<'static, Wild4Data>(WILD_DATA);
    for (
        num,
        GeneratePokeRadarData {
            name,
            seed,
            version,
            encounter,
            lead,
            location,
            shiny,
            index,
            results,
        },
    ) in data
        .wildgenerator4
        .generate_poke_radar
        .into_iter()
        .enumerate()
    {
        let min = [0; 6];
        let max = [31; 6];
        let natures = [true; 25];
        let powers = [true; 16];
        let encounter_slots = [true; 12];
        let replacement = [0; 2];
        let blocks = [0; 5];
        let profile = Profile4::new(
            "".to_string(),
            Game::from_bits_retain(version),
            12345,
            54321,
            false,
        );

        let encounter_areas = encounters4::get_encounters(
            Encounter::from(encounter),
            0,
            Game::NONE,
            true,
            0,
            false,
            replacement,
            blocks,
            &profile,
        );
        let encounter_area = encounter_areas
            .into_iter()
            .find(|ea| ea.get_location() == location)
            .unwrap();

        let filter = WildStateFilter4::new(
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
        let generator = WildGenerator4::new(
            0,
            9,
            0,
            Method::PokeRadar,
            Encounter::from(encounter),
            Lead::from_bits_retain(lead),
            shiny,
            &profile,
            &filter,
        );

        let states = generator.generate(seed, &encounter_area, index);

        assert_states(results, states, name, num);
    }
}
