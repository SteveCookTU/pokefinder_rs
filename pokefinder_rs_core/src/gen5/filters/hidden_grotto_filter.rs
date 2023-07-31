use crate::gen5::states::HiddenGrottoState;

#[derive(Copy, Clone)]
pub struct HiddenGrottoFilter {
    pub encounter_slots: [bool; 11],
    pub genders: [bool; 2],
    pub groups: [bool; 4],
}

impl HiddenGrottoFilter {
    pub fn new(encounter_slots: [bool; 11], genders: [bool; 2], groups: [bool; 4]) -> Self {
        Self {
            encounter_slots,
            genders,
            groups,
        }
    }

    pub fn compare_state(&self, state: &HiddenGrottoState) -> bool {
        !(!self.encounter_slots[state.slot as usize]
            || (!state.item && !self.genders[state.gender as usize])
            || !self.groups[state.group as usize])
    }
}
