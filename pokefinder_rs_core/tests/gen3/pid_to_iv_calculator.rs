use crate::{assert_states, get_test_data};
use pokefinder_rs_core::gen3::states::PIDToIVState;
use pokefinder_rs_core::gen3::tools::pid_to_iv_calculator;
use serde::Deserialize;

#[derive(Deserialize)]
struct PIDToIVCalculatorData<'a> {
    #[serde(rename = "calculateIVs", borrow)]
    calculate_ivs: Vec<CalculateIVsData<'a>>,
}

#[derive(Deserialize)]
struct CalculateIVsData<'a> {
    name: &'a str,
    pid: u32,
    results: Vec<CalculateIVsResult>,
}

#[derive(Deserialize, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct CalculateIVsResult {
    ivs: [u8; 6],
    method: u8,
    seed: u32,
}

impl From<PIDToIVState> for CalculateIVsResult {
    fn from(value: PIDToIVState) -> Self {
        Self {
            ivs: value.ivs,
            method: value.method.into(),
            seed: value.seed,
        }
    }
}

const JSON_DATA: &str = include_str!("pidtoivcalculator.json");

#[test]
fn calculate_ivs() {
    let data = get_test_data::<'static, PIDToIVCalculatorData>(JSON_DATA);

    for (num, data) in data.calculate_ivs.into_iter().enumerate() {
        let CalculateIVsData { name, pid, results } = data;

        let states = pid_to_iv_calculator::calculate_ivs(pid);

        assert_states(results, states, name, num);
    }
}
