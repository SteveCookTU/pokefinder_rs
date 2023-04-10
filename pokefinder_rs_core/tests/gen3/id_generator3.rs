use crate::{assert_states, get_test_data};
use pokefinder_rs_core::gen3::generators::IDGenerator3;
use pokefinder_rs_core::parents::filters::IDFilter;
use pokefinder_rs_core::parents::states::{IDState, IDStateT};
use serde::Deserialize;

#[derive(Deserialize)]
struct IDGenerator3Data<'a> {
    #[serde(borrow)]
    xdcolo: Vec<IDGenerator3XDColoData<'a>>,
    #[serde(borrow)]
    frlge: Vec<IDGenerator3FRLGEData<'a>>,
    #[serde(borrow)]
    rs: Vec<IDGenerator3RSData<'a>>,
}

#[derive(Deserialize)]
struct IDGenerator3XDColoData<'a> {
    name: &'a str,
    seed: u32,
    results: Vec<IDGenerator3Result>,
}

#[derive(Deserialize)]
struct IDGenerator3FRLGEData<'a> {
    name: &'a str,
    tid: u16,
    results: Vec<IDGenerator3Result>,
}

#[derive(Deserialize)]
struct IDGenerator3RSData<'a> {
    name: &'a str,
    seed: u16,
    results: Vec<IDGenerator3Result>,
}

#[derive(Deserialize, PartialEq, Ord, PartialOrd, Eq, Debug)]
struct IDGenerator3Result {
    advances: u32,
    tid: u16,
    sid: u16,
    tsv: u16,
}

impl From<IDState> for IDGenerator3Result {
    fn from(value: IDState) -> Self {
        Self {
            advances: value.get_advances(),
            tid: value.get_tid(),
            sid: value.get_sid(),
            tsv: value.get_tsv(),
        }
    }
}

const JSON_DATA: &str = include_str!("id3.json");

#[test]
fn generate_xdcolo() {
    let data = get_test_data::<'static, IDGenerator3Data>(JSON_DATA);

    for (num, generate_data) in data.xdcolo.into_iter().enumerate() {
        let IDGenerator3XDColoData {
            name,
            seed,
            results,
        } = generate_data;

        let filter = IDFilter::new(vec![], vec![], vec![], vec![]);
        let generator = IDGenerator3::new(0, 9, &filter);

        let states = generator.generate_xd_colo(seed);

        assert_states(results, states, name, num);
    }
}

#[test]
fn generate_frlge() {
    let data = get_test_data::<'static, IDGenerator3Data>(JSON_DATA);

    for (num, generate_data) in data.frlge.into_iter().enumerate() {
        let IDGenerator3FRLGEData { name, tid, results } = generate_data;

        let filter = IDFilter::new(vec![], vec![], vec![], vec![]);
        let generator = IDGenerator3::new(0, 9, &filter);

        let states = generator.generate_frlge(tid);

        assert_states(results, states, name, num);
    }
}

#[test]
fn generate_rs() {
    let data = get_test_data::<'static, IDGenerator3Data>(JSON_DATA);

    for (num, generate_data) in data.rs.into_iter().enumerate() {
        let IDGenerator3RSData {
            name,
            seed,
            results,
        } = generate_data;

        let filter = IDFilter::new(vec![], vec![], vec![], vec![]);
        let generator = IDGenerator3::new(0, 9, &filter);

        let states = generator.generate_rs(seed);

        assert_states(results, states, name, num);
    }
}
