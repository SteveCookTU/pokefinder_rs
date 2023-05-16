use crate::gen3::pack::pack_encounter_gen3;
use crate::gen3::text::{clean_string, load_pokemon};
use crate::gen3::WildEncounters;
use bzip2::write::BzEncoder;
use bzip2::Compression;
use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::blocking::Response;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

static ALTERING_CAVE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"AlteringCave_[2-9]").unwrap());

pub fn encounters(text: bool, mut resources_path: PathBuf) {
    const DATA: &str =
        "https://raw.githubusercontent.com/pret/pokefirered/master/src/data/wild_encounters.json";

    let data_raw = reqwest::blocking::get(DATA).unwrap();
    let data = serde_json::from_reader::<Response, WildEncounters>(data_raw).unwrap();

    let pokemon = load_pokemon();
    let encounters = &data.wild_encounter_groups[0].encounters;
    let mut fr = vec![];
    let mut map_names = vec![];
    for (map_number, encounter) in encounters
        .iter()
        .filter(|e| e.base_label.contains("FireRed"))
        .enumerate()
    {
        if ALTERING_CAVE_REGEX.is_match(&encounter.base_label) {
            continue;
        }

        let map_name = (map_number as u8, clean_string(encounter.map.to_string()));
        if !map_names.contains(&map_name) {
            map_names.push(map_name);
        }

        fr.push(map_number as u8);
        fr.extend(pack_encounter_gen3(encounter, &pokemon));
    }

    let mut lg = vec![];
    for (map_number, encounter) in encounters
        .iter()
        .filter(|e| e.base_label.contains("LeafGreen"))
        .enumerate()
    {
        if ALTERING_CAVE_REGEX.is_match(&encounter.base_label) {
            continue;
        }

        let map_name = (map_number as u8, clean_string(encounter.map.to_string()));
        if !map_names.contains(&map_name) {
            map_names.push(map_name);
        }

        lg.push(map_number as u8);
        lg.extend(pack_encounter_gen3(encounter, &pokemon));
    }

    let mut compressor = BzEncoder::new(vec![], Compression::best());
    compressor.write_all(&fr).unwrap();
    let data = compressor.finish().unwrap();

    resources_path.push("firered.bin");

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&resources_path)
        .unwrap();
    let mut writer = BufWriter::new(file);
    writer.write_all(&data).unwrap();

    compressor = BzEncoder::new(vec![], Compression::best());
    compressor.write_all(&lg).unwrap();
    let data = compressor.finish().unwrap();

    resources_path.pop();
    resources_path.push("leafgreen.bin");

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&resources_path)
        .unwrap();
    let mut writer = BufWriter::new(file);
    writer.write_all(&data).unwrap();

    if text {
        resources_path.pop();
        resources_path.push("frlg_en.txt");
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(resources_path)
            .unwrap();
        let mut writer = BufWriter::new(file);
        map_names.sort_by(|a, b| a.0.cmp(&b.0));
        for (i, (num, name)) in map_names.iter().enumerate() {
            writer
                .write_all(format!("{num},{name}").as_bytes())
                .unwrap();
            if i != map_names.len() - 1 {
                writer.write_all(b"\n").unwrap();
            }
        }
    }
}
