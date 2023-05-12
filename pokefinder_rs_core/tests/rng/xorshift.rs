use crate::get_test_data;
use pokefinder_rs_core::rng::{Rng, Xorshift};
use serde::Deserialize;

#[derive(Deserialize)]
struct XorshiftData<'a> {
    #[serde(borrow)]
    advance: Vec<XorshiftAdvanceData<'a>>,
    #[serde(borrow)]
    jump: Vec<XorshiftJumpData<'a>>,
    #[serde(borrow)]
    next: Vec<XorshiftNextData<'a>>,
}

#[derive(Deserialize)]
struct XorshiftAdvanceData<'a> {
    name: &'a str,
    seed: u64,
    advances: u32,
    result: u32,
}

#[derive(Deserialize)]
struct XorshiftJumpData<'a> {
    name: &'a str,
    seed: u64,
    advances: u32,
    result: u32,
}

#[derive(Deserialize)]
struct XorshiftNextData<'a> {
    name: &'a str,
    seed: u64,
    result: u32,
}

const JSON_DATA: &str = include_str!("xorshift.json");

#[test]
fn next() {
    let data = get_test_data::<'static, XorshiftData>(JSON_DATA);
    for (num, XorshiftNextData { name, seed, result }) in data.next.into_iter().enumerate() {
        let mut rng = Xorshift::new(seed, seed ^ (seed >> 32));
        assert_eq!(rng.next(), result, "Unequal result: {} - {}", num, name);
    }
}

#[test]
fn advance() {
    let data = get_test_data::<'static, XorshiftData>(JSON_DATA);
    for (
        num,
        XorshiftAdvanceData {
            name,
            seed,
            advances,
            result,
        },
    ) in data.advance.into_iter().enumerate()
    {
        let mut rng = Xorshift::new(seed, seed ^ (seed >> 32));
        rng.advance(advances - 1);
        assert_eq!(rng.next(), result, "Unequal result: {} - {}", num, name);
    }
}

#[test]
fn jump() {
    let data = get_test_data::<'static, XorshiftData>(JSON_DATA);
    for (
        num,
        XorshiftJumpData {
            name,
            seed,
            advances,
            result,
        },
    ) in data.jump.into_iter().enumerate()
    {
        let mut rng = Xorshift::new(seed, seed ^ (seed >> 32));
        rng.jump(advances - 1);
        assert_eq!(rng.next(), result, "Unequal result: {} - {}", num, name);
    }
}
