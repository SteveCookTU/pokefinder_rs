use bzip2::write::BzEncoder;
use bzip2::Compression;
use no_std_io::{Cursor, EndianRead, Error, ReadOutput, StreamContainer, StreamReader};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

static POKEMON: Lazy<HashMap<u16, u16>> = Lazy::new(|| {
    let mut pokemon = HashMap::with_capacity(9);
    pokemon.insert(27, 27);
    pokemon.insert(207, 207);
    pokemon.insert(332, 328);
    pokemon.insert(187, 187);
    pokemon.insert(231, 231);
    pokemon.insert(311, 283);
    pokemon.insert(41, 41);
    pokemon.insert(382, 304);
    pokemon.insert(194, 194);
    pokemon
});

#[derive(EndianRead, Default, Copy, Clone)]
struct Slot {
    min_level: u8,
    max_level: u8,
    specie: u16,
    _encounter_rate: u32,
    _snack_steps: u32,
}

#[derive(Default, Copy, Clone)]
struct Encounter {
    slots: [Slot; 3],
}

impl EndianRead for Encounter {
    fn try_read_le(_: &[u8]) -> Result<ReadOutput<Self>, Error> {
        unimplemented!()
    }

    fn try_read_be(bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        let mut reader = StreamContainer::new(bytes);
        let mut slots = [Slot::default(); 3];
        for slot in slots.iter_mut() {
            *slot = reader.read_stream_be::<Slot>()?;
        }

        Ok(ReadOutput::new(Self { slots }, reader.get_index()))
    }
}

pub fn encounters(text: bool, mut resources_path: PathBuf) {
    const XD_ENCOUNTERS: &[u8] = include_bytes!("./xd/pokespot.bin");

    let mut reader = StreamContainer::new(XD_ENCOUNTERS);
    let mut encounters = [Encounter::default(); 3];
    for encounter in encounters.iter_mut() {
        *encounter = reader.read_stream_be::<Encounter>().unwrap();
    }

    let mut xd = vec![];
    for (map_number, encounter) in encounters.into_iter().enumerate() {
        xd.push(map_number as u8);
        xd.push(0);

        for slot in encounter.slots {
            xd.extend(POKEMON.get(&slot.specie).unwrap().to_le_bytes());
            xd.push(slot.max_level);
            xd.push(slot.min_level);
        }
    }

    let mut compressor = BzEncoder::new(vec![], Compression::best());
    compressor.write_all(&xd).unwrap();
    let data = compressor.finish().unwrap();

    resources_path.push("xd.bin");

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
        resources_path.push("gales_en.txt");
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(resources_path)
            .unwrap();
        let mut writer = BufWriter::new(file);
        let map_names = [
            (0, "Rock Poke Spot"),
            (1, "Oasis Poke Spot"),
            (2, "Cave Poke Spot"),
        ];
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
