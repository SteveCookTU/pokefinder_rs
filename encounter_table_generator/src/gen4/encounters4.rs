use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Encounter {
    pub description: String,
    pub version: String,
    pub specie: i64,
    pub level: i64,
    pub method: String,
    pub shiny: Option<String>,
    pub form: Option<i64>,
}

#[derive(Deserialize)]
pub struct Encounters4 {
    pub starters: Vec<Encounter>,
    pub fossils: Vec<Encounter>,
    pub gifts: Vec<Encounter>,
    #[serde(rename = "gameCorner")]
    pub game_corner: Vec<Encounter>,
    pub stationary: Vec<Encounter>,
    pub legends: Vec<Encounter>,
    pub events: Vec<Encounter>,
    pub roamers: Vec<Encounter>,
}

pub fn embed_encounters(mut resource_path: PathBuf) {
    let data = serde_json::from_str::<Encounters4>(include_str!("./encounters.json")).unwrap();

    resource_path.push("encounter_data_4.rs");
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(resource_path)
        .unwrap();
    let mut writer = BufWriter::new(file);

    writer.write_all(b"use crate::enums::{Game, Method, Shiny};\nuse crate::gen4::StaticTemplate4;\n\n").unwrap();

    write_encounters("STARTERS", &mut writer, data.starters);
    write_encounters("FOSSILS", &mut writer, data.fossils);
    write_encounters("GIFTS", &mut writer, data.gifts);
    write_encounters("GAME_CORNER", &mut writer, data.game_corner);
    write_encounters("STATIONARY", &mut writer, data.stationary);
    write_encounters("LEGENDS", &mut writer, data.legends);
    write_encounters("EVENTS", &mut writer, data.events);
    write_encounters("ROAMERS", &mut writer, data.roamers);

    writer
        .write_all(b"pub(crate) static DIAMOND: &[u8] = include_bytes!(\"diamond.bin\");\n\n")
        .unwrap();
    writer
        .write_all(b"pub(crate) static HEART_GOLD: &[u8] = include_bytes!(\"heartgold.bin\");\n\n")
        .unwrap();
    writer
        .write_all(b"pub(crate) static HG_HEADBUTT: &[u8] = include_bytes!(\"hg_headbutt.bin\");\n\n")
        .unwrap();
    writer
        .write_all(b"pub(crate) static HGSS_BUG: &[u8] = include_bytes!(\"hgss_bug.bin\");\n\n")
        .unwrap();
    writer
        .write_all(b"pub(crate) static HGSS_SAFARI: &[u8] = include_bytes!(\"hgss_safari.bin\");\n\n")
        .unwrap();
    writer
        .write_all(b"pub(crate) static PEARL: &[u8] = include_bytes!(\"pearl.bin\");\n\n")
        .unwrap();
    writer
        .write_all(b"pub(crate) static PLATINUM: &[u8] = include_bytes!(\"platinum.bin\");\n\n")
        .unwrap();
    writer
        .write_all(b"pub(crate) static SOUL_SILVER: &[u8] = include_bytes!(\"soulsilver.bin\");\n\n")
        .unwrap();
    writer
        .write_all(b"pub(crate) static SS_HEADBUTT: &[u8] = include_bytes!(\"ss_headbutt.bin\");\n\n")
        .unwrap();

    writer.flush().unwrap();
}

fn write_encounters(name: &str, writer: &mut BufWriter<File>, encounters: Vec<Encounter>) {
    writer
        .write_all(
            format!(
                "pub(crate) static {}: [StaticTemplate4; {}] = [ ",
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
            format!("Game::from_bits_retain({}.bits() | {}.bits())", split.next().unwrap(), split.next().unwrap())
        } else {
            encounter.version.to_string()
        };

        writer
            .write_all(
                format!(
                    "StaticTemplate4::new({}, {}, {}, {}, {}, {})",
                    version,
                    encounter.specie,
                    encounter.form.unwrap_or_default(),
                    encounter.shiny.unwrap_or("Shiny::Random".to_string()),
                    encounter.level,
                    encounter.method
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