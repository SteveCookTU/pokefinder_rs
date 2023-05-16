use crate::gen8::field_encount_table::FieldEncountTableEntry;
use crate::gen8::ug_encount::UgEncountEntry;
use crate::gen8::ug_pokemon_data::UgPokemonEntry;
use crate::gen8::ug_rand_mark::UgRandMarkEntry;

pub fn pack_encounter_bdsp(encounter: &FieldEncountTableEntry) -> Vec<u8> {
    let mut data = vec![
        encounter.enc_rate_gr,
        encounter.enc_rate_wat,
        encounter.enc_rate_turi_boro,
        encounter.enc_rate_turi_ii,
        encounter.enc_rate_sugoi,
    ];

    for entry in encounter.ground_mons.iter() {
        data.extend(entry.mons_no.to_le_bytes());
        data.push(entry.maxlv);
        data.push(0);
    }

    for entry in encounter.tairyo.iter() {
        data.extend(entry.mons_no.to_le_bytes());
    }

    for entry in encounter.day.iter() {
        data.extend(entry.mons_no.to_le_bytes());
    }

    for entry in encounter.night.iter() {
        data.extend(entry.mons_no.to_le_bytes());
    }

    for entry in encounter.sway_grass.iter() {
        data.extend(entry.mons_no.to_le_bytes());
    }

    for entry in encounter.water_mons.iter() {
        data.extend(entry.mons_no.to_le_bytes());
        data.push(entry.maxlv);
        data.push(entry.minlv);
    }

    for entry in encounter.boro_mons.iter() {
        data.extend(entry.mons_no.to_le_bytes());
        data.push(entry.maxlv);
        data.push(entry.minlv);
    }

    for entry in encounter.ii_mons.iter() {
        data.extend(entry.mons_no.to_le_bytes());
        data.push(entry.maxlv);
        data.push(entry.minlv);
    }

    for entry in encounter.sugoi_mons.iter() {
        data.extend(entry.mons_no.to_le_bytes());
        data.push(entry.maxlv);
        data.push(entry.minlv);
    }

    data
}

pub fn pack_encounter_underground(
    rand_mark_room: &UgRandMarkEntry,
    special_pokemon_rates: Vec<(u16, u16)>,
    enabled_pokemon: Vec<&UgEncountEntry>,
    pokemon_data: &[UgPokemonEntry],
) -> Vec<u8> {
    let mut data = vec![
        rand_mark_room.min,
        rand_mark_room.max,
        enabled_pokemon.len() as u8,
        special_pokemon_rates.len() as u8,
    ];

    for rate in &rand_mark_room.typerate {
        data.push(*rate);
    }

    for (rate, species) in special_pokemon_rates {
        data.extend(rate.to_le_bytes());
        data.extend(species.to_le_bytes());
    }

    for enabled_pokemon in enabled_pokemon {
        let pokemon = pokemon_data
            .iter()
            .find(|p| p.monsno == enabled_pokemon.monsno)
            .unwrap();

        data.extend(enabled_pokemon.monsno.to_le_bytes());
        for rate in pokemon.flagrate.iter() {
            data.push(*rate);
        }
        data.push(enabled_pokemon.zukanflag);
        data.push(pokemon.rateup);
        data.push(pokemon.size);
        data.push(0);
    }

    data
}
