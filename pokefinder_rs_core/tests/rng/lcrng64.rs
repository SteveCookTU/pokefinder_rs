use crate::get_test_data;
use pokefinder_rs_core::rng::{BWRNG, BWRNGR};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
struct LCRNG64TestData<'a> {
    #[serde(borrow)]
    advance: [LCRNG64AdvanceData<'a>; 4],
    #[serde(borrow)]
    jump: [LCRNG64JumpData<'a>; 4],
    #[serde(borrow)]
    next: [LCRNG64NextData<'a>; 4],
}

#[derive(Clone, Deserialize)]
struct LCRNG64AdvanceData<'a> {
    name: &'a str,
    seed: u64,
    advances: u32,
    results: [u64; 2],
}

#[derive(Clone, Deserialize)]
struct LCRNG64JumpData<'a> {
    name: &'a str,
    seed: u64,
    advances: u32,
    results: [u64; 2],
}

#[derive(Clone, Deserialize)]
struct LCRNG64NextData<'a> {
    name: &'a str,
    seed: u64,
    results: [u64; 2],
}

const JSON_STR: &str = include_str!("lcrng64.json");

#[test]
fn advance() {
    let data = get_test_data::<'static, LCRNG64TestData>(JSON_STR);
    for LCRNG64AdvanceData {
        name,
        seed,
        advances,
        results,
    } in data.advance
    {
        assert_eq!(
            BWRNG::new(seed).advance(advances),
            results[0],
            "BWRNG: {}",
            name
        );
        assert_eq!(
            BWRNGR::new(seed).advance(advances),
            results[1],
            "BWRNGR: {}",
            name
        );
    }
}

#[test]
fn jump() {
    let data = get_test_data::<'static, LCRNG64TestData>(JSON_STR);
    for LCRNG64JumpData {
        name,
        seed,
        advances,
        results,
    } in data.jump
    {
        assert_eq!(
            BWRNG::new(seed).jump(advances),
            results[0],
            "BWRNG: {}",
            name
        );
        assert_eq!(
            BWRNGR::new(seed).jump(advances),
            results[1],
            "BWRNGR: {}",
            name
        );
    }
}

#[test]
fn next() {
    let data = get_test_data::<'static, LCRNG64TestData>(JSON_STR);
    for LCRNG64NextData {
        name,
        seed,
        results,
    } in data.next
    {
        assert_eq!(BWRNG::new(seed).next(), results[0], "BWRNG: {}", name);
        assert_eq!(BWRNGR::new(seed).next(), results[1], "BWRNGR: {}", name);
    }
}
