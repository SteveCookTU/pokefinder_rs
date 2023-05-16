use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use serde::Deserialize;
use crate::gen8::nests::Nests;

#[derive(Deserialize)]
pub struct Encounter {
    pub version: String,
    pub specie: u16,
    pub level: u8,
    pub form: Option<u8>,
    pub gender: Option<u8>,
    pub ability: Option<u8>,
    pub shiny: Option<String>,
    #[serde(rename = "ivCount")]
    pub iv_count: Option<u8>,
}

#[derive(Deserialize)]
pub struct Encounters8 {
    pub starters: Vec<Encounter>,
    pub gifts: Vec<Encounter>,
    pub fossils: Vec<Encounter>,
    pub stationary: Vec<Encounter>,
    pub roamers: Vec<Encounter>,
    pub legends: Vec<Encounter>,
    #[serde(rename = "ramanasParkPureSpace")]
    pub ramanas_park_pure_space: Vec<Encounter>,
    #[serde(rename = "ramanasParkStrangeSpace")]
    pub ramanas_park_strange_space: Vec<Encounter>,
    pub mythics: Vec<Encounter>,
}

pub fn embed_encounters(mut resource_path: PathBuf) {
    let data = serde_json::from_str::<Encounters8>(include_str!("./encounters.json")).unwrap();

    resource_path.push("encounter_data_8.rs");
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(resource_path)
        .unwrap();
    let mut writer = BufWriter::new(file);

    writer.write_all(b"use crate::enums::{Game, Shiny};\nuse crate::gen8::{Den, Raid};\nuse crate::parents::StaticTemplate;\n\n").unwrap();

    write_encounters("STARTERS", &mut writer, data.starters);
    write_encounters("GIFTS", &mut writer, data.gifts);
    write_encounters("FOSSILS", &mut writer, data.fossils);
    write_encounters("STATIONARY", &mut writer, data.stationary);
    write_encounters("ROAMERS", &mut writer, data.roamers);
    write_encounters("LEGENDS", &mut writer, data.legends);
    write_encounters("RAMANASPARKPURESPACE", &mut writer, data.ramanas_park_pure_space);
    write_encounters("RAMANASPARKSTRANGESPACE", &mut writer, data.ramanas_park_strange_space);
    write_encounters("MYTHICS", &mut writer, data.mythics);
    write_nests(&mut writer);

    writer
        .write_all(b"pub(crate) static BD: &[u8] = include_bytes!(\"bd.bin\");\n\n")
        .unwrap();
    writer
        .write_all(b"pub(crate) static BD_UNDERGROUND: &[u8] = include_bytes!(\"bd_underground.bin\");\n\n")
        .unwrap();
    writer
        .write_all(b"pub(crate) static SP: &[u8] = include_bytes!(\"sp.bin\");\n\n")
        .unwrap();
    writer
        .write_all(b"pub(crate) static SP_UNDERGROUND: &[u8] = include_bytes!(\"sp_underground.bin\");\n\n")
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
        writer
            .write_all(
                format!(
                    "StaticTemplate::new({}, {}, {}, {}, {}, {}, {}, {})",
                    encounter.version,
                    encounter.specie,
                    encounter.form.unwrap_or_default(),
                    encounter.shiny.unwrap_or("Shiny::Random".to_string()),
                    encounter.ability.unwrap_or(255),
                    encounter.gender.unwrap_or(255),
                    encounter.iv_count.unwrap_or_default(),
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

fn write_nests(writer: &mut BufWriter<File>) {
    let mut tables = serde_json::from_str::<Nests>(include_str!("./nests.json")).unwrap().tables;

    let len = tables.len();

    writer.write_all(format!("pub(crate) static NESTS: [Den; {}] = [ ", tables.len()).as_bytes()).unwrap();
    tables.sort_by(|a, b| u64::from_str_radix(&a.table_id, 16).unwrap().cmp(&u64::from_str_radix(&b.table_id, 16).unwrap()));

    for (i, table) in tables.into_iter().enumerate() {
        writer.write_all(format!("Den::new(0x{}, [", table.table_id).as_bytes()).unwrap();
        let sword = table.sword_entries;
        let mut len2 = sword.len();
        for (j, raid) in sword.into_iter().enumerate() {
            writer.write_all(format!("Raid::new({}, {}, Shiny::Random, {}, {}, {}, {}, [{}, {}, {}, {}, {}])", raid.species, raid.alt_form, raid.ability, raid.gender, raid.flawless_ivs, raid.is_gigantamax, raid.stars[0], raid.stars[1], raid.stars[2], raid.stars[3], raid.stars[4]).as_bytes()).unwrap();
            if j != len2 - 1 {
                writer.write_all(b", ").unwrap();
            }
        }
        writer.write_all(b"], [").unwrap();
        let shield = table.shield_entries;
        len2 = shield.len();
        for (j, raid) in shield.into_iter().enumerate() {
            writer.write_all(format!("Raid::new({}, {}, Shiny::Random, {}, {}, {}, {}, [{}, {}, {}, {}, {}])", raid.species, raid.alt_form, raid.ability, raid.gender, raid.flawless_ivs, raid.is_gigantamax, raid.stars[0], raid.stars[1], raid.stars[2], raid.stars[3], raid.stars[4]).as_bytes()).unwrap();
            if j != len2 - 1 {
                writer.write_all(b", ").unwrap();
            }
        }
        writer.write_all(b"])").unwrap();
        if i != len - 1 {
            writer.write_all(b", ").unwrap();
        }
    }
    writer.write_all(b"];").unwrap();
}