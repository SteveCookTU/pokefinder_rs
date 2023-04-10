#[derive(Copy, Clone)]
pub struct LockInfo {
    ignore: bool,
    gender: u8,
    gender_ratio: u8,
    nature: u8,
}

impl LockInfo {
    pub const fn default() -> Self {
        Self {
            ignore: true,
            gender: 0,
            gender_ratio: 0,
            nature: 0,
        }
    }

    pub const fn new(nature: u8, gender: u8, gender_ratio: u8) -> Self {
        Self {
            ignore: nature == 0 && gender == 0 && gender_ratio == 0,
            gender,
            gender_ratio,
            nature,
        }
    }

    pub const fn compare(&self, pid: u32) -> bool {
        if self.gender != 2 && self.gender != ((pid as u8) < self.gender_ratio) as u8 {
            false
        } else {
            self.nature == (pid % 25) as u8
        }
    }

    pub const fn get_ignore(&self) -> bool {
        self.ignore
    }
}
