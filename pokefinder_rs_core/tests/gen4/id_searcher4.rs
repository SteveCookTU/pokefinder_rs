use crate::gen4::ID_DATA;
use crate::{assert_states, get_test_data};
use pokefinder_rs_core::gen4::searchers::IDSearcher4;
use pokefinder_rs_core::gen4::states::IDState4;
use pokefinder_rs_core::parents::filters::IDFilter;
use serde::Deserialize;

#[derive(Deserialize)]
struct ID4Data<'a> {
    #[serde(borrow)]
    idsearcher4: IDSearcher4Data<'a>,
}

#[derive(Deserialize)]
struct IDSearcher4Data<'a> {
    #[serde(borrow)]
    search: Vec<SearcherData<'a>>,
}

#[derive(Deserialize)]
struct SearcherData<'a> {
    name: &'a str,
    tid: u16,
    #[serde(rename = "maxDelay")]
    max_delay: u32,
    #[serde(rename = "minDelay")]
    min_delay: u32,
    year: u16,
    results: Vec<SearcherResult>,
}

#[derive(Deserialize, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct SearcherResult {
    delay: u32,
    seed: u32,
    sid: u16,
    tid: u16,
    tsv: u16,
}

impl From<IDState4> for SearcherResult {
    fn from(value: IDState4) -> Self {
        Self {
            delay: value.delay,
            seed: value.seed,
            sid: value.base.sid,
            tid: value.base.tid,
            tsv: value.base.tsv,
        }
    }
}

#[test]
fn search() {
    let data = get_test_data::<'static, ID4Data>(ID_DATA);

    for (
        num,
        SearcherData {
            name,
            tid,
            max_delay,
            min_delay,
            year,
            results,
        },
    ) in data.idsearcher4.search.into_iter().enumerate()
    {
        let filter = IDFilter::new(vec![tid], vec![], vec![], vec![]);
        let searcher = IDSearcher4::new(&filter);

        searcher.start_search(false, year, min_delay, max_delay);

        let states = searcher.get_results();

        assert_states(results, states, name, num);
    }
}
