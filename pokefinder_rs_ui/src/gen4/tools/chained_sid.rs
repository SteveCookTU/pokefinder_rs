use egui::{Context, Widget};
use egui_extras::Column;
use pokefinder_rs_core::enums::Game;
use pokefinder_rs_core::gen4::ChainedSIDCalc;
use pokefinder_rs_core::parents::personal_loader;
use pokefinder_rs_core::util::translator;

pub struct ChainedSID {
    pokemon: u16,
    ability: usize,
    gender: u8,
    nature: u8,
    tid: u16,
    hp: u8,
    atk: u8,
    def: u8,
    spa: u8,
    spd: u8,
    spe: u8,
    entries: Vec<Entry>,
    calc: Option<ChainedSIDCalc>,
}

struct Entry {
    ivs: String,
    ability: &'static str,
    gender: &'static str,
    nature: &'static str,
}

impl Default for ChainedSID {
    fn default() -> Self {
        Self {
            pokemon: 1,
            ability: 0,
            gender: 0,
            nature: 0,
            tid: 0,
            hp: 0,
            atk: 0,
            def: 0,
            spa: 0,
            spd: 0,
            spe: 0,
            entries: vec![],
            calc: Default::default(),
        }
    }
}

static HEADERS: [&str; 4] = ["IVs", "Ability", "Gender", "Nature"];

impl ChainedSID {
    pub fn show(&mut self, ctx: &Context) -> bool {
        let mut close = false;

        egui::Window::new("Chained Shiny to SID").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Pokemon");
                    egui::ComboBox::new("chained_sid_pokemon", "")
                        .selected_text(if self.pokemon == 0 {
                            ""
                        } else {
                            translator::get_specie(self.pokemon)
                        })
                        .show_ui(ui, |ui| {
                            ui.style_mut().wrap = Some(false);
                            for i in 1..493 {
                                ui.selectable_value(
                                    &mut self.pokemon,
                                    i,
                                    translator::get_specie(i),
                                );
                            }
                        });
                    ui.add_space(5.0);
                    ui.label("Nature");
                    egui::ComboBox::new("chained_sid_nature", "")
                        .selected_text(&translator::get_natures()[self.nature as usize])
                        .show_ui(ui, |ui| {
                            for (i, nature) in translator::get_natures().iter().enumerate() {
                                ui.selectable_value(&mut self.nature, i as u8, nature);
                            }
                        });
                    ui.add_space(5.0);

                    let info = personal_loader::get_personal_info(Game::DPPT, self.pokemon, 0);

                    ui.label("Gender");
                    egui::ComboBox::new("chained_sid_gender", "")
                        .selected_text(translator::get_gender(self.gender))
                        .show_ui(ui, |ui| match info.get_gender() {
                            255 => {
                                ui.selectable_value(&mut self.gender, 2, translator::get_gender(2));
                            }
                            254 => {
                                ui.selectable_value(&mut self.gender, 1, translator::get_gender(1));
                            }
                            0 => {
                                ui.selectable_value(&mut self.gender, 0, translator::get_gender(0));
                            }
                            _ => {
                                ui.selectable_value(&mut self.gender, 0, translator::get_gender(0));
                                ui.selectable_value(&mut self.gender, 1, translator::get_gender(1));
                            }
                        });
                    ui.add_space(5.0);
                    ui.label("Ability");
                    egui::ComboBox::new("chained_sid_ability", "")
                        .selected_text(translator::get_ability(info.get_ability(self.ability)))
                        .show_ui(ui, |ui| {
                            if info.get_ability(0) == info.get_ability(1) {
                                ui.selectable_value(
                                    &mut self.ability,
                                    0,
                                    translator::get_ability(info.get_ability(0)),
                                );
                            } else {
                                ui.selectable_value(
                                    &mut self.ability,
                                    0,
                                    translator::get_ability(info.get_ability(0)),
                                );
                                ui.selectable_value(
                                    &mut self.ability,
                                    1,
                                    translator::get_ability(info.get_ability(1)),
                                );
                            }
                        });
                    ui.add_space(5.0);
                    ui.label("TID");
                    ui.add_enabled(
                        self.calc.is_none(),
                        egui::DragValue::new(&mut self.tid).clamp_range(0..=65535),
                    );
                    ui.add_space(5.0);
                });
                ui.add_space(80.0);
                ui.vertical(|ui| {
                    egui::Grid::new("chained_sid_grid")
                        .num_columns(2)
                        .spacing((20.0, 20.0))
                        .show(ui, |ui| {
                            ui.label("HP");
                            egui::DragValue::new(&mut self.hp)
                                .clamp_range(0..=31)
                                .ui(ui);
                            ui.end_row();
                            ui.label("Atk");
                            egui::DragValue::new(&mut self.atk)
                                .clamp_range(0..=31)
                                .ui(ui);
                            ui.end_row();
                            ui.label("Def");
                            egui::DragValue::new(&mut self.def)
                                .clamp_range(0..=31)
                                .ui(ui);
                            ui.end_row();
                            ui.label("SpA");
                            egui::DragValue::new(&mut self.spa)
                                .clamp_range(0..=31)
                                .ui(ui);
                            ui.end_row();
                            ui.label("SpD");
                            egui::DragValue::new(&mut self.spd)
                                .clamp_range(0..=31)
                                .ui(ui);
                            ui.end_row();
                            ui.label("Spe");
                            egui::DragValue::new(&mut self.spe)
                                .clamp_range(0..=31)
                                .ui(ui);
                            ui.end_row();
                        });
                });
            });
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    if let Some(calc) = self.calc.as_ref() {
                        if calc.sids.len() == 1 {
                            ui.label(format!("SID Found: {}", calc.sids[0]));
                        } else {
                            ui.label(format!("Possible Results: {}", calc.sids.len()));
                        }
                    } else {
                        ui.label("Possible Results: 8192");
                    }
                    ui.add_space(30.0);
                    if ui.button("Calculate").clicked() {
                        if self.calc.is_none() {
                            self.calc = Some(ChainedSIDCalc::new(self.tid));
                        }

                        if let Some(calc) = self.calc.as_mut() {
                            let info =
                                personal_loader::get_personal_info(Game::DPPT, self.pokemon, 0);
                            self.entries.push(Entry {
                                ivs: format!(
                                    "{}.{}.{}.{}.{}.{}",
                                    self.hp, self.atk, self.def, self.spa, self.spd, self.spe
                                ),
                                ability: translator::get_ability(info.get_ability(self.ability)),
                                gender: translator::get_gender(self.gender),
                                nature: &translator::get_natures()[self.nature as usize],
                            });
                            calc.add_entry(
                                self.hp,
                                self.atk,
                                self.def,
                                self.spa,
                                self.spd,
                                self.spe,
                                info.get_ability(self.ability),
                                self.gender,
                                self.nature,
                                info,
                            );
                            self.hp = 0;
                            self.atk = 0;
                            self.def = 0;
                            self.spa = 0;
                            self.spd = 0;
                            self.spe = 0;
                            self.nature = 0;
                            self.ability = 0;
                            self.gender = 0;
                        }
                    }
                    if ui.button("Clear").clicked() {
                        if self.calc.is_some() {
                            self.calc = None;
                        }
                        self.entries.clear();
                    }
                });
            });
            ui.vertical_centered_justified(|ui| {
                egui_extras::TableBuilder::new(ui)
                    .columns(Column::auto().clip(false), 4)
                    .auto_shrink([false; 2])
                    .striped(true)
                    .max_scroll_height(150.0)
                    .resizable(true)
                    .header(20.0, |mut header| {
                        for head in HEADERS {
                            header.col(|ui| {
                                ui.heading(head);
                            });
                        }
                    })
                    .body(|body| {
                        body.rows(15.0, self.entries.len(), |i, mut row| {
                            let entry = &self.entries[i];
                            row.col(|ui| {
                                ui.label(&entry.ivs);
                            });
                            row.col(|ui| {
                                ui.label(entry.ability);
                            });
                            row.col(|ui| {
                                ui.label(entry.gender);
                            });
                            row.col(|ui| {
                                ui.label(entry.nature);
                            });
                        });
                    });
                if ui.button("Close").clicked() {
                    close = true;
                }
            });
        });

        close
    }
}
