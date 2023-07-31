use crate::get_test_data;
use pokefinder_rs_core::enums::Game;
use pokefinder_rs_core::parents::personal_loader;
use pokefinder_rs_core::util::iv_checker;
use serde::Deserialize;

#[derive(Deserialize)]
struct IVCheckerData<'a> {
    #[serde(rename = "calculateIVRange", borrow)]
    calculate_iv_range: Vec<CalculateIVRangeData<'a>>,
}

#[derive(Deserialize)]
struct CalculateIVRangeData<'a> {
    name: &'a str,
    specie: u16,
    form: u8,
    version: u32,
    stats: Vec<[u16; 6]>,
    levels: Vec<u8>,
    nature: u8,
    characteristic: u8,
    #[serde(rename = "hiddenPower")]
    hidden_power: u8,
    results: [Vec<u8>; 6],
}

const JSON_DATA: &str = include_str!("ivchecker.json");

#[test]
fn calculate_iv_range() {
    let data = get_test_data::<'static, IVCheckerData>(JSON_DATA);

    for CalculateIVRangeData {
        name,
        specie,
        form,
        version,
        stats,
        levels,
        nature,
        characteristic,
        hidden_power,
        results,
    } in data.calculate_iv_range
    {
        let info =
            personal_loader::get_personal_info(Game::from_bits_retain(version), specie, form);
        let range = iv_checker::calculate_iv_range(
            &info.get_stats(),
            &stats,
            &levels,
            nature,
            characteristic,
            hidden_power,
        );
        assert_eq!(range, results, "Unequal result: {}", name)
    }
}
