use crate::parents::states::WildSearcherState;
use crate::parents::PersonalInfo;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct WildSearcherState3 {
    pub base: WildSearcherState,
}

impl WildSearcherState3 {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        seed: u32,
        pid: u32,
        ivs: [u8; 6],
        ability: u8,
        gender: u8,
        level: u8,
        nature: u8,
        shiny: u8,
        encounter_slot: u8,
        specie: u16,
        form: u8,
        info: &PersonalInfo,
    ) -> Self {
        Self {
            base: WildSearcherState::new(
                seed,
                pid,
                ivs,
                ability,
                gender,
                level,
                nature,
                shiny,
                encounter_slot,
                0,
                specie,
                form,
                info,
            ),
        }
    }
}
