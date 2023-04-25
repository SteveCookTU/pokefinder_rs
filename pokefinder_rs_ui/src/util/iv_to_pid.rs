use egui::{Context, Widget};
use egui_extras::Column;
use pokefinder_rs_core::enums::Method;
use pokefinder_rs_core::parents::states::IVtoPIDState;
use pokefinder_rs_core::util::{iv_to_pid_calculator, translator};

#[derive(Default)]
pub struct IVToPID {
    nature: u8,
    tid: u16,
    hp: u8,
    atk: u8,
    def: u8,
    spa: u8,
    spd: u8,
    spe: u8,
    results: Vec<Result>,
}

struct Result {
    seed: String,
    pid: String,
    method: &'static str,
    ability: String,
    eighth: &'static str,
    quarter: &'static str,
    half: &'static str,
    three_quarter: &'static str,
    sid: String,
}

impl From<IVtoPIDState> for Result {
    fn from(value: IVtoPIDState) -> Self {
        Self {
            seed: format!("{:0>8X}", value.get_seed()),
            pid: format!("{:0>8X}", value.get_pid()),
            method: match value.get_method() {
                Method::Method1 => "Method 1",
                Method::Method1Reverse => "Reverse Method 1",
                Method::Method2 => "Method 2",
                Method::Method4 => "Method 4",
                Method::CuteCharmDPPt => "Cute Charm (DPPt)",
                Method::CuteCharmHGSS => "Cute Charm (HGSS)",
                Method::XDColo => "XD/Colo",
                Method::Channel => "Channel",
                _ => "",
            },
            ability: format!("{}", value.get_pid() & 1),
            eighth: if value.get_pid() & 0xFF > 30 {
                "♂"
            } else {
                "♀"
            },
            quarter: if value.get_pid() & 0xFF > 63 {
                "♂"
            } else {
                "♀"
            },
            half: if value.get_pid() & 0xFF > 126 {
                "♂"
            } else {
                "♀"
            },
            three_quarter: if value.get_pid() & 0xFF > 190 {
                "♂"
            } else {
                "♀"
            },
            sid: value.get_sid().to_string(),
        }
    }
}

static HEADERS: [&str; 9] = [
    "Seed", "PID", "Method", "Ability", "12.5%", "25%", "50%", "75%", "SID",
];

impl IVToPID {
    pub fn show(&mut self, ctx: &Context) -> bool {
        let mut close = false;
        egui::Window::new("IVs to PID").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Nature");
                    egui::ComboBox::new("ivs_to_pid_nature", "")
                        .selected_text(&translator::get_natures()[self.nature as usize])
                        .show_ui(ui, |ui| {
                            for (i, nature) in translator::get_natures().iter().enumerate() {
                                ui.selectable_value(&mut self.nature, i as u8, nature);
                            }
                        });
                });
                ui.add_space(10.0);
                ui.vertical(|ui| {
                    ui.label("TID");
                    egui::DragValue::new(&mut self.tid)
                        .clamp_range(0..=65565)
                        .ui(ui);
                });
            });
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("HP");
                    egui::DragValue::new(&mut self.hp)
                        .clamp_range(0..=31)
                        .ui(ui);
                });
                ui.vertical(|ui| {
                    ui.label("Atk");
                    egui::DragValue::new(&mut self.atk)
                        .clamp_range(0..=31)
                        .ui(ui);
                });
                ui.vertical(|ui| {
                    ui.label("Def");
                    egui::DragValue::new(&mut self.def)
                        .clamp_range(0..=31)
                        .ui(ui);
                });
                ui.vertical(|ui| {
                    ui.label("SpA");
                    egui::DragValue::new(&mut self.spa)
                        .clamp_range(0..=31)
                        .ui(ui);
                });
                ui.vertical(|ui| {
                    ui.label("SpD");
                    egui::DragValue::new(&mut self.spd)
                        .clamp_range(0..=31)
                        .ui(ui);
                });
                ui.vertical(|ui| {
                    ui.label("Spe");
                    egui::DragValue::new(&mut self.spe)
                        .clamp_range(0..=31)
                        .ui(ui);
                });
            });
            ui.add_space(10.0);
            ui.vertical_centered_justified(|ui| {
                if ui.button("Find").clicked() {
                    self.find();
                }
                ui.add_space(10.0);
                egui_extras::TableBuilder::new(ui)
                    .columns(Column::auto(), 9)
                    .auto_shrink([false; 2])
                    .max_scroll_height(200.0)
                    .resizable(true)
                    .header(20.0, |mut header| {
                        for head in HEADERS {
                            header.col(|ui| {
                                ui.heading(head);
                            });
                        }
                    })
                    .body(|body| {
                        body.rows(15.0, self.results.len(), |i, mut row| {
                            let res = &self.results[i];
                            row.col(|ui| {
                                ui.label(&res.seed);
                            });
                            row.col(|ui| {
                                ui.label(&res.pid);
                            });
                            row.col(|ui| {
                                ui.label(res.method);
                            });
                            row.col(|ui| {
                                ui.label(&res.ability);
                            });
                            row.col(|ui| {
                                ui.label(res.eighth);
                            });
                            row.col(|ui| {
                                ui.label(res.quarter);
                            });
                            row.col(|ui| {
                                ui.label(res.half);
                            });
                            row.col(|ui| {
                                ui.label(res.three_quarter);
                            });
                            row.col(|ui| {
                                ui.label(&res.sid);
                            });
                        });
                    });
                ui.add_space(10.0);
                if ui.button("Close").clicked() {
                    close = true;
                }
            });
        });
        close
    }

    fn find(&mut self) {
        self.results = iv_to_pid_calculator::calculate_pids(
            self.hp,
            self.atk,
            self.def,
            self.spa,
            self.spd,
            self.spe,
            self.nature,
            self.tid,
        )
        .into_iter()
        .map(|r| r.into())
        .collect();
    }
}
