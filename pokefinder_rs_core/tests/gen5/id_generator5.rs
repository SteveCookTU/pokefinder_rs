use crate::assert_states;
use pokefinder_rs_core::enums::{DSType, Game, Language};
use pokefinder_rs_core::gen5::generators::IDGenerator5;
use pokefinder_rs_core::gen5::Profile5;
use pokefinder_rs_core::parents::filters::IDFilter;
use pokefinder_rs_core::parents::states::IDState;
use serde::Deserialize;

#[derive(Deserialize)]
struct ID5Data<'a> {
    #[serde(borrow)]
    generate: Vec<ID5GeneratorData<'a>>,
}

#[derive(Deserialize)]
struct ID5GeneratorData<'a> {
    name: &'a str,
    seed: u64,
    version: u32,
    results: Vec<IDGeneratorResult>,
}

#[derive(Deserialize, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct IDGeneratorResult {
    advances: u32,
    sid: u16,
    tid: u16,
    tsv: u16,
}

impl From<IDState> for IDGeneratorResult {
    fn from(value: IDState) -> Self {
        Self {
            advances: value.advances,
            sid: value.sid,
            tid: value.tid,
            tsv: value.tsv,
        }
    }
}

const JSON_DATA: &str = include_str!("id5.json");

#[test]
fn generate() {
    let data = serde_json::from_str::<ID5Data>(JSON_DATA).unwrap();

    for (
        num,
        ID5GeneratorData {
            name,
            seed,
            version,
            results,
        },
    ) in data.generate.into_iter().enumerate()
    {
        let profile = Profile5::new(
            "-".to_string(),
            Game::from_bits_retain(version),
            12345,
            54321,
            0,
            [false, false, false, false],
            0,
            0,
            0,
            false,
            0,
            0,
            false,
            false,
            false,
            DSType::DS,
            Language::English,
        );

        let filter = IDFilter::new(vec![], vec![], vec![], vec![]);
        let generator = IDGenerator5::new(0, 9, &profile, &filter);

        let states = generator.generate(seed, 0, false, false);

        assert_states(results, states, name, num);
    }
}
