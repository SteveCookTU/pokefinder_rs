use crate::gen4::pack::{
    pack_encounter_hgss, pack_encounter_hgss_bug, pack_encounter_hgss_headbutt,
};
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
    let hg_encounters = Narc::new(StreamContainer::new(include_bytes!("./hgss/hg_encount")));
    let ss_encounters = Narc::new(StreamContainer::new(include_bytes!("./hgss/ss_encount")));
    const MAP_HEADERS: &[u8] = include_bytes!("./hgss/mapheaders.bin");
    let all_map_names = read_map_names(include_bytes!("./hgss/mapnames.bin"));
    const LOCATION_MODIFIERS: &str = include_str!("./location_modifier.json");

    let mut map_headers = vec![];
    let mut reader = StreamContainer::new(MAP_HEADERS);
    for _ in 0..540 {
        map_headers.push(reader.read_byte_stream(24).unwrap());
    }

    let location_modifiers = serde_json::from_str::<LocationModifiers>(LOCATION_MODIFIERS)
        .unwrap()
        .hgss;

    let mut hg = vec![];
    let mut ss = vec![];
    let mut map_names = vec![];

    for map_header in map_headers {
        let encounter_id = map_header[0];
        if [7, 31, 32, 33, 34, 35, 36, 37, 84, 107, 11, 12, 13].contains(&encounter_id) {
            continue;
        }

        if encounter_id != u8::MAX {
            let location_number = map_header[18];
            let mut location_name = all_map_names[location_number as usize].as_str();

            if let Some(location) = location_modifiers.get(location_name) {
                if let Some(&modifier) = location.get(encounter_id.to_string().as_str()) {
                    location_name = modifier;
                }
            }

            let map_name = (encounter_id, location_name);
            map_names.push(map_name);
            hg.push(encounter_id);
            hg.extend(pack_encounter_hgss(
                &hg_encounters.elements[encounter_id as usize],
            ));

            ss.push(encounter_id);
            ss.extend(pack_encounter_hgss(
                &ss_encounters.elements[encounter_id as usize],
            ));
        }
    }

    let mut compressor = BzEncoder::new(vec![], Compression::best());
    compressor.write_all(&hg).unwrap();
    let data = compressor.finish().unwrap();

    resources_path.push("heartgold.bin");

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&resources_path)
        .unwrap();
    let mut writer = BufWriter::new(file);
    writer.write_all(&data).unwrap();

    compressor = BzEncoder::new(vec![], Compression::best());
    compressor.write_all(&ss).unwrap();
    let data = compressor.finish().unwrap();

    resources_path.pop();
    resources_path.push("soulsilver.bin");

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&resources_path)
        .unwrap();
    let mut writer = BufWriter::new(file);
    writer.write_all(&data).unwrap();

    if text {
        map_names.push((142, "Bug Contest"));
        map_names.push((143, "Bug Contest (Tuesday)"));
        map_names.push((144, "Bug Contest (Thursday)"));
        map_names.push((145, "Bug Contest (Saturday)"));
        map_names.push((146, "Azalea Town"));
        map_names.push((147, "Pewter City"));
        map_names.push((148, "Safari Zone Gate"));
        map_names.push((149, "Safari Zone (Plains)"));
        map_names.push((150, "Safari Zone (Meadow)"));
        map_names.push((151, "Safari Zone (Savannah)"));
        map_names.push((152, "Safari Zone (Peak)"));
        map_names.push((153, "Safari Zone (Rocky Beach)"));
        map_names.push((154, "Safari Zone (Wetland)"));
        map_names.push((155, "Safari Zone (Forest)"));
        map_names.push((156, "Safari Zone (Swamp)"));
        map_names.push((157, "Safari Zone (Marshland)"));
        map_names.push((158, "Safari Zone (Wasteland)"));
        map_names.push((159, "Safari Zone (Mountain)"));
        map_names.push((160, "Safari Zone (Desert)"));

        resources_path.pop();
        resources_path.push("hgss_en.txt");
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

pub fn bug(mut resources_path: PathBuf) {
    const BUG_ENCOUNT: &[u8] = include_bytes!("./hgss/mushi_encount.bin");

    let bug = pack_encounter_hgss_bug(BUG_ENCOUNT);

    let mut compressor = BzEncoder::new(vec![], Compression::best());
    compressor.write_all(&bug).unwrap();
    let data = compressor.finish().unwrap();

    resources_path.push("hgss_bug.bin");

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&resources_path)
        .unwrap();
    let mut writer = BufWriter::new(file);
    writer.write_all(&data).unwrap();
}

pub fn headbutt(mut resources_path: PathBuf) {
    let hg_headbutt_encount = Narc::new(StreamContainer::new(include_bytes!("./hgss/hg_headbutt")))
        .elements
        .into_iter()
        .filter(|x| x.len() != 4)
        .collect::<Vec<_>>();
    let ss_headbutt_encount = Narc::new(StreamContainer::new(include_bytes!("./hgss/ss_headbutt")))
        .elements
        .into_iter()
        .filter(|x| x.len() != 4)
        .collect::<Vec<_>>();

    const LOCATIONS: [u8; 60] = [
        111, 112, 113, 114, 115, 116, 117, 118, 121, 92, 122, 123, 124, 125, 127, 129, 131, 103,
        104, 105, 1, 3, 4, 8, 17, 21, 22, 25, 26, 38, 39, 52, 57, 59, 67, 68, 95, 96, 147, 97, 98,
        99, 100, 0, 2, 5, 146, 27, 58, 85, 128, 24, 20, 137, 71, 102, 148, 136, 125, 87,
    ];

    let mut hg_headbutt = vec![];
    for (i, encounter) in hg_headbutt_encount.into_iter().enumerate() {
        if i == 58 {
            continue;
        }

        hg_headbutt.push(LOCATIONS[i]);
        hg_headbutt.extend(pack_encounter_hgss_headbutt(&encounter));
    }

    let mut ss_headbutt = vec![];
    for (i, encounter) in ss_headbutt_encount.into_iter().enumerate() {
        if i == 58 {
            continue;
        }

        ss_headbutt.push(LOCATIONS[i]);
        ss_headbutt.extend(pack_encounter_hgss_headbutt(&encounter));
    }

    let mut compressor = BzEncoder::new(vec![], Compression::best());
    compressor.write_all(&hg_headbutt).unwrap();
    let data = compressor.finish().unwrap();

    resources_path.push("hg_headbutt.bin");

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&resources_path)
        .unwrap();
    let mut writer = BufWriter::new(file);
    writer.write_all(&data).unwrap();

    compressor = BzEncoder::new(vec![], Compression::best());
    compressor.write_all(&ss_headbutt).unwrap();
    let data = compressor.finish().unwrap();

    resources_path.pop();
    resources_path.push("ss_headbutt.bin");

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&resources_path)
        .unwrap();
    let mut writer = BufWriter::new(file);
    writer.write_all(&data).unwrap();
}

pub fn safari(mut resources_path: PathBuf) {
    let safari_encount = Narc::new(StreamContainer::new(include_bytes!("./hgss/safari"))).elements;

    let mut safari = vec![];
    const LOCATION_START: u8 = 149;

    for (i, safari_encount) in safari_encount.into_iter().enumerate() {
        safari.push(LOCATION_START + (i as u8));

        let waterflag = [1, 4, 5, 7, 8].contains(&(i as u8));
        safari.push(u8::from(waterflag));

        let mut stream = StreamContainer::new(safari_encount.as_slice());
        let tall_grass_encounters = stream.read_stream_le::<u8>().unwrap();
        let surfing_encounters = stream.read_stream_le::<u8>().unwrap();
        let old_rod_encounters = stream.read_stream_le::<u8>().unwrap();
        let good_rod_encounters = stream.read_stream_le::<u8>().unwrap();
        let super_rod_encounters = stream.read_stream_le::<u8>().unwrap();
        let _padding = stream.read_stream_le::<[u8; 3]>().unwrap();

        let encounters = [
            tall_grass_encounters,
            surfing_encounters,
            old_rod_encounters,
            good_rod_encounters,
            super_rod_encounters,
        ];

        'enc: for encounter in encounters {
            for _ in 0..30 {
                let specie = stream.read_stream_le::<u16>().unwrap();
                let level = stream.read_stream_le::<u8>().unwrap();
                let _padding = stream.read_stream_le::<u8>();

                safari.extend(specie.to_le_bytes());
                safari.push(level);
                safari.push(0);
            }

            for _ in 0..(encounter * 3) {
                let specie = stream.read_stream_le::<u16>().unwrap();
                let level = stream.read_stream_le::<u8>().unwrap();
                let _padding = stream.read_stream_le::<u8>();

                safari.extend(specie.to_le_bytes());
                safari.push(level);
                safari.push(0);
            }

            let mut first_block_type = vec![];
            let mut first_block_quantity = vec![];
            let mut second_block_type = vec![];
            let mut second_block_quantity = vec![];

            for _ in 0..encounter {
                first_block_type.push(stream.read_stream_le::<u8>().unwrap());
                first_block_quantity.push(stream.read_stream_le::<u8>().unwrap());
                second_block_type.push(stream.read_stream_le::<u8>().unwrap());
                second_block_quantity.push(stream.read_stream_le::<u8>().unwrap());
            }

            for ty in first_block_type {
                safari.push(ty);
            }

            for quantity in first_block_quantity {
                safari.push(quantity);
            }

            for ty in second_block_type {
                safari.push(ty);
            }

            for quantity in second_block_quantity {
                safari.push(quantity);
            }

            if !waterflag {
                break 'enc;
            }
        }

        if !waterflag {
            safari.extend([0; 624]);
        }
    }

    let mut compressor = BzEncoder::new(vec![], Compression::best());
    compressor.write_all(&safari).unwrap();
    let data = compressor.finish().unwrap();

    resources_path.push("hgss_safari.bin");

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&resources_path)
        .unwrap();
    let mut writer = BufWriter::new(file);
    writer.write_all(&data).unwrap();
}
