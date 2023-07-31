use crate::enums::{Encounter, Game};
use crate::gen5::{
    DreamRadarTemplate, EncounterArea5, HiddenGrottoArea, HiddenGrottoSlot, Profile5,
};
use crate::parents::{personal_loader, Profile, Slot, StaticTemplate};
use crate::resources::encounter_data_5::{
    BLACK, BLACK2, BW2_GROTTO, DREAM_RADAR, EVENTS, FOSSILS, GIFTS, LEGENDS, ROAMERS, STARTERS,
    STATIONARY, WHITE, WHITE2,
};
use crate::util;
use crate::util::encounter_slot::{DynamicSlot, StaticSlot};
use no_std_io::{Cursor, EndianRead, Error, ReadOutput, StreamContainer, StreamReader};

#[derive(EndianRead, Copy, Clone, Default)]
struct GrottoSlot {
    specie: u16,
    max_level: u8,
    min_level: u8,
    gender: u8,
}

#[derive(EndianRead, Copy, Clone, Default)]
struct WildEncounter5Season {
    grass_rate: u8,
    grass_double_rate: u8,
    grass_special_rate: u8,
    surf_rate: u8,
    surf_special_rate: u8,
    fish_rate: u8,
    fish_special_rate: u8,
    grass: [StaticSlot; 12],
    grass_double: [StaticSlot; 12],
    grass_special: [StaticSlot; 12],
    surf: [DynamicSlot; 5],
    surf_special: [DynamicSlot; 5],
    fish: [DynamicSlot; 5],
    fish_special: [DynamicSlot; 5],
}

struct WildEncounter5 {
    location: u8,
    season_count: u8,
    seasons: Vec<WildEncounter5Season>,
}

impl EndianRead for WildEncounter5 {
    fn try_read_le(bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        let mut reader = StreamContainer::new(bytes);
        let location = reader.read_stream_le::<u8>()?;
        let season_count = reader.read_stream_le::<u8>()?;

        let mut seasons = vec![];
        for _ in 0..season_count {
            seasons.push(reader.read_stream_le::<WildEncounter5Season>()?);
        }

        Ok(ReadOutput::new(
            Self {
                location,
                season_count,
                seasons,
            },
            reader.get_index(),
        ))
    }

    fn try_read_be(_: &[u8]) -> Result<ReadOutput<Self>, Error> {
        unimplemented!()
    }
}

#[derive(EndianRead)]
struct WildEncounterGrotto {
    location: u8,
    pokemon: [GrottoSlot; 12],
    items: [u16; 16],
    hidden_items: [u16; 16],
}

pub fn get_dream_radar_encounters() -> &'static [DreamRadarTemplate] {
    &DREAM_RADAR
}

pub fn get_dream_radar_encounter(index: usize) -> &'static DreamRadarTemplate {
    &DREAM_RADAR[index]
}

pub fn get_hidden_grotto_encounters() -> Vec<HiddenGrottoArea> {
    let data = util::decompress(BW2_GROTTO);
    let info = personal_loader::get_personal_table(Game::BW2);

    let mut reader = StreamContainer::new(data);

    let mut encounters = vec![];

    while let Ok(entry) = reader.read_stream_le::<WildEncounterGrotto>() {
        let mut pokemon = [None; 12];
        pokemon.iter_mut().zip(entry.pokemon).for_each(|(p, slot)| {
            *p = Some(HiddenGrottoSlot::new(
                slot.specie,
                slot.gender,
                slot.min_level,
                slot.max_level,
                &info[slot.specie as usize],
            ));
        });

        let pokemon = pokemon.map(|p| p.unwrap());

        encounters.push(HiddenGrottoArea::new(
            entry.location,
            pokemon,
            entry.items,
            entry.hidden_items,
        ));
    }

    encounters
}

pub fn get_encounters(encounter: Encounter, season: u8, profile: &Profile5) -> Vec<EncounterArea5> {
    let version = profile.get_version();
    let compressed_data = match version {
        Game::BLACK => BLACK,
        Game::BLACK2 => BLACK2,
        Game::WHITE => WHITE,
        _ => WHITE2,
    };

    let data = util::decompress(compressed_data);
    let mut reader = StreamContainer::new(data);

    let mut encounters = vec![];

    while let Ok(entry) = reader.read_stream_le::<WildEncounter5>() {
        let entry_season = if season < entry.season_count {
            &entry.seasons[season as usize]
        } else {
            &entry.seasons[0]
        };

        match encounter {
            Encounter::Grass if entry_season.grass_rate != 0 => {
                let mut slots = Vec::with_capacity(12);
                for slot in &entry_season.grass {
                    slots.push(Slot::new_with_form(
                        slot.species & 0x7ff,
                        (slot.species >> 11) as u8,
                        slot.level,
                        slot.level,
                        personal_loader::get_personal_info(
                            version,
                            slot.species & 0x7ff,
                            (slot.species >> 11) as u8,
                        ),
                    ));
                }
                encounters.push(EncounterArea5::new(
                    entry.location,
                    entry_season.grass_rate,
                    encounter,
                    slots,
                ));
            }
            Encounter::DoubleGrass if entry_season.grass_double_rate != 0 => {
                let mut slots = Vec::with_capacity(12);
                for slot in &entry_season.grass_double {
                    slots.push(Slot::new_with_form(
                        slot.species & 0x7ff,
                        (slot.species >> 11) as u8,
                        slot.level,
                        slot.level,
                        personal_loader::get_personal_info(
                            version,
                            slot.species & 0x7ff,
                            (slot.species >> 11) as u8,
                        ),
                    ));
                }
                encounters.push(EncounterArea5::new(
                    entry.location,
                    entry_season.grass_double_rate,
                    encounter,
                    slots,
                ));
            }
            Encounter::SpecialGrass if entry_season.grass_special_rate != 0 => {
                let mut slots = Vec::with_capacity(12);
                for slot in &entry_season.grass_special {
                    slots.push(Slot::new_with_form(
                        slot.species & 0x7ff,
                        (slot.species >> 11) as u8,
                        slot.level,
                        slot.level,
                        personal_loader::get_personal_info(
                            version,
                            slot.species & 0x7ff,
                            (slot.species >> 11) as u8,
                        ),
                    ));
                }
                encounters.push(EncounterArea5::new(
                    entry.location,
                    entry_season.grass_special_rate,
                    encounter,
                    slots,
                ));
            }
            Encounter::Surfing if entry_season.surf_rate != 0 => {
                let mut slots = Vec::with_capacity(5);
                for slot in &entry_season.surf {
                    slots.push(Slot::new_with_form(
                        slot.species & 0x7ff,
                        (slot.species >> 11) as u8,
                        slot.min_level,
                        slot.max_level,
                        personal_loader::get_personal_info(
                            version,
                            slot.species & 0x7ff,
                            (slot.species >> 11) as u8,
                        ),
                    ));
                }
                encounters.push(EncounterArea5::new(
                    entry.location,
                    entry_season.surf_rate,
                    encounter,
                    slots,
                ));
            }
            Encounter::SpecialSurf if entry_season.surf_special_rate != 0 => {
                let mut slots = Vec::with_capacity(5);
                for slot in &entry_season.surf_special {
                    slots.push(Slot::new_with_form(
                        slot.species & 0x7ff,
                        (slot.species >> 11) as u8,
                        slot.min_level,
                        slot.max_level,
                        personal_loader::get_personal_info(
                            version,
                            slot.species & 0x7ff,
                            (slot.species >> 11) as u8,
                        ),
                    ));
                }
                encounters.push(EncounterArea5::new(
                    entry.location,
                    entry_season.surf_special_rate,
                    encounter,
                    slots,
                ));
            }
            Encounter::SuperRod if entry_season.fish_rate != 0 => {
                let mut slots = Vec::with_capacity(5);
                for slot in &entry_season.fish {
                    slots.push(Slot::new_with_form(
                        slot.species & 0x7ff,
                        (slot.species >> 11) as u8,
                        slot.min_level,
                        slot.max_level,
                        personal_loader::get_personal_info(
                            version,
                            slot.species & 0x7ff,
                            (slot.species >> 11) as u8,
                        ),
                    ));
                }
                encounters.push(EncounterArea5::new(
                    entry.location,
                    entry_season.fish_rate,
                    encounter,
                    slots,
                ));
            }
            Encounter::SpecialSuperRod if entry_season.fish_special_rate != 0 => {
                let mut slots = Vec::with_capacity(5);
                for slot in &entry_season.fish_special {
                    slots.push(Slot::new_with_form(
                        slot.species & 0x7ff,
                        (slot.species >> 11) as u8,
                        slot.min_level,
                        slot.max_level,
                        personal_loader::get_personal_info(
                            version,
                            slot.species & 0x7ff,
                            (slot.species >> 11) as u8,
                        ),
                    ));
                }
                encounters.push(EncounterArea5::new(
                    entry.location,
                    entry_season.fish_special_rate,
                    encounter,
                    slots,
                ));
            }
            _ => {}
        }
    }

    encounters
}

pub fn get_static_encounters(index: usize) -> &'static [StaticTemplate] {
    match index {
        0 => STARTERS.as_slice(),
        1 => FOSSILS.as_slice(),
        2 => GIFTS.as_slice(),
        3 => STATIONARY.as_slice(),
        4 => LEGENDS.as_slice(),
        5 => EVENTS.as_slice(),
        _ => ROAMERS.as_slice(),
    }
}

pub fn get_static_encounter(ty: usize, index: usize) -> &'static StaticTemplate {
    let template = get_static_encounters(ty);
    &template[index]
}
