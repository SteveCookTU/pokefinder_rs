use crate::gen3::WILD_DATA;
use crate::get_test_data;
use pokefinder_rs_core::enums::{Encounter, Game, Lead, Method};
use pokefinder_rs_core::gen3::filters::WildStateFilter3;
use pokefinder_rs_core::gen3::generators::WildGenerator3;
use pokefinder_rs_core::gen3::searchers::WildSearcher3;
use pokefinder_rs_core::gen3::{encounters3, Profile3};
use serde::Deserialize;

#[derive(Deserialize)]
struct Wild3Data<'a> {
    #[serde(borrow)]
    wildsearcher3: Wild3SearcherData<'a>,
}

#[derive(Deserialize)]
struct Wild3SearcherData<'a> {
    #[serde(borrow)]
    search: Vec<Wild3SearcherSearchData<'a>>,
}

#[derive(Deserialize)]
struct Wild3SearcherSearchData<'a> {
    name: &'a str,
    min: [u8; 6],
    max: [u8; 6],
    version: u32,
    method: u8,
    encounter: u8,
    lead: u8,
    location: u8,
    results: usize,
}

#[test]
fn search() {
    let data = get_test_data::<'static, Wild3Data>(WILD_DATA);

    for (num, search_data) in data.wildsearcher3.search.into_iter().enumerate() {
        let Wild3SearcherSearchData {
            name,
            min,
            max,
            version,
            method,
            encounter,
            lead,
            location,
            results,
        } = search_data;

        let natures = [true; 25];
        let powers = [true; 16];
        let encounter_slots = [true; 12];

        let profile = Profile3::new(
            "-".to_string(),
            Game::from_bits_truncate(version),
            12345,
            54321,
            false,
        );

        let encounter_areas = encounters3::get_encounters(
            Encounter::from(encounter),
            Game::from_bits_truncate(version),
        );

        let encounter_area = encounter_areas
            .into_iter()
            .find(|encounter_area| encounter_area.location == location)
            .unwrap();

        let filter = WildStateFilter3::new(
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

        let lead = Lead::from_bits_retain(lead);

        let searcher = WildSearcher3::new(
            Method::from(method),
            Encounter::from(encounter),
            lead,
            &encounter_area,
            &profile,
            &filter,
        );

        searcher.start_search(min, max);
        let states = searcher.get_results();

        assert_eq!(
            states.len(),
            results,
            "Unequal results size: {} - {}",
            name,
            num
        );

        for (i, state) in states.into_iter().enumerate() {
            let generator = WildGenerator3::new(
                0,
                0,
                0,
                Method::from(method),
                Encounter::from(encounter),
                if lead > Lead::SYNCHRONIZE_END {
                    lead
                } else {
                    Lead::from_bits_retain(lead.bits() + state.base.base.base.nature)
                },
                &profile,
                &filter,
            );
            let generate_states = generator.generate(state.base.seed, &encounter_area);
            assert_eq!(
                generate_states.len(),
                1,
                "No generator state found: {} - {} - {}",
                name,
                num,
                i
            );
            assert_eq!(
                state.base.base, generate_states[0].base,
                "States not equal: {} - {} - {}",
                name, num, i
            );
        }
    }
}
