use crate::enums::{Encounter, Game};
use crate::gen3::{EncounterArea3, ShadowTemplate};
use crate::parents::personal_loader::get_personal_table;
use crate::parents::{EncounterArea, Slot, StaticTemplate};
use crate::resources::encounter_data_3::{
    CHANNEL, EMERALD, EVENTS, FIRERED, FOSSILS, GALES_COLO, GALES_COLO_SHADOW, GAME_CORNER, GIFTS,
    LEAFGREEN, LEGENDS, RUBY, SAPPHIRE, STARTERS, STATIONARY, XD,
};
use crate::util;
use crate::util::encounter_slot::{DynamicSlot, StaticSlot};
use no_std_io::{Cursor, EndianRead, Error, ReadOutput, StreamContainer, StreamReader};

struct WildEncounter3 {
    location: u8,
    grass_rate: u8,
    surf_rate: u8,
    rock_rate: u8,
    fish_rate: u8,
    grass: [StaticSlot; 12],
    surf: [DynamicSlot; 5],
    rock: [DynamicSlot; 5],
    old: [DynamicSlot; 2],
    good: [DynamicSlot; 3],
    super_rod: [DynamicSlot; 5],
}

impl EndianRead for WildEncounter3 {
    fn try_read_le(bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        let mut reader = StreamContainer::new(bytes);
        let location = reader.read_stream_le()?;
        let grass_rate = reader.read_stream_le()?;
        let surf_rate = reader.read_stream_le()?;
        let rock_rate = reader.read_stream_le()?;
        let fish_rate = reader.read_stream_le()?;

        let _padding = reader.read_stream_le::<u8>();

        let mut grass = [StaticSlot::default(); 12];
        for g in grass.iter_mut() {
            *g = reader.read_stream_le()?;
        }
        let mut surf = [DynamicSlot::default(); 5];
        for s in surf.iter_mut() {
            *s = reader.read_stream_le()?;
        }
        let mut rock = [DynamicSlot::default(); 5];
        for r in rock.iter_mut() {
            *r = reader.read_stream_le()?;
        }
        let mut old = [DynamicSlot::default(); 2];
        for o in old.iter_mut() {
            *o = reader.read_stream_le()?;
        }
        let mut good = [DynamicSlot::default(); 3];
        for g in good.iter_mut() {
            *g = reader.read_stream_le()?;
        }
        let mut super_rod = [DynamicSlot::default(); 5];
        for s in super_rod.iter_mut() {
            *s = reader.read_stream_le()?;
        }
        Ok(ReadOutput::new(
            Self {
                location,
                grass_rate,
                surf_rate,
                rock_rate,
                fish_rate,
                grass,
                surf,
                rock,
                old,
                good,
                super_rod,
            },
            reader.get_index(),
        ))
    }

    fn try_read_be(_: &[u8]) -> Result<ReadOutput<Self>, Error> {
        unimplemented!()
    }
}

struct WildEncounterPokeSpot {
    location: u8,
    spot: [DynamicSlot; 3],
}

impl EndianRead for WildEncounterPokeSpot {
    fn try_read_le(bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        let mut reader = StreamContainer::new(bytes);
        let location = reader.read_stream_le()?;
        let _padding = reader.read_stream_le::<u8>();
        let mut spot = [DynamicSlot::default(); 3];
        for s in spot.iter_mut() {
            *s = reader.read_stream_le()?;
        }
        Ok(ReadOutput::new(Self { location, spot }, reader.get_index()))
    }

    fn try_read_be(_: &[u8]) -> Result<ReadOutput<Self>, Error> {
        unimplemented!()
    }
}

pub fn get_encounters(encounter: Encounter, version: Game) -> Vec<EncounterArea3> {
    let compressed_data = if version == Game::EMERALD {
        &EMERALD[2..]
    } else if version == Game::FIRE_RED {
        &FIRERED[2..]
    } else if version == Game::LEAF_GREEN {
        &LEAFGREEN[2..]
    } else if version == Game::RUBY {
        &RUBY[2..]
    } else {
        &SAPPHIRE[2..]
    };

    let data = util::decompress(compressed_data);

    let info = get_personal_table(version);

    let mut encounters = vec![];

    let mut reader = StreamContainer::new(data);

    while let Ok(entry) = reader.read_stream_le::<WildEncounter3>() {
        match encounter {
            Encounter::Grass if entry.grass_rate != 0 => {
                let mut slots = Vec::with_capacity(12);
                for slot in entry.grass {
                    slots.push(Slot::new_with_form(
                        slot.specie & 0x7FF,
                        (slot.specie >> 11) as u8,
                        slot.level,
                        slot.level,
                        &info[(slot.specie as usize) & 0x7FF],
                    ));
                }
                encounters.push(EncounterArea3::new(
                    entry.location,
                    entry.grass_rate,
                    encounter,
                    slots,
                ));
            }
            Encounter::Surfing if entry.surf_rate != 0 => {
                let mut slots = Vec::with_capacity(5);
                for slot in entry.surf {
                    slots.push(Slot::new(
                        slot.specie,
                        slot.min_level,
                        slot.max_level,
                        &info[slot.specie as usize],
                    ));
                }
                encounters.push(EncounterArea3::new(
                    entry.location,
                    entry.surf_rate,
                    encounter,
                    slots,
                ));
            }
            Encounter::RockSmash if entry.rock_rate != 0 => {
                let mut slots = Vec::with_capacity(5);
                for slot in entry.rock {
                    slots.push(Slot::new(
                        slot.specie,
                        slot.min_level,
                        slot.max_level,
                        &info[slot.specie as usize],
                    ));
                }
                encounters.push(EncounterArea3::new(
                    entry.location,
                    entry.rock_rate,
                    encounter,
                    slots,
                ));
            }
            Encounter::OldRod if entry.fish_rate != 0 => {
                let mut slots = Vec::with_capacity(2);
                for slot in entry.old {
                    slots.push(Slot::new(
                        slot.specie,
                        slot.min_level,
                        slot.max_level,
                        &info[slot.specie as usize],
                    ));
                }
                encounters.push(EncounterArea3::new(
                    entry.location,
                    entry.fish_rate,
                    encounter,
                    slots,
                ));
            }
            Encounter::GoodRod if entry.fish_rate != 0 => {
                let mut slots = Vec::with_capacity(3);
                for slot in entry.good {
                    slots.push(Slot::new(
                        slot.specie,
                        slot.min_level,
                        slot.max_level,
                        &info[slot.specie as usize],
                    ));
                }
                encounters.push(EncounterArea3::new(
                    entry.location,
                    entry.fish_rate,
                    encounter,
                    slots,
                ));
            }
            Encounter::SuperRod if entry.fish_rate != 0 => {
                let mut slots = Vec::with_capacity(5);
                for slot in entry.super_rod {
                    slots.push(Slot::new(
                        slot.specie,
                        slot.min_level,
                        slot.max_level,
                        &info[slot.specie as usize],
                    ));
                }
                encounters.push(EncounterArea3::new(
                    entry.location,
                    entry.fish_rate,
                    encounter,
                    slots,
                ));
            }
            _ => {}
        }
    }

    encounters
}

pub fn get_poke_spot_encounters() -> Vec<EncounterArea> {
    let data = util::decompress(&XD[2..]);
    let info = get_personal_table(Game::GEN3);

    let mut reader = StreamContainer::new(data);

    let mut encounters = Vec::new();
    while let Ok(entry) = reader.read_stream_le::<WildEncounterPokeSpot>() {
        let mut slots = Vec::with_capacity(3);
        for slot in entry.spot {
            slots.push(Slot::new(
                slot.specie,
                slot.min_level,
                slot.max_level,
                &info[slot.specie as usize],
            ));
        }
        encounters.push(EncounterArea::new(
            entry.location,
            0,
            Encounter::Grass,
            slots,
        ));
    }

    encounters
}

pub fn get_shadow_teams() -> &'static [ShadowTemplate] {
    GALES_COLO_SHADOW.as_slice()
}

pub fn get_shadow_team(index: usize) -> &'static ShadowTemplate {
    &get_shadow_teams()[index]
}

pub fn get_static_encounters(ty: usize) -> &'static [StaticTemplate] {
    match ty {
        0 => STARTERS.as_slice(),
        1 => FOSSILS.as_slice(),
        2 => GIFTS.as_slice(),
        3 => GAME_CORNER.as_slice(),
        4 => STATIONARY.as_slice(),
        5 => LEGENDS.as_slice(),
        6 => EVENTS.as_slice(),
        7 => GALES_COLO.as_slice(),
        _ => CHANNEL.as_slice(),
    }
}

pub fn get_static_encounter(ty: usize, index: usize) -> &'static StaticTemplate {
    &get_static_encounters(ty)[index]
}
