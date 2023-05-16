use crate::gen8::area_name::AreaName;
use crate::gen8::field_encount_table::FieldEncountTable;
use crate::gen8::map_info::MapInfo;
use crate::gen8::pack::{pack_encounter_bdsp, pack_encounter_underground};
use crate::gen8::ug_encount::{UgEncount, UgEncountEntry};
use crate::gen8::ug_pokemon_data::UgPokemonData;
use crate::gen8::ug_rand_mark::UgRandMark;
use crate::gen8::ug_special_pokemon::UgSpecialPokemon;
use bzip2::write::BzEncoder;
use bzip2::Compression;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

#[derive(Deserialize)]
struct LocationModifiers<'a> {
    #[serde(borrow)]
    bdsp: HashMap<&'a str, HashMap<&'a str, &'a str>>,
}

pub fn encounters(text: bool, mut resources_path: PathBuf) {
    const D_ENCOUNTERS: &str = include_str!("./bdsp/FieldEncountTable_d.json");
    const P_ENCOUNTERS: &str = include_str!("./bdsp/FieldEncountTable_p.json");
    const MAP_INFO: &str = include_str!("./bdsp/MapInfo.json");
    const AREA_NAME: &str = include_str!("./bdsp/english_dp_fld_areaname.json");
    const LOCATION_MODIFIERS: &str = include_str!("./location_modifier.json");

    let d_encounters = serde_json::from_str::<FieldEncountTable>(D_ENCOUNTERS)
        .unwrap()
        .table;
    let p_encounters = serde_json::from_str::<FieldEncountTable>(P_ENCOUNTERS)
        .unwrap()
        .table;
    let map_info = serde_json::from_str::<MapInfo>(MAP_INFO).unwrap().zone_data;
    let area_name = serde_json::from_str::<AreaName>(AREA_NAME)
        .unwrap()
        .label_data_array;
    let location_modifiers = serde_json::from_str::<LocationModifiers>(LOCATION_MODIFIERS)
        .unwrap()
        .bdsp;

    let mut d = vec![];
    let mut p = vec![];
    let mut map_names = vec![];

    for (map_number, encounter) in d_encounters.into_iter().enumerate() {
        if [
            14, 126, 127, 128, 129, 130, 131, 132, 133, 64, 65, 66, 67, 68, 70, 71, 72, 73, 74, 76,
            77, 78, 79, 80, 31, 33, 35, 36, 37, 38, 39, 44, 45, 46,
        ]
        .contains(&map_number)
        {
            continue;
        }

        let zone_id = encounter.zone_id;
        let Some(zone_data) = map_info.iter().find(|z| z.zone_id == zone_id) else {
            continue;
        };

        let place_name = &zone_data.poke_place_name;
        let Some(label_data) = area_name.iter().find(|area| &area.label_name == place_name) else {
            continue;
        };

        let mut location_name = label_data.word_data_array[0].str.as_str();
        if let Some(location) = location_modifiers.get(location_name) {
            if let Some(modifier) = location.get(map_number.to_string().as_str()) {
                location_name = modifier;
            }
        }

        let map_name = (map_number, location_name);
        map_names.push(map_name);

        d.push(map_number as u8);
        d.extend(pack_encounter_bdsp(&encounter));
    }

    for (map_number, encounter) in p_encounters.into_iter().enumerate() {
        if [
            14, 126, 127, 128, 129, 130, 131, 132, 133, 64, 65, 66, 67, 68, 70, 71, 72, 73, 74, 76,
            77, 78, 79, 80, 31, 33, 35, 36, 37, 38, 39, 44, 45, 46,
        ]
        .contains(&map_number)
        {
            continue;
        }

        let zone_id = encounter.zone_id;
        let Some(zone_data) = map_info.iter().find(|z| z.zone_id == zone_id) else {
            continue;
        };

        let place_name = &zone_data.poke_place_name;
        if !area_name.iter().any(|area| &area.label_name == place_name) {
            continue;
        };

        p.push(map_number as u8);
        p.extend(pack_encounter_bdsp(&encounter));
    }

    let mut compressor = BzEncoder::new(vec![], Compression::best());
    compressor.write_all(&d).unwrap();
    let mut data = compressor.finish().unwrap();

    resources_path.push("bd.bin");

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&resources_path)
        .unwrap();
    let mut writer = BufWriter::new(file);
    writer.write_all(&data).unwrap();

    compressor = BzEncoder::new(vec![], Compression::best());
    compressor.write_all(&p).unwrap();
    data = compressor.finish().unwrap();

    resources_path.pop();
    resources_path.push("sp.bin");

    file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&resources_path)
        .unwrap();
    writer = BufWriter::new(file);
    writer.write_all(&data).unwrap();

    if text {
        resources_path.pop();
        resources_path.push("bdsp_en.txt");
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

pub fn underground(mut resources_path: PathBuf) {
    static ENCOUNT: Lazy<HashMap<&'static str, Vec<UgEncountEntry>>> = Lazy::new(|| {
        let mut map = HashMap::with_capacity(12);
        map.insert(
            "UgEncount_02",
            serde_json::from_str::<UgEncount>(include_str!("./bdsp/UgEncount_02.json"))
                .unwrap()
                .table,
        );
        map.insert(
            "UgEncount_03",
            serde_json::from_str::<UgEncount>(include_str!("./bdsp/UgEncount_02.json"))
                .unwrap()
                .table,
        );
        map.insert(
            "UgEncount_04",
            serde_json::from_str::<UgEncount>(include_str!("./bdsp/UgEncount_02.json"))
                .unwrap()
                .table,
        );
        map.insert(
            "UgEncount_05",
            serde_json::from_str::<UgEncount>(include_str!("./bdsp/UgEncount_02.json"))
                .unwrap()
                .table,
        );
        map.insert(
            "UgEncount_06",
            serde_json::from_str::<UgEncount>(include_str!("./bdsp/UgEncount_02.json"))
                .unwrap()
                .table,
        );
        map.insert(
            "UgEncount_07",
            serde_json::from_str::<UgEncount>(include_str!("./bdsp/UgEncount_02.json"))
                .unwrap()
                .table,
        );
        map.insert(
            "UgEncount_08",
            serde_json::from_str::<UgEncount>(include_str!("./bdsp/UgEncount_02.json"))
                .unwrap()
                .table,
        );
        map.insert(
            "UgEncount_09",
            serde_json::from_str::<UgEncount>(include_str!("./bdsp/UgEncount_02.json"))
                .unwrap()
                .table,
        );
        map.insert(
            "UgEncount_10",
            serde_json::from_str::<UgEncount>(include_str!("./bdsp/UgEncount_02.json"))
                .unwrap()
                .table,
        );
        map.insert(
            "UgEncount_11",
            serde_json::from_str::<UgEncount>(include_str!("./bdsp/UgEncount_02.json"))
                .unwrap()
                .table,
        );
        map.insert(
            "UgEncount_12",
            serde_json::from_str::<UgEncount>(include_str!("./bdsp/UgEncount_02.json"))
                .unwrap()
                .table,
        );
        map.insert(
            "UgEncount_20",
            serde_json::from_str::<UgEncount>(include_str!("./bdsp/UgEncount_02.json"))
                .unwrap()
                .table,
        );
        map
    });

    let pokemon_data =
        serde_json::from_str::<UgPokemonData>(include_str!("./bdsp/UgPokemonData.json"))
            .unwrap()
            .table;
    let rand_mark = serde_json::from_str::<UgRandMark>(include_str!("./bdsp/UgRandMark.json"))
        .unwrap()
        .table;
    let special_pokemon =
        serde_json::from_str::<UgSpecialPokemon>(include_str!("./bdsp/UgSpecialPokemon.json"))
            .unwrap()
            .sheet1;

    let mut d = vec![];
    let mut p = vec![];

    for room_id in 2..20 {
        let special_pokemon_room = special_pokemon
            .iter()
            .filter(|x| x.id == room_id)
            .collect::<Vec<_>>();

        let mut special_pokemon_rates_d = special_pokemon_room
            .iter()
            .filter_map(|x| {
                if x.version != 3 {
                    Some((x.dspecialrate, x.monsno))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        special_pokemon_rates_d.sort_by(|(a, _), (b, _)| b.cmp(a));

        let mut special_pokemon_rates_p = special_pokemon_room
            .iter()
            .filter_map(|x| {
                if x.version != 2 {
                    Some((x.pspecialrate, x.monsno))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        special_pokemon_rates_p.sort_by(|(a, _), (b, _)| b.cmp(a));

        let rand_mark_room = rand_mark.iter().find(|x| x.id == room_id).unwrap();
        let ug_encount = ENCOUNT.get(rand_mark_room.file_name.as_str()).unwrap();
        let enabled_pokemon_d = ug_encount
            .iter()
            .filter(|x| x.version != 3)
            .collect::<Vec<_>>();
        let enabled_pokemon_p = ug_encount
            .iter()
            .filter(|x| x.version != 2)
            .collect::<Vec<_>>();

        d.push(room_id);
        d.extend(pack_encounter_underground(
            rand_mark_room,
            special_pokemon_rates_d,
            enabled_pokemon_d,
            &pokemon_data,
        ));

        p.push(room_id);
        p.extend(pack_encounter_underground(
            rand_mark_room,
            special_pokemon_rates_p,
            enabled_pokemon_p,
            &pokemon_data,
        ));
    }

    let mut compressor = BzEncoder::new(vec![], Compression::best());
    compressor.write_all(&d).unwrap();
    let mut data = compressor.finish().unwrap();

    resources_path.push("bd_underground.bin");

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&resources_path)
        .unwrap();
    let mut writer = BufWriter::new(file);
    writer.write_all(&data).unwrap();

    compressor = BzEncoder::new(vec![], Compression::best());
    compressor.write_all(&p).unwrap();
    data = compressor.finish().unwrap();

    resources_path.pop();
    resources_path.push("sp_underground.bin");

    file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&resources_path)
        .unwrap();
    writer = BufWriter::new(file);
    writer.write_all(&data).unwrap();
}
