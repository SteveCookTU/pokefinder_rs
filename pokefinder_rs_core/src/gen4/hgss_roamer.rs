use crate::rng::PokeRNG;

/// Generates routes HGSS roamers will appear on
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct HGSSRoamer {
    /// Entei route
    pub entei_route: u8,
    /// Latias/Latios route
    pub lati_route: u8,
    /// Raikou route
    pub raikou_route: u8,
    /// Number of PRNG calls used to determine roamer locations
    pub skips: u8,
}

fn get_route_j(prng: u16) -> u8 {
    let val = (prng & 15) as u8;
    if val < 11 {
        val + 29
    } else {
        val + 31
    }
}

fn get_route_k(prng: u16) -> u8 {
    let val = (prng % 25) as u8;
    if val > 21 {
        val + (val % 20)
    } else {
        val + 1
    }
}

impl HGSSRoamer {
    /// Construct a new [`HGSSRoamer`] struct
    pub fn new(seed: u32, roamers: [bool; 3], routes: [u8; 3]) -> Self {
        let mut new = Self {
            entei_route: 0,
            lati_route: 0,
            raikou_route: 0,
            skips: 0,
        };

        let mut rng = PokeRNG::new(seed);

        if roamers[0] {
            while {
                new.skips += 1;
                new.raikou_route = get_route_j(rng.next_u16());
                routes[0] == new.raikou_route
            } {}
        }

        if roamers[1] {
            while {
                new.skips += 1;
                new.entei_route = get_route_j(rng.next_u16());
                routes[1] == new.entei_route
            } {}
        }

        if roamers[2] {
            while {
                new.skips += 1;
                new.lati_route = get_route_k(rng.next_u16());
                routes[2] == new.lati_route
            } {}
        }

        new
    }

    /// Returns string of roamer locations
    pub fn get_route_string(&self) -> String {
        let mut route = String::new();
        if self.raikou_route != 0 {
            route += &format!("R: {} ", self.raikou_route);
        }
        if self.entei_route != 0 {
            route += &format!("E: {} ", self.entei_route);
        }
        if self.lati_route != 0 {
            route += &format!("L: {}", self.lati_route);
        }

        route
    }
}
