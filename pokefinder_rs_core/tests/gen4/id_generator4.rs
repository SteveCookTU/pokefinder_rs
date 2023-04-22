use crate::gen4::ID_DATA;
use crate::{assert_states, get_test_data};
use pokefinder_rs_core::gen4::generators::IDGenerator4;
use pokefinder_rs_core::gen4::states::IDState4;
use pokefinder_rs_core::parents::filters::IDFilter;
use serde::Deserialize;

#[derive(Deserialize)]
struct ID4Data<'a> {
    #[serde(borrow)]
    idgenerator4: IDGenerator4Data<'a>,
}

#[derive(Deserialize)]
struct IDGenerator4Data<'a> {
    #[serde(borrow)]
    generate: Vec<GeneratorData<'a>>,
}

#[derive(Deserialize)]
struct GeneratorData<'a> {
    name: &'a str,
    tid: u16,
    #[serde(rename = "maxDelay")]
    max_delay: u32,
    #[serde(rename = "minDelay")]
    min_delay: u32,
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    results: Vec<GeneratorResult>,
}

#[derive(Deserialize, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct GeneratorResult {
    delay: u32,
    seconds: u8,
    seed: u32,
    sid: u16,
    tid: u16,
    tsv: u16,
}

impl From<IDState4> for GeneratorResult {
    fn from(value: IDState4) -> Self {
        Self {
            delay: value.delay,
            seconds: value.seconds,
            seed: value.seed,
            sid: value.base.sid,
            tid: value.base.tid,
            tsv: value.base.tsv,
        }
    }
}

#[test]
fn generate() {
    let data = get_test_data::<'static, ID4Data>(ID_DATA);

    for (
        num,
        GeneratorData {
            name,
            tid,
            max_delay,
            min_delay,
            year,
            month,
            day,
            hour,
            minute,
            results,
        },
    ) in data.idgenerator4.generate.into_iter().enumerate()
    {
        let filter = IDFilter::new(vec![tid], vec![], vec![], vec![]);
        let generator = IDGenerator4::new(
            min_delay, max_delay, year, month, day, hour, minute, &filter,
        );

        let states = generator.generate();

        assert_states(results, states, name, num);
    }
}
