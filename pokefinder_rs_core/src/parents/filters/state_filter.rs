pub trait StateFilter {
    fn get_natures(&self) -> [bool; 25];
    fn get_powers(&self) -> [bool; 16];
    fn get_max(&self) -> [u8; 6];
    fn get_min(&self) -> [u8; 6];
    fn skip(&self) -> bool;
    fn get_ability(&self) -> u8;
    fn get_gender(&self) -> u8;
    fn get_shiny(&self) -> u8;

    fn compare_ability(&self, ability: u8) -> bool {
        self.skip() || self.get_ability() == 255 || self.get_ability() == ability
    }

    fn compare_gender(&self, gender: u8) -> bool {
        self.skip() || self.get_gender() == 255 || self.get_gender() == gender
    }

    fn compare_hidden_power(&self, hidden_power: u8) -> bool {
        self.skip() || self.get_powers()[hidden_power as usize]
    }

    fn compare_iv(&self, ivs: [u8; 6]) -> bool {
        if self.skip() {
            true
        } else {
            let min = self.get_min();
            let max = self.get_max();
            for (i, &iv) in ivs.iter().enumerate() {
                if iv < min[i] || iv > max[i] {
                    return false;
                }
            }
            true
        }
    }

    fn compare_nature(&self, nature: u8) -> bool {
        self.skip() || self.get_natures()[nature as usize]
    }

    fn compare_shiny(&self, shiny: u8) -> bool {
        self.skip() || self.get_shiny() == 255 || (self.get_shiny() & shiny) >= 1
    }
}

pub trait WildStateFilter: StateFilter {
    fn get_encounter_slots(&self) -> [bool; 12];

    fn compare_encounter_slot(&self, encounter_slot: u8) -> bool {
        self.skip() || self.get_encounter_slots()[encounter_slot as usize]
    }
}
