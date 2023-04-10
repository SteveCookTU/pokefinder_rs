use crate::enums::Method;
use crate::rng::XDRNG;

fn recover_poke_rng_iv_method_12(
    hp: u8,
    atk: u8,
    def: u8,
    spa: u8,
    spd: u8,
    spe: u8,
    seeds: &mut [u32],
) -> usize {
    const ADD: u32 = 0x6073;
    const MULT: u32 = 0x41c64e6d;
    const MOD: u32 = 0x67d3;
    const PAT: u32 = 0xd3e;
    const INC: u32 = 0x4034;

    let mut size = 0;

    let first = ((hp as u32) | ((atk as u32) << 5) | ((def as u32) << 10)) << 16;
    let second = ((spe as u32) | ((spa as u32) << 5) | ((spd as u32) << 10)) << 16;

    let diff = (second.wrapping_sub(first.wrapping_mul(MULT))) >> 16;
    let start1 = (diff.wrapping_mul(MOD).wrapping_add(INC) >> 16).wrapping_mul(PAT) % MOD;
    let start2 =
        ((diff ^ 0x8000).wrapping_mul(MOD).wrapping_add(INC) >> 16).wrapping_mul(PAT) % MOD;

    for low in (start1..0x10000).step_by(MOD as usize) {
        let seed = first | low;
        if (seed.wrapping_mul(MULT).wrapping_add(ADD) & 0x7fff0000) == second {
            seeds[size] = seed;
            size += 1;
            seeds[size] = seed ^ 0x80000000;
            size += 1;
        }
    }

    for low in (start2..0x10000).step_by(MOD as usize) {
        let seed = first | low;
        if (seed.wrapping_mul(MULT).wrapping_add(ADD) & 0x7fff0000) == second {
            seeds[size] = seed;
            size += 1;
            seeds[size] = seed ^ 0x80000000;
            size += 1;
        }
    }

    size
}

fn recover_poke_rng_iv_method_4(
    hp: u8,
    atk: u8,
    def: u8,
    spa: u8,
    spd: u8,
    spe: u8,
    seeds: &mut [u32],
) -> usize {
    const ADD: u32 = 0xe97e7b6a;
    const MULT: u32 = 0xc2a29a69;
    const MOD: u32 = 0x3a89;
    const PAT: u32 = 0x2e4c;
    const INC: u32 = 0x5831;

    let mut size = 0;

    let first = ((hp as u32) | ((atk as u32) << 5) | ((def as u32) << 10)) << 16;
    let second = ((spe as u32) | ((spa as u32) << 5) | ((spd as u32) << 10)) << 16;

    let diff = (second.wrapping_sub(first.wrapping_mul(MULT).wrapping_add(ADD))) >> 16;
    let start1 = (diff.wrapping_mul(MOD).wrapping_add(INC) >> 16).wrapping_mul(PAT) % MOD;
    let start2 =
        ((diff ^ 0x8000).wrapping_mul(MOD).wrapping_add(INC) >> 16).wrapping_mul(PAT) % MOD;

    for low in (start1..0x10000).step_by(MOD as usize) {
        let seed = first | low;
        if (seed.wrapping_mul(MULT).wrapping_add(ADD) & 0x7fff0000) == second {
            seeds[size] = seed;
            size += 1;
            seeds[size] = seed ^ 0x80000000;
            size += 1;
        }
    }

    for low in (start2..0x10000).step_by(MOD as usize) {
        let seed = first | low;
        if (seed.wrapping_mul(MULT).wrapping_add(ADD) & 0x7fff0000) == second {
            seeds[size] = seed;
            size += 1;
            seeds[size] = seed ^ 0x80000000;
            size += 1;
        }
    }

    size
}

pub fn recover_channel_iv(
    hp: u8,
    atk: u8,
    def: u8,
    spa: u8,
    spd: u8,
    spe: u8,
    seeds: &mut [u32],
) -> usize {
    let mut size = 0;

    const MULT: u32 = 0x45c82be5;
    const SUB: u32 = 0xcaf65b56;
    const BASE: u64 = 0x22e415eea37d41a;

    const PRIME: u32 = 3;
    const ADD: u64 = 0x300000000;
    const RMAX: u32 = 0x18000000;
    const SKIP: u64 = 0x661D29;

    let first = (hp as u32) << 27;

    let t = ((spe as u64) << 27)
        .wrapping_sub((MULT as u64).wrapping_mul(first as u64))
        .wrapping_sub(SUB as u64)
        & 0xFFFFFFFF;
    let x = t.wrapping_mul(PRIME as u64) % (MULT as u64);
    let kmax = (BASE.wrapping_sub(t) >> 32) as u32;

    let mut k = 0;
    while k <= kmax {
        let mut r = x.wrapping_add(SKIP.wrapping_mul(k as u64)) % MULT as u64;

        let mut m = (r % PRIME as u64) as u8;
        if m != 0 {
            m = if m == 1 { 2 } else { 1 };
            r = r.wrapping_add(SKIP.wrapping_mul(m as u64));
            k = k.wrapping_add(m as u32);
        }

        let mut tmp = t.wrapping_add(0x100000000u64.wrapping_mul(k as u64));
        while r < (RMAX as u64) && k <= kmax {
            let seed = first | ((tmp / (MULT as u64)) as u32);
            let mut rng = XDRNG::new(seed);
            if (rng.next() >> 27) as u8 == atk
                && (rng.next() >> 27) as u8 == def
                && (rng.advance(2) >> 27) as u8 == spa
                && (rng.next() >> 27) as u8 == spd
            {
                seeds[size] = seed;
                size += 1;
            }

            r = r.wrapping_add(SKIP.wrapping_mul(PRIME as u64));
            k = k.wrapping_add(PRIME);
            tmp = tmp.wrapping_add(ADD);
        }

        k = k.wrapping_add(
            ((MULT as u64)
                .wrapping_sub(r)
                .wrapping_add(SKIP)
                .wrapping_sub(1)
                / SKIP) as u32,
        );
    }

    size
}

#[allow(clippy::too_many_arguments)]
pub fn recover_poke_rng_iv(
    hp: u8,
    atk: u8,
    def: u8,
    spa: u8,
    spd: u8,
    spe: u8,
    seeds: &mut [u32],
    method: Method,
) -> usize {
    if method == Method::Method4 {
        recover_poke_rng_iv_method_4(hp, atk, def, spa, spd, spe, seeds)
    } else {
        recover_poke_rng_iv_method_12(hp, atk, def, spa, spd, spe, seeds)
    }
}

pub fn recover_poke_rng_pid(pid: u32, seeds: &mut [u32]) -> usize {
    const ADD: u32 = 0x6073;
    const MULT: u32 = 0x41c64e6d;
    const MOD: u32 = 0x67d3;
    const PAT: u32 = 0xd3e;
    const INC: u32 = 0x4034;

    let mut size = 0;

    let first = pid << 16;
    let second = pid & 0xFFFF0000;

    let diff = (second.wrapping_sub(first.wrapping_mul(MULT))) >> 16;
    let start = (diff.wrapping_mul(MOD).wrapping_add(INC) >> 16).wrapping_mul(PAT) % MOD;

    for low in (start..0x10000).step_by(MOD as usize) {
        let seed = first | low;
        if (seed.wrapping_mul(MULT).wrapping_add(ADD) & 0xffff0000) == second {
            seeds[size] = seed;
            size += 1;
        }
    }

    size
}

pub fn recover_xdrng_iv(
    hp: u8,
    atk: u8,
    def: u8,
    spa: u8,
    spd: u8,
    spe: u8,
    seeds: &mut [u32],
) -> usize {
    let mut size = 0;

    const MULT: u32 = 0x343FD;
    const SUB: u32 = 0x259EC4;
    const BASE: u64 = 0x343fabc02;

    let first = ((hp as u32) | ((atk as u32) << 5) | ((def as u32) << 10)) << 16;
    let second = ((spe as u32) | ((spa as u32) << 5) | ((spd as u32) << 10)) << 16;

    let mut t = (second as u64)
        .wrapping_sub((MULT as u64).wrapping_mul(first as u64))
        .wrapping_sub(SUB as u64)
        & 0x7FFFFFFF;
    let kmax = (BASE.wrapping_sub(t) >> 31) as u32;

    let mut k = 0;
    while k <= kmax {
        if t % (MULT as u64) < 0x10000 {
            let seed = first | ((t / MULT as u64) as u32);
            seeds[size] = seed;
            size += 1;
            seeds[size] = seed ^ 0x80000000;
            size += 1;
        }

        t = t.wrapping_add(0x80000000);
        k += 1;
    }

    size
}

pub fn recover_xdrng_pid(pid: u32, seeds: &mut [u32]) -> usize {
    let mut size = 0;

    const MULT: u32 = 0x343FD;
    const SUB: u32 = 0x259EC4;
    const BASE: u64 = 0x343fabc02;

    let first = pid & 0xFFFF0000;
    let second = pid << 16;

    let mut t = (second as u64)
        .wrapping_sub((MULT as u64).wrapping_mul(first as u64))
        .wrapping_sub(SUB as u64)
        & 0xFFFFFFFF;
    let kmax = (BASE.wrapping_sub(t) >> 32) as u32;

    let mut k = 0;
    while k <= kmax {
        if t % (MULT as u64) < 0x10000 {
            let seed = first | ((t / MULT as u64) as u32);
            seeds[size] = seed;
            size += 1;
        }

        t = t.wrapping_add(0x100000000);
        k += 1;
    }

    size
}
