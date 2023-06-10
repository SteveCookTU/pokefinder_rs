use crate::enums::{Buttons, Game};
use crate::resources::i18n;
use crate::util;
use once_cell::sync::Lazy;
use std::collections::HashMap;

#[allow(dead_code)]
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
    Species,
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
    read_file_to_vec(Translation::Species, &mut species);
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

/// Gets the string for a specified ability.
///
/// # Panics
///
/// This function will panic if the specified ability does not exist.
/// ```should_panic
/// # use pokefinder_rs_core::util::translator::get_ability;
/// let ability = 65535;
/// get_ability(ability);
/// ```
///
/// # Example
/// ```
/// # use pokefinder_rs_core::util::translator::get_ability;
/// let ability = 1;
/// assert_eq!(get_ability(ability), "Stench");
/// ```
pub fn get_ability(ability: u16) -> &'static str {
    &ABILITIES[ability as usize - 1]
}

/// Gets the string for a specified characteristic.
///
/// # Panics
///
/// This function will panic if the characteristic does not exist.
/// ```should_panic
/// # use pokefinder_rs_core::util::translator::get_characteristic;
/// let characteristic = 255;
/// get_characteristic(characteristic);
/// ```
///
/// # Example
/// ```
/// # use pokefinder_rs_core::util::translator::get_characteristic;
/// let characteristic = 0;
/// assert_eq!(get_characteristic(characteristic), "Loves to eat");
/// ```
pub fn get_characteristic(characteristic: u8) -> &'static str {
    &CHARACTERISTICS[characteristic as usize]
}

/// Gets all the characteristic strings.
///
/// # Example
/// ```
/// # use pokefinder_rs_core::util::translator::get_characteristics;
/// let characteristics = get_characteristics();
/// ```
pub fn get_characteristics() -> &'static [String] {
    &CHARACTERISTICS
}

/// Gets the form string for the specified species and form.
///
/// # Panics
///
/// This function will panic if the specified species and form combination does not exist.
/// ```should_panic
/// # use pokefinder_rs_core::util::translator::get_form;
/// let species = 1;
/// let form = 1;
/// get_form(species, form);
/// ```
///
/// # Example
/// ```
/// # use pokefinder_rs_core::util::translator::get_form;
/// let species = 479;
/// let form = 1;
/// assert_eq!(get_form(species, form), "Heat");
/// ```
pub fn get_form(species: u16, form: u8) -> &'static str {
    &FORMS[&(((form as u16) << 11) | species)]
}

/// Gets the string for the specified [`Game`].
///
/// If no bits are set, this functions will get the last version available.
///
/// # Panics
///
/// This function will panic if the passed in [`Game`] is obtained using [`Game::from_bits_retain()`]
/// and the lowest bit set is outside of the range.
/// ```should_panic
/// # use pokefinder_rs_core::enums::Game;
/// use pokefinder_rs_core::util::translator::get_game;
/// let game = Game::from_bits_retain(0x80000000);
/// get_game(game);
/// ```
///
/// # Example
/// ```
/// # use pokefinder_rs_core::enums::Game;
/// use pokefinder_rs_core::util::translator::get_game;
/// let game = Game::FIRE_RED;
/// assert_eq!(get_game(game), "Fire Red");
/// ```
pub fn get_game(version: Game) -> &'static str {
    let index = version.bits().trailing_zeros();
    if index == 32 {
        &GAMES[GAMES.len() - 1]
    } else {
        &GAMES[index as usize]
    }
}

/// Gets the string for the specified gender.
///
/// # Panic
/// ```should_panic
/// # use pokefinder_rs_core::util::translator::get_gender;
/// let gender = 4;
/// get_gender(gender);
/// ```
///
/// # Example
/// ```
/// # use pokefinder_rs_core::util::translator::get_gender;
/// let gender = 0;
/// assert_eq!(get_gender(gender), "♂");
/// ```
pub fn get_gender(gender: u8) -> &'static str {
    GENDERS[gender as usize]
}

/// Gets all the gender strings.
///
/// # Example
/// ```
/// # use pokefinder_rs_core::util::translator::get_genders;
/// let genders = get_genders();
/// ```
pub fn get_genders() -> &'static [&'static str; 3] {
    &GENDERS
}

/// Gets the string for a specified hidden power.
///
/// # Panics
///
/// This function will panic if the hidden power does not exist.
/// ```should_panic
/// # use pokefinder_rs_core::util::translator::get_hidden_power;
/// let power = 17;
/// get_hidden_power(power);
/// ```
///
/// # Example
/// ```
/// # use pokefinder_rs_core::util::translator::get_hidden_power;
/// let power = 0;
/// assert_eq!(get_hidden_power(power), "Fighting");
/// ```
pub fn get_hidden_power(power: u8) -> &'static str {
    &HIDDEN_POWERS[power as usize]
}

/// Gets all the hidden power strings.
///
/// # Example
/// ```
/// # use pokefinder_rs_core::util::translator::get_hidden_powers;
/// let hidden_powers = get_hidden_powers();
/// ```
pub fn get_hidden_powers() -> &'static [String] {
    &HIDDEN_POWERS
}

/// Gets the string for a specified item number.
///
/// # Panics
///
/// This function will panic if the specified item number does not exist.
/// ```should_panic
/// # use pokefinder_rs_core::util::translator::get_item;
/// let item = 65535;
/// get_item(item);
/// ```
///
/// # Example
/// ```
/// # use pokefinder_rs_core::util::translator::get_item;
/// let item = 1;
/// assert_eq!(get_item(item), "Master Ball");
/// ```
pub fn get_item(item: u16) -> &'static str {
    &ITEMS[item as usize]
}

/// Gets the strings for a specified list of item numbers.
///
/// # Panics
///
/// This function will panic if a specified item number does not exist.
/// ```should_panic
/// # use pokefinder_rs_core::util::translator::get_items;
/// let items = [1, 65535];
/// get_items(&items);
/// ```
///
/// # Example
/// ```
/// # use pokefinder_rs_core::util::translator::get_items;
/// let items = [1, 2];
/// assert_eq!(&get_items(&items), &["Master Ball", "Ultra Ball"]);
/// ```
pub fn get_items(items: &[u16]) -> Vec<&'static str> {
    items
        .iter()
        .map(|&num| ITEMS[num as usize].as_str())
        .collect()
}

/// Gets the string equivalent for a specified button.
///
/// # Panics
///
/// This function will panic if the specified keypress does not exist.
/// ```should_panic
/// # use pokefinder_rs_core::enums::Buttons;
/// # use pokefinder_rs_core::util::translator::get_keypress;
/// let keypress = 13;
/// get_keypress(keypress);
/// ```
///
/// # Example
/// ```
/// # use pokefinder_rs_core::enums::Buttons;
/// # use pokefinder_rs_core::util::translator::get_keypress;
/// let keypress = 0;
/// assert_eq!(get_keypress(keypress), "R");
/// ```
pub fn get_keypress(keypress: u8) -> &'static str {
    BUTTONS[keypress as usize]
}

/// Gets the string equivalent for a specified [`Buttons`] bitflag.
///
/// # Example
/// ```
/// # use pokefinder_rs_core::enums::Buttons;
/// # use pokefinder_rs_core::util::translator::get_keypresses;
/// let keypresses = Buttons::B | Buttons::A;
/// assert_eq!(&get_keypresses(keypresses), "A + B");
/// ```
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

/// Gets the strings for a provided list of locations and [`Game`] version.
///
/// # Panics
///
/// This function will panic if a specified location number does not exist.
/// ```should_panic
/// # use pokefinder_rs_core::enums::Game;
/// # use pokefinder_rs_core::util::translator::get_locations;
/// let locations = [65535, 2, 3];
/// get_locations(&locations, Game::FRLG);
/// ```
///
/// # Example
/// ```
/// # use pokefinder_rs_core::enums::Game;
/// # use pokefinder_rs_core::util::translator::get_locations;
/// let locations = [1, 2, 3];
/// assert_eq!(
///     &get_locations(&locations, Game::FRLG),
///     &[
///         "Seven Island Tanoby Ruins Liptoo Chamber".to_string(),
///         "Seven Island Tanoby Ruins Weepth Chamber".to_string(),
///         "Seven Island Tanoby Ruins Dilford Chamber".to_string()]
///     );
/// ```
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

/// Returns a string for a specified move.
///
/// # Panics
///
/// This function will panic if the specified move does not exist.
/// ```should_panic
/// # use pokefinder_rs_core::util::translator::get_move;
/// let mov = 65535;
/// get_move(mov);
/// ```
///
/// # Example
/// ```
/// # use pokefinder_rs_core::util::translator::get_move;
/// let mov = 18;
/// assert_eq!(get_move(mov), "Whirlwind");
/// ```
pub fn get_move(mov: u16) -> &'static str {
    &MOVES[mov as usize]
}

/// Returns a string for a specified nature.
///
/// # Panics
///
/// This function will panic if the specified nature does not exist.
/// ```should_panic
/// # use pokefinder_rs_core::util::translator::get_nature;
/// let nature = 255;
/// get_nature(nature);
/// ```
///
/// # Example
/// ```
/// # use pokefinder_rs_core::util::translator::get_nature;
/// let nature = 0;
/// assert_eq!(get_nature(nature), "Hardy");
/// ```
pub fn get_nature(nature: u8) -> &'static str {
    &NATURES[nature as usize]
}

/// Gets all the nature strings.
///
/// # Example
/// ```
/// # use pokefinder_rs_core::util::translator::get_natures;
/// let natures = get_natures();
/// ```
pub fn get_natures() -> &'static [String] {
    &NATURES
}

/// Returns the string for a specified species.
///
/// # Panics
///
/// This function will panic if the specified species number does not exist.
/// ```should_panic
/// # use pokefinder_rs_core::util::translator::get_species;
/// let species = 65535;
/// let species_name = get_species(species);
/// ```
///
/// # Example
/// ```
/// # use pokefinder_rs_core::util::translator::get_species;
/// let species = 50;
/// let species_name = get_species(species);
/// assert_eq!(species_name, "Diglett");
/// ```
pub fn get_species(species: u16) -> &'static str {
    &SPECIES[(species as usize) - 1]
}

/// Returns the string for a specified species and form.
///
/// Unlike [`get_form`], this function also includes the species
/// and will fallback to the regular species name
/// if the specified form is not found.
///
/// # Panics
///
/// This function will panic if the specified species number does not exist.
/// ```should_panic
/// # use pokefinder_rs_core::util::translator::get_species_with_form;
/// let species = 65535;
/// let form = 0;
/// let species_name = get_species_with_form(species, form);
/// ```
///
/// # Example
/// ```
/// # use pokefinder_rs_core::util::translator::get_species_with_form;
/// let species = 479;
/// let form = 1;
/// let species_name = get_species_with_form(species, form);
/// assert_eq!(&species_name, "Rotom (Heat)");
/// ```
pub fn get_species_with_form(species: u16, form: u8) -> String {
    if let Some(it) = FORMS.get(&(((form as u16) << 11) | species)) {
        format!("{} ({})", SPECIES[(species as usize) - 1], it)
    } else {
        SPECIES[(species as usize) - 1].to_string()
    }
}

/// Returns the strings for a specified list of species numbers
///
/// # Panics
///
/// This function will panic if a specified species number does not exist.
/// ```should_panic
/// # use pokefinder_rs_core::util::translator::get_species_list;
/// let species = [2, 3, 65535];
/// let species_list = get_species_list(&species);
/// ```
///
/// Species numbers are the the National Pokedex numbers from the games.
/// # Example
/// ```
/// # use pokefinder_rs_core::util::translator::get_species_list;
/// let species = [2, 3, 4];
/// let species_list = get_species_list(&species);
/// assert_eq!(&species_list, &["Ivysaur", "Venusaur", "Charmander"]);
/// ```
pub fn get_species_list(species: &[u16]) -> Vec<String> {
    species
        .iter()
        .map(|&num| get_species_with_form(num & 0x7ff, (num >> 11) as u8))
        .collect()
}
