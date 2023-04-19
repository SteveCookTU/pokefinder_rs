use crate::enums::Method;
use crate::parents::PersonalInfo;
use crate::rng::{lcrng_reverse, PokeRNGR};
use std::collections::HashSet;

#[derive(Default, Clone)]
pub struct ChainedSIDCalc {
    pub sids: Vec<u16>,
    pub tid: u16,
}

impl ChainedSIDCalc {
    pub fn new(tid: u16) -> Self {
        let sids = (0..=0xFFFF).step_by(8).collect::<Vec<u16>>();
        Self { sids, tid }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn add_entry(
        &mut self,
        hp: u8,
        atk: u8,
        def: u8,
        spa: u8,
        spd: u8,
        spe: u8,
        ability: u16,
        gender: u8,
        nature: u8,
        info: &PersonalInfo,
    ) {
        let mut pids = vec![];

        let mut seeds = [0; 6];
        let size = lcrng_reverse::recover_poke_rng_iv(
            hp,
            atk,
            def,
            spa,
            spd,
            spe,
            &mut seeds,
            Method::Method1,
        );
        for rev_seed in seeds.into_iter().take(size) {
            let mut rng = PokeRNGR::new(rev_seed);

            let mut adjust = 0;
            for j in 0..13 {
                adjust |= ((rng.next_u16() & 1) as u32) << (15 - j);
            }

            let pid2 = rng.next_u16();
            let pid1 = rng.next_u16();

            let adjust_low = adjust | ((pid1 & 7) as u32);
            let ability_num = info.get_ability((adjust_low & 1) as usize);

            let gender_num = match info.get_gender() {
                255 => 2,
                254 => 1,
                0 => 0,
                _ => (((adjust_low & 255) as u8) < info.get_gender()) as u8,
            };

            if ability == ability_num && gender == gender_num {
                pids.push((adjust_low, pid2 as u32));
            }
        }

        let mut new_sids = HashSet::new();
        for &sid in &self.sids {
            for &pair in &pids {
                let mut adjust_high = pair.0 ^ (self.tid as u32) ^ (sid as u32);
                adjust_high &= 0xFFF8;
                adjust_high = adjust_high.wrapping_add(pair.1 & 7);

                let pid = (adjust_high << 16) | pair.0;
                if ((pid % 25) as u8) == nature {
                    new_sids.insert(sid);
                }
            }
        }

        self.sids = new_sids.into_iter().collect::<Vec<_>>();
    }
}
