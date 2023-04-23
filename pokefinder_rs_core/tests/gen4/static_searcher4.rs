use crate::gen4::STATIC_DATA;
use crate::get_test_data;
use pokefinder_rs_core::enums::{Game, Lead, Method};
use pokefinder_rs_core::gen4::filters::StateFilter4;
use pokefinder_rs_core::gen4::generators::StaticGenerator4;
use pokefinder_rs_core::gen4::searchers::StaticSearcher4;
use pokefinder_rs_core::gen4::{encounters4, Profile4};
use serde::Deserialize;

#[derive(Deserialize)]
struct Static4Data<'a> {
    #[serde(borrow)]
    staticsearcher4: Static4SearcherData<'a>,
}

#[derive(Deserialize)]
struct Static4SearcherData<'a> {
    #[serde(rename = "searchMethod1", borrow)]
    search_method_1: Vec<Method1Data<'a>>,
    #[serde(rename = "searchMethodJ", borrow)]
    search_method_j: Vec<MethodJData<'a>>,
    #[serde(rename = "searchMethodK", borrow)]
    search_method_k: Vec<MethodKData<'a>>,
}

#[derive(Deserialize)]
struct Method1Data<'a> {
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
    category: u8,
    pokemon: u8,
    results: usize,
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
    category: u8,
    pokemon: u8,
    lead: u8,
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
    category: u8,
    pokemon: u8,
    lead: u8,
    results: usize,
}

#[test]
fn search_method_1() {
    let data = get_test_data::<'static, Static4Data>(STATIC_DATA);

    for (
        num,
        Method1Data {
            name,
            min,
            max,
            min_advance,
            max_advance,
            min_delay,
            max_delay,
            version,
            category,
            pokemon,
            results,
        },
    ) in data.staticsearcher4.search_method_1.into_iter().enumerate()
    {
        let natures = [true; 25];
        let powers = [true; 16];
        let profile = Profile4::new(
            "-".to_string(),
            Game::from_bits_retain(version),
            12345,
            54321,
            false,
        );

        let static_template =
            encounters4::get_static_encounter(category as usize, pokemon as usize);
        let filter = StateFilter4::new(255, 255, 255, false, min, max, natures, powers);
        let searcher = StaticSearcher4::new(
            min_advance,
            max_advance,
            min_delay,
            max_delay,
            Method::Method1,
            Lead::NONE,
            &profile,
            &filter,
        );

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
            let generator = StaticGenerator4::new(
                state.advances,
                0,
                0,
                Method::Method1,
                Lead::NONE,
                &profile,
                &filter,
            );
            let generator_states = generator.generate(state.base.seed, static_template);

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
fn search_method_j() {
    let data = get_test_data::<'static, Static4Data>(STATIC_DATA);

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
            category,
            pokemon,
            lead,
            results,
        },
    ) in data.staticsearcher4.search_method_j.into_iter().enumerate()
    {
        let natures = [true; 25];
        let powers = [true; 16];
        let profile = Profile4::new(
            "-".to_string(),
            Game::from_bits_retain(version),
            12345,
            54321,
            false,
        );

        let lead = Lead::from_bits_retain(lead);

        let static_template =
            encounters4::get_static_encounter(category as usize, pokemon as usize);
        let filter = StateFilter4::new(255, 255, 255, false, min, max, natures, powers);
        let searcher = StaticSearcher4::new(
            min_advance,
            max_advance,
            min_delay,
            max_delay,
            Method::MethodJ,
            lead,
            &profile,
            &filter,
        );

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
            let generator = StaticGenerator4::new(
                state.advances,
                0,
                0,
                Method::MethodJ,
                if lead > Lead::SYNCHRONIZE_END {
                    lead
                } else {
                    Lead::from_bits_retain(lead.bits() + state.base.base.nature)
                },
                &profile,
                &filter,
            );
            let generator_states = generator.generate(state.base.seed, static_template);

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
fn search_method_k() {
    let data = get_test_data::<'static, Static4Data>(STATIC_DATA);

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
            category,
            pokemon,
            lead,
            results,
        },
    ) in data.staticsearcher4.search_method_k.into_iter().enumerate()
    {
        let natures = [true; 25];
        let powers = [true; 16];
        let profile = Profile4::new(
            "-".to_string(),
            Game::from_bits_retain(version),
            12345,
            54321,
            false,
        );

        let lead = Lead::from_bits_retain(lead);

        let static_template =
            encounters4::get_static_encounter(category as usize, pokemon as usize);
        let filter = StateFilter4::new(255, 255, 255, false, min, max, natures, powers);
        let searcher = StaticSearcher4::new(
            min_advance,
            max_advance,
            min_delay,
            max_delay,
            Method::MethodK,
            lead,
            &profile,
            &filter,
        );

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
            let generator = StaticGenerator4::new(
                state.advances,
                0,
                0,
                Method::MethodK,
                if lead > Lead::SYNCHRONIZE_END {
                    lead
                } else {
                    Lead::from_bits_retain(lead.bits() + state.base.base.nature)
                },
                &profile,
                &filter,
            );
            let generator_states = generator.generate(state.base.seed, static_template);

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
