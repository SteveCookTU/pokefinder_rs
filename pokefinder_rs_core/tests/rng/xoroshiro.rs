use crate::get_test_data;
use pokefinder_rs_core::rng::{Rng, Xoroshiro, XoroshiroBDSP};
use serde::Deserialize;

#[derive(Deserialize)]
struct XoroshiroData<'a> {
    #[serde(borrow)]
    normal: XoroshiroNormalData<'a>,
    #[serde(borrow)]
    bdsp: XoroshiroBDSPData<'a>,
}

#[derive(Deserialize)]
struct XoroshiroNormalData<'a> {
    #[serde(borrow)]
    advance: Vec<AdvanceData<'a>>,
    #[serde(borrow)]
    jump: Vec<JumpData<'a>>,
    #[serde(borrow)]
    next: Vec<NextData<'a>>,
}

#[derive(Deserialize)]
struct XoroshiroBDSPData<'a> {
    #[serde(borrow)]
    advance: Vec<AdvanceData<'a>>,
    #[serde(borrow)]
    jump: Vec<JumpData<'a>>,
    #[serde(borrow)]
    next: Vec<NextData<'a>>,
}

#[derive(Deserialize)]
struct AdvanceData<'a> {
    name: &'a str,
    seed: u64,
    advances: u32,
    result: u64,
}

#[derive(Deserialize)]
struct JumpData<'a> {
    name: &'a str,
    seed: u64,
    advances: u32,
    result: u64,
}

#[derive(Deserialize)]
struct NextData<'a> {
    name: &'a str,
    seed: u64,
    result: u64,
}

const JSON_DATA: &str = include_str!("xoroshiro.json");

#[test]
fn normal_advance() {
    let data = get_test_data::<'static, XoroshiroData>(JSON_DATA);
    for (
        num,
        AdvanceData {
            name,
            seed,
            advances,
            result,
        },
    ) in data.normal.advance.into_iter().enumerate()
    {
        let mut rng: Xoroshiro = seed.into();
        rng.advance(advances - 1);
        assert_eq!(rng.next(), result, "Unequal result: {} - {}", num, name);
    }
}

#[test]
fn normal_jump() {
    let data = get_test_data::<'static, XoroshiroData>(JSON_DATA);
    for (
        num,
        JumpData {
            name,
            seed,
            advances,
            result,
        },
    ) in data.normal.jump.into_iter().enumerate()
    {
        let mut rng: Xoroshiro = seed.into();
        rng.jump(advances - 1);
        assert_eq!(rng.next(), result, "Unequal result: {} - {}", num, name);
    }
}

#[test]
fn normal_next() {
    let data = get_test_data::<'static, XoroshiroData>(JSON_DATA);
    for (num, NextData { name, seed, result }) in data.normal.next.into_iter().enumerate() {
        let mut rng: Xoroshiro = seed.into();
        assert_eq!(rng.next(), result, "Unequal result: {} - {}", num, name);
    }
}

#[test]
fn bdsp_advance() {
    let data = get_test_data::<'static, XoroshiroData>(JSON_DATA);
    for (
        num,
        AdvanceData {
            name,
            seed,
            advances,
            result,
        },
    ) in data.bdsp.advance.into_iter().enumerate()
    {
        let mut rng: XoroshiroBDSP = seed.into();
        rng.advance(advances - 1);
        assert_eq!(rng.next(), result, "Unequal result: {} - {}", num, name);
    }
}

#[test]
fn bdsp_jump() {
    let data = get_test_data::<'static, XoroshiroData>(JSON_DATA);
    for (
        num,
        JumpData {
            name,
            seed,
            advances,
            result,
        },
    ) in data.bdsp.jump.into_iter().enumerate()
    {
        let mut rng: XoroshiroBDSP = seed.into();
        rng.jump(advances - 1);
        assert_eq!(rng.next(), result, "Unequal result: {} - {}", num, name);
    }
}

#[test]
fn bdsp_next() {
    let data = get_test_data::<'static, XoroshiroData>(JSON_DATA);
    for (num, NextData { name, seed, result }) in data.bdsp.next.into_iter().enumerate() {
        let mut rng: XoroshiroBDSP = seed.into();
        assert_eq!(rng.next(), result, "Unequal result: {} - {}", num, name);
    }
}
