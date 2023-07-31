use crate::get_test_data;
use pokefinder_rs_core::rng::{Rng, MT};
use serde::Deserialize;

#[derive(Deserialize)]
struct MTTestData<'a> {
    #[serde(borrow)]
    advance: [MTAdvanceData<'a>; 4],
    #[serde(borrow)]
    next: [MTNextData<'a>; 4],
}

#[derive(Deserialize)]
struct MTAdvanceData<'a> {
    name: &'a str,
    seed: u32,
    advances: u32,
    result: u32,
}

#[derive(Deserialize)]
struct MTNextData<'a> {
    name: &'a str,
    seed: u32,
    result: u32,
}

const JSON_STR: &str = include_str!("mt.json");

#[test]
fn advance() {
    let data = get_test_data::<'static, MTTestData>(JSON_STR);
    for MTAdvanceData {
        name,
        seed,
        advances,
        result,
    } in data.advance
    {
        let mut rng = MT::new(seed);
        rng.advance(advances - 1);
        assert_eq!(rng.next(), result, "MT: {}", name);
    }
}

#[test]
fn next() {
    let data = get_test_data::<'static, MTTestData>(JSON_STR);
    for MTNextData { name, seed, result } in data.next {
        let mut rng = MT::new(seed);
        assert_eq!(rng.next(), result, "MT: {}", name);
    }
}
