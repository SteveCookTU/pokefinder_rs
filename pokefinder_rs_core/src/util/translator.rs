#![allow(dead_code)]

use crate::enums::{Buttons, Game};
use crate::resources::i18n;
use crate::util;
use once_cell::sync::Lazy;
use std::collections::HashMap;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum Language {
    German,
    English,
    Spanish,
    French,
    Italian,
    Japanese,
    Korean,
    Chinese,
    Count,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[allow(clippy::upper_case_acronyms)]
enum Translation {
    Ability,
    BDSP,
    BW2,
    BW,
    Characteristic,
    DPPt,
    E,
    Form,
    FRLG,
    Gales,
    Game,
    HGSS,
    Item,
    Move,
    Nature,
    Power,
    RS,
    Specie,
    SwSh,
    Count,
}

static LANGUAGE: Lazy<Language> = Lazy::new(|| Language::English);
static ABILITIES: Lazy<Vec<String>> = Lazy::new(|| {
    let mut abilities = vec![];
    read_file_to_vec(Translation::Ability, &mut abilities);
    abilities
});
static CHARACTERISTICS: Lazy<Vec<String>> = Lazy::new(|| {
    let mut characteristics = vec![];
    read_file_to_vec(Translation::Characteristic, &mut characteristics);
    characteristics
});
static FORMS: Lazy<HashMap<u16, String>> = Lazy::new(|| read_file(Translation::Form));
static GAMES: Lazy<Vec<String>> = Lazy::new(|| {
    let mut games = vec![];
    read_file_to_vec(Translation::Game, &mut games);
    games
});
static HIDDEN_POWERS: Lazy<Vec<String>> = Lazy::new(|| {
    let mut hidden_powers = vec![];
    read_file_to_vec(Translation::Power, &mut hidden_powers);
    hidden_powers
});
static ITEMS: Lazy<Vec<String>> = Lazy::new(|| {
    let mut items = vec![];
    read_file_to_vec(Translation::Item, &mut items);
    items
});
static MOVES: Lazy<Vec<String>> = Lazy::new(|| {
    let mut moves = vec![];
    read_file_to_vec(Translation::Move, &mut moves);
    moves
});
static NATURES: Lazy<Vec<String>> = Lazy::new(|| {
    let mut natures = vec![];
    read_file_to_vec(Translation::Nature, &mut natures);
    natures
});
static SPECIES: Lazy<Vec<String>> = Lazy::new(|| {
    let mut species = vec![];
    read_file_to_vec(Translation::Specie, &mut species);
    species
});
static GENDERS: [&str; 3] = ["♂", "♀", "-"];
static BUTTONS: [&str; 12] = [
    "R", "L", "X", "Y", "A", "B", "Select", "Start", "Right", "Left", "Up", "Down",
];

fn read_file_to_vec(translation: Translation, strings: &mut Vec<String>) {
    let index = ((*LANGUAGE as usize) * Translation::Count as usize) + translation as usize;
    let start = i18n::INDEXES[index] + 2;
    let end = i18n::INDEXES[index + 1];

    let compressed_data = &i18n::I18N[start..end];
    let data = util::decompress(compressed_data);

    let mut i = 0;
    while i < data.len() {
        let it = data[i..].iter().position(|&i| i == 0).unwrap_or_default();
        strings.push(String::from_utf8_lossy(&data[i..(i + it)]).to_string());
        i += it + 1;
    }
}

fn read_file(translation: Translation) -> HashMap<u16, String> {
    let index = ((*LANGUAGE as usize) * Translation::Count as usize) + translation as usize;
    let start = i18n::INDEXES[index] + 2;
    let end = i18n::INDEXES[index + 1];

    let compressed_data = &i18n::I18N[start..end];
    let data = util::decompress(compressed_data);

    let mut strings = HashMap::new();
    let mut i = 0;
    while i < data.len() {
        let it = data[i..].iter().position(|&i| i == 0).unwrap_or_default();
        let line = String::from_utf8_lossy(&data[i..(i + it)]);
        let mut line = line.split(',');
        let num = line.next().unwrap().parse::<u16>().unwrap();
        let line = line.next().unwrap().to_string();
        strings.insert(num, line);
        i += it + 1;
    }

    strings
}

pub fn get_ability(ability: u16) -> &'static str {
    &ABILITIES[ability as usize]
}

pub fn get_characteristic(characteristic: u8) -> &'static str {
    &CHARACTERISTICS[characteristic as usize]
}

pub fn get_characteristics() -> &'static [String] {
    &CHARACTERISTICS
}

pub fn get_form(specie: u16, form: u8) -> &'static str {
    &FORMS[&(((form as u16) << 11) | specie)]
}

pub fn get_game(version: Game) -> &'static str {
    let index = version.bits().trailing_zeros();
    if index == 32 {
        &GAMES[GAMES.len() - 1]
    } else {
        &GAMES[index as usize]
    }
}

pub fn get_gender(gender: u8) -> &'static str {
    GENDERS[gender as usize]
}

pub fn get_genders() -> &'static [&'static str; 3] {
    &GENDERS
}

pub fn get_hidden_power(power: u8) -> &'static str {
    &HIDDEN_POWERS[power as usize]
}

pub fn get_hidden_powers() -> &'static [String] {
    &HIDDEN_POWERS
}

pub fn get_item(item: u16) -> &'static str {
    &ITEMS[item as usize]
}

pub fn get_items(items: &[u16]) -> Vec<&'static str> {
    items
        .iter()
        .map(|&num| ITEMS[num as usize].as_str())
        .collect()
}

pub fn get_keypress(keypress: u8) -> &'static str {
    BUTTONS[keypress as usize]
}

pub fn get_keypresses(keypresses: Buttons) -> String {
    if keypresses == Buttons::NONE {
        return "None".to_string();
    }

    let mut result = String::new();
    for (i, key) in BUTTONS.iter().enumerate() {
        if (keypresses.bits() & (1 << i)) != 0 {
            if !result.is_empty() {
                result += " + ";
            }
            result += key;
        }
    }

    result
}

pub fn get_locations(nums: &[u16], game: Game) -> Vec<String> {
    let translation = if (game & Game::EMERALD) != Game::NONE {
        Translation::E
    } else if (game & Game::FRLG) != Game::NONE {
        Translation::FRLG
    } else if (game & Game::RS) != Game::NONE {
        Translation::RS
    } else if (game & Game::GALES) != Game::NONE {
        Translation::Gales
    } else if (game & Game::DPPT) != Game::NONE {
        Translation::DPPt
    } else if (game & Game::HGSS) != Game::NONE {
        Translation::HGSS
    } else if (game & Game::BW) != Game::NONE {
        Translation::BW
    } else if (game & Game::BW2) != Game::NONE {
        Translation::BW2
    } else if (game & Game::SWSH) != Game::NONE {
        Translation::SwSh
    } else {
        Translation::BDSP
    };

    let map = read_file(translation);
    nums.iter().map(|num| map[num].to_string()).collect()
}

pub fn get_move(mov: u16) -> &'static str {
    &MOVES[mov as usize]
}

pub fn get_nature(nature: u8) -> &'static str {
    &NATURES[nature as usize]
}

pub fn get_natures() -> &'static [String] {
    &NATURES
}

pub fn get_specie(specie: u16) -> &'static str {
    &SPECIES[specie as usize]
}

pub fn get_specie_with_form(specie: u16, form: u8) -> String {
    if let Some(it) = FORMS.get(&(((form as u16) << 11) | specie)) {
        format!("{} ({})", SPECIES[(specie as usize) - 1], it)
    } else {
        SPECIES[(specie as usize) - 1].to_string()
    }
}

pub fn get_species(specie: &[u16]) -> Vec<&'static str> {
    specie
        .iter()
        .map(|&num| SPECIES[(num as usize) - 1].as_str())
        .collect()
}
