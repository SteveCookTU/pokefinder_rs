use crate::gen3::WildEncounter;
use once_cell::sync::Lazy;
use std::collections::HashMap;

static UNOWN: Lazy<HashMap<&'static str, [u8; 12]>> = Lazy::new(|| {
    let mut map = HashMap::new();

    map.insert(
        "MAP_SEVEN_ISLAND_TANOBY_RUINS_MONEAN_CHAMBER",
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27],
    );
    map.insert(
        "MAP_SEVEN_ISLAND_TANOBY_RUINS_LIPTOO_CHAMBER",
        [2, 2, 2, 3, 3, 3, 7, 7, 7, 20, 20, 14],
    );
    map.insert(
        "MAP_SEVEN_ISLAND_TANOBY_RUINS_WEEPTH_CHAMBER",
        [13, 13, 13, 13, 18, 18, 18, 18, 8, 8, 4, 4],
    );
    map.insert(
        "MAP_SEVEN_ISLAND_TANOBY_RUINS_DILFORD_CHAMBER",
        [15, 15, 11, 11, 9, 9, 17, 17, 17, 16, 16, 16],
    );
    map.insert(
        "MAP_SEVEN_ISLAND_TANOBY_RUINS_SCUFIB_CHAMBER",
        [24, 24, 19, 19, 6, 6, 6, 5, 5, 5, 10, 10],
    );
    map.insert(
        "MAP_SEVEN_ISLAND_TANOBY_RUINS_RIXY_CHAMBER",
        [21, 21, 21, 22, 22, 22, 23, 23, 12, 12, 1, 1],
    );
    map.insert(
        "MAP_SEVEN_ISLAND_TANOBY_RUINS_VIAPOIS_CHAMBER",
        [25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 26],
    );

    map
});

pub fn pack_encounter_gen3(encounter: &WildEncounter, pokemon: &HashMap<String, u16>) -> Vec<u8> {
    let mut data = Vec::with_capacity(133);

    if let Some(land) = &encounter.land_mons {
        data.push(land.encounter_rate);
    } else {
        data.push(0);
    }

    if let Some(water) = &encounter.water_mons {
        data.push(water.encounter_rate);
    } else {
        data.push(0);
    }

    if let Some(rock) = &encounter.rock_smash_mons {
        data.push(rock.encounter_rate);
    } else {
        data.push(0);
    }

    if let Some(fish) = &encounter.fishing_mons {
        data.push(fish.encounter_rate);
    } else {
        data.push(0);
    }

    data.push(0);

    if let Some(land) = &encounter.land_mons {
        for (i, slot) in land.mons.iter().enumerate() {
            let mut specie = *pokemon.get(&slot.species).unwrap();
            if let Some(entry) = UNOWN.get(encounter.map.as_str()) {
                let form = entry[i];
                specie |= (form as u16) << 11;
            }
            data.extend(specie.to_le_bytes());
            data.push(slot.min_level);
            data.push(0);
        }
    } else {
        data.extend([0; 12 * 4]);
    }

    if let Some(water) = &encounter.water_mons {
        for slot in water.mons.iter() {
            data.extend(pokemon.get(&slot.species).unwrap().to_le_bytes());
            data.push(slot.max_level);
            data.push(slot.min_level);
        }
    } else {
        data.extend([0; 5 * 4]);
    }

    if let Some(rock) = &encounter.rock_smash_mons {
        for slot in rock.mons.iter() {
            data.extend(pokemon.get(&slot.species).unwrap().to_le_bytes());
            data.push(slot.max_level);
            data.push(slot.min_level);
        }
    } else {
        data.extend([0; 5 * 4]);
    }

    if let Some(fish) = &encounter.fishing_mons {
        for slot in fish.mons.iter() {
            data.extend(pokemon.get(&slot.species).unwrap().to_le_bytes());
            data.push(slot.max_level);
            data.push(slot.min_level);
        }
    } else {
        data.extend([0; 10 * 4]);
    }

    data
}
