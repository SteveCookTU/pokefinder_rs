use crate::gen3::STATIC_DATA;
use crate::get_test_data;
use pokefinder_rs_core::enums::{Game, Method};
use pokefinder_rs_core::gen3::filters::StateFilter3;
use pokefinder_rs_core::gen3::generators::StaticGenerator3;
use pokefinder_rs_core::gen3::searchers::StaticSearcher3;
use pokefinder_rs_core::gen3::{encounters3, Profile3};
use serde::Deserialize;

#[derive(Deserialize)]
struct Static3Data<'a> {
    #[serde(borrow)]
    staticsearcher3: StaticSearcher3Data<'a>,
}

#[derive(Deserialize)]
struct StaticSearcher3Data<'a> {
    #[serde(borrow)]
    search: Vec<StaticSearcher3SearchData<'a>>,
}

#[derive(Deserialize)]
struct StaticSearcher3SearchData<'a> {
    name: &'a str,
    min: [u8; 6],
    max: [u8; 6],
    version: u32,
    method: u8,
    category: usize,
    pokemon: usize,
    results: usize,
}

#[test]
fn search() {
    let data = get_test_data::<'static, Static3Data>(STATIC_DATA);

    for (num, search_data) in data.staticsearcher3.search.into_iter().enumerate() {
        let StaticSearcher3SearchData {
            name,
            min,
            max,
            version,
            method,
            category,
            pokemon,
            results,
        } = search_data;

        let natures = [true; 25];
        let powers = [true; 16];
        let profile = Profile3::new(
            "-".to_string(),
            Game::from_bits_truncate(version),
            12345,
            54321,
            false,
        );

        let static_template = encounters3::get_static_encounter(category, pokemon);
        let filter = StateFilter3::new(255, 255, 255, false, min, max, natures, powers);
        let searcher = StaticSearcher3::new(Method::from(method), &profile, &filter);

        searcher.start_search(min, max, static_template);
        let states = searcher.get_results();

        assert_eq!(
            states.len(),
            results,
            "Unequal results size: {} - {}",
            name,
            num
        );

        for (i, state) in states.into_iter().enumerate() {
            let generator = StaticGenerator3::new(0, 0, 0, Method::from(method), &profile, &filter);
            let generate_states = generator.generate(state.seed, static_template);
            assert_eq!(
                generate_states.len(),
                1,
                "No generator state found: {} - {} - {}",
                name,
                num,
                i
            );
            assert_eq!(
                state.base, generate_states[0].base,
                "States not equal: {} - {} - {}",
                name, num, i
            );
        }
    }
}
