use crate::get_test_data;
use pokefinder_rs_core::enums::Encounter;
use pokefinder_rs_core::util::encounter_slot;
use serde::Deserialize;

#[derive(Deserialize)]
struct EncounterSlotData<'a> {
    #[serde(rename = "hSlot", borrow)]
    h_slot: Vec<SlotData<'a>>,
    #[serde(rename = "jSlot", borrow)]
    j_slot: Vec<SlotData<'a>>,
    #[serde(rename = "kSlot", borrow)]
    k_slot: Vec<SlotData<'a>>,
    #[serde(borrow)]
    bdsp: Vec<SlotData<'a>>,
}

#[derive(Deserialize)]
struct SlotData<'a> {
    name: &'a str,
    encounter: u8,
    rand: Vec<u16>,
}

const JSON_DATA: &str = include_str!("encounterslot.json");

#[test]
fn h_slot() {
    let data = get_test_data::<'static, EncounterSlotData>(JSON_DATA);

    for (
        num,
        SlotData {
            name,
            encounter,
            rand,
        },
    ) in data.h_slot.into_iter().enumerate()
    {
        for (i, rand) in rand.into_iter().enumerate() {
            assert_eq!(
                encounter_slot::h_slot((rand % 100) as u8, Encounter::from(encounter)),
                i as u8,
                "Unequal result: {} - {} - {}",
                name,
                num,
                i
            );
        }
    }
}

#[test]
fn j_slot() {
    let data = get_test_data::<'static, EncounterSlotData>(JSON_DATA);

    for (
        num,
        SlotData {
            name,
            encounter,
            rand,
        },
    ) in data.j_slot.into_iter().enumerate()
    {
        for (i, rand) in rand.into_iter().enumerate() {
            assert_eq!(
                encounter_slot::j_slot((rand / 656) as u8, Encounter::from(encounter)),
                i as u8,
                "Unequal result: {} - {} - {}",
                name,
                num,
                i
            );
        }
    }
}

#[test]
fn k_slot() {
    let data = get_test_data::<'static, EncounterSlotData>(JSON_DATA);

    for (
        num,
        SlotData {
            name,
            encounter,
            rand,
        },
    ) in data.k_slot.into_iter().enumerate()
    {
        for (i, rand) in rand.into_iter().enumerate() {
            assert_eq!(
                encounter_slot::k_slot((rand % 100) as u8, Encounter::from(encounter)),
                i as u8,
                "Unequal result: {} - {} - {}",
                name,
                num,
                i
            );
        }
    }
}

#[test]
fn bdsp_slot() {
    let data = get_test_data::<'static, EncounterSlotData>(JSON_DATA);

    for (
        num,
        SlotData {
            name,
            encounter,
            rand,
        },
    ) in data.bdsp.into_iter().enumerate()
    {
        for (i, rand) in rand.into_iter().enumerate() {
            assert_eq!(
                encounter_slot::bdsp_slot(rand as u8, Encounter::from(encounter)),
                i as u8,
                "Unequal result: {} - {} - {}",
                name,
                num,
                i
            );
        }
    }
}
