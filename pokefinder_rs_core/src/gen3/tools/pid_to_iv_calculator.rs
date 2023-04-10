use crate::enums::Method;
use crate::gen3::states::PIDToIVState;
use crate::rng::{lcrng_reverse, PokeRNG, PokeRNGR, XDRNG, XDRNGR};

pub fn calculate_ivs(pid: u32) -> Vec<PIDToIVState> {
    let mut states = vec![];

    let mut results_1 = calc_method_124(pid);
    let mut results_2 = calculate_method_xd_colo(pid);
    let mut results_3 = calc_method_channel(pid);

    states.append(&mut results_1);
    states.append(&mut results_2);
    states.append(&mut results_3);

    states
}

fn calc_method_124(pid: u32) -> Vec<PIDToIVState> {
    let mut states = vec![];

    let mut seeds = [0; 3];
    let size = lcrng_reverse::recover_poke_rng_pid(pid, &mut seeds);
    for rev_seed in seeds.into_iter().take(size) {
        let seed = PokeRNGR::new(rev_seed).next();

        let mut forward = PokeRNG::new(rev_seed);
        forward.advance(1);

        let iv1 = forward.next_u16();
        let iv2 = forward.next_u16();
        let iv3 = forward.next_u16();

        states.push(PIDToIVState::new_from_parts(
            seed,
            iv1,
            iv2,
            Method::Method1,
        ));
        states.push(PIDToIVState::new_from_parts(
            seed,
            iv2,
            iv3,
            Method::Method2,
        ));
        states.push(PIDToIVState::new_from_parts(
            seed,
            iv1,
            iv3,
            Method::Method4,
        ));
    }

    states
}

fn calc_method_channel(pid: u32) -> Vec<PIDToIVState> {
    let mut states = vec![];

    let mut seeds = [0; 2];
    let mut size = lcrng_reverse::recover_xdrng_pid(pid, &mut seeds);
    for rev_seed in seeds.into_iter().take(size) {
        let mut backward = XDRNGR::new(rev_seed);
        let sid = backward.next_u16();
        let seed = backward.next();

        let mut forward = XDRNG::new(seed);
        forward.advance(1);

        let mut high = forward.next_u16();
        let low = forward.next_u16();

        if (if low > 7 { 0 } else { 1 }) != (high ^ 40122 ^ sid) {
            high ^= 0x8000;
        }

        let val = ((high as u32) << 16) | (low as u32);
        if val == pid {
            forward.advance(3);
            let hp = (forward.next() >> 27) as u8;
            let atk = (forward.next() >> 27) as u8;
            let def = (forward.next() >> 27) as u8;
            let spe = (forward.next() >> 27) as u8;
            let spa = (forward.next() >> 27) as u8;
            let spd = (forward.next() >> 27) as u8;

            states.push(PIDToIVState::new(
                seed,
                hp,
                atk,
                def,
                spa,
                spd,
                spe,
                Method::Channel,
            ));
        }
    }

    size = lcrng_reverse::recover_xdrng_pid(pid ^ 0x80000000, &mut seeds);

    for rev_seed in seeds.into_iter().take(size) {
        let mut backward = XDRNGR::new(rev_seed);
        let sid = backward.next_u16();
        let seed = backward.next();

        let mut forward = XDRNG::new(seed);
        forward.advance(1);

        let mut high = forward.next_u16();
        let low = forward.next_u16();

        if (if low > 7 { 0 } else { 1 }) != (high ^ 40122 ^ sid) {
            high ^= 0x8000;
        }

        let val = ((high as u32) << 16) | (low as u32);
        if val == pid {
            forward.advance(3);
            let hp = (forward.next() >> 27) as u8;
            let atk = (forward.next() >> 27) as u8;
            let def = (forward.next() >> 27) as u8;
            let spe = (forward.next() >> 27) as u8;
            let spa = (forward.next() >> 27) as u8;
            let spd = (forward.next() >> 27) as u8;

            states.push(PIDToIVState::new(
                seed,
                hp,
                atk,
                def,
                spa,
                spd,
                spe,
                Method::Channel,
            ));
        }
    }

    states
}

fn calculate_method_xd_colo(pid: u32) -> Vec<PIDToIVState> {
    let mut states = vec![];

    let mut seeds = [0; 2];
    let size = lcrng_reverse::recover_xdrng_pid(pid, &mut seeds);
    for rev_seed in seeds.into_iter().take(size) {
        let mut backward = XDRNGR::new(rev_seed);
        backward.advance(1);

        let iv2 = backward.next_u16();
        let iv1 = backward.next_u16();
        let seed = backward.next();

        states.push(PIDToIVState::new_from_parts(seed, iv1, iv2, Method::XDColo));
    }

    states
}
