use crate::enums::{Encounter, Game};
use crate::gen4::{EncounterArea4, Profile4, StaticTemplate4};
use crate::parents::{personal_loader, PersonalInfo, Profile, Slot};
use crate::resources::encounter_data_4::{
    DIAMOND, EVENTS, FOSSILS, GAME_CORNER, GIFTS, HEART_GOLD, HGSS_BUG, HGSS_SAFARI, HG_HEADBUTT,
    LEGENDS, PEARL, PLATINUM, ROAMERS, SOUL_SILVER, SS_HEADBUTT, STARTERS, STATIONARY,
};
use crate::util;
use crate::util::encounter_slot::{DynamicSlot, StaticSlot};
use no_std_io::{Cursor, EndianRead, Error, ReadOutput, StreamContainer, StreamReader};

const GREAT_MARSH_DP: [u16; 12] = [55, 183, 194, 195, 298, 315, 397, 399, 400, 451, 453, 455];
const GREAT_MARSH_DP_DEX: [u16; 14] = [
    46, 55, 102, 115, 193, 285, 315, 316, 397, 451, 452, 453, 454, 455,
];

const GREAT_MARSH_PT: [u16; 8] = [114, 193, 194, 195, 357, 451, 453, 455];
const GREAT_MARSH_PT_DEX: [u16; 15] = [
    46, 102, 114, 115, 193, 195, 285, 316, 352, 357, 451, 452, 453, 454, 455,
];

const TROPHY_GARDEN_DP: [u16; 16] = [
    35, 39, 52, 113, 133, 137, 173, 174, 183, 298, 311, 312, 351, 438, 439, 440,
];
const TROPHY_GARDEN_PT: [u16; 16] = [
    35, 39, 52, 113, 132, 133, 173, 174, 183, 298, 311, 312, 351, 438, 439, 440,
];

struct WildEncounterDPPt {
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
    radar: [u16; 4],
    ruby: [u16; 2],
    sapphire: [u16; 2],
    emerald: [u16; 2],
    fire_red: [u16; 2],
    leaf_green: [u16; 2],
    surf: [DynamicSlot; 5],
    old_rod: [DynamicSlot; 5],
    good_rod: [DynamicSlot; 5],
    super_rod: [DynamicSlot; 5],
}

impl EndianRead for WildEncounterDPPt {
    fn try_read_le(bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        let mut reader = StreamContainer::new(bytes);
        let location = reader.read_stream_le()?;
        let grass_rate = reader.read_stream_le()?;
        let surf_rate = reader.read_stream_le()?;
        let old_rate = reader.read_stream_le()?;
        let good_rate = reader.read_stream_le()?;
        let super_rate = reader.read_stream_le()?;

        let mut grass = [StaticSlot::default(); 12];
        for g in grass.iter_mut() {
            *g = reader.read_stream_le()?;
        }
        let mut swarm = [0; 2];
        for s in swarm.iter_mut() {
            *s = reader.read_stream_le::<u16>()?;
        }
        let mut day = [0; 2];
        for d in day.iter_mut() {
            *d = reader.read_stream_le::<u16>()?;
        }
        let mut night = [0; 2];
        for n in night.iter_mut() {
            *n = reader.read_stream_le::<u16>()?;
        }
        let mut radar = [0; 4];
        for r in radar.iter_mut() {
            *r = reader.read_stream_le::<u16>()?;
        }
        let mut ruby = [0; 2];
        for r in ruby.iter_mut() {
            *r = reader.read_stream_le::<u16>()?;
        }
        let mut sapphire = [0; 2];
        for s in sapphire.iter_mut() {
            *s = reader.read_stream_le::<u16>()?;
        }
        let mut emerald = [0; 2];
        for e in emerald.iter_mut() {
            *e = reader.read_stream_le::<u16>()?;
        }
        let mut fire_red = [0; 2];
        for f in fire_red.iter_mut() {
            *f = reader.read_stream_le::<u16>()?;
        }
        let mut leaf_green = [0; 2];
        for l in leaf_green.iter_mut() {
            *l = reader.read_stream_le::<u16>()?;
        }
        let mut surf = [DynamicSlot::default(); 5];
        for s in surf.iter_mut() {
            *s = reader.read_stream_le()?;
        }
        let mut old_rod = [DynamicSlot::default(); 5];
        for o in old_rod.iter_mut() {
            *o = reader.read_stream_le()?;
        }
        let mut good_rod = [DynamicSlot::default(); 5];
        for g in good_rod.iter_mut() {
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
                old_rate,
                good_rate,
                super_rate,
                grass,
                swarm,
                day,
                night,
                radar,
                ruby,
                sapphire,
                emerald,
                fire_red,
                leaf_green,
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

struct HGSSEncounterGrass {
    slots: [u16; 36],
    level: [u8; 12],
}

impl EndianRead for HGSSEncounterGrass {
    fn try_read_le(bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        let mut reader = StreamContainer::new(bytes);
        let mut slots = [0; 36];
        for s in slots.iter_mut() {
            *s = reader.read_stream_le::<u16>()?;
        }

        let level = reader.read_stream_le::<[u8; 12]>()?;
        Ok(ReadOutput::new(Self { slots, level }, reader.get_index()))
    }

    fn try_read_be(_: &[u8]) -> Result<ReadOutput<Self>, Error> {
        unimplemented!()
    }
}

struct WildEncounterHGSS {
    location: u8,
    grass_rate: u8,
    surf_rate: u8,
    rock_rate: u8,
    old_rate: u8,
    good_rate: u8,
    super_rate: u8,
    grass: HGSSEncounterGrass,
    hoenn_sound: [u16; 2],
    sinnoh_sound: [u16; 2],
    surf: [DynamicSlot; 5],
    rock: [DynamicSlot; 2],
    old_rod: [DynamicSlot; 5],
    good_rod: [DynamicSlot; 5],
    super_rod: [DynamicSlot; 5],
    swarm: [u16; 4],
}

impl EndianRead for WildEncounterHGSS {
    fn try_read_le(bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        let mut reader = StreamContainer::new(bytes);
        let location = reader.read_stream_le()?;
        let grass_rate = reader.read_stream_le()?;
        let surf_rate = reader.read_stream_le()?;
        let rock_rate = reader.read_stream_le()?;
        let old_rate = reader.read_stream_le()?;
        let good_rate = reader.read_stream_le()?;
        let super_rate = reader.read_stream_le()?;

        let _padding = reader.read_stream_le::<u8>()?;

        let grass = reader.read_stream_le::<HGSSEncounterGrass>()?;
        let mut hoenn_sound = [0; 2];
        for h in hoenn_sound.iter_mut() {
            *h = reader.read_stream_le::<u16>()?;
        }
        let mut sinnoh_sound = [0; 2];
        for s in sinnoh_sound.iter_mut() {
            *s = reader.read_stream_le::<u16>()?;
        }
        let mut surf = [DynamicSlot::default(); 5];
        for s in surf.iter_mut() {
            *s = reader.read_stream_le()?;
        }
        let mut rock = [DynamicSlot::default(); 2];
        for r in rock.iter_mut() {
            *r = reader.read_stream_le()?;
        }
        let mut old_rod = [DynamicSlot::default(); 5];
        for o in old_rod.iter_mut() {
            *o = reader.read_stream_le()?;
        }
        let mut good_rod = [DynamicSlot::default(); 5];
        for g in good_rod.iter_mut() {
            *g = reader.read_stream_le()?;
        }
        let mut super_rod = [DynamicSlot::default(); 5];
        for s in super_rod.iter_mut() {
            *s = reader.read_stream_le()?;
        }
        let mut swarm = [0; 4];
        for s in swarm.iter_mut() {
            *s = reader.read_stream_le::<u16>()?;
        }
        Ok(ReadOutput::new(
            Self {
                location,
                grass_rate,
                surf_rate,
                rock_rate,
                old_rate,
                good_rate,
                super_rate,
                grass,
                hoenn_sound,
                sinnoh_sound,
                surf,
                rock,
                old_rod,
                good_rod,
                super_rod,
                swarm,
            },
            reader.get_index(),
        ))
    }

    fn try_read_be(_: &[u8]) -> Result<ReadOutput<Self>, Error> {
        unimplemented!()
    }
}

struct WildEncounterHGSSBug {
    location: u8,
    bug: [DynamicSlot; 10],
}

impl EndianRead for WildEncounterHGSSBug {
    fn try_read_le(bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        let mut reader = StreamContainer::new(bytes);

        let location = reader.read_stream_le()?;
        let _padding = reader.read_stream_le::<u8>()?;
        let mut bug = [DynamicSlot::default(); 10];
        for b in bug.iter_mut() {
            *b = reader.read_stream_le()?;
        }

        Ok(ReadOutput::new(Self { location, bug }, reader.get_index()))
    }

    fn try_read_be(_: &[u8]) -> Result<ReadOutput<Self>, Error> {
        unimplemented!()
    }
}

struct WildEncounterHGSSHeadbutt {
    location: u8,
    has_special: bool,
    slots: [DynamicSlot; 18],
}

impl EndianRead for WildEncounterHGSSHeadbutt {
    fn try_read_le(bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        let mut reader = StreamContainer::new(bytes);

        let location = reader.read_stream_le()?;
        let has_special = reader.read_stream_le()?;

        let mut slots = [DynamicSlot::default(); 18];
        for s in slots.iter_mut() {
            *s = reader.read_stream_le()?;
        }

        Ok(ReadOutput::new(
            Self {
                location,
                has_special,
                slots,
            },
            reader.get_index(),
        ))
    }

    fn try_read_be(_: &[u8]) -> Result<ReadOutput<Self>, Error> {
        unimplemented!()
    }
}

struct WildSubEncounterHGSSSafari<const TOTAL: usize, const INDIVIDUAL: usize> {
    normal: [StaticSlot; 30],
    block: [StaticSlot; TOTAL],
    ty1: [u8; INDIVIDUAL],
    quantity1: [u8; INDIVIDUAL],
    ty2: [u8; INDIVIDUAL],
    quantity2: [u8; INDIVIDUAL],
}

impl<const TOTAL: usize, const INDIVIDUAL: usize> EndianRead
    for WildSubEncounterHGSSSafari<TOTAL, INDIVIDUAL>
{
    fn try_read_le(bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        let mut reader = StreamContainer::new(bytes);

        let mut normal = [StaticSlot::default(); 30];
        for n in normal.iter_mut() {
            *n = reader.read_stream_le()?;
        }

        let mut block = [StaticSlot::default(); TOTAL];
        for b in block.iter_mut() {
            *b = reader.read_stream_le()?;
        }

        let ty1 = reader.read_stream_le()?;
        let quantity1 = reader.read_stream_le()?;
        let ty2 = reader.read_stream_le()?;
        let quantity2 = reader.read_stream_le()?;

        Ok(ReadOutput::new(
            Self {
                normal,
                block,
                ty1,
                quantity1,
                ty2,
                quantity2,
            },
            reader.get_index(),
        ))
    }

    fn try_read_be(_: &[u8]) -> Result<ReadOutput<Self>, Error> {
        unimplemented!()
    }
}

#[derive(EndianRead)]
struct WildEncounterHGSSSafari {
    location: u8,
    has_water: bool,
    grass: WildSubEncounterHGSSSafari<30, 10>,
    surf: WildSubEncounterHGSSSafari<9, 3>,
    old_rod: WildSubEncounterHGSSSafari<6, 2>,
    good_rod: WildSubEncounterHGSSSafari<6, 2>,
    super_rod: WildSubEncounterHGSSSafari<6, 2>,
}

fn modify_radio(
    pokemon: &mut [Slot],
    entry: &WildEncounterHGSS,
    info: &'static [PersonalInfo],
    radio: usize,
) {
    let (species1, species2) = match radio {
        1 => (entry.hoenn_sound[0], entry.hoenn_sound[1]),
        2 => (entry.sinnoh_sound[0], entry.sinnoh_sound[1]),
        _ => return,
    };

    pokemon[2].set_species(species1, &info[species1 as usize]);
    pokemon[3].set_species(species1, &info[species1 as usize]);
    pokemon[4].set_species(species2, &info[species2 as usize]);
    pokemon[5].set_species(species2, &info[species2 as usize]);
}

fn modify_swarm_hgss(
    pokemon: &mut [Slot],
    entry: &WildEncounterHGSS,
    info: &'static [PersonalInfo],
    encounter: Encounter,
    swarm: bool,
) {
    if swarm {
        let species;
        match encounter {
            Encounter::Grass => {
                species = entry.swarm[0];
                pokemon[0].set_species(species, &info[species as usize]);
                pokemon[1].set_species(species, &info[species as usize]);
            }
            Encounter::Surfing => {
                species = entry.swarm[1];
                pokemon[0].set_species(species, &info[species as usize]);
            }
            Encounter::OldRod => {
                species = entry.swarm[3];
                pokemon[2].set_species(species, &info[species as usize]);
            }
            Encounter::GoodRod => {
                species = entry.swarm[3];
                for i in [0, 2, 3] {
                    pokemon[i].set_species(species, &info[species as usize]);
                }
            }
            Encounter::SuperRod => {
                species = entry.swarm[3];
                pokemon.iter_mut().take(5).for_each(|p| {
                    p.set_species(species, &info[species as usize]);
                })
            }
            _ => {}
        }
    }
}

fn modify_time_hgss(
    pokemon: &mut [Slot],
    entry: &WildEncounterHGSS,
    info: &'static [PersonalInfo],
    encounter: Encounter,
    time: usize,
) {
    if time != 0 && time != 1 {
        return;
    }
    match encounter {
        Encounter::GoodRod => {
            pokemon[3].set_species(entry.swarm[2], &info[entry.swarm[2] as usize]);
        }
        Encounter::SuperRod => {
            pokemon[1].set_species(entry.swarm[2], &info[entry.swarm[2] as usize]);
        }
        _ => {}
    }
}

fn get_hgss_safari(
    encounter: Encounter,
    time: usize,
    blocks: [u8; 5],
    info: &'static [PersonalInfo],
) -> Vec<EncounterArea4> {
    let data = util::decompress(HGSS_SAFARI);

    let mut encounters = vec![];

    let mut reader = StreamContainer::new(data);
    while let Ok(entry) = reader.read_stream_le::<WildEncounterHGSSSafari>() {
        let mut block = 0;
        let mut slots = Vec::with_capacity(10);
        let mut safari_slots;
        let mut safari_block_slots;
        match encounter {
            Encounter::Grass => {
                for i in 0..10 {
                    safari_slots = &entry.grass.normal[(10 * time)..];
                    safari_block_slots = &entry.grass.block[(10 * time)..];

                    let mut species = safari_slots[i].species;
                    let mut level = safari_slots[i].level;
                    while block < 10 {
                        if blocks[entry.grass.ty1[block] as usize] >= entry.grass.quantity1[block]
                            && blocks[entry.grass.ty2[block] as usize]
                                >= entry.grass.quantity2[block]
                        {
                            species = safari_block_slots[block].species;
                            level = safari_block_slots[block].level;
                            block += 1;
                            break;
                        }
                        block += 1;
                    }
                    slots.push(Slot::new(species, level, level, &info[species as usize]));
                }
                encounters.push(EncounterArea4::new(entry.location, 0, encounter, slots));
            }
            Encounter::Surfing => {
                if entry.has_water {
                    safari_slots = &entry.surf.normal[(10 * time)..];
                    safari_block_slots = &entry.surf.block[(3 * time)..];

                    for safari_slot in safari_slots.iter().take(10) {
                        let mut species = safari_slot.species;
                        let mut level = safari_slot.level;
                        while block < 3 {
                            if blocks[entry.surf.ty1[block] as usize] >= entry.surf.quantity1[block]
                                && blocks[entry.surf.ty2[block] as usize]
                                    >= entry.surf.quantity2[block]
                            {
                                species = safari_block_slots[block].species;
                                level = safari_block_slots[block].level;
                                block += 1;
                                break;
                            }
                            block += 1;
                        }
                        slots.push(Slot::new(species, level, level, &info[species as usize]));
                    }
                    encounters.push(EncounterArea4::new(entry.location, 0, encounter, slots));
                }
            }
            Encounter::OldRod => {
                if entry.has_water {
                    safari_slots = &entry.old_rod.normal[(10 * time)..];
                    safari_block_slots = &entry.old_rod.block[(2 * time)..];

                    for safari_slot in safari_slots.iter().take(10) {
                        let mut species = safari_slot.species;
                        let mut level = safari_slot.level;
                        while block < 2 {
                            if blocks[entry.old_rod.ty1[block] as usize]
                                >= entry.old_rod.quantity1[block]
                                && blocks[entry.old_rod.ty2[block] as usize]
                                    >= entry.old_rod.quantity2[block]
                            {
                                species = safari_block_slots[block].species;
                                level = safari_block_slots[block].level;
                                block += 1;
                                break;
                            }
                            block += 1;
                        }
                        slots.push(Slot::new(species, level, level, &info[species as usize]));
                    }
                    encounters.push(EncounterArea4::new(entry.location, 25, encounter, slots));
                }
            }
            Encounter::GoodRod => {
                if entry.has_water {
                    safari_slots = &entry.good_rod.normal[(10 * time)..];
                    safari_block_slots = &entry.good_rod.block[(2 * time)..];

                    for safari_slot in safari_slots.iter().take(10) {
                        let mut species = safari_slot.species;
                        let mut level = safari_slot.level;
                        while block < 2 {
                            if blocks[entry.good_rod.ty1[block] as usize]
                                >= entry.good_rod.quantity1[block]
                                && blocks[entry.good_rod.ty2[block] as usize]
                                    >= entry.good_rod.quantity2[block]
                            {
                                species = safari_block_slots[block].species;
                                level = safari_block_slots[block].level;
                                block += 1;
                                break;
                            }
                            block += 1;
                        }
                        slots.push(Slot::new(species, level, level, &info[species as usize]));
                    }
                    encounters.push(EncounterArea4::new(entry.location, 50, encounter, slots));
                }
            }
            Encounter::SuperRod => {
                if entry.has_water {
                    safari_slots = &entry.super_rod.normal[(10 * time)..];
                    safari_block_slots = &entry.super_rod.block[(2 * time)..];

                    for safari_slot in safari_slots.iter().take(10) {
                        let mut species = safari_slot.species;
                        let mut level = safari_slot.level;
                        while block < 2 {
                            if blocks[entry.super_rod.ty1[block] as usize]
                                >= entry.super_rod.quantity1[block]
                                && blocks[entry.super_rod.ty2[block] as usize]
                                    >= entry.super_rod.quantity2[block]
                            {
                                species = safari_block_slots[block].species;
                                level = safari_block_slots[block].level;
                                block += 1;
                                break;
                            }
                            block += 1;
                        }
                        slots.push(Slot::new(species, level, level, &info[species as usize]));
                    }
                    encounters.push(EncounterArea4::new(entry.location, 75, encounter, slots));
                }
            }
            _ => {}
        }
    }

    encounters
}

#[allow(clippy::too_many_arguments)]
fn get_hgss(
    version: Game,
    encounter: Encounter,
    radio: usize,
    swarm: bool,
    dex: bool,
    time: usize,
    blocks: [u8; 5],
    info: &'static [PersonalInfo],
) -> Vec<EncounterArea4> {
    let mut encounters = vec![];

    let compressed_data;
    let data;
    let mut reader;

    if encounter == Encounter::BugCatchingContest {
        data = util::decompress(HGSS_BUG);
        reader = StreamContainer::new(data);

        if dex {
            let _unused = reader
                .read_stream_le::<WildEncounterHGSSBug>()
                .expect("Failed to read non-dex HGSS Bug entry");
        }
        while let Ok(entry) = reader.read_stream_le::<WildEncounterHGSSBug>() {
            let mut slots = Vec::with_capacity(10);
            for slot in &entry.bug {
                slots.push(Slot::new(
                    slot.species,
                    slot.min_level,
                    slot.max_level,
                    &info[slot.species as usize],
                ));
            }

            encounters.push(EncounterArea4::new(entry.location, 0, encounter, slots));

            if !dex {
                break;
            }
        }
    } else if matches!(
        encounter,
        Encounter::Headbutt | Encounter::HeadbuttAlt | Encounter::HeadbuttSpecial
    ) {
        compressed_data = if version == Game::HEART_GOLD {
            HG_HEADBUTT
        } else {
            SS_HEADBUTT
        };
        let data = util::decompress(compressed_data);
        reader = StreamContainer::new(data);
        let tree = encounter as usize - Encounter::Headbutt as usize;

        while let Ok(entry) = reader.read_stream_le::<WildEncounterHGSSHeadbutt>() {
            if encounter != Encounter::HeadbuttSpecial || entry.has_special {
                let mut slots = Vec::with_capacity(6);

                let tree_slot = &entry.slots[(6 * tree)..];
                for slot in tree_slot.iter().take(6) {
                    slots.push(Slot::new(
                        slot.species,
                        slot.min_level,
                        slot.max_level,
                        &info[slot.species as usize],
                    ));
                }
                encounters.push(EncounterArea4::new(entry.location, 0, encounter, slots));
            }
        }
    } else {
        compressed_data = if version == Game::HEART_GOLD {
            HEART_GOLD
        } else {
            SOUL_SILVER
        };

        data = util::decompress(compressed_data);
        reader = StreamContainer::new(data);
        while let Ok(entry) = reader.read_stream_le::<WildEncounterHGSS>() {
            match encounter {
                Encounter::Grass if entry.grass_rate != 0 => {
                    let mut slots = Vec::with_capacity(12);

                    let species = &entry.grass.slots[(time * 12)..];
                    for (i, &species) in species.iter().take(12).enumerate() {
                        slots.push(Slot::new(
                            species,
                            entry.grass.level[i],
                            entry.grass.level[i],
                            &info[species as usize],
                        ));
                    }
                    modify_radio(&mut slots, &entry, info, radio);
                    modify_swarm_hgss(&mut slots, &entry, info, encounter, swarm);
                    encounters.push(EncounterArea4::new(
                        entry.location,
                        entry.grass_rate,
                        encounter,
                        slots,
                    ));
                }
                Encounter::Surfing if entry.surf_rate != 0 => {
                    let mut slots = Vec::with_capacity(5);

                    for slot in &entry.surf {
                        slots.push(Slot::new(
                            slot.species,
                            slot.min_level,
                            slot.max_level,
                            &info[slot.species as usize],
                        ));
                    }

                    modify_swarm_hgss(&mut slots, &entry, info, encounter, swarm);
                    encounters.push(EncounterArea4::new(
                        entry.location,
                        entry.surf_rate,
                        encounter,
                        slots,
                    ));
                }
                Encounter::RockSmash if entry.rock_rate != 0 => {
                    let mut slots = Vec::with_capacity(2);

                    for slot in &entry.rock {
                        slots.push(Slot::new(
                            slot.species,
                            slot.min_level,
                            slot.max_level,
                            &info[slot.species as usize],
                        ));
                    }

                    encounters.push(EncounterArea4::new(
                        entry.location,
                        entry.rock_rate,
                        encounter,
                        slots,
                    ));
                }
                Encounter::OldRod if entry.old_rate != 0 => {
                    let mut slots = Vec::with_capacity(5);

                    for slot in &entry.old_rod {
                        slots.push(Slot::new(
                            slot.species,
                            slot.min_level,
                            slot.max_level,
                            &info[slot.species as usize],
                        ));
                    }

                    modify_swarm_hgss(&mut slots, &entry, info, encounter, swarm);
                    encounters.push(EncounterArea4::new(
                        entry.location,
                        entry.old_rate,
                        encounter,
                        slots,
                    ));
                }
                Encounter::GoodRod if entry.good_rate != 0 => {
                    let mut slots = Vec::with_capacity(5);

                    for slot in &entry.good_rod {
                        slots.push(Slot::new(
                            slot.species,
                            slot.min_level,
                            slot.max_level,
                            &info[slot.species as usize],
                        ));
                    }
                    modify_time_hgss(&mut slots, &entry, info, encounter, time);
                    modify_swarm_hgss(&mut slots, &entry, info, encounter, swarm);
                    encounters.push(EncounterArea4::new(
                        entry.location,
                        entry.good_rate,
                        encounter,
                        slots,
                    ));
                }
                Encounter::SuperRod if entry.super_rate != 0 => {
                    let mut slots = Vec::with_capacity(5);

                    for slot in &entry.super_rod {
                        slots.push(Slot::new(
                            slot.species,
                            slot.min_level,
                            slot.max_level,
                            &info[slot.species as usize],
                        ));
                    }
                    modify_time_hgss(&mut slots, &entry, info, encounter, time);
                    modify_swarm_hgss(&mut slots, &entry, info, encounter, swarm);
                    encounters.push(EncounterArea4::new(
                        entry.location,
                        entry.super_rate,
                        encounter,
                        slots,
                    ));
                }
                _ => {}
            }
        }

        encounters.append(&mut get_hgss_safari(encounter, time, blocks, info));
    }

    encounters
}

fn modify_dual(
    pokemon: &mut [Slot],
    entry: &WildEncounterDPPt,
    info: &'static [PersonalInfo],
    dual: Game,
) {
    let (specie1, specie2) = if dual == Game::RUBY {
        (entry.ruby[0], entry.ruby[1])
    } else if dual == Game::SAPPHIRE {
        (entry.sapphire[0], entry.sapphire[1])
    } else if dual == Game::EMERALD {
        (entry.emerald[0], entry.emerald[1])
    } else if dual == Game::FIRE_RED {
        (entry.fire_red[0], entry.fire_red[1])
    } else if dual == Game::LEAF_GREEN {
        (entry.leaf_green[0], entry.leaf_green[1])
    } else {
        return;
    };

    pokemon[8].set_species(specie1, &info[specie1 as usize]);
    pokemon[9].set_species(specie2, &info[specie2 as usize]);
}

fn modify_great_marsh(
    pokemon: &mut [Slot],
    replacement: [u16; 2],
    info: &'static [PersonalInfo],
    location: u8,
) {
    if (23..=28).contains(&location) && replacement[0] != 0 {
        let species = replacement[0];
        pokemon[6].set_species(species, &info[species as usize]);
        pokemon[7].set_species(species, &info[species as usize]);
    }
}

fn modify_radar(
    pokemon: &mut [Slot],
    entry: &WildEncounterDPPt,
    info: &'static [PersonalInfo],
    radar: bool,
) {
    if radar {
        pokemon[4].set_species(entry.radar[0], &info[entry.radar[0] as usize]);
        pokemon[5].set_species(entry.radar[1], &info[entry.radar[1] as usize]);
        pokemon[10].set_species(entry.radar[2], &info[entry.radar[2] as usize]);
        pokemon[11].set_species(entry.radar[3], &info[entry.radar[3] as usize]);
    }
}

fn modify_swarm_dppt(
    pokemon: &mut [Slot],
    entry: &WildEncounterDPPt,
    info: &'static [PersonalInfo],
    swarm: bool,
) {
    if swarm {
        pokemon[4].set_species(entry.radar[0], &info[entry.swarm[0] as usize]);
        pokemon[5].set_species(entry.radar[1], &info[entry.swarm[1] as usize]);
    }
}

fn modify_time_dppt(
    pokemon: &mut [Slot],
    entry: &WildEncounterDPPt,
    info: &'static [PersonalInfo],
    time: usize,
) {
    let (specie1, specie2) = if time == 1 {
        (entry.day[0], entry.day[1])
    } else if time == 2 {
        (entry.night[0], entry.night[1])
    } else {
        return;
    };

    pokemon[2].set_species(specie1, &info[specie1 as usize]);
    pokemon[3].set_species(specie2, &info[specie2 as usize]);
}

fn modify_trophy_garden(
    pokemon: &mut [Slot],
    replacement: [u16; 2],
    info: &'static [PersonalInfo],
    location: u8,
) {
    if location == 117 && replacement[0] != 0 && replacement[1] != 0 {
        let specie1 = replacement[0];
        let specie2 = replacement[1];

        pokemon[6].set_species(specie1, &info[specie1 as usize]);
        pokemon[7].set_species(specie2, &info[specie2 as usize]);
    }
}

#[allow(clippy::too_many_arguments)]
fn get_dppt(
    version: Game,
    encounter: Encounter,
    dual: Game,
    radar: bool,
    swarm: bool,
    time: usize,
    replacement: [u16; 2],
    info: &'static [PersonalInfo],
) -> Vec<EncounterArea4> {
    let compressed_data = if version == Game::DIAMOND {
        DIAMOND
    } else if version == Game::PEARL {
        PEARL
    } else {
        PLATINUM
    };

    let data = util::decompress(compressed_data);
    let mut reader = StreamContainer::new(data);

    let mut encounters = vec![];

    while let Ok(entry) = reader.read_stream_le::<WildEncounterDPPt>() {
        match encounter {
            Encounter::Grass if entry.grass_rate != 0 => {
                let mut slots = Vec::with_capacity(12);
                for slot in &entry.grass {
                    slots.push(Slot::new(
                        slot.species,
                        slot.level,
                        slot.level,
                        &info[slot.species as usize],
                    ));
                }
                modify_swarm_dppt(&mut slots, &entry, info, swarm);
                modify_time_dppt(&mut slots, &entry, info, time);
                modify_radar(&mut slots, &entry, info, radar);
                modify_great_marsh(&mut slots, replacement, info, entry.location);
                modify_trophy_garden(&mut slots, replacement, info, entry.location);
                modify_dual(&mut slots, &entry, info, dual);
                encounters.push(EncounterArea4::new(
                    entry.location,
                    entry.grass_rate,
                    encounter,
                    slots,
                ));
            }
            Encounter::Surfing if entry.surf_rate != 0 => {
                let mut slots = Vec::with_capacity(5);
                for slot in &entry.surf {
                    slots.push(Slot::new(
                        slot.species,
                        slot.min_level,
                        slot.max_level,
                        &info[slot.species as usize],
                    ));
                }
                encounters.push(EncounterArea4::new(
                    entry.location,
                    entry.surf_rate,
                    encounter,
                    slots,
                ));
            }
            Encounter::OldRod if entry.old_rate != 0 => {
                let mut slots = Vec::with_capacity(5);
                for slot in &entry.old_rod {
                    slots.push(Slot::new(
                        slot.species,
                        slot.min_level,
                        slot.max_level,
                        &info[slot.species as usize],
                    ));
                }
                encounters.push(EncounterArea4::new(
                    entry.location,
                    entry.old_rate,
                    encounter,
                    slots,
                ));
            }
            Encounter::GoodRod if entry.good_rate != 0 => {
                let mut slots = Vec::with_capacity(5);
                for slot in &entry.good_rod {
                    slots.push(Slot::new(
                        slot.species,
                        slot.min_level,
                        slot.max_level,
                        &info[slot.species as usize],
                    ));
                }
                encounters.push(EncounterArea4::new(
                    entry.location,
                    entry.good_rate,
                    encounter,
                    slots,
                ));
            }
            Encounter::SuperRod if entry.super_rate != 0 => {
                let mut slots = Vec::with_capacity(5);
                for slot in &entry.super_rod {
                    slots.push(Slot::new(
                        slot.species,
                        slot.min_level,
                        slot.max_level,
                        &info[slot.species as usize],
                    ));
                }
                encounters.push(EncounterArea4::new(
                    entry.location,
                    entry.super_rate,
                    encounter,
                    slots,
                ));
            }
            _ => {}
        }
    }

    encounters
}

/// Gets wild encounters for the `encounter` and `profile`
#[allow(clippy::too_many_arguments)]
pub fn get_encounters(
    encounter: Encounter,
    time: usize,
    dual: Game,
    radar: bool,
    radio: usize,
    swarm: bool,
    replacement: [u16; 2],
    blocks: [u8; 5],
    profile: &Profile4,
) -> Vec<EncounterArea4> {
    let version = profile.get_version();
    let info = personal_loader::get_personal_table(version);
    if (version & Game::DPPT) != Game::NONE {
        get_dppt(
            version,
            encounter,
            dual,
            radar,
            swarm,
            time,
            replacement,
            info,
        )
    } else {
        get_hgss(
            version,
            encounter,
            radio,
            swarm,
            profile.get_national_dex(),
            time,
            blocks,
            info,
        )
    }
}

/// Returns the changing pokemon of the Great Marsh
pub fn get_great_marsh_pokemon(profile: &Profile4) -> &'static [u16] {
    if (profile.get_version() & Game::DP) != Game::NONE {
        if profile.get_national_dex() {
            GREAT_MARSH_DP_DEX.as_slice()
        } else {
            GREAT_MARSH_DP.as_slice()
        }
    } else if profile.get_national_dex() {
        GREAT_MARSH_PT_DEX.as_slice()
    } else {
        GREAT_MARSH_PT.as_slice()
    }
}

/// Gets static encounters from the `ty`
pub fn get_static_encounters(ty: usize) -> &'static [StaticTemplate4] {
    match ty {
        0 => STARTERS.as_slice(),
        1 => FOSSILS.as_slice(),
        2 => GIFTS.as_slice(),
        3 => GAME_CORNER.as_slice(),
        4 => STATIONARY.as_slice(),
        5 => LEGENDS.as_slice(),
        6 => EVENTS.as_slice(),
        _ => ROAMERS.as_slice(),
    }
}

/// Gets a static encounter from the `ty` and `index`
pub fn get_static_encounter(ty: usize, index: usize) -> &'static StaticTemplate4 {
    &get_static_encounters(ty)[index]
}

/// Returns the changing pokemon of the Trophy Garden
pub fn get_trophy_garden_pokemon(profile: &'static Profile4) -> &'static [u16] {
    if (profile.get_version() & Game::DP) != Game::NONE {
        TROPHY_GARDEN_DP.as_slice()
    } else {
        TROPHY_GARDEN_PT.as_slice()
    }
}
