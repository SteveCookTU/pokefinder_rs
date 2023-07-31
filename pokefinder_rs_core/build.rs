use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut path = PathBuf::new();
    path.push(dir);
    path.push("src");
    path.push("resources");
    encounter_table_generator::gen3::emerald::encounters(false, path.clone());
    encounter_table_generator::gen3::frlg::encounters(false, path.clone());
    encounter_table_generator::gen3::rs::encounters(false, path.clone());
    encounter_table_generator::gen3::xd::encounters(false, path.clone());
    encounter_table_generator::gen4::dp::encounters(false, path.clone());
    encounter_table_generator::gen4::pt::encounters(path.clone());
    encounter_table_generator::gen4::hgss::encounters(false, path.clone());
    encounter_table_generator::gen4::hgss::safari(path.clone());
    encounter_table_generator::gen4::hgss::bug(path.clone());
    encounter_table_generator::gen4::hgss::headbutt(path.clone());
    encounter_table_generator::gen5::bw::encounters(false, path.clone());
    encounter_table_generator::gen5::bw2::encounters(false, path.clone());
    encounter_table_generator::gen5::bw2::hidden_grotto(path.clone());
    encounter_table_generator::gen8::bdsp::encounters(false, path.clone());
    encounter_table_generator::gen8::bdsp::underground(path.clone());

    encounter_table_generator::gen3::embed_encounters(path.clone());
    encounter_table_generator::gen4::embed_encounters(path.clone());
    encounter_table_generator::gen5::embed_encounters(path.clone());
    encounter_table_generator::gen8::embed_encounters(path);
}
