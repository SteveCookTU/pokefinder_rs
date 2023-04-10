use crate::get_test_data;
use pokefinder_rs_core::rng::{PokeRNG, PokeRNGR, ARNG, ARNGR, XDRNG, XDRNGR};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
struct LCRNGTestData<'a> {
    #[serde(borrow)]
    advance: [LCRNGAdvanceData<'a>; 4],
    #[serde(borrow)]
    jump: [LCRNGJumpData<'a>; 4],
    #[serde(borrow)]
    next: [LCRNGNextData<'a>; 4],
}

#[derive(Clone, Deserialize)]
struct LCRNGAdvanceData<'a> {
    name: &'a str,
    seed: u32,
    advances: u32,
    results: [u32; 6],
}

#[derive(Clone, Deserialize)]
struct LCRNGJumpData<'a> {
    name: &'a str,
    seed: u32,
    advances: u32,
    results: [u32; 6],
}

#[derive(Clone, Deserialize)]
struct LCRNGNextData<'a> {
    name: &'a str,
    seed: u32,
    results: [u32; 6],
}

const JSON_STR: &str = include_str!("lcrng.json");

#[test]
fn advance() {
    let data = get_test_data::<'static, LCRNGTestData>(JSON_STR);
    for LCRNGAdvanceData {
        name,
        seed,
        advances,
        results,
    } in data.advance
    {
        assert_eq!(
            ARNG::new(seed).advance(advances),
            results[0],
            "ARNG: {}",
            name
        );
        assert_eq!(
            ARNGR::new(seed).advance(advances),
            results[1],
            "ARNGR: {}",
            name
        );
        assert_eq!(
            PokeRNG::new(seed).advance(advances),
            results[2],
            "PokeRNG: {}",
            name
        );
        assert_eq!(
            PokeRNGR::new(seed).advance(advances),
            results[3],
            "PokeRNGR: {}",
            name
        );
        assert_eq!(
            XDRNG::new(seed).advance(advances),
            results[4],
            "XDRNG: {}",
            name
        );
        assert_eq!(
            XDRNGR::new(seed).advance(advances),
            results[5],
            "XDRNGR: {}",
            name
        );
    }
}

#[test]
fn jump() {
    let data = get_test_data::<'static, LCRNGTestData>(JSON_STR);
    for LCRNGJumpData {
        name,
        seed,
        advances,
        results,
    } in data.jump
    {
        assert_eq!(ARNG::new(seed).jump(advances), results[0], "ARNG: {}", name);
        assert_eq!(
            ARNGR::new(seed).jump(advances),
            results[1],
            "ARNGR: {}",
            name
        );
        assert_eq!(
            PokeRNG::new(seed).jump(advances),
            results[2],
            "PokeRNG: {}",
            name
        );
        assert_eq!(
            PokeRNGR::new(seed).jump(advances),
            results[3],
            "PokeRNGR: {}",
            name
        );
        assert_eq!(
            XDRNG::new(seed).jump(advances),
            results[4],
            "XDRNG: {}",
            name
        );
        assert_eq!(
            XDRNGR::new(seed).jump(advances),
            results[5],
            "XDRNGR: {}",
            name
        );
    }
}

#[test]
fn next() {
    let data = get_test_data::<'static, LCRNGTestData>(JSON_STR);
    for LCRNGNextData {
        name,
        seed,
        results,
    } in data.next
    {
        assert_eq!(ARNG::new(seed).next(), results[0], "ARNG: {}", name);
        assert_eq!(ARNGR::new(seed).next(), results[1], "ARNGR: {}", name);
        assert_eq!(PokeRNG::new(seed).next(), results[2], "PokeRNG: {}", name);
        assert_eq!(PokeRNGR::new(seed).next(), results[3], "PokeRNGR: {}", name);
        assert_eq!(XDRNG::new(seed).next(), results[4], "XDRNG: {}", name);
        assert_eq!(XDRNGR::new(seed).next(), results[5], "XDRNGR: {}", name);
    }
}
