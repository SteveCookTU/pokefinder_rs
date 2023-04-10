use serde::Deserialize;
use std::fmt::Debug;

mod gen3;
mod rng;
mod util;

pub fn get_test_data<'a, T: Deserialize<'a>>(json: &'static str) -> T {
    serde_json::from_str(json).expect("Failed to parse test data")
}

pub fn assert_states<From: Into<To>, To: PartialEq + Ord + PartialOrd + Eq + Debug>(
    results: Vec<To>,
    states: Vec<From>,
    name: &str,
    num: usize,
) {
    assert_eq!(
        states.len(),
        results.len(),
        "Unequal results size: {} - {}",
        name,
        num
    );

    for (i, state) in states.into_iter().enumerate() {
        let result: To = state.into();
        assert_eq!(
            result, results[i],
            "Unequal state: {} - {} - {}",
            name, num, i
        );
    }
}
