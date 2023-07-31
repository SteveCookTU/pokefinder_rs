use crate::gen4::pack::pack_encounter_dppt;
use crate::gen4::text::read_map_names;
use crate::gen4::LocationModifiers;
use crate::narc::Narc;
use bzip2::write::BzEncoder;
use bzip2::Compression;
use no_std_io::{StreamContainer, StreamReader};
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

pub fn encounters(text: bool, mut resources_path: PathBuf) {
    let d_encounters = Narc::new(StreamContainer::new(include_bytes!("./dp/d_enc_data.narc")));
    let p_encounters = Narc::new(StreamContainer::new(include_bytes!("./dp/p_enc_data.narc")));
    const MAP_HEADERS: &[u8] = include_bytes!("./dp/mapheaders.bin");
    let all_map_names = read_map_names(include_bytes!("./dp/mapnames.bin"));
    const LOCATION_MODIFIERS: &str = include_str!("./location_modifier.json");

    let mut map_headers = vec![];
    let mut reader = StreamContainer::new(MAP_HEADERS);
    for _ in 0..559 {
        map_headers.push(reader.read_byte_stream(24).unwrap());
    }

    let location_modifiers = serde_json::from_str::<LocationModifiers>(LOCATION_MODIFIERS)
        .unwrap()
        .dppt;

    let mut d = vec![];
    let mut p = vec![];
    let mut map_names = vec![];

    for map_header in map_headers {
        let encounter_id = (map_header[14] as u16) | ((map_header[15] as u16) << 8);
        if [
            14, 126, 127, 128, 129, 130, 131, 133, 64, 65, 66, 67, 68, 70, 71, 72, 73, 74, 76, 77,
            78, 79, 80, 31, 33, 35, 36, 37, 38, 39, 44, 45, 46,
        ]
        .contains(&encounter_id)
        {
            continue;
        }

        if encounter_id != u16::MAX {
            let location_number = (map_header[18] as u16) | ((map_header[19] as u16) << 8);
            let mut location_name = all_map_names[location_number as usize].as_str();

            if let Some(location) = location_modifiers.get(location_name) {
                if let Some(&modifier) = location.get(encounter_id.to_string().as_str()) {
                    location_name = modifier;
                }
            }

            let map_name = (encounter_id, location_name);
            map_names.push(map_name);
            d.push(encounter_id as u8);
            d.extend(pack_encounter_dppt(
                &d_encounters.elements[encounter_id as usize],
            ));

            p.push(encounter_id as u8);
            p.extend(pack_encounter_dppt(
                &p_encounters.elements[encounter_id as usize],
            ));
        }
    }

    let mut compressor = BzEncoder::new(vec![], Compression::best());
    compressor.write_all(&d).unwrap();
    let data = compressor.finish().unwrap();

    resources_path.push("diamond.bin");

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&resources_path)
        .unwrap();
    let mut writer = BufWriter::new(file);
    writer.write_all(&data).unwrap();

    compressor = BzEncoder::new(vec![], Compression::best());
    compressor.write_all(&p).unwrap();
    let data = compressor.finish().unwrap();

    resources_path.pop();
    resources_path.push("pearl.bin");

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
        resources_path.push("dppt_en.txt");
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
