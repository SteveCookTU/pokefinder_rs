use crate::enums::Game;
use crate::parents::{
    PersonalInfo, PERSONAL_B2W2, PERSONAL_BDSP, PERSONAL_BW, PERSONAL_D, PERSONAL_HGSS, PERSONAL_P,
    PERSONAL_PT, PERSONAL_RSEFRLG, PERSONAL_SWSH,
};

/// Gets the [`PersonalInfo`] slice for the specified `version`
pub const fn get_personal_table(version: Game) -> &'static [PersonalInfo] {
    if (version.bits() & Game::GEN3.bits()) != Game::NONE.bits() {
        &PERSONAL_RSEFRLG
    } else if (version.bits() & Game::DIAMOND.bits()) != Game::NONE.bits() {
        &PERSONAL_D
    } else if (version.bits() & Game::PEARL.bits()) != Game::NONE.bits() {
        &PERSONAL_P
    } else if (version.bits() & Game::PLATINUM.bits()) != Game::NONE.bits() {
        &PERSONAL_PT
    } else if (version.bits() & Game::HGSS.bits()) != Game::NONE.bits() {
        &PERSONAL_HGSS
    } else if (version.bits() & Game::BW.bits()) != Game::NONE.bits() {
        &PERSONAL_BW
    } else if (version.bits() & Game::BW2.bits()) != Game::NONE.bits() {
        &PERSONAL_B2W2
    } else if (version.bits() & Game::SWSH.bits()) != Game::NONE.bits() {
        &PERSONAL_SWSH
    } else {
        &PERSONAL_BDSP
    }
}

/// Gets the [`PersonalInfo`] for the specified `species` and `form` based on the `version`
pub const fn get_personal_info(version: Game, species: u16, form: u8) -> &'static PersonalInfo {
    let info = get_personal_table(version);
    let base = &info[species as usize];
    let form_index = base.get_form_stats_index();
    if form == 0 || form_index == 0 {
        base
    } else {
        &info[(form_index as usize) + (form as usize) - 1]
    }
}
