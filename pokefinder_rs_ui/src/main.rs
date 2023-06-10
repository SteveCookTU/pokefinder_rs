#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use serde::{Deserialize, Serialize};
use pokefinder_rs_core::enums::{Game, Method};
use pokefinder_rs_core::gen3::filters::StateFilter3;
use pokefinder_rs_core::gen3::generators::EggGenerator3;
use pokefinder_rs_core::gen3::Profile3;
use pokefinder_rs_core::gen3::states::EggState3;
use pokefinder_rs_core::gen4::filters::StateFilter4;
use pokefinder_rs_core::gen4::generators::EggGenerator4;
use pokefinder_rs_core::gen4::Profile4;
use pokefinder_rs_core::gen4::states::EggGeneratorState4;
use pokefinder_rs_core::parents::Daycare;
use pokefinder_rs_ui::app::PokeFinder;

#[derive(Deserialize, Serialize)]
struct JsonResult {
    ability: u8,
    #[serde(rename = "abilityIndex")]
    ability_index: u16,
    advances: u32,
    call: u8,
    characteristic: u8,
    chatot: u8,
    gender: u8,
    #[serde(rename = "hiddenPower")]
    hidden_power: u8,
    #[serde(rename = "hiddenPowerStrength")]
    hidden_power_strength: u8,
    inheritance: [u8; 6],
    ivs: [u8; 6],
    level: u8,
    nature: u8,
    #[serde(rename = "pickupAdvances")]
    pickup_advances: u32,
    pid: u32,
    shiny: u8,
    stats: [u16; 6]
}

impl From<EggGeneratorState4> for JsonResult {
    fn from(value: EggGeneratorState4) -> Self {
        Self {
            ability: value.base.base.base.ability,
            ability_index: value.base.base.base.ability_index,
            advances: value.base.advances,
            call: value.call,
            characteristic: value.base.base.base.characteristic,
            chatot: value.chatot,
            gender: value.base.base.base.gender,
            hidden_power: value.base.base.base.hidden_power,
            hidden_power_strength: value.base.base.base.hidden_power_strength,
            inheritance: value.base.base.inheritance,
            ivs: value.base.base.base.ivs,
            level: value.base.base.base.level,
            nature: value.base.base.base.nature,
            pickup_advances: value.pickup_advances,
            pid: value.base.base.base.pid,
            shiny: value.base.base.base.shiny,
            stats: value.base.base.base.stats,
        }
    }
}

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let file = OpenOptions::new().write(true).create(true).truncate(true).open("results.json").unwrap();
    let mut writer = BufWriter::new(file);

    let seed = 0;
    let seed_pickup = 0;
    let version = 1024;
    let pokemon = 314;
    let parent_ivs = [[31, 31, 31, 31, 31, 31], [31, 31, 31, 31, 31 , 31]];
    let parent_ability = [0, 1];
    let parent_gender = [0, 1];
    let parent_item = [0, 0];
    let masuda = false;
    let parent_nature = [0, 0];

    let min = [0; 6];
    let max = [31; 6];
    let natures = [true; 25];
    let powers = [true; 16];
    let profile = Profile4::new(
        "-".to_string(),
        Game::from_bits_retain(version),
        12345,
        54321,
        false,
    );

    let daycare = Daycare::new(
        parent_ivs,
        parent_ability,
        parent_gender,
        parent_item,
        parent_nature,
        pokemon,
        masuda,
    );
    let filter = StateFilter4::new(255, 255, 255, false, min, max, natures, powers);
    let generator = EggGenerator4::new(0, 9, 0, 0, 9, 0, &daycare, &profile, &filter);

    let states = generator.generate(seed, seed_pickup);


    let mut results = vec![];

    states.into_iter().for_each(|s| {
        let result = JsonResult::from(s);
        results.push(result);
    });

    serde_json::to_writer_pretty(writer, &results).unwrap();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| Box::new(PokeFinder::new(cc))),
    )
}

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn main() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "pokefinder_rs_ui", // hardcode it
            web_options,
            Box::new(|cc| Box::new(PokeFinder::new(cc))),
        )
        .await
        .expect("failed to start eframe");
    });
}
