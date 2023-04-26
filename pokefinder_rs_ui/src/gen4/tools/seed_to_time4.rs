use crate::gen4::tools::SearchCoinFlips;
use crate::DialogResult;
use egui::{Context, Direction, Layout, Widget};
use egui_extras::Column;
use pokefinder_rs_core::enums::Game;
use pokefinder_rs_core::gen4::{seed_to_time_calculator4, SeedTime4, SeedTimeCalibrate4};
use pokefinder_rs_core::util::utilities4;

pub struct SeedToTime4 {
    mode: Game,
    dppt: DPPtData,
    hgss: HGSSData,
    search_coin_slips: SearchCoinFlips,
}

impl Default for SeedToTime4 {
    fn default() -> Self {
        Self {
            mode: Game::DPPT,
            dppt: Default::default(),
            hgss: Default::default(),
            search_coin_slips: SearchCoinFlips::default(),
        }
    }
}

struct DPPtData {
    generate_view: Vec<SearchResult>,
    generate: Vec<SeedTime4>,
    target: usize,
    calibrate_view: Vec<CalibrateResult>,
    calibrate: Vec<SeedTimeCalibrate4>,
    seed: u32,
    year: u16,
    use_second: bool,
    second: u8,
    delay_minus: i32,
    delay_plus: i32,
    second_minus: i32,
    second_plus: i32,
    coin_flips: String,
}

impl Default for DPPtData {
    fn default() -> Self {
        Self {
            generate_view: vec![],
            generate: vec![],
            target: 0,
            calibrate_view: vec![],
            calibrate: vec![],
            seed: 0,
            year: 0,
            use_second: false,
            second: 0,
            delay_minus: -10,
            delay_plus: 10,
            second_minus: -1,
            second_plus: 1,
            coin_flips: "Coin Flips: ".to_string(),
        }
    }
}

struct HGSSData {
    generate: Vec<SearchResult>,
    calibrate: Vec<CalibrateResult>,
    seed: u32,
    year: u16,
    second: u8,
    delay_minus: i32,
    delay_plus: i32,
    second_minus: i32,
    second_plus: i32,
    raikou: bool,
    entei: bool,
    lati: bool,
    raiku_num: u8,
    entei_num: u8,
    lati_num: u8,
}

impl Default for HGSSData {
    fn default() -> Self {
        Self {
            generate: vec![],
            calibrate: vec![],
            seed: 0,
            year: 2000,
            second: 0,
            delay_minus: -10,
            delay_plus: 10,
            second_minus: -1,
            second_plus: 1,
            raikou: false,
            entei: false,
            lati: false,
            raiku_num: 0,
            entei_num: 0,
            lati_num: 0,
        }
    }
}

struct SearchResult {
    seed: String,
    date_time: String,
}

impl From<SeedTime4> for SearchResult {
    fn from(value: SeedTime4) -> Self {
        Self {
            seed: value.date_time.to_string(),
            date_time: value.delay.to_string(),
        }
    }
}

struct CalibrateResult {
    seed: String,
    date_time: String,
    delay: String,
    coin_flips: String,
    calls: String,
    roamer_locations: String,
}

impl From<SeedTimeCalibrate4> for CalibrateResult {
    fn from(value: SeedTimeCalibrate4) -> Self {
        Self {
            seed: format!("{:0>8X}", value.seed),
            date_time: value.base.date_time.to_string(),
            delay: value.base.delay.to_string(),
            coin_flips: value.get_sequence(),
            calls: value.get_sequence(),
            roamer_locations: if let Some(roamer) = value.roamer {
                roamer.get_route_string()
            } else {
                String::new()
            },
        }
    }
}

static SEARCH_HEADERS: [&str; 3] = ["Seed", "Date/Time", "Target"];
static CALIBRATE_HEADERS: [&str; 6] = [
    "Seed",
    "Date/Time",
    "Delay",
    "Coin Flips",
    "Calls",
    "Roamer Locations",
];

impl SeedToTime4 {
    pub fn show(&mut self, ctx: &Context) -> bool {
        let mut close = false;

        egui::Window::new("Gen 4 Seed to Time")
            .resizable(true)
            .show(ctx, |ui| {
                ui.style_mut().wrap = Some(false);
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.mode, Game::DPPT, "DPPt");
                    ui.selectable_value(&mut self.mode, Game::HGSS, "HGSS");
                });
                ui.separator();
                ui.heading("Search");
                ui.separator();
                if self.mode == Game::DPPT {
                    ui.horizontal(|ui| {
                        ui.label("Seed");
                        egui::DragValue::new(&mut self.dppt.seed)
                            .hexadecimal(8, false, true)
                            .clamp_range(0..=u32::MAX)
                            .ui(ui);
                        ui.add_space(5.0);
                        ui.label("Year");
                        egui::DragValue::new(&mut self.dppt.year)
                            .clamp_range(2000..=2099)
                            .ui(ui);
                        ui.add_space(5.0);
                        ui.checkbox(&mut self.dppt.use_second, "Second");
                        ui.add_enabled(
                            self.dppt.use_second,
                            egui::DragValue::new(&mut self.dppt.second).clamp_range(0..=59),
                        );
                        ui.add_space(5.0);
                        if ui.button("Generate").clicked() {
                            self.search_coin_slips.reset();
                            self.dppt.target = 0;
                            self.dppt.generate = seed_to_time_calculator4::calculate_times(
                                self.dppt.seed,
                                self.dppt.year,
                                self.dppt.use_second,
                                self.dppt.second,
                            );
                            self.dppt.generate_view = self
                                .dppt
                                .generate
                                .clone()
                                .into_iter()
                                .map(|r| r.into())
                                .collect();
                            self.dppt.coin_flips =
                                format!("Coin Flips: {}", utilities4::coin_flips(self.dppt.seed));
                        }
                    });
                    ui.label(&self.dppt.coin_flips);
                    ui.vertical_centered(|ui| {
                        ui.push_id("seed_to_time4_dppt_search_table", |ui| {
                            egui_extras::TableBuilder::new(ui)
                                .columns(Column::auto(), 3)
                                .striped(true)
                                .resizable(true)
                                .auto_shrink([false; 2])
                                .max_scroll_height(150.0)
                                .header(20.0, |mut header| {
                                    for head in SEARCH_HEADERS {
                                        header.col(|ui| {
                                            ui.heading(head);
                                        });
                                    }
                                })
                                .body(|body| {
                                    body.rows(15.0, self.dppt.generate_view.len(), |i, mut row| {
                                        let r = &self.dppt.generate_view[i];
                                        row.col(|ui| {
                                            ui.label(&r.seed);
                                        });
                                        row.col(|ui| {
                                            ui.label(&r.date_time);
                                        });
                                        row.col(|ui| {
                                            ui.radio_value(&mut self.dppt.target, i, "");
                                        });
                                    });
                                });
                        });
                    });
                } else {
                }

                ui.separator();
                ui.heading("Calibrate");
                ui.separator();
                if self.mode == Game::DPPT {
                    egui::Grid::new("seed_to_time4_dppt_calibrate_params")
                        .num_columns(8)
                        .show(ui, |ui| {
                            ui.label("");
                            ui.with_layout(
                                Layout::centered_and_justified(Direction::LeftToRight),
                                |ui| {
                                    ui.label("-");
                                },
                            );
                            ui.with_layout(
                                Layout::centered_and_justified(Direction::LeftToRight),
                                |ui| {
                                    ui.label("+");
                                },
                            );
                            ui.label("");
                            ui.with_layout(
                                Layout::centered_and_justified(Direction::LeftToRight),
                                |ui| {
                                    ui.label("-");
                                },
                            );
                            ui.with_layout(
                                Layout::centered_and_justified(Direction::LeftToRight),
                                |ui| {
                                    ui.label("+");
                                },
                            );
                            ui.end_row();
                            ui.label("Delay");
                            egui::DragValue::new(&mut self.dppt.delay_minus)
                                .clamp_range(i32::MIN..=0)
                                .ui(ui);
                            egui::DragValue::new(&mut self.dppt.delay_plus)
                                .clamp_range(0..=i32::MAX)
                                .ui(ui);
                            ui.label("Second");
                            egui::DragValue::new(&mut self.dppt.second_minus)
                                .clamp_range(-59..=0)
                                .ui(ui);
                            egui::DragValue::new(&mut self.dppt.second_plus)
                                .clamp_range(0..=59)
                                .ui(ui);
                            if ui.button("Search Flips").clicked() {
                                self.search_coin_slips.show = true;
                            }
                            if ui.button("Generate").clicked()
                                && self.dppt.target < self.dppt.generate.len()
                            {
                                self.search_coin_slips.reset();
                                let target = &self.dppt.generate[self.dppt.target];
                                self.dppt.calibrate = seed_to_time_calculator4::calibrate(
                                    self.dppt.delay_minus,
                                    self.dppt.delay_plus,
                                    self.dppt.second_minus,
                                    self.dppt.second_plus,
                                    target,
                                );
                                self.dppt.calibrate_view = self
                                    .dppt
                                    .calibrate
                                    .clone()
                                    .into_iter()
                                    .map(|res| res.into())
                                    .collect();
                            }
                        });
                    ui.vertical_centered_justified(|ui| {
                        ui.push_id("seed_to_time4_dppt_calibrate_table", |ui| {
                            egui_extras::TableBuilder::new(ui)
                                .columns(Column::auto(), 4)
                                .resizable(true)
                                .auto_shrink([false; 2])
                                .max_scroll_height(150.0)
                                .header(20.0, |mut header| {
                                    for &head in CALIBRATE_HEADERS.iter().take(4) {
                                        header.col(|ui| {
                                            ui.heading(head);
                                        });
                                    }
                                })
                                .body(|body| {
                                    body.rows(
                                        15.0,
                                        if self.search_coin_slips.get_results().is_empty() {
                                            self.dppt.calibrate.len()
                                        } else {
                                            self.search_coin_slips
                                                .get_results()
                                                .iter()
                                                .filter(|&&poss| poss)
                                                .count()
                                        },
                                        |i, mut row| {
                                            let (r, _) =
                                                if !self.search_coin_slips.get_results().is_empty()
                                                {
                                                    self.dppt
                                                        .calibrate_view
                                                        .iter()
                                                        .zip(self.search_coin_slips.get_results())
                                                        .filter(|(_, &pass)| pass)
                                                        .nth(i)
                                                        .unwrap()
                                                } else {
                                                    (&self.dppt.calibrate_view[i], &true)
                                                };
                                            row.col(|ui| {
                                                ui.label(&r.seed);
                                            });
                                            row.col(|ui| {
                                                ui.label(&r.date_time);
                                            });
                                            row.col(|ui| {
                                                ui.label(&r.delay);
                                            });
                                            row.col(|ui| {
                                                ui.label(&r.coin_flips);
                                            });
                                        },
                                    );
                                });
                        });
                    });
                } else {
                }
                ui.vertical_centered_justified(|ui| {
                    if ui.button("Close").clicked() {
                        close = true;
                    }
                });
            });

        match self.search_coin_slips.show(ctx, &self.dppt.calibrate) {
            DialogResult::Pending => {}
            DialogResult::Okay | DialogResult::Cancel => {
                self.search_coin_slips.show = false;
            }
        };

        close
    }
}
