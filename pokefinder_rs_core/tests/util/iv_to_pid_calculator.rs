use crate::{assert_states, get_test_data};
use pokefinder_rs_core::parents::states::IVtoPIDState;
use pokefinder_rs_core::util::iv_to_pid_calculator;
use serde::Deserialize;

#[derive(Deserialize)]
struct IVToPIDCalculatorData<'a> {
    #[serde(rename = "calculatePIDs", borrow)]
    calculate_pids: Vec<CalculatePIDData<'a>>,
}

#[derive(Deserialize)]
struct CalculatePIDData<'a> {
    name: &'a str,
    ivs: [u8; 6],
    nature: u8,
    tid: u16,
    results: Vec<CalculatePIDResult>,
}

#[derive(Deserialize, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct CalculatePIDResult {
    method: u8,
    pid: u32,
    seed: u32,
    sid: u16,
}

impl From<IVtoPIDState> for CalculatePIDResult {
    fn from(value: IVtoPIDState) -> Self {
        Self {
            method: value.get_method() as u8,
            pid: value.get_pid(),
            seed: value.get_seed(),
            sid: value.get_sid(),
        }
    }
}

const JSON_DATA: &str = include_str!("ivtopidcalculator.json");

#[test]
fn calculate_pids() {
    let data = get_test_data::<'static, IVToPIDCalculatorData>(JSON_DATA);

    for (
        num,
        CalculatePIDData {
            name,
            ivs,
            nature,
            tid,
            results,
        },
    ) in data.calculate_pids.into_iter().enumerate()
    {
        let states = iv_to_pid_calculator::calculate_pids(
            ivs[0], ivs[1], ivs[2], ivs[3], ivs[4], ivs[5], nature, tid,
        );

        assert_states(results, states, name, num);
    }
}
