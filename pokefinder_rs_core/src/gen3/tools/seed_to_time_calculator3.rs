use crate::rng::PokeRNGR;
use crate::util::{Date, DateTime, Time};

/// Calculates the closest 16-bit seed from the given `seed`
pub fn calculate_origin_seed(seed: u32, advances: &mut u32) -> u16 {
    let mut rng = PokeRNGR::new(seed);
    while rng.seed > 0xFFFF {
        rng.next();
        *advances += 1;
    }

    rng.seed as u16
}

/// Calculates date/times that give the `seed` in the given `year`
pub fn calculate_times(seed: u32, year: u16) -> Vec<DateTime> {
    let mut states = vec![];

    let start = Date::default();
    let mut date = Date::new_ymd(year, 1, 1);
    let end = Date::new_ymd(year, 12, 31);

    while date <= end {
        let days = start.days_to(&date) - if year > 2000 { 366 } else { 0 } + 1;

        for hour in 0..24 {
            for minute in 0..60 {
                let mut v = 1440u32
                    .wrapping_mul(days)
                    .wrapping_add(960u32.wrapping_mul(hour / 10))
                    .wrapping_add(60u32.wrapping_mul(hour % 10))
                    .wrapping_add(16u32.wrapping_mul(minute / 10))
                    .wrapping_add(minute % 10);
                v = (v >> 16) ^ (v & 0xFFFF);
                if v == seed {
                    states.push(DateTime::new_from_parts(
                        &date,
                        &Time::new_hms(hour as u8, minute as u8, 0),
                    ));
                }
            }
        }

        date += 1;
    }

    states
}
