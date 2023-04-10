use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct DateParts {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

const NUMBERS: [[char; 2]; 100] = compute_numbers();
const MONTH_DAYS: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

const fn compute_numbers() -> [[char; 2]; 100] {
    let mut strings = [[' '; 2]; 100];

    let mut i = 0;
    while i < strings.len() {
        if i < 10 {
            strings[i][0] = '0';
            strings[i][1] = (b'0' + i as u8) as char;
        } else {
            strings[i][0] = (b'0' + (i / 10) as u8) as char;
            strings[i][1] = (b'0' + (i % 10) as u8) as char;
        }

        i += 1;
    }

    strings
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct Date {
    pub jd: u32,
}

impl Default for Date {
    fn default() -> Self {
        Self { jd: 2451545 }
    }
}

#[inline]
fn is_leap_year(year: u16) -> bool {
    year % 4 == 0
}

impl Date {
    pub const fn new(jd: u32) -> Self {
        Self { jd }
    }

    pub const fn new_ymd(year: u16, month: u8, day: u8) -> Self {
        let a = if month < 3 { 1 } else { 0 };
        let y = (year as u32).wrapping_add(4800).wrapping_sub(a);
        let m = (month as u32)
            .wrapping_add(12u32.wrapping_mul(a))
            .wrapping_sub(3);
        Self {
            jd: (day as u32)
                .wrapping_add(153u32.wrapping_mul(m).wrapping_add(2) / 5)
                .wrapping_sub(32045)
                .wrapping_add(365u32.wrapping_mul(y))
                .wrapping_add(y / 4)
                .wrapping_sub(y / 100)
                .wrapping_add(y / 400),
        }
    }

    pub const fn add_days(&self, days: u32) -> Self {
        Self {
            jd: self.jd.wrapping_add(days),
        }
    }

    pub const fn day(&self) -> u8 {
        self.get_parts().day
    }

    pub fn days_in_month(year: u16, month: u8) -> u8 {
        if month == 2 && is_leap_year(year) {
            29
        } else {
            MONTH_DAYS[(month - 1) as usize]
        }
    }

    pub const fn day_of_week(&self) -> u8 {
        (self.jd.wrapping_add(1) % 7) as u8
    }

    pub const fn days_to(&self, other: &Date) -> u32 {
        other.jd - self.jd
    }

    pub const fn get_parts(&self) -> DateParts {
        let a = self.jd.wrapping_add(32044);
        let b = 4u32.wrapping_mul(a).wrapping_add(3) / 146097;
        let c = a.wrapping_sub(146097u32.wrapping_mul(b) / 4);

        let d = 4u32.wrapping_mul(c).wrapping_add(3) / 1461;
        let e = c.wrapping_sub(1461u32.wrapping_mul(d) / 4);
        let m = 5u32.wrapping_mul(e).wrapping_add(2) / 153;

        let year = 100u32
            .wrapping_mul(b)
            .wrapping_add(d)
            .wrapping_sub(4800)
            .wrapping_add(m / 10) as u16;
        let month = m.wrapping_add(3).wrapping_sub(12u32.wrapping_mul(m / 10)) as u8;
        let day = e
            .wrapping_sub(153u32.wrapping_mul(m).wrapping_add(2) / 5)
            .wrapping_add(1) as u8;

        DateParts { year, month, day }
    }

    pub const fn month(&self) -> u8 {
        self.get_parts().month
    }

    pub const fn year(&self) -> u16 {
        self.get_parts().year
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let parts = self.get_parts();
        write!(
            f,
            "20{}-{}-{}",
            format_args!(
                "{}{}",
                NUMBERS[parts.year.wrapping_sub(2000) as usize][0],
                NUMBERS[parts.year.wrapping_sub(2000) as usize][1]
            ),
            format_args!(
                "{}{}",
                NUMBERS[parts.month as usize][0], NUMBERS[parts.month as usize][1]
            ),
            format_args!(
                "{}{}",
                NUMBERS[parts.day as usize][0], NUMBERS[parts.day as usize][1]
            )
        )
    }
}

impl AddAssign<u32> for Date {
    fn add_assign(&mut self, rhs: u32) {
        self.jd = self.jd.wrapping_add(rhs);
    }
}

impl Add<u32> for Date {
    type Output = Date;

    fn add(self, rhs: u32) -> Self::Output {
        Self {
            jd: self.jd.wrapping_add(rhs),
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Default, Debug)]
pub struct Time {
    pub md: u32,
}

impl Time {
    pub const fn new(seconds: u32) -> Self {
        Self { md: seconds }
    }

    pub const fn new_hms(hour: u8, minute: u8, second: u8) -> Self {
        Self {
            md: (hour as u32)
                .wrapping_mul(3600)
                .wrapping_add((minute as u32).wrapping_mul(60))
                .wrapping_add(second as u32),
        }
    }

    pub fn add_seconds(&mut self, seconds: u32) -> u32 {
        self.md = self.md.wrapping_add(seconds);
        let mut days = 0;
        while self.md >= 86400 {
            self.md = self.md.wrapping_sub(86400);
            days += 1;
        }
        days
    }

    pub const fn hour(&self) -> u8 {
        (self.md / 3600) as u8
    }

    pub const fn minute(&self) -> u8 {
        ((self.md % 3600) / 60) as u8
    }

    pub const fn second(&self) -> u8 {
        (self.md % 60) as u8
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{}",
            format_args!(
                "{}{}",
                NUMBERS[self.hour() as usize][0],
                NUMBERS[self.hour() as usize][1]
            ),
            format_args!(
                "{}{}",
                NUMBERS[self.minute() as usize][0],
                NUMBERS[self.minute() as usize][1]
            ),
            format_args!(
                "{}{}",
                NUMBERS[self.second() as usize][0],
                NUMBERS[self.second() as usize][1]
            )
        )
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Default, Debug)]
pub struct DateTime {
    pub date: Date,
    pub time: Time,
}

impl DateTime {
    pub const fn new_from_parts(date: &Date, time: &Time) -> Self {
        Self {
            date: *date,
            time: *time,
        }
    }

    pub const fn new(jd: u32, seconds: u32) -> Self {
        Self {
            date: Date::new(jd),
            time: Time::new(seconds),
        }
    }

    pub fn new_ymd_hms(year: u16, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> Self {
        Self {
            date: Date::new_ymd(year, month, day),
            time: Time::new_hms(hour, minute, second),
        }
    }

    pub fn add_seconds(&mut self, seconds: u32) {
        let days = self.time.add_seconds(seconds);
        self.date = self.date.add_days(days);
    }

    pub fn add_secs(&self, seconds: u32) -> Self {
        let mut dt = *self;
        dt.add_seconds(seconds);
        dt
    }

    pub const fn date(&self) -> Date {
        self.date
    }

    pub const fn time(&self) -> Time {
        self.time
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.date, self.time)
    }
}
