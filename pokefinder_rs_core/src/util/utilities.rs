use crate::rng::BWRNG;
use bzip2_rs::DecoderReader;
use std::io::Read;

fn advance_probability_table(rng: &mut BWRNG) -> u32 {
    let mut count = 5;

    rng.advance(1);

    if rng.next_u32_max(101) > 50 {
        count += 1;
        rng.advance(1);
    }

    if rng.next_u32_max(101) > 30 {
        count += 1;
        rng.advance(1);
    }

    if rng.next_u32_max(101) > 25 {
        count += 1;
        if rng.next_u32_max(101) > 30 {
            count += 1;
            rng.advance(1);
        }
    }

    if rng.next_u32_max(101) > 20 {
        count += 1;
        if rng.next_u32_max(101) > 25 {
            count += 1;
            if rng.next_u32_max(101) > 33 {
                count += 1;
                rng.advance(1);
            }
        }
    }

    count
}

fn get_pitch(result: u8) -> String {
    let pitch = if result < 20 {
        "L"
    } else if result < 40 {
        "ML"
    } else if result < 60 {
        "M"
    } else if result < 80 {
        "MH"
    } else {
        "H"
    };

    format!("{pitch} {result}")
}

pub(crate) fn decompress(compressed_data: &[u8]) -> Vec<u8> {
    let mut reader = DecoderReader::new(compressed_data);
    let mut contents = vec![];
    reader
        .read_to_end(&mut contents)
        .expect("Failed to decompress");
    contents
}

/// Utility functions used by Gen 3 tools
pub mod utilities3 {
    use crate::util::{Date, DateTime};

    /// Calculates initial seed for Gen 3 games based on a [`DateTime`].
    ///
    /// # Example
    /// ```
    /// # use pokefinder_rs_core::util::DateTime;
    /// # use pokefinder_rs_core::util::utilities3::calc_seed;
    /// let date_time = DateTime::new_ymd_hms(2000, 6, 7, 19, 0, 0);
    /// let seed = calc_seed(&date_time);
    /// assert_eq!(seed, 0x843F);
    /// ```
    pub fn calc_seed(date_time: &DateTime) -> u16 {
        let date = &date_time.date();
        let time = &date_time.time();

        let d = Date::default()
            .days_to(date)
            .wrapping_sub(if date.year() > 2000 { 366 } else { 0 })
            .wrapping_add(1);

        let h = time.hour() as u32;
        let m = time.minute() as u32;

        let seed = 1440u32
            .wrapping_mul(d)
            .wrapping_add(960u32.wrapping_mul(h / 10))
            .wrapping_add(60u32.wrapping_mul(h % 10))
            .wrapping_add(16u32.wrapping_mul(m / 10))
            .wrapping_add(m % 10);

        ((seed >> 16) ^ (seed & 0xFFFF)) as u16
    }
}

/// Utility functions used by Gen 4 tools
pub mod utilities4 {
    use crate::rng::{PokeRNG, MT};
    use crate::util::utilities::get_pitch;
    use crate::util::DateTime;

    /// Calculates initial seed for Gen 4 games based on a [`DateTime`] and a specified `delay`.
    ///
    /// # Example
    /// ```
    /// # use pokefinder_rs_core::util::DateTime;
    /// # use pokefinder_rs_core::util::utilities4::calc_seed;
    /// let date_time = DateTime::new_ymd_hms(2000, 11, 5, 8, 15, 30);
    /// let seed = calc_seed(&date_time, 0);
    /// assert_eq!(seed, 0x64080000);
    /// ```
    pub fn calc_seed(date_time: &DateTime, delay: u32) -> u32 {
        let date = &date_time.date();
        let time = &date_time.time();

        let parts = date.get_parts();

        let ab = parts
            .month
            .wrapping_mul(parts.day)
            .wrapping_add(time.minute())
            .wrapping_add(time.second());
        let cd = time.hour();
        (((ab as u32) << 24) | ((cd as u32) << 16))
            .wrapping_add(delay)
            .wrapping_add(parts.year as u32)
            .wrapping_sub(2000)
    }

    /// Returns the coin flip sequence from a `seed`.
    ///
    /// # Example
    /// ```
    /// # use pokefinder_rs_core::util::utilities4::coin_flips;
    /// let seed = 0x12345678;
    /// let flips = coin_flips(seed);
    /// assert_eq!(&flips, "H, T, T, T, H, T, H, H, T, T, T, T, H, H, H, H, T, T, T, T");
    /// ```
    pub fn coin_flips(seed: u32) -> String {
        let mut coins = String::new();

        let mut mt = MT::new(seed);
        for i in 0..20 {
            coins += if (mt.next() & 1) == 0 { "T" } else { "H" };
            if i != 19 {
                coins += ", ";
            }
        }

        coins
    }

    /// Calculates the call sequence from a `seed` and `skips`.
    ///
    /// Skips are commonly based on roamers that have been released.
    ///
    /// # Example
    /// ```
    /// # use pokefinder_rs_core::gen4::HGSSRoamer;
    /// # use pokefinder_rs_core::util::utilities4::get_calls;
    /// let seed = 0x12345678;
    /// let roamer = HGSSRoamer::new(seed, [false; 3], [0; 3]);
    /// let calls = get_calls(seed, roamer.skips);
    /// assert_eq!(&calls, "K, E, K, E, P, E, P, E, P, P, P, P, K, P, K, P, E, K, E, E");
    /// ```
    pub fn get_calls(seed: u32, skips: u8) -> String {
        let mut calls = String::new();

        if skips > 0 {
            calls += "(";
        }

        let mut rng = PokeRNG::new(seed);
        for i in 0..(20 + skips) {
            let call = rng.next_u16_max::<true>(3) as u8;
            calls += match call {
                0 => "E",
                1 => "K",
                _ => "P",
            };

            if i != (20 + skips - 1) {
                if skips != 0 && skips == i + 1 {
                    calls += " skipped) ";
                } else {
                    calls += ", ";
                }
            }
        }

        calls
    }

    /// Returns the chatot pitch of a PRNG state
    ///
    /// # Example
    /// ```
    /// # use pokefinder_rs_core::util::utilities4::get_chatot;
    /// let prng = 15;
    /// let chatot = get_chatot(prng);
    /// assert_eq!("L 15".to_string(), chatot);
    /// ```
    pub fn get_chatot(prng: u8) -> String {
        get_pitch(prng)
    }
}

/// Utility functions used by Gen 5 tools
pub mod utilities5 {
    use crate::enums::Game;
    use crate::gen5::Profile5;
    use crate::parents::Profile;
    use crate::rng::BWRNG;
    use crate::util::utilities::{advance_probability_table, get_pitch};

    /// Returns a modified PID to force a specified gender
    ///
    /// Takes a PID (commonly provided by a generator) and modifies it to match
    /// a specified gender and gender ratio. A mutable [`BWRNG`] is necessary to match the
    /// advancement of the games when used to predict game state.
    ///
    /// # Example
    /// ```
    /// # use pokefinder_rs_core::rng::BWRNG;
    /// # use pokefinder_rs_core::util::utilities5::force_gender;
    /// let mut rng = BWRNG::new(0x1234567887654321);
    /// let pid = 0x12345678;
    /// let gender = 0;
    /// let gender_ratio = 127;
    /// let new_pid = force_gender(pid, &mut rng, gender, gender_ratio);
    /// assert_eq!(new_pid, 0x123456ED);
    /// ```
    pub fn force_gender(pid: u32, rng: &mut BWRNG, gender: u8, gender_ratio: u8) -> u32 {
        let val = match gender_ratio {
            0 => (rng.next_u32_max(0xf6) + 8) as u8,
            254 => (rng.next_u32_max(8) + 1) as u8,
            _ if gender == 0 => {
                (rng.next_u32_max((0xfe - gender_ratio) as u32) + gender_ratio as u32) as u8
            }
            _ if gender == 1 => (rng.next_u32_max(gender_ratio as u32 - 1) + 1) as u8,
            _ => {
                rng.advance(1);
                0
            }
        };

        (pid & 0xFFFFFF00) | (val as u32)
    }

    /// Returns the chatot pitch of a PRNG state
    ///
    /// # Example
    /// ```
    /// # use pokefinder_rs_core::util::utilities5::get_chatot;
    /// let prng = 15;
    /// let chatot = get_chatot(prng);
    /// assert_eq!("L 15".to_string(), chatot);
    /// ```
    pub fn get_chatot(prng: u8) -> String {
        get_pitch(prng)
    }

    /// Calculates the initial set of advances based on the specified [`Profile5`].
    ///
    /// Based on the version of the specified [`Profile5`], this calls [`initial_advances_bw`] or
    /// [`initial_advances_bw2`]. This can be used when profiles are dynamic.
    ///
    /// # Example
    /// ```
    /// # use pokefinder_rs_core::enums::{DSType, Game, Language};
    /// # use pokefinder_rs_core::gen5::Profile5;
    /// # use pokefinder_rs_core::util::utilities5::initial_advances;
    /// let profile = Profile5::new("-".to_string(), Game::BW2, 12345, 54321, 0, [false; 4], 0, 0, 0, false, 0, 0, false, false, false, DSType::DS, Language::English);
    /// let seed = 0x1234567887654321;
    /// let initial_advances_id = initial_advances(seed, &profile);
    /// ```
    pub fn initial_advances(seed: u64, profile: &Profile5) -> u32 {
        if (profile.get_version() & Game::BW) != Game::NONE {
            initial_advances_bw(seed)
        } else {
            initial_advances_bw2(seed, profile.get_memory_link())
        }
    }

    /// Calculates the initial set of advances for Pokemon Black and White.
    ///
    /// # Example
    /// ```
    /// # use pokefinder_rs_core::util::utilities5::initial_advances_bw;
    /// let seed = 0x1234567887654321;
    /// let initial_advances_id = initial_advances_bw(seed);
    /// ```
    pub fn initial_advances_bw(seed: u64) -> u32 {
        let mut rng = BWRNG::new(seed);
        let mut count = 0;
        for _ in 0..5 {
            count += advance_probability_table(&mut rng);
        }
        count
    }

    /// Calculates the initial set of advances for Pokemon Black 2 and White 2.
    ///
    /// `memory` is usually retrieved from a [`Profile5`] struct.
    ///
    /// # Example
    /// ```
    /// # use pokefinder_rs_core::enums::{DSType, Game, Language};
    /// # use pokefinder_rs_core::gen5::Profile5;
    /// # use pokefinder_rs_core::util::utilities5::initial_advances_bw2;
    /// # let profile = Profile5::new("-".to_string(), Game::BW2, 12345, 54321, 0, [false; 4], 0, 0, 0, false, 0, 0, false, false, false, DSType::DS, Language::English);
    /// let seed = 0x1234567887654321;
    /// let initial_advances_id = initial_advances_bw2(seed, profile.get_memory_link());
    /// ```
    pub fn initial_advances_bw2(seed: u64, memory: bool) -> u32 {
        let mut rng = BWRNG::new(seed);
        let mut count = 0;
        for i in 0..5 {
            count += advance_probability_table(&mut rng);
            if i == 0 {
                count += if memory { 2 } else { 3 };
                rng.advance(if memory { 2 } else { 3 });
            }
        }

        for _ in 0..100 {
            count += 3;
            let rand1 = rng.next_u32_max(15) as u8;
            let rand2 = rng.next_u32_max(15) as u8;
            let rand3 = rng.next_u32_max(15) as u8;

            if rand1 != rand2 && rand1 != rand3 && rand2 != rand3 {
                break;
            }
        }

        count
    }

    /// Calculates the initial set of advances for Pokemon Black and White ID generation.
    ///
    /// # Example
    /// ```
    /// # use pokefinder_rs_core::util::utilities5::initial_advances_bwid;
    /// let seed = 0x1234567887654321;
    /// let initial_advances_id = initial_advances_bwid(seed);
    /// ```
    pub fn initial_advances_bwid(seed: u64) -> u32 {
        let mut rng = BWRNG::new(seed);
        let mut count = 2;
        for _ in 0..3 {
            count += advance_probability_table(&mut rng);
        }
        count
    }

    /// Calculates the initial set of advances for Pokemon Black 2 and White 2 ID generation.
    ///
    /// # Example
    /// ```
    /// # use pokefinder_rs_core::util::utilities5::initial_advances_bw2id;
    /// let seed = 0x1234567887654321;
    /// let initial_advances_id = initial_advances_bw2id(seed);
    /// ```
    pub fn initial_advances_bw2id(seed: u64) -> u32 {
        let mut rng = BWRNG::new(seed);
        let mut count = 10;
        for i in 0..3 {
            count += advance_probability_table(&mut rng);
            if i == 0 {
                rng.advance(2);
            } else if i == 1 {
                rng.advance(4);
            }
        }
        count
    }

    /// Calculates the initial set of advances before ID generation based on the specified
    /// Gen 5 game version.
    ///
    /// This function expects either `Game::BW` or `Game::BW2` to be passed for the version.
    /// This function can be used where `version` is dynamic and will call [`initial_advances_bw`]
    /// or [`initial_advances_bw2id`] depending on the version.
    ///
    /// # Example
    /// ```
    /// # use pokefinder_rs_core::enums::Game;
    /// # use pokefinder_rs_core::util::utilities5::initial_advances_id;
    /// let seed = 0x1234567887654321;
    /// let version = Game::BW;
    /// let initial_advances_id = initial_advances_id(seed, version);
    /// ```
    pub fn initial_advances_id(seed: u64, version: Game) -> u32 {
        if (version & Game::BW) != Game::NONE {
            initial_advances_bwid(seed)
        } else {
            initial_advances_bw2id(seed)
        }
    }
}
