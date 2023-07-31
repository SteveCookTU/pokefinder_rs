#[derive(Copy, Clone)]
pub struct PGF {
    pub sid: u16,
    pub species: u16,
    pub tid: u16,
    pub egg: bool,
    pub ability: u8,
    pub gender: u8,
    pub ivs: [u8; 6],
    pub level: u8,
    pub nature: u8,
    pub shiny: u8,
}

impl From<&[u8]> for PGF {
    fn from(value: &[u8]) -> Self {
        Self {
            sid: u16::from_le_bytes(value[0x2..0x4].try_into().unwrap()),
            species: u16::from_le_bytes(value[0x1a..0x1c].try_into().unwrap()),
            tid: u16::from_le_bytes(value[0x0..0x2].try_into().unwrap()),
            egg: value[0x5c] == 1,
            ability: value[0x36],
            gender: value[0x35],
            ivs: [
                value[0x43],
                value[0x44],
                value[0x45],
                value[0x47],
                value[0x48],
                value[0x46],
            ],
            level: value[0x5b],
            nature: value[0x34],
            shiny: value[0x37],
        }
    }
}

impl PGF {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        tid: u16,
        sid: u16,
        species: u16,
        nature: u8,
        gender: u8,
        ability: u8,
        shiny: u8,
        level: u8,
        hp: u8,
        atk: u8,
        def: u8,
        spa: u8,
        spd: u8,
        spe: u8,
        egg: bool,
    ) -> Self {
        Self {
            sid,
            species,
            tid,
            egg,
            ability,
            gender,
            ivs: [hp, atk, def, spa, spd, spe],
            level,
            nature,
            shiny,
        }
    }

    pub fn get_advances(&self) -> u8 {
        let mut advances = 8;

        for iv in self.ivs {
            if iv == 255 {
                advances += 2;
            }
        }

        if self.gender == 0 || self.gender == 1 {
            advances += 2;
        }

        if self.nature == 255 {
            advances += 2;
        }

        advances
    }

    pub fn get_iv(&self, index: usize) -> u8 {
        self.ivs[index]
    }
}
