use serde::Deserialize;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct ShadowLock {
    pub description: String,
    pub nature: u8,
    pub gender: u8,
    #[serde(rename = "genderRatio")]
    pub gender_ratio: u8,
}

#[derive(Deserialize)]
pub struct ShadowEncounter {
    pub description: String,
    pub version: String,
    pub specie: u16,
    pub level: u8,
    pub locks: Vec<ShadowLock>,
    pub count: u8,
    #[serde(rename = "type")]
    pub r#type: String,
    pub shiny: Option<String>,
}

#[derive(Deserialize)]
pub struct Encounter {
    pub description: String,
    pub version: String,
    pub specie: u16,
    pub level: u8,
    pub form: Option<u8>,
    pub shiny: Option<String>,
}

#[derive(Deserialize)]
pub struct Encounters3 {
    pub starters: Vec<Encounter>,
    pub fossils: Vec<Encounter>,
    pub gifts: Vec<Encounter>,
    #[serde(rename = "gameCorner")]
    pub game_corner: Vec<Encounter>,
    pub stationary: Vec<Encounter>,
    pub legends: Vec<Encounter>,
    pub events: Vec<Encounter>,
    #[serde(rename = "galesColo")]
    pub gales_colo: Vec<Encounter>,
    #[serde(rename = "galesColoShadow")]
    pub gales_colo_shadow: Vec<ShadowEncounter>,
    pub channel: Vec<Encounter>,
}

pub fn embed_encounters(mut resource_path: PathBuf) {
    let data = serde_json::from_str::<Encounters3>(include_str!("./encounters.json")).unwrap();

    resource_path.push("encounter_data_3.rs");
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(resource_path)
        .unwrap();
    let mut writer = BufWriter::new(file);

    writer.write_all(b"use crate::enums::{Game, ShadowType, Shiny};\nuse crate::gen3::{LockInfo, ShadowTemplate};\nuse crate::parents::StaticTemplate;\n\n").unwrap();

    write_encounters("STARTERS", &mut writer, data.starters);
    write_encounters("FOSSILS", &mut writer, data.fossils);
    write_encounters("GIFTS", &mut writer, data.gifts);
    write_encounters("GAME_CORNER", &mut writer, data.game_corner);
    write_encounters("STATIONARY", &mut writer, data.stationary);
    write_encounters("LEGENDS", &mut writer, data.legends);
    write_encounters("EVENTS", &mut writer, data.events);
    write_encounters("GALES_COLO", &mut writer, data.gales_colo);
    write_shadow_encounters("GALES_COLO_SHADOW", &mut writer, data.gales_colo_shadow);
    write_encounters("CHANNEL", &mut writer, data.channel);

    writer
        .write_all(b"pub(crate) static EMERALD: &[u8] = include_bytes!(\"emerald.bin\");\n\n")
        .unwrap();
    writer
        .write_all(b"pub(crate) static FIRERED: &[u8] = include_bytes!(\"firered.bin\");\n\n")
        .unwrap();
    writer
        .write_all(b"pub(crate) static LEAFGREEN: &[u8] = include_bytes!(\"leafgreen.bin\");\n\n")
        .unwrap();
    writer
        .write_all(b"pub(crate) static RUBY: &[u8] = include_bytes!(\"ruby.bin\");\n\n")
        .unwrap();
    writer
        .write_all(b"pub(crate) static SAPPHIRE: &[u8] = include_bytes!(\"sapphire.bin\");\n\n")
        .unwrap();
    writer
        .write_all(b"pub(crate) static XD: &[u8] = include_bytes!(\"xd.bin\");\n\n")
        .unwrap();

    writer.flush().unwrap();
}

fn write_encounters(name: &str, writer: &mut BufWriter<File>, encounters: Vec<Encounter>) {
    writer
        .write_all(
            format!(
                "pub(crate) static {}: [StaticTemplate; {}] = [ ",
                name,
                encounters.len()
            )
            .as_bytes(),
        )
        .unwrap();

    let len = encounters.len();

    for (i, encounter) in encounters.into_iter().enumerate() {
        let version = if encounter.version.contains('|') {
            let mut split = encounter.version.split(" | ");
            format!(
                "Game::from_bits_retain({}.bits() | {}.bits())",
                split.next().unwrap(),
                split.next().unwrap()
            )
        } else {
            encounter.version.to_string()
        };

        writer
            .write_all(
                format!(
                    "StaticTemplate::new({}, {}, {}, {}, 255, 255, 0, {})",
                    version,
                    encounter.specie,
                    encounter.form.unwrap_or_default(),
                    encounter.shiny.unwrap_or("Shiny::Random".to_string()),
                    encounter.level
                )
                .as_bytes(),
            )
            .unwrap();
        if i != len - 1 {
            writer.write_all(b", ").unwrap();
        }
    }

    writer.write_all(b" ];\n\n").unwrap();
}

fn write_shadow_encounters(
    name: &str,
    writer: &mut BufWriter<File>,
    encounters: Vec<ShadowEncounter>,
) {
    writer
        .write_all(
            format!(
                "pub static {}: [ShadowTemplate; {}] = [ ",
                name,
                encounters.len()
            )
            .as_bytes(),
        )
        .unwrap();

    let len = encounters.len();

    for (i, encounter) in encounters.into_iter().enumerate() {
        let mut locks = "[ ".to_string();
        for j in 0..encounter.count {
            let lock = &encounter.locks[j as usize];
            locks += &format!(
                "LockInfo::new({}, {}, {})",
                lock.nature, lock.gender, lock.gender_ratio
            );
            if j != encounter.count - 1 || encounter.count != 5 {
                locks += ", "
            }
        }
        for j in 0..(5 - encounter.count) {
            locks += "LockInfo::default()";
            if j != (5 - encounter.count) - 1 {
                locks += ", "
            }
        }
        locks += " ]";

        let version = if encounter.version.contains('|') {
            let mut split = encounter.version.split(" | ");
            format!(
                "Game::from_bits_retain({}.bits() | {}.bits())",
                split.next().unwrap(),
                split.next().unwrap()
            )
        } else {
            encounter.version.to_string()
        };

        writer
            .write_all(
                format!(
                    "ShadowTemplate::new({}, {}, {}, {}, {}, {}, {})",
                    version,
                    encounter.specie,
                    encounter.shiny.unwrap_or("Shiny::Random".to_string()),
                    encounter.level,
                    locks,
                    encounter.count,
                    encounter.r#type
                )
                .as_bytes(),
            )
            .unwrap();
        if i != len - 1 {
            writer.write_all(b", ").unwrap();
        }
    }

    writer.write_all(b" ];\n\n").unwrap();
}
