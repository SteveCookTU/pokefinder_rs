use crate::gen4::{SeedTime4, SeedTimeCalibrate4};
use crate::util::{Date, DateTime};

/// Generates date/times for a seed
///
/// Can filter by a specific second with `force_second` and`forced_second`
pub fn calculate_times(
    seed: u32,
    year: u16,
    force_second: bool,
    forced_second: u8,
) -> Vec<SeedTime4> {
    let ab = (seed >> 24) as u8;
    let cd = (seed >> 16) & 0xFF;
    let efgh = seed & 0xFFFF;

    let hour = cd.min(23) as u8;
    let delay = if cd > 23 {
        efgh.wrapping_add(2000u32.wrapping_sub(year as u32))
            .wrapping_add(cd.wrapping_sub(23).wrapping_mul(0x10000))
    } else {
        efgh.wrapping_add(2000u32.wrapping_sub(year as u32))
    };

    let mut results = vec![];

    for month in 1..=12 {
        let max_days = Date::days_in_month(year, month);
        for day in 1..=max_days {
            for minute in 0..60 {
                for second in 0..60 {
                    if (ab as u16)
                        == ((month as u16)
                            .wrapping_mul(day as u16)
                            .wrapping_add(minute as u16)
                            .wrapping_add(second as u16) & 0xFF)
                        && (!force_second || second == forced_second)
                    {
                        results.push(SeedTime4::new(
                            DateTime::new_ymd_hms(year, month, day, hour, minute, second),
                            delay,
                        ));
                    }
                }
            }
        }
    }

    results
}

/// Generates calibration results within `minus_delay`, `plus_delay`, `minus_second`, and `plus_second`
/// from the selected `target`
pub fn calibrate(
    minus_delay: i32,
    plus_delay: i32,
    minus_second: i32,
    plus_second: i32,
    target: &SeedTime4,
) -> Vec<SeedTimeCalibrate4> {
    let time = &target.date_time;
    let delay = target.delay;

    let mut results = Vec::with_capacity(
        ((plus_delay - minus_delay + 1) * (plus_second - minus_second + 1)) as usize,
    );

    for second_offset in minus_second..=plus_second {
        let offset = time.add_secs(second_offset as u32);
        for delay_offset in minus_delay..=plus_delay {
            results.push(SeedTimeCalibrate4::new(
                offset,
                delay.wrapping_add(delay_offset as u32),
            ));
        }
    }

    results
}

/// Generates calibration results for roamers within `minus_delay`, `plus_delay`, `minus_second`,
/// and `plus_second` from the selected `target`
pub fn calibrate_roamers(
    minus_delay: i32,
    plus_delay: i32,
    minus_second: i32,
    plus_second: i32,
    roamers: [bool; 3],
    routes: [u8; 3],
    target: &SeedTime4,
) -> Vec<SeedTimeCalibrate4> {
    let time = &target.date_time;
    let delay = target.delay;

    let mut results = Vec::with_capacity(
        ((plus_delay - minus_delay + 1) * (plus_second - minus_second + 1)) as usize,
    );

    for second_offset in minus_second..=plus_second {
        let offset = time.add_secs(second_offset as u32);
        for delay_offset in minus_delay..=plus_delay {
            results.push(SeedTimeCalibrate4::new_with_roamer(
                offset,
                delay.wrapping_add(delay_offset as u32),
                roamers,
                routes,
            ));
        }
    }

    results
}
