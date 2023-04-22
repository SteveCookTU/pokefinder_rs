use crate::get_test_data;
use pokefinder_rs_core::enums::Game;
use pokefinder_rs_core::gen4::ChainedSIDCalc;
use pokefinder_rs_core::parents::personal_loader::get_personal_info;
use serde::Deserialize;

#[derive(Deserialize)]
struct ChainedSIDCalcTestData<'a> {
    #[serde(borrow)]
    addentry: Vec<AddEntryTestData<'a>>,
}

#[derive(Deserialize)]
struct AddEntryTestData<'a> {
    name: &'a str,
    tid: u16,
    version: u32,
    pokemon: u16,
    ability: Vec<u8>,
    ivs: Vec<[u8; 6]>,
    gender: Vec<u8>,
    nature: Vec<u8>,
    result: u16,
}

const JSON_DATA: &str = include_str!("chainedsid.json");

#[test]
fn add_entry() {
    let data = get_test_data::<'static, ChainedSIDCalcTestData>(JSON_DATA);

    for AddEntryTestData {
        name,
        tid,
        version,
        pokemon,
        ability,
        ivs,
        gender,
        nature,
        result,
    } in data.addentry.into_iter()
    {
        assert_eq!(
            ability.len(),
            ivs.len(),
            "Ability and IVs length mismatch: {}",
            name
        );
        assert_eq!(
            ivs.len(),
            gender.len(),
            "IVs and Gender length mismatch: {}",
            name
        );
        assert_eq!(
            gender.len(),
            nature.len(),
            "Gender and Nature length mismatch: {}",
            name
        );

        let info = get_personal_info(Game::from_bits_retain(version), pokemon, 0);

        let mut calc = ChainedSIDCalc::new(tid);
        for (((ability, ivs), gender), nature) in ability
            .into_iter()
            .zip(ivs.into_iter())
            .zip(gender.into_iter())
            .zip(nature.into_iter())
        {
            calc.add_entry(
                ivs[0],
                ivs[1],
                ivs[2],
                ivs[3],
                ivs[4],
                ivs[5],
                info.get_ability(ability as usize),
                gender,
                nature,
                info,
            );
        }

        assert_eq!(calc.sids.len(), 1, "SIDs lenth is not 1 - {}", name);
        assert_eq!(calc.sids[0], result, "Unequal SID and result - {}", name);
    }
}
