use crate::{assert_states, get_test_data};
use pokefinder_rs_core::gen8::generators::IDGenerator8;
use pokefinder_rs_core::gen8::states::IDState8;
use pokefinder_rs_core::parents::filters::IDFilter;
use serde::Deserialize;

#[derive(Deserialize)]
struct IDGenerator8Data<'a> {
    #[serde(borrow)]
    generate: Vec<GenerateData<'a>>,
}

#[derive(Deserialize)]
struct GenerateData<'a> {
    name: &'a str,
    seed0: u64,
    seed1: u64,
    results: Vec<GenerateResult>,
}

#[derive(Deserialize, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct GenerateResult {
    advances: u32,
    #[serde(rename = "displayTID")]
    display_tid: u32,
    sid: u16,
    tid: u16,
    tsv: u16,
}

impl From<IDState8> for GenerateResult {
    fn from(value: IDState8) -> Self {
        Self {
            advances: value.advances,
            display_tid: value.display_tid,
            sid: value.sid,
            tid: value.tid,
            tsv: value.tsv,
        }
    }
}

const JSON_DATA: &str = include_str!("id8.json");

#[test]
fn generate() {
    let data = get_test_data::<'static, IDGenerator8Data>(JSON_DATA);
    for (
        num,
        GenerateData {
            name,
            seed0,
            seed1,
            results,
        },
    ) in data.generate.into_iter().enumerate()
    {
        let filter = IDFilter::new(vec![], vec![], vec![], vec![]);
        let generator = IDGenerator8::new(0, 9, &filter);

        let states = generator.generate(seed0, seed1);

        assert_states(results, states, name, num);
    }
}
