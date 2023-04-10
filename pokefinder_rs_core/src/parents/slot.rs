use crate::parents::PersonalInfo;

#[derive(Copy, Clone)]
pub struct Slot {
    pub(crate) info: &'static PersonalInfo,
    pub(crate) specie: u16,
    pub(crate) form: u8,
    pub(crate) min_level: u8,
    pub(crate) max_level: u8,
}

impl Slot {
    pub fn new(specie: u16, min_level: u8, max_level: u8, info: &'static PersonalInfo) -> Self {
        Self {
            info,
            specie,
            form: 0,
            min_level,
            max_level,
        }
    }

    pub fn new_with_form(
        specie: u16,
        form: u8,
        min_level: u8,
        max_level: u8,
        info: &'static PersonalInfo,
    ) -> Self {
        Self {
            info,
            specie,
            form,
            min_level,
            max_level,
        }
    }

    pub fn get_form(&self) -> u8 {
        self.form
    }

    pub fn get_info(&self) -> &'static PersonalInfo {
        self.info
    }

    pub fn get_max_level(&self) -> u8 {
        self.max_level
    }

    pub fn get_min_level(&self) -> u8 {
        self.min_level
    }

    pub fn get_specie(&self) -> u16 {
        self.specie
    }

    pub fn set_specie(&mut self, specie: u16, info: &'static PersonalInfo) {
        self.specie = specie;
        self.info = info;
    }
}
