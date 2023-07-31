use serde::Deserialize;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

#[derive(Deserialize)]
struct Encounters5 {
    #[serde(rename = "dreamRadar")]
    dream_radar: Vec<Encounter>,
    starters: Vec<Encounter>,
    fossils: Vec<Encounter>,
    gifts: Vec<Encounter>,
    stationary: Vec<Encounter>,
    legends: Vec<Encounter>,
    events: Vec<Encounter>,
    roamers: Vec<Encounter>,
}

#[derive(Deserialize)]
struct Encounter {
    #[serde(default)]
    version: String,
    specie: u16,
    #[serde(default)]
    level: u8,
    #[serde(default = "default_ability_gender")]
    ability: u8,
    #[serde(default = "default_ability_gender")]
    gender: u8,
    #[serde(default = "default_shiny")]
    shiny: String,
    #[serde(default)]
    form: u8,
}

fn default_shiny() -> String {
    "Shiny::Random".to_string()
}

fn default_ability_gender() -> u8 {
    255
}

pub fn embed_encounters(mut resource_path: PathBuf) {
    let data = serde_json::from_str::<Encounters5>(include_str!("./encounters.json")).unwrap();

    resource_path.push("encounter_data_5.rs");
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(resource_path)
        .unwrap();
    let mut writer = BufWriter::new(file);

    writer
        .write_all(
            b"use crate::enums::{Game, Shiny};\nuse crate::parents::StaticTemplate;\nuse crate::gen5::DreamRadarTemplate;\n\n",
        )
        .unwrap();

    write_encounters("DREAM_RADAR", &mut writer, data.dream_radar);
    write_encounters("STARTERS", &mut writer, data.starters);
    write_encounters("FOSSILS", &mut writer, data.fossils);
    write_encounters("GIFTS", &mut writer, data.gifts);
    write_encounters("STATIONARY", &mut writer, data.stationary);
    write_encounters("LEGENDS", &mut writer, data.legends);
    write_encounters("EVENTS", &mut writer, data.events);
    write_encounters("ROAMERS", &mut writer, data.roamers);

    writer
        .write_all(b"pub(crate) static BLACK: &[u8] = include_bytes!(\"black.bin\");\n\n")
        .unwrap();
    writer
        .write_all(b"pub(crate) static BLACK2: &[u8] = include_bytes!(\"black2.bin\");\n\n")
        .unwrap();
    writer
        .write_all(b"pub(crate) static BW2_GROTTO: &[u8] = include_bytes!(\"bw2_grotto.bin\");\n\n")
        .unwrap();
    writer
        .write_all(b"pub(crate) static WHITE: &[u8] = include_bytes!(\"white.bin\");\n\n")
        .unwrap();
    writer
        .write_all(b"pub(crate) static WHITE2: &[u8] = include_bytes!(\"white2.bin\");\n\n")
        .unwrap();

    writer.flush().unwrap();
}

fn write_encounters(name: &str, writer: &mut BufWriter<File>, encounters: Vec<Encounter>) {
    writer
        .write_all(
            format!(
                "pub(crate) static {}: [{}; {}] = [ ",
                name,
                if name == "DREAM_RADAR" {
                    "DreamRadarTemplate"
                } else {
                    "StaticTemplate"
                },
                encounters.len()
            )
            .as_bytes(),
        )
        .unwrap();

    let len = encounters.len();

    for (i, encounter) in encounters.into_iter().enumerate() {
        if name == "DREAM_RADAR" {
            writer
                .write_all(
                    format!(
                        "DreamRadarTemplate::new({}, {}, {})",
                        encounter.specie, encounter.form, encounter.ability
                    )
                    .as_bytes(),
                )
                .unwrap();
        } else {
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
                        "StaticTemplate::new({}, {}, {}, {}, {}, {}, 0, {})",
                        version,
                        encounter.specie,
                        encounter.form,
                        encounter.shiny,
                        encounter.ability,
                        encounter.gender,
                        encounter.level,
                    )
                    .as_bytes(),
                )
                .unwrap();
        }

        if i != len - 1 {
            writer.write_all(b", ").unwrap();
        }
    }

    writer.write_all(b" ];\n\n").unwrap();
}
