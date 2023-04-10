use crate::enums::Encounter;

const fn compute_table<const SIZE: usize, const GREATER: bool>(ranges: [u8; SIZE]) -> [u8; 100] {
    let mut table = [0; 100];

    let mut r = if GREATER { 99 } else { 0 };
    let mut i = 0;
    while i < SIZE {
        let range = ranges[i];
        if GREATER {
            while r >= range {
                table[r as usize] = i as u8;
                if r == 0 {
                    break;
                }
                r -= 1;
            }
        } else {
            while r < range {
                table[r as usize] = i as u8;
                r += 1;
            }
        }

        i += 1;
    }

    table
}

const GRASS: [u8; 100] =
    compute_table::<12, false>([20, 40, 50, 60, 70, 80, 85, 90, 94, 98, 99, 100]);
const ROCK_SMASH: [u8; 100] = compute_table::<2, false>([80, 100]);
const BUG: [u8; 100] = compute_table::<10, true>([80, 60, 50, 40, 30, 20, 15, 10, 5, 0]);
const HEADBUTT: [u8; 100] = compute_table::<6, false>([50, 65, 80, 90, 95, 100]);

const WATER_0: [u8; 100] = compute_table::<2, false>([70, 100]);
const WATER_1: [u8; 100] = compute_table::<3, false>([60, 80, 100]);
const WATER_2: [u8; 100] = compute_table::<5, false>([40, 80, 95, 99, 100]);
const WATER_3: [u8; 100] = compute_table::<5, false>([40, 70, 85, 95, 100]);
const WATER_4: [u8; 100] = compute_table::<5, false>([60, 90, 95, 99, 100]);

pub fn h_slot(rand: u8, encounter: Encounter) -> u8 {
    match encounter {
        Encounter::OldRod => WATER_0[rand as usize],
        Encounter::GoodRod => WATER_1[rand as usize],
        Encounter::SuperRod => WATER_2[rand as usize],
        Encounter::Surfing | Encounter::RockSmash => WATER_4[rand as usize],
        _ => GRASS[rand as usize],
    }
}

pub fn j_slot(rand: u8, encounter: Encounter) -> u8 {
    match encounter {
        Encounter::GoodRod | Encounter::SuperRod => WATER_2[rand as usize],
        Encounter::OldRod | Encounter::Surfing => WATER_4[rand as usize],
        _ => GRASS[rand as usize],
    }
}

pub fn k_slot(rand: u8, encounter: Encounter) -> u8 {
    match encounter {
        Encounter::OldRod | Encounter::GoodRod | Encounter::SuperRod => WATER_3[rand as usize],
        Encounter::Surfing => WATER_4[rand as usize],
        Encounter::BugCatchingContest => BUG[rand as usize],
        Encounter::Headbutt | Encounter::HeadbuttAlt | Encounter::HeadbuttSpecial => {
            HEADBUTT[rand as usize]
        }
        Encounter::RockSmash => ROCK_SMASH[rand as usize],
        _ => GRASS[rand as usize],
    }
}

pub fn bdsp_slot(rand: u8, encounter: Encounter) -> u8 {
    match encounter {
        Encounter::GoodRod | Encounter::SuperRod => WATER_2[rand as usize],
        Encounter::OldRod | Encounter::Surfing => WATER_4[rand as usize],
        _ => GRASS[rand as usize],
    }
}
