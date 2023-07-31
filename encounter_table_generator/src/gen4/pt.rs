use crate::gen4::pack::pack_encounter_dppt;
use crate::narc::Narc;
use bzip2::write::BzEncoder;
use bzip2::Compression;
use no_std_io::{StreamContainer, StreamReader};
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

pub fn encounters(mut resources_path: PathBuf) {
    let pt_encounters = Narc::new(StreamContainer::new(include_bytes!(
        "./pt/pl_enc_data.narc"
    )));
    const MAP_HEADERS: &[u8] = include_bytes!("./pt/mapheaders.bin");

    let mut map_headers = vec![];
    let mut reader = StreamContainer::new(MAP_HEADERS);
    for _ in 0..593 {
        map_headers.push(reader.read_byte_stream(24).unwrap());
    }

    let mut pt = vec![];

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
            pt.push(encounter_id as u8);
            pt.extend(pack_encounter_dppt(
                &pt_encounters.elements[encounter_id as usize],
            ));
        }
    }

    let mut compressor = BzEncoder::new(vec![], Compression::best());
    compressor.write_all(&pt).unwrap();
    let data = compressor.finish().unwrap();

    resources_path.push("platinum.bin");

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&resources_path)
        .unwrap();
    let mut writer = BufWriter::new(file);
    writer.write_all(&data).unwrap();
}
