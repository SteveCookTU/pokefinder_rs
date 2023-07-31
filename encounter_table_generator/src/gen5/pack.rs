use no_std_io::{EndianRead, StreamContainer, StreamReader};

#[derive(EndianRead, Default, Copy, Clone)]
struct DynamicSlot {
    specie: u16,
    min_level: u8,
    max_level: u8,
}

#[derive(EndianRead, Default, Copy, Clone)]
struct StaticSlot {
    species: u16,
    level: u8,
    _padding: u8,
}

#[derive(EndianRead, Default, Copy, Clone)]
struct EncounterSeason {
    grass_rate: u8,
    grass_double_rate: u8,
    grass_special_rate: u8,
    surf_rate: u8,
    surf_special_rate: u8,
    fish_rate: u8,
    fish_special_rate: u8,
    #[no_std_io(pad_before = 1)]
    grass: [StaticSlot; 12],
    grass_double: [StaticSlot; 12],
    grass_special: [StaticSlot; 12],
    surf: [DynamicSlot; 5],
    surf_special: [DynamicSlot; 5],
    fish: [DynamicSlot; 5],
    fish_special: [DynamicSlot; 5],
}

#[derive(EndianRead)]
struct Encounter {
    seasons: [EncounterSeason; 1],
}

#[derive(EndianRead)]
struct EncounterSeasons {
    seasons: [EncounterSeason; 4],
}

pub fn pack_encounter_gen5(encounter: &[u8]) -> Vec<u8> {
    let seasons = if encounter.len() == 232 {
        StreamContainer::new(encounter)
            .read_stream_le::<Encounter>()
            .unwrap()
            .seasons
            .to_vec()
    } else {
        StreamContainer::new(encounter)
            .read_stream_le::<EncounterSeasons>()
            .unwrap()
            .seasons
            .to_vec()
    };

    let mut data = vec![(seasons.len() as u8)];
    for season in seasons {
        data.push(season.grass_rate);
        data.push(season.grass_double_rate);
        data.push(season.grass_special_rate);
        data.push(season.surf_rate);
        data.push(season.surf_special_rate);
        data.push(season.fish_rate);
        data.push(season.fish_special_rate);

        for slot in season.grass {
            data.extend(&slot.species.to_le_bytes()[..2]);
            data.push(slot.level);
            data.push(0);
        }

        for slot in season.grass_double {
            data.extend(&slot.species.to_le_bytes()[..2]);
            data.push(slot.level);
            data.push(0);
        }

        for slot in season.grass_special {
            data.extend(&slot.species.to_le_bytes()[..2]);
            data.push(slot.level);
            data.push(0);
        }

        for slot in season.surf {
            data.extend(&slot.specie.to_le_bytes()[..2]);
            data.push(slot.max_level);
            data.push(slot.min_level);
        }

        for slot in season.surf_special {
            data.extend(&slot.specie.to_le_bytes()[..2]);
            data.push(slot.max_level);
            data.push(slot.min_level);
        }

        for slot in season.fish {
            data.extend(&slot.specie.to_le_bytes()[..2]);
            data.push(slot.max_level);
            data.push(slot.min_level);
        }

        for slot in season.fish_special {
            data.extend(&slot.specie.to_le_bytes()[..2]);
            data.push(slot.max_level);
            data.push(slot.min_level);
        }
    }

    data
}
