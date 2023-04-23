use crate::gen4::WILD_DATA;
use crate::get_test_data;
use pokefinder_rs_core::enums::{Encounter, Game, Lead, Method};
use pokefinder_rs_core::gen4::filters::WildStateFilter4;
use pokefinder_rs_core::gen4::generators::WildGenerator4;
use pokefinder_rs_core::gen4::searchers::WildSearcher4;
use pokefinder_rs_core::gen4::{encounters4, Profile4};
use pokefinder_rs_core::parents::EncounterAreaT;
use serde::Deserialize;

#[derive(Deserialize)]
struct Wild4Data<'a> {
    #[serde(borrow)]
    wildsearcher4: WildSearcherData<'a>,
}

#[derive(Deserialize)]
struct WildSearcherData<'a> {
    #[serde(rename = "searchMethodJ", borrow)]
    search_method_j: Vec<MethodJData<'a>>,
    #[serde(rename = "searchMethodK", borrow)]
    search_method_k: Vec<MethodKData<'a>>,
    #[serde(rename = "searchPokeRadar", borrow)]
    search_poke_radar: Vec<PokeRadarData<'a>>,
}

#[derive(Deserialize)]
struct MethodJData<'a> {
    name: &'a str,
    min: [u8; 6],
    max: [u8; 6],
    #[serde(rename = "minAdvance")]
    min_advance: u32,
    #[serde(rename = "maxAdvance")]
    max_advance: u32,
    #[serde(rename = "minDelay")]
    min_delay: u32,
    #[serde(rename = "maxDelay")]
    max_delay: u32,
    version: u32,
    encounter: u8,
    lead: u8,
    location: u8,
    results: usize,
}

#[derive(Deserialize)]
struct MethodKData<'a> {
    name: &'a str,
    min: [u8; 6],
    max: [u8; 6],
    #[serde(rename = "minAdvance")]
    min_advance: u32,
    #[serde(rename = "maxAdvance")]
    max_advance: u32,
    #[serde(rename = "minDelay")]
    min_delay: u32,
    #[serde(rename = "maxDelay")]
    max_delay: u32,
    version: u32,
    encounter: u8,
    lead: u8,
    location: u8,
    results: usize,
}

#[derive(Deserialize)]
struct PokeRadarData<'a> {
    name: &'a str,
    min: [u8; 6],
    max: [u8; 6],
    #[serde(rename = "minAdvance")]
    min_advance: u32,
    #[serde(rename = "maxAdvance")]
    max_advance: u32,
    #[serde(rename = "minDelay")]
    min_delay: u32,
    #[serde(rename = "maxDelay")]
    max_delay: u32,
    version: u32,
    encounter: u8,
    lead: u8,
    location: u8,
    shiny: bool,
    index: u8,
    results: usize,
}

#[test]
pub fn search_method_j() {
    let data = get_test_data::<'static, Wild4Data>(WILD_DATA);

    for (
        num,
        MethodJData {
            name,
            min,
            max,
            min_advance,
            max_advance,
            min_delay,
            max_delay,
            version,
            encounter,
            lead,
            location,
            results,
        },
    ) in data.wildsearcher4.search_method_j.into_iter().enumerate()
    {
        let natures = [true; 25];
        let powers = [true; 16];
        let encounter_slots = [true; 12];
        let replacement = [0; 2];
        let blocks = [0; 5];
        let profile = Profile4::new(
            "-".to_string(),
            Game::from_bits_retain(version),
            12345,
            54321,
            false,
        );

        let lead = Lead::from_bits_retain(lead);

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
        let searcher = WildSearcher4::new(
            min_advance,
            max_advance,
            min_delay,
            max_delay,
            Method::MethodJ,
            Encounter::from(encounter),
            lead,
            false,
            &encounter_area,
            &profile,
            &filter,
        );

        searcher.start_search(min, max, 0);
        let states = searcher.get_results();
        assert_eq!(
            states.len(),
            results,
            "Unequal results size: {} - {}",
            name,
            num
        );

        for (i, state) in states.into_iter().enumerate() {
            let generator = WildGenerator4::new(
                state.advances,
                0,
                0,
                Method::MethodJ,
                Encounter::from(encounter),
                if lead > Lead::SYNCHRONIZE_END {
                    lead
                } else {
                    Lead::from_bits_retain(lead.bits() + state.base.base.base.nature)
                },
                false,
                &profile,
                &filter,
            );
            let generator_states = generator.generate(state.base.seed, &encounter_area, 0);

            assert_eq!(
                generator_states.len(),
                1,
                "No generator state found: {} - {} - {}",
                name,
                num,
                i
            );
            assert_eq!(
                state.base.base, generator_states[0].base.base,
                "States not equal: {} - {} - {}",
                name, num, i
            );
        }
    }
}

#[test]
pub fn search_method_k() {
    let data = get_test_data::<'static, Wild4Data>(WILD_DATA);

    for (
        num,
        MethodKData {
            name,
            min,
            max,
            min_advance,
            max_advance,
            min_delay,
            max_delay,
            version,
            encounter,
            lead,
            location,
            results,
        },
    ) in data.wildsearcher4.search_method_k.into_iter().enumerate()
    {
        let natures = [true; 25];
        let powers = [true; 16];
        let encounter_slots = [true; 12];
        let replacement = [0; 2];
        let blocks = [0; 5];
        let profile = Profile4::new(
            "-".to_string(),
            Game::from_bits_retain(version),
            12345,
            54321,
            false,
        );

        let lead = Lead::from_bits_retain(lead);

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
        let searcher = WildSearcher4::new(
            min_advance,
            max_advance,
            min_delay,
            max_delay,
            Method::MethodK,
            Encounter::from(encounter),
            lead,
            false,
            &encounter_area,
            &profile,
            &filter,
        );

        searcher.start_search(min, max, 0);
        let states = searcher.get_results();
        assert_eq!(
            states.len(),
            results,
            "Unequal results size: {} - {}",
            name,
            num
        );

        for (i, state) in states.into_iter().enumerate() {
            let generator = WildGenerator4::new(
                state.advances,
                0,
                0,
                Method::MethodK,
                Encounter::from(encounter),
                if lead > Lead::SYNCHRONIZE_END {
                    lead
                } else {
                    Lead::from_bits_retain(lead.bits() + state.base.base.base.nature)
                },
                false,
                &profile,
                &filter,
            );
            let generator_states = generator.generate(state.base.seed, &encounter_area, 0);

            assert_eq!(
                generator_states.len(),
                1,
                "No generator state found: {} - {} - {}",
                name,
                num,
                i
            );
            assert_eq!(
                state.base.base, generator_states[0].base.base,
                "States not equal: {} - {} - {}",
                name, num, i
            );
        }
    }
}

#[test]
pub fn search_poke_radar() {
    let data = get_test_data::<'static, Wild4Data>(WILD_DATA);

    for (
        num,
        PokeRadarData {
            name,
            min,
            max,
            min_advance,
            max_advance,
            min_delay,
            max_delay,
            version,
            encounter,
            lead,
            location,
            shiny,
            index,
            results,
        },
    ) in data.wildsearcher4.search_poke_radar.into_iter().enumerate()
    {
        let natures = [true; 25];
        let powers = [true; 16];
        let encounter_slots = [true; 12];
        let replacement = [0; 2];
        let blocks = [0; 5];
        let profile = Profile4::new(
            "-".to_string(),
            Game::from_bits_retain(version),
            12345,
            54321,
            false,
        );

        let lead = Lead::from_bits_retain(lead);

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
        let searcher = WildSearcher4::new(
            min_advance,
            max_advance,
            min_delay,
            max_delay,
            Method::PokeRadar,
            Encounter::from(encounter),
            lead,
            shiny,
            &encounter_area,
            &profile,
            &filter,
        );

        searcher.start_search(min, max, index);
        let states = searcher.get_results();
        assert_eq!(
            states.len(),
            results,
            "Unequal results size: {} - {}",
            name,
            num
        );

        for (i, state) in states.into_iter().enumerate() {
            let generator = WildGenerator4::new(
                state.advances,
                0,
                0,
                Method::PokeRadar,
                Encounter::from(encounter),
                if lead > Lead::SYNCHRONIZE_END {
                    lead
                } else {
                    Lead::from_bits_retain(lead.bits() + state.base.base.base.nature)
                },
                shiny,
                &profile,
                &filter,
            );
            let generator_states = generator.generate(state.base.seed, &encounter_area, index);

            assert_eq!(
                generator_states.len(),
                1,
                "No generator state found: {} - {} - {}",
                name,
                num,
                i
            );
            assert_eq!(
                state.base.base, generator_states[0].base.base,
                "States not equal: {} - {} - {}",
                name, num, i
            );
        }
    }
}
