use crate::enums::{Encounter, Game};
use crate::gen8::{EncounterArea8, Pokemon, Profile8, SpecialPokemon, TypeSize, UndergroundArea};
use crate::parents::{personal_loader, PersonalInfo, Profile, Slot, StaticTemplate};
use crate::resources::encounter_data_8::{
    BD, BD_UNDERGROUND, FOSSILS, GIFTS, LEGENDS, MYTHICS, RAMANASPARKPURESPACE,
    RAMANASPARKSTRANGESPACE, ROAMERS, SP, SP_UNDERGROUND, STARTERS, STATIONARY,
};
use crate::util;
use crate::util::encounter_slot::{DynamicSlot, StaticSlot};
use no_std_io::{Cursor, EndianRead, Error, ReadOutput, StreamContainer, StreamReader};

const GREAT_MARSH: [u16; 12] = [55, 183, 194, 195, 298, 315, 397, 399, 400, 451, 453, 455];
const GREAT_MARSH_DEX: [u16; 14] = [
    46, 55, 102, 115, 193, 285, 315, 316, 397, 451, 452, 453, 454, 455,
];

const TROPHY_GARDEN: [u16; 16] = [
    35, 39, 52, 113, 133, 137, 173, 174, 183, 298, 311, 312, 351, 438, 439, 440,
];

#[derive(EndianRead)]
struct UndergroundSlot {
    specie: u16,
    flag_rates: [u8; 6],
    flag: u8,
    rate_up: u8,
    size: u8,
    _padding: u8,
}

struct WildEncounter8 {
    location: u8,
    grass_rate: u8,
    surf_rate: u8,
    old_rate: u8,
    good_rate: u8,
    super_rate: u8,
    grass: [StaticSlot; 12],
    swarm: [u16; 2],
    day: [u16; 2],
    night: [u16; 2],
    radar: [u16; 2],
    surf: [DynamicSlot; 5],
    old_rod: [DynamicSlot; 5],
    good_rod: [DynamicSlot; 5],
    super_rod: [DynamicSlot; 5],
}

impl EndianRead for WildEncounter8 {
    fn try_read_le(bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        let mut reader = StreamContainer::new(bytes);
        let location = reader.read_stream_le::<u8>()?;
        let grass_rate = reader.read_stream_le::<u8>()?;
        let surf_rate = reader.read_stream_le::<u8>()?;
        let old_rate = reader.read_stream_le::<u8>()?;
        let good_rate = reader.read_stream_le::<u8>()?;
        let super_rate = reader.read_stream_le::<u8>()?;
        let mut grass = [StaticSlot::default(); 12];
        for s in grass.iter_mut() {
            *s = reader.read_stream_le::<StaticSlot>()?;
        }
        let swarm = [
            reader.read_stream_le::<u16>()?,
            reader.read_stream_le::<u16>()?,
        ];
        let day = [
            reader.read_stream_le::<u16>()?,
            reader.read_stream_le::<u16>()?,
        ];
        let night = [
            reader.read_stream_le::<u16>()?,
            reader.read_stream_le::<u16>()?,
        ];
        let radar = [
            reader.read_stream_le::<u16>()?,
            reader.read_stream_le::<u16>()?,
        ];
        let mut surf = [DynamicSlot::default(); 5];
        for s in surf.iter_mut() {
            *s = reader.read_stream_le::<DynamicSlot>()?;
        }
        let mut old_rod = [DynamicSlot::default(); 5];
        for s in old_rod.iter_mut() {
            *s = reader.read_stream_le::<DynamicSlot>()?;
        }
        let mut good_rod = [DynamicSlot::default(); 5];
        for s in good_rod.iter_mut() {
            *s = reader.read_stream_le::<DynamicSlot>()?;
        }
        let mut super_rod = [DynamicSlot::default(); 5];
        for s in super_rod.iter_mut() {
            *s = reader.read_stream_le::<DynamicSlot>()?;
        }

        Ok(ReadOutput::new(
            WildEncounter8 {
                location,
                grass_rate,
                surf_rate,
                old_rate,
                good_rate,
                super_rate,
                grass,
                swarm,
                day,
                night,
                radar,
                surf,
                old_rod,
                good_rod,
                super_rod,
            },
            reader.get_index(),
        ))
    }

    fn try_read_be(_: &[u8]) -> Result<ReadOutput<Self>, Error> {
        unimplemented!()
    }
}

struct WildEncounterUnderground {
    location: u8,
    min_pokemon: u8,
    max_pokemon: u8,
    type_rates: [u8; 18],
    special_pokemon: Vec<SpecialPokemon>,
    pokemon_slots: Vec<UndergroundSlot>,
}

impl EndianRead for WildEncounterUnderground {
    fn try_read_le(bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        let mut reader = StreamContainer::new(bytes);
        let location = reader.read_stream_le::<u8>()?;
        let min_pokemon = reader.read_stream_le::<u8>()?;
        let max_pokemon = reader.read_stream_le::<u8>()?;
        let pokemon_count = reader.read_stream_le::<u8>()?;
        let special_rate_count = reader.read_stream_le::<u8>()?;
        let type_rates = reader.read_stream_le::<[u8; 18]>()?;
        let mut special_pokemon = Vec::with_capacity(special_rate_count as usize);
        for _ in 0..special_rate_count {
            special_pokemon.push(reader.read_stream_le::<SpecialPokemon>()?);
        }
        let mut pokemon_slots = Vec::with_capacity(pokemon_count as usize);
        for _ in 0..pokemon_count {
            pokemon_slots.push(reader.read_stream_le::<UndergroundSlot>()?);
        }

        Ok(ReadOutput::new(
            WildEncounterUnderground {
                location,
                min_pokemon,
                max_pokemon,
                type_rates,
                special_pokemon,
                pokemon_slots,
            },
            reader.get_index(),
        ))
    }

    fn try_read_be(_: &[u8]) -> Result<ReadOutput<Self>, Error> {
        unimplemented!()
    }
}

fn modify_great_marsh(
    pokemon: &mut [Slot],
    replacement: [u16; 2],
    info: &'static [PersonalInfo],
    location: u8,
) {
    if (23..=28).contains(&location) && replacement[0] != 0 {
        let specie = replacement[0];
        pokemon[6].set_specie(specie, &info[specie as usize]);
        pokemon[7].set_specie(specie, &info[specie as usize]);
    }
}

fn modify_radar(
    mons: &mut [Slot],
    entry: &WildEncounter8,
    info: &'static [PersonalInfo],
    radar: bool,
) {
    if radar {
        mons[4].set_specie(entry.radar[0], &info[entry.radar[0] as usize]);
        mons[5].set_specie(entry.radar[1], &info[entry.radar[1] as usize]);
        mons[10].set_specie(entry.radar[2], &info[entry.radar[2] as usize]);
        mons[11].set_specie(entry.radar[3], &info[entry.radar[3] as usize]);
    }
}

fn modify_swarm(
    mons: &mut [Slot],
    entry: &WildEncounter8,
    info: &'static [PersonalInfo],
    swarm: bool,
) {
    if swarm {
        mons[0].set_specie(entry.swarm[0], &info[entry.swarm[0] as usize]);
        mons[1].set_specie(entry.swarm[1], &info[entry.swarm[1] as usize]);
    }
}

fn modify_time(mons: &mut [Slot], entry: &WildEncounter8, info: &'static [PersonalInfo], time: u8) {
    let (specie1, specie2) = match time {
        1 => (entry.day[0], entry.day[1]),
        2 => (entry.night[0], entry.night[1]),
        _ => return,
    };

    mons[2].set_specie(specie1, &info[specie1 as usize]);
    mons[3].set_specie(specie2, &info[specie2 as usize]);
}

fn modify_trophy_garden(
    mons: &mut [Slot],
    replacement: [u16; 2],
    info: &'static [PersonalInfo],
    location: u8,
) {
    if location == 117 && !replacement.contains(&0) {
        let specie1 = replacement[0];
        let specie2 = replacement[1];
        mons[6].set_specie(specie1, &info[specie1 as usize]);
        mons[7].set_specie(specie2, &info[specie2 as usize]);
    }
}

fn get_bdsp(
    encounter: Encounter,
    time: u8,
    radar: bool,
    swarm: bool,
    version: Game,
    replacement: [u16; 2],
    info: &'static [PersonalInfo],
) -> Vec<EncounterArea8> {
    let compressed_data = if version == Game::BD {
        &BD[2..]
    } else {
        &SP[2..]
    };

    let data = util::decompress(compressed_data);
    let mut reader = StreamContainer::new(data);
    let mut encounters = vec![];
    while let Ok(entry) = reader.read_stream_le::<WildEncounter8>() {
        match encounter {
            Encounter::Grass if entry.grass_rate != 0 => {
                let mut slots = Vec::with_capacity(12);
                for slot in entry.grass {
                    slots.push(Slot::new(
                        slot.specie,
                        slot.level,
                        slot.level,
                        &info[slot.specie as usize],
                    ));
                }
                modify_swarm(&mut slots, &entry, info, swarm);
                modify_time(&mut slots, &entry, info, time);
                modify_radar(&mut slots, &entry, info, radar);
                modify_great_marsh(&mut slots, replacement, info, entry.location);
                modify_trophy_garden(&mut slots, replacement, info, entry.location);
                encounters.push(EncounterArea8::new(
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
                encounters.push(EncounterArea8::new(
                    entry.location,
                    entry.surf_rate,
                    encounter,
                    slots,
                ));
            }
            Encounter::OldRod if entry.old_rate != 0 => {
                let mut slots = Vec::with_capacity(5);
                for slot in entry.old_rod {
                    slots.push(Slot::new(
                        slot.specie,
                        slot.min_level,
                        slot.max_level,
                        &info[slot.specie as usize],
                    ));
                }
                encounters.push(EncounterArea8::new(
                    entry.location,
                    entry.surf_rate,
                    encounter,
                    slots,
                ));
            }
            Encounter::GoodRod if entry.good_rate != 0 => {
                let mut slots = Vec::with_capacity(5);
                for slot in entry.good_rod {
                    slots.push(Slot::new(
                        slot.specie,
                        slot.min_level,
                        slot.max_level,
                        &info[slot.specie as usize],
                    ));
                }
                encounters.push(EncounterArea8::new(
                    entry.location,
                    entry.surf_rate,
                    encounter,
                    slots,
                ));
            }
            Encounter::SuperRod if entry.super_rate != 0 => {
                let mut slots = Vec::with_capacity(5);
                for slot in entry.super_rod {
                    slots.push(Slot::new(
                        slot.specie,
                        slot.min_level,
                        slot.max_level,
                        &info[slot.specie as usize],
                    ));
                }
                encounters.push(EncounterArea8::new(
                    entry.location,
                    entry.surf_rate,
                    encounter,
                    slots,
                ));
            }
            _ => {}
        }
    }

    encounters
}

pub fn get_encounters(
    encounter: Encounter,
    time: u8,
    radar: bool,
    swarm: bool,
    replacement: [u16; 2],
    profile: &Profile8,
) -> Vec<EncounterArea8> {
    let info = personal_loader::get_personal_table(profile.get_version());
    get_bdsp(
        encounter,
        time,
        radar,
        swarm,
        profile.get_version(),
        replacement,
        info,
    )
}

pub fn get_great_marsh_pokemon(profile: &Profile8) -> &[u16] {
    if profile.get_national_dex() {
        &GREAT_MARSH_DEX
    } else {
        &GREAT_MARSH
    }
}

pub fn get_static_encounters(index: usize) -> &'static [StaticTemplate] {
    match index {
        0 => STARTERS.as_slice(),
        1 => GIFTS.as_slice(),
        2 => FOSSILS.as_slice(),
        3 => STATIONARY.as_slice(),
        4 => ROAMERS.as_slice(),
        5 => LEGENDS.as_slice(),
        6 => RAMANASPARKPURESPACE.as_slice(),
        7 => RAMANASPARKSTRANGESPACE.as_slice(),
        _ => MYTHICS.as_slice(),
    }
}

pub fn get_static_encounter(ty: usize, index: usize) -> &'static StaticTemplate {
    &get_static_encounters(ty)[index]
}

pub fn get_trophy_garden_pokemon() -> [u16; 16] {
    TROPHY_GARDEN
}

pub fn get_underground_encounters(
    story_flag: u8,
    diglett: bool,
    profile: &Profile8,
) -> Vec<UndergroundArea> {
    let version = profile.get_version();
    let compressed_data = if version == Game::BD {
        &BD_UNDERGROUND[2..]
    } else {
        &SP_UNDERGROUND[2..]
    };

    let data = util::decompress(compressed_data);
    let mut reader = StreamContainer::new(data);
    let mut encounters = vec![];
    let base = personal_loader::get_personal_table(version);
    while let Ok(entry) = reader.read_stream_le::<WildEncounterUnderground>() {
        let mut pokemon = vec![];
        let mut types = vec![];
        for pokemon_slot in entry.pokemon_slots {
            if pokemon_slot.flag <= story_flag {
                let specie = pokemon_slot.specie;
                let info = &base[specie as usize];

                let type_count = if info.get_type(0) == info.get_type(1) {
                    1
                } else {
                    2
                };
                for j in 0..type_count {
                    let ty = info.get_type(j);
                    let value = ty as u16 + 10u16.pow(pokemon_slot.size as u32);
                    let type_size = TypeSize {
                        value,
                        size: pokemon_slot.size,
                        ty,
                    };
                    types.push(type_size);
                }

                let flag_rate = if diglett {
                    (pokemon_slot.flag_rates[(story_flag - 1) as usize] as u16)
                        * pokemon_slot.rate_up as u16
                } else {
                    pokemon_slot.flag_rates[(story_flag - 1) as usize] as u16
                };

                let mon = Pokemon {
                    rate: flag_rate,
                    specie,
                    size: pokemon_slot.size,
                    ty: [info.get_type(0), info.get_type(1)],
                };
                pokemon.push(mon);
            }
        }

        encounters.push(UndergroundArea::new(
            entry.location,
            entry.min_pokemon,
            entry.max_pokemon,
            pokemon,
            entry.special_pokemon,
            entry.type_rates,
            types,
        ));
    }

    encounters
}
