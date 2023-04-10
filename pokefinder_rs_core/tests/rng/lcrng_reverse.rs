use crate::get_test_data;
use pokefinder_rs_core::enums::Method;
use pokefinder_rs_core::rng::lcrng_reverse;
use serde::Deserialize;

#[derive(Deserialize)]
struct LCRNGReverseData<'a> {
    #[serde(rename = "recoverChannelIV", borrow)]
    recover_channel_iv: Vec<LCRNGReverseChannelIVData<'a>>,
    #[serde(rename = "recoverPokeRNGIV", borrow)]
    recover_poke_rng_iv: Vec<LCRNGReversePokeRNGIVData<'a>>,
    #[serde(rename = "recoverPokeRNGPID_data", borrow)]
    recover_poke_rng_pid: Vec<LCRNGReversePokeRNGPIDData<'a>>,
    #[serde(rename = "recoverXDRNGIV", borrow)]
    recover_xd_rng_iv: Vec<LCRNGReverseXDRNGIVData<'a>>,
    #[serde(rename = "recoverXDRNGPID", borrow)]
    recover_xd_rng_pid: Vec<LCRNGReverseXDRNGPIDData<'a>>,
}

#[derive(Deserialize)]
struct LCRNGReverseChannelIVData<'a> {
    name: &'a str,
    ivs: [u8; 6],
    results: Vec<u32>,
}

#[derive(Deserialize)]
struct LCRNGReversePokeRNGIVData<'a> {
    name: &'a str,
    method: u8,
    ivs: [u8; 6],
    results: Vec<u32>,
}

#[derive(Deserialize)]
struct LCRNGReversePokeRNGPIDData<'a> {
    name: &'a str,
    #[serde(rename = "method")]
    _method: u8,
    pid: u32,
    results: Vec<u32>,
}

#[derive(Deserialize)]
struct LCRNGReverseXDRNGIVData<'a> {
    name: &'a str,
    ivs: [u8; 6],
    results: Vec<u32>,
}

#[derive(Deserialize)]
struct LCRNGReverseXDRNGPIDData<'a> {
    name: &'a str,
    pid: u32,
    results: Vec<u32>,
}

const JSON_DATA: &str = include_str!("lcrngreverse.json");

#[test]
fn recover_channel_iv() {
    let data = get_test_data::<'static, LCRNGReverseData>(JSON_DATA);

    for (num, data) in data.recover_channel_iv.into_iter().enumerate() {
        let LCRNGReverseChannelIVData { name, ivs, results } = data;

        let mut seeds = [0; 12];
        let size = lcrng_reverse::recover_channel_iv(
            ivs[0], ivs[1], ivs[2], ivs[3], ivs[4], ivs[5], &mut seeds,
        );

        assert_eq!(
            size,
            results.len(),
            "Non-matching result length: {} - {}",
            name,
            num
        );

        for (seed, result) in seeds.into_iter().zip(results.into_iter()) {
            assert_eq!(seed, result, "Non-matching result: {} - {}", name, num);
        }
    }
}

#[test]
fn recover_poke_rng_iv() {
    let data = get_test_data::<'static, LCRNGReverseData>(JSON_DATA);

    for (num, data) in data.recover_poke_rng_iv.into_iter().enumerate() {
        let LCRNGReversePokeRNGIVData {
            name,
            method,
            ivs,
            results,
        } = data;

        let mut seeds = [0; 6];
        let size = lcrng_reverse::recover_poke_rng_iv(
            ivs[0],
            ivs[1],
            ivs[2],
            ivs[3],
            ivs[4],
            ivs[5],
            &mut seeds,
            Method::from(method),
        );

        assert_eq!(
            size,
            results.len(),
            "Non-matching result length: {} - {}",
            name,
            num
        );

        for (seed, result) in seeds.into_iter().zip(results.into_iter()) {
            assert_eq!(seed, result, "Non-matching result: {} - {}", name, num);
        }
    }
}

#[test]
fn recover_poke_rng_pid() {
    let data = get_test_data::<'static, LCRNGReverseData>(JSON_DATA);

    for (num, data) in data.recover_poke_rng_pid.into_iter().enumerate() {
        let LCRNGReversePokeRNGPIDData {
            name, pid, results, ..
        } = data;

        let mut seeds = [0; 3];
        let size = lcrng_reverse::recover_poke_rng_pid(pid, &mut seeds);

        assert_eq!(
            size,
            results.len(),
            "Non-matching result length: {} - {}",
            name,
            num
        );

        for (seed, result) in seeds.into_iter().zip(results.into_iter()) {
            assert_eq!(seed, result, "Non-matching result: {} - {}", name, num);
        }
    }
}

#[test]
fn recover_xdrng_iv() {
    let data = get_test_data::<'static, LCRNGReverseData>(JSON_DATA);

    for (num, data) in data.recover_xd_rng_iv.into_iter().enumerate() {
        let LCRNGReverseXDRNGIVData { name, ivs, results } = data;

        let mut seeds = [0; 6];
        let size = lcrng_reverse::recover_xdrng_iv(
            ivs[0], ivs[1], ivs[2], ivs[3], ivs[4], ivs[5], &mut seeds,
        );

        assert_eq!(
            size,
            results.len(),
            "Non-matching result length: {} - {}",
            name,
            num
        );

        for (seed, result) in seeds.into_iter().zip(results.into_iter()) {
            assert_eq!(seed, result, "Non-matching result: {} - {}", name, num);
        }
    }
}

#[test]
fn recover_xdrng_pid() {
    let data = get_test_data::<'static, LCRNGReverseData>(JSON_DATA);

    for (num, data) in data.recover_xd_rng_pid.into_iter().enumerate() {
        let LCRNGReverseXDRNGPIDData { name, pid, results } = data;

        let mut seeds = [0; 3];
        let size = lcrng_reverse::recover_xdrng_pid(pid, &mut seeds);

        assert_eq!(
            size,
            results.len(),
            "Non-matching result length: {} - {}",
            name,
            num
        );

        for (seed, result) in seeds.into_iter().zip(results.into_iter()) {
            assert_eq!(seed, result, "Non-matching result: {} - {}", name, num);
        }
    }
}
