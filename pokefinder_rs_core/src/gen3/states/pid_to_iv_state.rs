use crate::enums::Method;

#[derive(Copy, Clone)]
pub struct PIDToIVState {
    pub seed: u32,
    pub method: Method,
    pub ivs: [u8; 6],
}

impl PIDToIVState {
    pub const fn new_from_parts(seed: u32, iv1: u16, iv2: u16, method: Method) -> Self {
        let mut ivs = [0; 6];
        ivs[0] = (iv1 & 0x1f) as u8;
        ivs[1] = ((iv1 >> 5) & 0x1f) as u8;
        ivs[2] = ((iv1 >> 10) & 0x1f) as u8;
        ivs[3] = ((iv2 >> 5) & 0x1f) as u8;
        ivs[4] = ((iv2 >> 10) & 0x1f) as u8;
        ivs[5] = (iv2 & 0x1f) as u8;
        Self { seed, method, ivs }
    }

    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        seed: u32,
        hp: u8,
        atk: u8,
        def: u8,
        spa: u8,
        spd: u8,
        spe: u8,
        method: Method,
    ) -> Self {
        Self {
            seed,
            method,
            ivs: [hp, atk, def, spa, spd, spe],
        }
    }
}
