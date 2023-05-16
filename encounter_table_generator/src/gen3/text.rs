use once_cell::sync::Lazy;
use regex::{Regex, RegexBuilder};
use std::collections::HashMap;

static ROUTE_REGEX: Lazy<Regex> = Lazy::new(|| {
    RegexBuilder::new(r"Route(\d+)")
        .case_insensitive(true)
        .build()
        .unwrap()
});

static ROOM_REGEX: Lazy<Regex> = Lazy::new(|| {
    RegexBuilder::new(r"Room(\d+)")
        .case_insensitive(true)
        .build()
        .unwrap()
});

static UNDERGROUND_REGEX_1: Lazy<Regex> = Lazy::new(|| {
    RegexBuilder::new(r"B(\d+)F")
        .case_insensitive(true)
        .build()
        .unwrap()
});

static UNDERGROUND_REGEX_2: Lazy<Regex> = Lazy::new(|| {
    RegexBuilder::new(r"(\d+)F")
        .case_insensitive(true)
        .build()
        .unwrap()
});

static UNDERGROUND_REGEX_3: Lazy<Regex> = Lazy::new(|| {
    RegexBuilder::new(r"(\d+)R")
        .case_insensitive(true)
        .build()
        .unwrap()
});

pub fn clean_string(mut map_string: String) -> String {
    map_string = map_string.replace("MAP_", "").replace('_', " ");

    let strings = map_string.split(' ');
    let mut output = Vec::new();
    for string in strings {
        if let Some(captures) = ROUTE_REGEX.captures(string) {
            output.push(format!("Route {}", captures.get(1).unwrap().as_str()));
        } else if let Some(captures) = ROOM_REGEX.captures(string) {
            output.push(format!("Room {}", captures.get(1).unwrap().as_str()));
        } else if string == "SSANNE" {
            output.push("S.S Anne".to_string());
        } else if string == "UNDERWATER1" {
            output.push("Underwater Route 124".to_string());
        } else if string == "UNDERWATER2" {
            output.push("Underwater Route 126".to_string());
        } else if UNDERGROUND_REGEX_1.is_match(string)
            || UNDERGROUND_REGEX_2.is_match(string)
            || UNDERGROUND_REGEX_3.is_match(string)
        {
            output.push(string.to_string());
        } else {
            let string = string.to_lowercase();
            let mut c = string.chars();
            output.push(c.next().unwrap().to_uppercase().chain(c).collect());
        }
    }

    output.join(" ")
}

pub fn load_pokemon() -> HashMap<String, u16> {
    const POKEMON: &str = include_str!("../../../resources/i18n/en/species_en.txt");

    let mut pokemon = HashMap::new();
    for (i, mut line) in POKEMON.lines().enumerate() {
        line = match line {
            "Nidoran♂" => "NIDORAN_M",
            "Nidoran♀" => "NIDORAN_F",
            _ => line,
        };
        pokemon.insert(format!("SPECIES_{}", line.to_uppercase()), i as u16 + 1);
    }

    pokemon
}
