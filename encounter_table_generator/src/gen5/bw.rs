use crate::gen5::pack::pack_encounter_gen5;
use crate::gen5::text::read_map_names;
use crate::gen5::LocationModifiers;
use crate::narc::Narc;
use bzip2::write::BzEncoder;
use bzip2::Compression;
use no_std_io::{StreamContainer, StreamReader};
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

pub fn encounters(text: bool, mut resources_path: PathBuf) {
    let b_encounters = Narc::new(StreamContainer::new(include_bytes!("./bw/b_encount")));
    let w_encounters = Narc::new(StreamContainer::new(include_bytes!("./bw/w_encount")));
    const MAP_HEADERS: &[u8] = include_bytes!("./bw/mapheaders.bin");
    let all_map_names = read_map_names(include_bytes!("./bw/mapnames.bin"));
    const LOCATION_MODIFIERS: &str = include_str!("./location_modifier.json");

    let mut map_headers = vec![];
    let mut reader = StreamContainer::new(MAP_HEADERS);
    for _ in 0..427 {
        map_headers.push(reader.read_byte_stream(48).unwrap());
    }

    let location_modifiers = serde_json::from_str::<LocationModifiers>(LOCATION_MODIFIERS)
        .unwrap()
        .bw;

    let mut b = vec![];
    let mut w = vec![];
    let mut map_names = vec![];

    for map_header in map_headers {
        let encounter_id = (map_header[20] as u16) | ((map_header[21] as u16) << 8);
        if [
            16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 32, 31, 12, 13, 14, 33, 34, 35,
            36, 37, 10, 38, 39,
        ]
        .contains(&encounter_id)
        {
            continue;
        }

        if encounter_id != u16::MAX {
            let location_number = map_header[26] as u16;
            let mut location_name = all_map_names[location_number as usize].as_str();

            if let Some(location) = location_modifiers.get(location_name) {
                if let Some(&modifier) = location.get(encounter_id.to_string().as_str()) {
                    location_name = modifier;
                }
            }

            let map_name = (encounter_id, location_name);
            map_names.push(map_name);
            b.push(encounter_id as u8);
            b.extend(pack_encounter_gen5(
                &b_encounters.elements[encounter_id as usize],
            ));

            w.push(encounter_id as u8);
            w.extend(pack_encounter_gen5(
                &w_encounters.elements[encounter_id as usize],
            ));
        }
    }

    let mut compressor = BzEncoder::new(vec![], Compression::best());
    compressor.write_all(&b).unwrap();
    let data = compressor.finish().unwrap();

    resources_path.push("black.bin");

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&resources_path)
        .unwrap();
    let mut writer = BufWriter::new(file);
    writer.write_all(&data).unwrap();

    compressor = BzEncoder::new(vec![], Compression::best());
    compressor.write_all(&w).unwrap();
    let data = compressor.finish().unwrap();

    resources_path.pop();
    resources_path.push("white.bin");

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
        resources_path.push("bw_en.txt");
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
