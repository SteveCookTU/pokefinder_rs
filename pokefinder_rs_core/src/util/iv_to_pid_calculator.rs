use crate::enums::Method;
use crate::parents::states::IVtoPIDState;
use crate::rng::{lcrng_reverse, PokeRNGR, XDRNG, XDRNGR};

const GENDER_THRESHOLDS: [u8; 5] = [0, 0x32, 0x4b, 0x96, 0xc8];

#[allow(clippy::too_many_arguments)]
fn calc_method_12(
    hp: u8,
    atk: u8,
    def: u8,
    spa: u8,
    spd: u8,
    spe: u8,
    nature: u8,
    tid: u16,
) -> Vec<IVtoPIDState> {
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

    let mut states = vec![];

    for rev_seed in seeds.iter().copied().take(size) {
        let mut rng = PokeRNGR::new(rev_seed);

        let high = rng.next_u16();
        let low = rng.next_u16();
        let mut sid = (high ^ low ^ tid) & 0xFFF8;
        let seed = rng.next();

        let mut pid = ((high as u32) << 16) | (low as u32);
        if ((pid % 25) as u8) == nature {
            states.push(IVtoPIDState::new(seed, pid, sid, Method::Method1));
        }

        pid = ((low as u32) << 16) | (high as u32);
        if ((pid % 25) as u8) == nature {
            states.push(IVtoPIDState::new(seed, pid, sid, Method::Method1));
        }

        if low / 0x5556 != 0 && ((high / 0xa3e) as u8) == nature {
            for thresh in GENDER_THRESHOLDS {
                pid = (nature as u32) + (thresh as u32);
                sid = ((pid ^ (tid as u32)) & 0xFFF8) as u16;
                states.push(IVtoPIDState::new(seed, pid, sid, Method::CuteCharmDPPt));
            }
        }

        if low % 3 != 0 && ((high % 25) as u8) == nature {
            for thresh in GENDER_THRESHOLDS {
                pid = (nature as u32) + (thresh as u32);
                sid = ((pid ^ (tid as u32)) & 0xFFF8) as u16;
                states.push(IVtoPIDState::new(seed, pid, sid, Method::CuteCharmHGSS));
            }
        }
    }

    for rev_seed in seeds.into_iter().take(size) {
        let mut rng = PokeRNGR::new(rev_seed);

        rng.advance(1);
        let high = rng.next_u16();
        let low = rng.next_u16();
        let sid = (high ^ low ^ tid) & 0xFFF8;
        let seed = rng.next();

        let pid = ((high as u32) << 16) | (low as u32);
        if ((pid % 25) as u8) == nature {
            states.push(IVtoPIDState::new(seed, pid, sid, Method::Method2));
        }
    }

    states
}

#[allow(clippy::too_many_arguments)]
fn calc_method_4(
    hp: u8,
    atk: u8,
    def: u8,
    spa: u8,
    spd: u8,
    spe: u8,
    nature: u8,
    tid: u16,
) -> Vec<IVtoPIDState> {
    let mut seeds = [0; 6];
    let size = lcrng_reverse::recover_poke_rng_iv(
        hp,
        atk,
        def,
        spa,
        spd,
        spe,
        &mut seeds,
        Method::Method4,
    );

    let mut states = vec![];

    for rev_seed in seeds.iter().copied().take(size) {
        let mut rng = PokeRNGR::new(rev_seed);

        let high = rng.next_u16();
        let low = rng.next_u16();
        let sid = (high ^ low ^ tid) & 0xFFF8;
        let seed = rng.next();

        let pid = ((high as u32) << 16) | (low as u32);
        if ((pid % 25) as u8) == nature {
            states.push(IVtoPIDState::new(seed, pid, sid, Method::Method4));
        }
    }

    states
}

#[allow(clippy::too_many_arguments)]
fn calc_method_channel(
    hp: u8,
    atk: u8,
    def: u8,
    spa: u8,
    spd: u8,
    spe: u8,
    nature: u8,
) -> Vec<IVtoPIDState> {
    let mut seeds = [0; 12];
    let size = lcrng_reverse::recover_channel_iv(hp, atk, def, spa, spd, spe, &mut seeds);

    let mut states = vec![];

    for rev_seed in seeds.iter().copied().take(size) {
        let mut rng = XDRNGR::new(rev_seed);
        rng.advance(3);

        let low = rng.next_u16();
        let mut high = rng.next_u16();
        let sid = rng.next_u16();
        let seed = rng.next();

        let comparison = if low > 7 { 0 } else { 1 };

        if comparison != (high ^ sid ^ 40122) {
            high ^= 0x8000;
        }

        let pid = ((high as u32) << 16) | (low as u32);
        if ((pid % 25) as u8) == nature {
            states.push(IVtoPIDState::new(seed, pid, sid, Method::Channel));
        }
    }

    states
}

#[allow(clippy::too_many_arguments)]
fn calc_method_xd_colo(
    hp: u8,
    atk: u8,
    def: u8,
    spa: u8,
    spd: u8,
    spe: u8,
    nature: u8,
    tid: u16,
) -> Vec<IVtoPIDState> {
    let mut seeds = [0; 6];
    let size = lcrng_reverse::recover_xdrng_iv(hp, atk, def, spa, spd, spe, &mut seeds);

    let mut states = vec![];

    for rev_seed in seeds.iter().copied().take(size) {
        let seed = XDRNGR::new(rev_seed).next();

        let mut rng = XDRNG::new(rev_seed);
        rng.advance(1);

        let high = rng.next_u16();
        let low = rng.next_u16();
        let sid = (high ^ low ^ tid) & 0xFFF8;

        let pid = ((high as u32) << 16) | (low as u32);
        if ((pid % 25) as u8) == nature {
            states.push(IVtoPIDState::new(seed, pid, sid, Method::XDColo));
        }
    }

    states
}

/// Computes PIDs from the specified IVs for [`Method::Method1`], [`Method::Method2`],
/// [`Method::Method4`], [`Method::XDColo`], and [`Method::Channel`].
///
/// For each method, the IVs are reversed into a list of seeds used to seed a PRNG. The PRNGs are
/// then advanced to find potential PIDs. Passing in a valid TID provides results accurate SIDs that
/// would have the pokemon be shiny.
///
/// # Example
/// ```
/// # use pokefinder_rs_core::util::iv_to_pid_calculator::calculate_pids;
/// let ivs = [5, 10, 15, 20, 25, 30];
/// let nature = 3;
/// let tid = 48263;
/// let pids = calculate_pids(ivs[0], ivs[1], ivs[2], ivs[3], ivs[4], ivs[5], nature, tid);
/// ```
#[allow(clippy::too_many_arguments)]
pub fn calculate_pids(
    hp: u8,
    atk: u8,
    def: u8,
    spa: u8,
    spd: u8,
    spe: u8,
    nature: u8,
    tid: u16,
) -> Vec<IVtoPIDState> {
    let mut states = vec![];

    states.append(&mut calc_method_12(
        hp, atk, def, spa, spd, spe, nature, tid,
    ));
    states.append(&mut calc_method_4(hp, atk, def, spa, spd, spe, nature, tid));
    states.append(&mut calc_method_xd_colo(
        hp, atk, def, spa, spd, spe, nature, tid,
    ));
    states.append(&mut calc_method_channel(
        hp, atk, def, spa, spd, spe, nature,
    ));

    states
}
