use crate::{assert_states, get_test_data};
use pokefinder_rs_core::gen4::{tools::seed_to_time_calculator4, SeedTime4};
use serde::Deserialize;

#[derive(Deserialize)]
struct SeedToTime4TestData<'a> {
    #[serde(rename = "calculateTimes", borrow)]
    calculate_times: Vec<CalculateTimeData<'a>>,
}

#[derive(Deserialize)]
struct CalculateTimeData<'a> {
    name: &'a str,
    seed: u32,
    year: u16,
    results: Vec<CalculateTimeResult>,
}

#[derive(Deserialize, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct CalculateTimeResult {
    day: u8,
    delay: u32,
    hour: u8,
    minute: u8,
    month: u8,
    second: u8,
    year: u16,
}

impl From<SeedTime4> for CalculateTimeResult {
    fn from(value: SeedTime4) -> Self {
        Self {
            day: value.date_time.date.day(),
            delay: value.delay,
            hour: value.date_time.time.hour(),
            minute: value.date_time.time.minute(),
            month: value.date_time.date.month(),
            second: value.date_time.time.second(),
            year: value.date_time.date.year(),
        }
    }
}

const JSON_DATA: &str = include_str!("seedtotime4.json");

#[test]
fn calculate_times() {
    let data = get_test_data::<'static, SeedToTime4TestData>(JSON_DATA);

    for (
        num,
        CalculateTimeData {
            name,
            seed,
            year,
            results,
        },
    ) in data.calculate_times.into_iter().enumerate()
    {
        let states = seed_to_time_calculator4::calculate_times(seed, year, true, 0);

        assert_states(results, states, name, num);
    }
}
