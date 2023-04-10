use crate::get_test_data;
use pokefinder_rs_core::util::{Date, DateParts, DateTime, Time};
use serde::Deserialize;

#[derive(Deserialize)]
struct DateTimeUtilData<'a> {
    #[serde(borrow)]
    datetime: DateTimeData<'a>,
    #[serde(borrow)]
    date: DateData<'a>,
    #[serde(borrow)]
    time: TimeData<'a>,
}

#[derive(Deserialize)]
struct DateTimeData<'a> {
    #[serde(rename = "addSecs", borrow)]
    add_secs: Vec<DateTimeAddSecsData<'a>>,
}

#[derive(Deserialize)]
struct DateTimeAddSecsData<'a> {
    name: &'a str,
    seconds: u32,
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
}

#[derive(Deserialize)]
struct DateData<'a> {
    #[serde(rename = "getParts", borrow)]
    get_parts: Vec<DateGetPartsData<'a>>,
    #[serde(rename = "dayOfWeek", borrow)]
    day_of_week: Vec<DateDayOfWeekData<'a>>,
}

#[derive(Deserialize)]
struct DateGetPartsData<'a> {
    name: &'a str,
    jd: u32,
    results: DateGetPartsResult,
}

#[derive(Deserialize)]
struct DateGetPartsResult {
    year: u16,
    month: u8,
    day: u8,
}

#[derive(Deserialize)]
struct DateDayOfWeekData<'a> {
    name: &'a str,
    year: u16,
    month: u8,
    day: u8,
    result: u8,
}

#[derive(Deserialize)]
struct TimeData<'a> {
    #[serde(rename = "addSeconds", borrow)]
    add_seconds: Vec<TimeAddSecondsData<'a>>,
}

#[derive(Deserialize)]
struct TimeAddSecondsData<'a> {
    name: &'a str,
    seconds: u32,
    #[serde(rename = "expectedDays")]
    expected_days: u32,
    hour: u8,
    minute: u8,
    second: u8,
}

const JSON_DATA: &str = include_str!("datetime.json");

#[test]
fn add_secs() {
    let data = get_test_data::<'static, DateTimeUtilData>(JSON_DATA);

    for (num, data) in data.datetime.add_secs.into_iter().enumerate() {
        let DateTimeAddSecsData {
            name,
            seconds,
            year,
            month,
            day,
            hour,
            minute,
            second,
        } = data;

        let result = DateTime::new_ymd_hms(year, month, day, hour, minute, second);

        let dt = DateTime::default().add_secs(seconds);

        assert_eq!(dt, result, "Unequal result: {} - {}", name, num);
    }
}

#[test]
fn get_parts() {
    let data = get_test_data::<'static, DateTimeUtilData>(JSON_DATA);

    for (num, data) in data.date.get_parts.into_iter().enumerate() {
        let DateGetPartsData { name, jd, results } = data;

        let result = DateParts {
            year: results.year,
            month: results.month,
            day: results.day,
        };

        let date = Date::new(jd);

        assert_eq!(
            date.get_parts(),
            result,
            "Unequal result: {} - {}",
            name,
            num
        );
    }
}

#[test]
fn day_of_week() {
    let data = get_test_data::<'static, DateTimeUtilData>(JSON_DATA);

    for (num, data) in data.date.day_of_week.into_iter().enumerate() {
        let DateDayOfWeekData {
            name,
            year,
            month,
            day,
            result,
        } = data;

        let date = Date::new_ymd(year, month, day);

        assert_eq!(
            date.day_of_week(),
            result,
            "Unequal result: {} - {}",
            name,
            num
        );
    }
}

#[test]
fn add_seconds() {
    let data = get_test_data::<'static, DateTimeUtilData>(JSON_DATA);

    for (num, data) in data.time.add_seconds.into_iter().enumerate() {
        let TimeAddSecondsData {
            name,
            seconds,
            expected_days,
            hour,
            minute,
            second,
        } = data;

        let result = Time::new_hms(hour, minute, second);
        let mut t = Time::default();
        let days = t.add_seconds(seconds);

        assert_eq!(
            days, expected_days,
            "Unequal expected days: {} - {}",
            name, num
        );
        assert_eq!(t, result, "Unequal result: {} - {}", name, num);
    }
}
