use crate::util::nature;
use std::collections::HashSet;

const IV_ORDER: [usize; 6] = [0, 1, 2, 5, 3, 4];

fn calculate_stat(stat_index: usize, base_stat: u16, iv: u8, level: u8, nature: u8) -> u16 {
    let mut stat = 2u16
        .wrapping_mul(base_stat)
        .wrapping_add(iv as u16)
        .wrapping_mul(level as u16)
        / 100;
    if stat_index == 0 {
        stat += level as u16 + 10;
    } else {
        stat = nature::compute_stat(stat + 5, nature, stat_index);
    }
    stat
}

fn calculate_ivs(
    base_stats: &[u8],
    stats: &[u16],
    level: u8,
    nature: u8,
    characteristic: u8,
) -> [Vec<u8>; 6] {
    let mut min_ivs = vec![31; 6];
    let mut max_ivs = vec![0; 6];

    for i in 0..6 {
        for iv in 0..32 {
            let stat = calculate_stat(i, base_stats[i] as u16, iv, level, nature);
            if stat == stats[i] {
                min_ivs[i] = iv.min(min_ivs[i]);
                max_ivs[i] = iv.max(max_ivs[i]);
            }
        }
    }

    let mut possible = [vec![], vec![], vec![], vec![], vec![], vec![]];
    let mut indexes = vec![0, 1, 2, 3, 4, 5];
    let mut characteristic_high = 31;
    if characteristic != 255 {
        let stat = IV_ORDER[characteristic as usize / 5];
        let result = characteristic % 5;
        indexes = indexes.into_iter().filter(|&i| i != stat).collect();

        for i in min_ivs[stat]..=max_ivs[stat] {
            if (i % 5) == result {
                characteristic_high = i;
            }
        }

        possible[stat].push(characteristic_high);
    }

    for i in indexes.into_iter() {
        for iv in min_ivs[i]..=max_ivs[i] {
            if iv > characteristic_high {
                break;
            }
            possible[i].push(iv);
        }
    }
    possible
}

pub fn calculate_iv_range(
    base_stats: &[u8],
    stats: &[[u16; 6]],
    level: &[u8],
    nature: u8,
    characteristic: u8,
    hidden_power: u8,
) -> [Vec<u8>; 6] {
    let mut ivs = [vec![], vec![], vec![], vec![], vec![], vec![]];

    for i in 0..stats.len() {
        let current = calculate_ivs(base_stats, &stats[i], level[i], nature, characteristic);
        if i == 0 {
            ivs = current;
        } else {
            for j in 0..6 {
                let mut temp = vec![];
                for iv in ivs[j].iter() {
                    if current[j].contains(iv) {
                        temp.push(*iv)
                    }
                }
                ivs[j] = temp;
            }
        }
    }

    if hidden_power != 255 {
        let mut possible = [vec![], vec![], vec![], vec![], vec![], vec![]];
        for i in 0..6 {
            if ivs[i].iter().any(|&iv| iv % 2 == 0) {
                possible[i].push(0);
            }
            if ivs[i].iter().any(|&iv| iv % 2 == 1) {
                possible[i].push(1);
            }
        }

        let mut temp = [vec![], vec![], vec![], vec![], vec![], vec![]];
        for &hp in &possible[0] {
            let hp_val: u8 = hp;
            for &atk in &possible[1] {
                let atk_val = ((hp_val as u16) + 2 * (atk as u16)) as u8;
                for &def in &possible[2] {
                    let def_val = ((atk_val as u16) + 4 * (def as u16)) as u8;
                    for &spa in &possible[3] {
                        let spa_val = ((def_val as u16) + 16 * (spa as u16)) as u8;
                        for &spd in &possible[4] {
                            let spd_val = ((spa_val as u16) + 32 * (spd as u16)) as u8;
                            for &spe in &possible[5] {
                                let ty =
                                    (((spd_val as u16) + 8 * (spe as u16)) * 15 / 63) as u8;
                                if ty == hidden_power {
                                    for &iv in ivs[0].iter().filter(|&&iv| iv % 2 == hp) {
                                        temp[0].push(iv);
                                    }
                                    for &iv in ivs[1].iter().filter(|&&iv| iv % 2 == atk) {
                                        temp[1].push(iv);
                                    }
                                    for &iv in ivs[2].iter().filter(|&&iv| iv % 2 == def) {
                                        temp[2].push(iv);
                                    }
                                    for &iv in ivs[3].iter().filter(|&&iv| iv % 2 == spa) {
                                        temp[3].push(iv);
                                    }
                                    for &iv in ivs[4].iter().filter(|&&iv| iv % 2 == spd) {
                                        temp[4].push(iv);
                                    }
                                    for &iv in ivs[5].iter().filter(|&&iv| iv % 2 == spe) {
                                        temp[5].push(iv);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        for i in 0..6 {
            let set = temp[i].iter().copied().collect::<HashSet<_>>();
            ivs[i] = set.into_iter().collect();
            ivs[i].sort();
        }
    }

    ivs
}

pub fn next_level(base_stats: &[u8], ivs: &[impl AsRef<[u8]>], level: u8, nature: u8) -> [u8; 6] {
    let mut levels = [level; 6];

    for i in 0..6 {
        let stat_ivs = ivs[i].as_ref();
        if stat_ivs.len() < 2 {
            continue;
        }

        let mut l = level + 1;
        while levels[i] == level && l <= 100 {
            for j in 1..stat_ivs.len() {
                let previous = calculate_stat(i, base_stats[i] as u16, stat_ivs[j - 1], l, nature);
                let current = calculate_stat(i, base_stats[i] as u16, stat_ivs[j], l, nature);
                if previous < current {
                    levels[i] = l;
                    break;
                }
            }
            l += 1;
        }
    }

    levels
}
