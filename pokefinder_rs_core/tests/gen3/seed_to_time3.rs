use crate::{assert_states, get_test_data};
use pokefinder_rs_core::gen3::tools::seed_to_time_calculator3;
use pokefinder_rs_core::util::DateTime;
use serde::Deserialize;

#[derive(Deserialize)]
struct SeedToTime3Data<'a> {
    #[serde(rename = "calculateOriginSeed", borrow)]
    calculate_origin_seed: Vec<CalculateOriginSeedData<'a>>,
    #[serde(rename = "calculateTimes", borrow)]
    calculate_times: Vec<CalculateTimesData<'a>>,
}

#[derive(Deserialize)]
struct CalculateOriginSeedData<'a> {
    name: &'a str,
    seed: u32,
    result: CalculateOriginSeedResult,
}

#[derive(Deserialize)]
struct CalculateOriginSeedResult {
    origin: u16,
    advances: u32,
}

#[derive(Deserialize)]
struct CalculateTimesData<'a> {
    name: &'a str,
    seed: u32,
    year: u16,
    results: Vec<CalculateTimesResult>,
}

#[derive(Deserialize, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct CalculateTimesResult {
    day: u8,
    hour: u8,
    minute: u8,
    month: u8,
    year: u16,
}

impl From<DateTime> for CalculateTimesResult {
    fn from(value: DateTime) -> Self {
        Self {
            day: value.date().day(),
            hour: value.time().hour(),
            minute: value.time().minute(),
            month: value.date().month(),
            year: value.date().year(),
        }
    }
}

const JSON_DATA: &str = include_str!("seedtotime3.json");

#[test]
fn calculate_origin_seed() {
    let data = get_test_data::<'static, SeedToTime3Data>(JSON_DATA);

    for (num, data) in data.calculate_origin_seed.into_iter().enumerate() {
        let CalculateOriginSeedData { name, seed, result } = data;

        let mut advances = 0;
        let origin = seed_to_time_calculator3::calculate_origin_seed(seed, &mut advances);

        assert_eq!(result.origin, origin, "Unequal origin: {} - {}", name, num);
        assert_eq!(
            result.advances, advances,
            "Unequal advances: {} - {}",
            name, num
        );
    }
}

#[test]
fn calculate_times() {
    let data = get_test_data::<'static, SeedToTime3Data>(JSON_DATA);

    for (num, data) in data.calculate_times.into_iter().enumerate() {
        let CalculateTimesData {
            name,
            seed,
            year,
            results,
        } = data;

        let states = seed_to_time_calculator3::calculate_times(seed, year);

        assert_states(results, states, name, num);
    }
}
