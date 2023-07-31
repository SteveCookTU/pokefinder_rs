use no_std_io::{Cursor, EndianRead, Error, ReadOutput, StreamContainer, StreamReader};

#[derive(EndianRead, Default, Copy, Clone)]
struct DynamicSlot {
    max_level: u8,
    min_level: u8,
    _padding: [u8; 2],
    species: u32,
}

#[derive(EndianRead, Default, Copy, Clone)]
struct StaticSlot {
    level: u8,
    _padding: [u8; 3],
    species: u32,
}

#[derive(EndianRead, Default, Copy, Clone)]
struct EncounterDPPt {
    grass_rate: u32,
    grass: [StaticSlot; 12],
    swarm: [u32; 2],
    noon: [u32; 2],
    night: [u32; 2],
    radar: [u32; 4],
    _forms: [u32; 5],
    _anoon: u32,
    ruby: [u32; 2],
    sapphire: [u32; 2],
    emerald: [u32; 2],
    firered: [u32; 2],
    leafgreen: [u32; 2],
    surf_rate: u32,
    surf: [DynamicSlot; 5],
    _rock_rate: u32,
    _rock: [DynamicSlot; 5],
    old_rate: u32,
    old: [DynamicSlot; 5],
    good_rate: u32,
    good: [DynamicSlot; 5],
    super_rate: u32,
    super_rod: [DynamicSlot; 5],
}

pub fn pack_encounter_dppt(encounter: &[u8]) -> Vec<u8> {
    let entry = StreamContainer::new(encounter)
        .read_stream_le::<EncounterDPPt>()
        .unwrap();

    let mut data = vec![
        entry.grass_rate as u8,
        entry.surf_rate as u8,
        entry.old_rate as u8,
        entry.good_rate as u8,
        entry.super_rate as u8,
    ];

    for slot in entry.grass {
        data.extend(&slot.species.to_le_bytes()[..2]);
        data.push(slot.level);
        data.push(0);
    }

    for species in entry.swarm {
        data.extend(&species.to_le_bytes()[..2]);
    }

    for species in entry.noon {
        data.extend(&species.to_le_bytes()[..2]);
    }

    for species in entry.night {
        data.extend(&species.to_le_bytes()[..2]);
    }

    for species in entry.radar {
        data.extend(&species.to_le_bytes()[..2]);
    }

    for species in entry.ruby {
        data.extend(&species.to_le_bytes()[..2]);
    }

    for species in entry.sapphire {
        data.extend(&species.to_le_bytes()[..2]);
    }

    for species in entry.emerald {
        data.extend(&species.to_le_bytes()[..2]);
    }

    for species in entry.firered {
        data.extend(&species.to_le_bytes()[..2]);
    }

    for species in entry.leafgreen {
        data.extend(&species.to_le_bytes()[..2]);
    }

    for slot in entry.surf {
        data.extend(&slot.species.to_le_bytes()[..2]);
        data.push(slot.max_level);
        data.push(slot.min_level);
    }

    for slot in entry.old {
        data.extend(&slot.species.to_le_bytes()[..2]);
        data.push(slot.max_level);
        data.push(slot.min_level);
    }

    for slot in entry.good {
        data.extend(&slot.species.to_le_bytes()[..2]);
        data.push(slot.max_level);
        data.push(slot.min_level);
    }

    for slot in entry.super_rod {
        data.extend(&slot.species.to_le_bytes()[..2]);
        data.push(slot.max_level);
        data.push(slot.min_level);
    }

    data
}

#[derive(EndianRead, Default, Copy, Clone)]
struct DynamicSlotHGSS {
    min_level: u8,
    max_level: u8,
    species: u16,
}

#[derive(Default, Copy, Clone)]
struct EncounterGrass {
    level: [u8; 12],
    morning: [u16; 12],
    day: [u16; 12],
    night: [u16; 12],
}

impl EndianRead for EncounterGrass {
    fn try_read_le(bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        let mut reader = StreamContainer::new(bytes);
        let level = reader.read_stream_le()?;
        let mut morning = [0; 12];
        for r in morning.iter_mut() {
            *r = reader.read_stream_le()?;
        }
        let mut day = [0; 12];
        for r in day.iter_mut() {
            *r = reader.read_stream_le()?;
        }
        let mut night = [0; 12];
        for r in night.iter_mut() {
            *r = reader.read_stream_le()?;
        }

        Ok(ReadOutput::new(
            Self {
                level,
                morning,
                day,
                night,
            },
            reader.get_index(),
        ))
    }

    fn try_read_be(_: &[u8]) -> Result<ReadOutput<Self>, Error> {
        unimplemented!()
    }
}

#[derive(Copy, Clone, Default)]
struct EncounterHGSS {
    grass_rate: u8,
    surf_rate: u8,
    rock_rate: u8,
    old_rate: u8,
    good_rate: u8,
    super_rate: u8,
    _pad: u16,
    grass: EncounterGrass,
    hoenn_sound: [u16; 2],
    sinnoh_sound: [u16; 2],
    surf: [DynamicSlotHGSS; 5],
    rock: [DynamicSlotHGSS; 2],
    old: [DynamicSlotHGSS; 5],
    good: [DynamicSlotHGSS; 5],
    super_rod: [DynamicSlotHGSS; 5],
    swarm: [u16; 4],
}

impl EndianRead for EncounterHGSS {
    fn try_read_le(bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        let mut reader = StreamContainer::new(bytes);
        let grass_rate = reader.read_stream_le()?;
        let surf_rate = reader.read_stream_le()?;
        let rock_rate = reader.read_stream_le()?;
        let old_rate = reader.read_stream_le()?;
        let good_rate = reader.read_stream_le()?;
        let super_rate = reader.read_stream_le()?;
        let _pad = reader.read_stream_le()?;
        let grass = reader.read_stream_le()?;
        let hoenn_sound = [reader.read_stream_le()?, reader.read_stream_le()?];
        let sinnoh_sound = [reader.read_stream_le()?, reader.read_stream_le()?];
        let surf = [
            reader.read_stream_le()?,
            reader.read_stream_le()?,
            reader.read_stream_le()?,
            reader.read_stream_le()?,
            reader.read_stream_le()?,
        ];
        let rock = [reader.read_stream_le()?, reader.read_stream_le()?];
        let old = [
            reader.read_stream_le()?,
            reader.read_stream_le()?,
            reader.read_stream_le()?,
            reader.read_stream_le()?,
            reader.read_stream_le()?,
        ];
        let good = [
            reader.read_stream_le()?,
            reader.read_stream_le()?,
            reader.read_stream_le()?,
            reader.read_stream_le()?,
            reader.read_stream_le()?,
        ];
        let super_rod = [
            reader.read_stream_le()?,
            reader.read_stream_le()?,
            reader.read_stream_le()?,
            reader.read_stream_le()?,
            reader.read_stream_le()?,
        ];
        let swarm = [
            reader.read_stream_le()?,
            reader.read_stream_le()?,
            reader.read_stream_le()?,
            reader.read_stream_le()?,
        ];

        Ok(ReadOutput::new(
            Self {
                grass_rate,
                surf_rate,
                rock_rate,
                old_rate,
                good_rate,
                super_rate,
                _pad,
                grass,
                hoenn_sound,
                sinnoh_sound,
                surf,
                rock,
                old,
                good,
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

pub fn pack_encounter_hgss(encounter: &[u8]) -> Vec<u8> {
    let entry = StreamContainer::new(encounter)
        .read_stream_le::<EncounterHGSS>()
        .unwrap();

    let mut data = vec![
        entry.grass_rate,
        entry.surf_rate,
        entry.rock_rate,
        entry.old_rate,
        entry.good_rate,
        entry.super_rate,
        0,
    ];

    for specie in entry.grass.morning {
        data.extend(specie.to_le_bytes());
    }

    for specie in entry.grass.day {
        data.extend(specie.to_le_bytes());
    }

    for specie in entry.grass.night {
        data.extend(specie.to_le_bytes());
    }

    for level in entry.grass.level {
        data.push(level);
    }

    for specie in entry.hoenn_sound {
        data.extend(specie.to_le_bytes());
    }

    for specie in entry.sinnoh_sound {
        data.extend(specie.to_le_bytes());
    }

    for slot in entry.surf {
        data.extend(slot.species.to_le_bytes());
        data.push(slot.max_level);
        data.push(slot.min_level);
    }

    for slot in entry.rock {
        data.extend(slot.species.to_le_bytes());
        data.push(slot.max_level);
        data.push(slot.min_level);
    }

    for slot in entry.old {
        data.extend(slot.species.to_le_bytes());
        data.push(slot.max_level);
        data.push(slot.min_level);
    }

    for slot in entry.good {
        data.extend(slot.species.to_le_bytes());
        data.push(slot.max_level);
        data.push(slot.min_level);
    }

    for slot in entry.super_rod {
        data.extend(slot.species.to_le_bytes());
        data.push(slot.max_level);
        data.push(slot.min_level);
    }

    for species in entry.swarm {
        data.extend(species.to_le_bytes());
    }

    data
}

#[derive(EndianRead, Default, Copy, Clone)]
struct HeadbuttSlot {
    specie: u16,
    min_level: u8,
    max_level: u8,
}

#[derive(EndianRead, Default, Copy, Clone)]
struct _HeadbuttTree {
    _x: u16,
    _y: u16,
}

struct HeadbuttEncounter {
    _tree_count: u16,
    special_tree_count: u16,
    tree: [HeadbuttSlot; 12],
    special_tree: [HeadbuttSlot; 6],
}

impl EndianRead for HeadbuttEncounter {
    fn try_read_le(bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        let mut reader = StreamContainer::new(bytes);
        let _tree_count = reader.read_stream_le::<u16>()?;
        let special_tree_count = reader.read_stream_le()?;
        let mut tree = [HeadbuttSlot::default(); 12];
        for t in tree.iter_mut() {
            *t = reader.read_stream_le()?;
        }
        let mut special_tree = [HeadbuttSlot::default(); 6];
        for t in special_tree.iter_mut() {
            *t = reader.read_stream_le()?;
        }

        Ok(ReadOutput::new(
            Self {
                _tree_count,
                special_tree_count,
                tree,
                special_tree,
            },
            reader.get_index(),
        ))
    }

    fn try_read_be(_: &[u8]) -> Result<ReadOutput<Self>, Error> {
        unimplemented!()
    }
}

#[derive(EndianRead, Default, Copy, Clone)]
struct BugSlot {
    specie: u16,
    min_level: u8,
    max_level: u8,
    _rate: u8,
    _score: u8,
    _dummy: [u8; 2],
}

#[derive(Copy, Clone, Default)]
struct BugEncounterArea {
    slots: [BugSlot; 10],
}

impl EndianRead for BugEncounterArea {
    fn try_read_le(bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        let mut reader = StreamContainer::new(bytes);
        let mut slots = [BugSlot::default(); 10];
        for slot in slots.iter_mut() {
            *slot = reader.read_stream_le()?;
        }

        Ok(ReadOutput::new(Self { slots }, reader.get_index()))
    }

    fn try_read_be(_: &[u8]) -> Result<ReadOutput<Self>, Error> {
        unimplemented!()
    }
}

#[derive(Copy, Clone, Default)]
struct BugEncounter {
    areas: [BugEncounterArea; 4],
}

impl EndianRead for BugEncounter {
    fn try_read_le(bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        let mut reader = StreamContainer::new(bytes);
        let mut areas = [BugEncounterArea::default(); 4];
        for area in areas.iter_mut() {
            *area = reader.read_stream_le()?;
        }

        Ok(ReadOutput::new(Self { areas }, reader.get_index()))
    }

    fn try_read_be(_: &[u8]) -> Result<ReadOutput<Self>, Error> {
        unimplemented!()
    }
}

pub fn pack_encounter_hgss_bug(encounter: &[u8]) -> Vec<u8> {
    let entry = StreamContainer::new(encounter)
        .read_stream_le::<BugEncounter>()
        .unwrap();

    let mut data = vec![];
    const LOCATION_START: u8 = 142;

    for (i, area) in entry.areas.into_iter().enumerate() {
        data.push(LOCATION_START - (i as u8));
        data.push(0);
        for slot in area.slots {
            data.extend(slot.specie.to_le_bytes());
            data.push(slot.max_level);
            data.push(slot.min_level);
        }
    }

    data
}

pub fn pack_encounter_hgss_headbutt(encounter: &[u8]) -> Vec<u8> {
    let entry = StreamContainer::new(encounter)
        .read_stream_le::<HeadbuttEncounter>()
        .unwrap();

    let mut data = vec![];
    data.push(u8::from(entry.special_tree_count != 0));

    for slot in entry.tree {
        data.extend(slot.specie.to_le_bytes());
        data.push(slot.max_level);
        data.push(slot.min_level);
    }

    if entry.special_tree_count != 0 {
        for slot in entry.special_tree {
            data.extend(slot.specie.to_le_bytes());
            data.push(slot.max_level);
            data.push(slot.min_level);
        }
    } else {
        data.extend([0; 24]);
    }

    data
}
