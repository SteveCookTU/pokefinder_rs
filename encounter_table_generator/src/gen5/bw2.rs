use crate::gen5::pack::pack_encounter_gen5;
use crate::gen5::text::read_map_names;
use crate::gen5::LocationModifiers;
use crate::narc::Narc;
use bzip2::write::BzEncoder;
use bzip2::Compression;
use no_std_io::{Cursor, StreamContainer, StreamReader};
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

pub fn encounters(text: bool, mut resources_path: PathBuf) {
    let b_encounters = Narc::new(StreamContainer::new(include_bytes!("./bw2/b2_encount")));
    let w_encounters = Narc::new(StreamContainer::new(include_bytes!("./bw2/w2_encount")));
    const MAP_HEADERS: &[u8] = include_bytes!("./bw2/mapheaders.bin");
    let all_map_names = read_map_names(include_bytes!("./bw2/mapnames.bin"));
    const LOCATION_MODIFIERS: &str = include_str!("./location_modifier.json");

    let mut map_headers = vec![];
    let mut reader = StreamContainer::new(MAP_HEADERS);
    for _ in 0..615 {
        map_headers.push(reader.read_byte_stream(48).unwrap());
    }

    let location_modifiers = serde_json::from_str::<LocationModifiers>(LOCATION_MODIFIERS)
        .unwrap()
        .bw2;

    let mut b = vec![];
    let mut w = vec![];
    let mut map_names = vec![];

    for map_header in map_headers {
        let encounter_id = map_header[20];
        if [
            84, 86, 44, 35, 105, 15, 16, 17, 19, 51, 53, 54, 55, 57, 59, 60, 56, 58, 78, 79, 88,
            89, 62, 63, 64, 66, 67, 68, 69, 70, 38, 39, 40, 41,
        ]
        .contains(&encounter_id)
        {
            continue;
        }

        if encounter_id != u8::MAX {
            let location_number = map_header[26];
            let mut location_name = all_map_names[location_number as usize].as_str();

            if let Some(location) = location_modifiers.get(location_name) {
                if let Some(&modifier) = location.get(encounter_id.to_string().as_str()) {
                    location_name = modifier;
                }
            }

            let map_name = (encounter_id, location_name);
            map_names.push(map_name);
            b.push(encounter_id);
            b.extend(pack_encounter_gen5(
                &b_encounters.elements[encounter_id as usize],
            ));

            w.push(encounter_id);
            w.extend(pack_encounter_gen5(
                &w_encounters.elements[encounter_id as usize],
            ));
        }
    }

    let mut compressor = BzEncoder::new(vec![], Compression::best());
    compressor.write_all(&b).unwrap();
    let data = compressor.finish().unwrap();

    resources_path.push("black2.bin");

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
    resources_path.push("white2.bin");

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&resources_path)
        .unwrap();
    let mut writer = BufWriter::new(file);
    writer.write_all(&data).unwrap();

    if text {
        map_names.push((135, "Route 6 (Cave)"));
        map_names.push((136, "Route 13 (Giant Chasm)"));
        map_names.push((137, "Abundant Shrine (Pond)"));
        map_names.push((138, "Route 3 (Pond)"));
        resources_path.pop();
        resources_path.push("bw2_en.txt");
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

pub fn hidden_grotto(mut resources_path: PathBuf) {
    let bw_encounters = Narc::new(StreamContainer::new(include_bytes!("./bw2/grotto")));
    let locations = [
        45, 106, 126, 107, 135, 111, 121, 136, 118, 34, 130, 131, 123, 137, 9, 8, 101, 138, 100,
        127,
    ];

    let mut bw = vec![];

    for (encounter, location) in bw_encounters
        .elements
        .into_iter()
        .zip(locations.into_iter())
    {
        let mut reader = StreamContainer::new(encounter);
        bw.push(location);
        bw.push(0u8);

        let mut species = [0; 12];
        let mut max_level = [0; 12];
        let mut min_level = [0; 12];
        let mut gender = [0; 12];
        let mut item = [0; 16];
        let mut hidden_item = [0; 16];

        for i in 0..3 {
            for j in 0..4 {
                species[i + j * 3] = reader.read_stream_le::<u16>().unwrap();
            }

            for j in 0..4 {
                max_level[i + j * 3] = reader.read_stream_le::<u8>().unwrap();
            }

            for j in 0..4 {
                min_level[i + j * 3] = reader.read_stream_le::<u8>().unwrap();
            }

            for j in 0..4 {
                gender[i + j * 3] = reader.read_stream_le::<u8>().unwrap();
            }

            let _form = reader.read_stream_le::<u32>().unwrap();
            let _padding = reader.read_stream_le::<u16>().unwrap();
        }

        reader.set_index(0x9c);
        for i in 0..4 {
            for j in 0..4 {
                item[i + j * 3] = reader.read_stream_le::<u16>().unwrap();
            }
        }

        for i in 0..4 {
            for j in 0..4 {
                hidden_item[i + j * 3] = reader.read_stream_le::<u16>().unwrap();
            }
        }

        for i in 0..12 {
            bw.extend(species[i].to_le_bytes());
            bw.push(max_level[i]);
            bw.push(min_level[i]);
            bw.push(gender[i]);
            bw.push(0);
        }

        for item in item {
            bw.extend(item.to_le_bytes());
        }

        for item in hidden_item {
            bw.extend(item.to_le_bytes());
        }
    }

    resources_path.push("bw2_grotto.bin");

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&resources_path)
        .unwrap();
    let mut writer = BufWriter::new(file);
    writer.write_all(&bw).unwrap();
}
