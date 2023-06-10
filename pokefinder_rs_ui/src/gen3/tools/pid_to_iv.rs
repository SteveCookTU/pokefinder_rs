use egui::{Context, Widget};
use egui_extras::Column;
use pokefinder_rs_core::enums::Method;
use pokefinder_rs_core::gen3::tools::pid_to_iv_calculator;

const HEADERS: [&str; 3] = ["Seed", "Method", "IVs"];

#[derive(Default)]
pub struct PIDToIV {
    pid: u32,
    results: Vec<PIDToIVResult>,
}

#[derive(Clone, Default)]
pub struct PIDToIVResult {
    seed: String,
    method: &'static str,
    ivs: String,
}

impl PIDToIV {
    pub fn show(&mut self, ctx: &Context) -> bool {
        let mut close = false;

        egui::Window::new("PID to IVs").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("PID");
                egui::DragValue::new(&mut self.pid)
                    .clamp_range(0..=u32::MAX)
                    .hexadecimal(8, true, true)
                    .ui(ui);
                ui.add_space(5.0);
                if ui.button("Generate").clicked() {
                    self.results.clear();
                    self.results = pid_to_iv_calculator::calculate_ivs(self.pid)
                        .into_iter()
                        .map(|state| PIDToIVResult {
                            seed: format!("{:0>8X}", state.seed),
                            method: match state.method {
                                Method::Method1 => "Method 1",
                                Method::Method2 => "Method 2",
                                Method::Method4 => "Method 4",
                                Method::XDColo => "XD/Colo",
                                Method::Channel => "Channel",
                                _ => {
                                    unreachable!()
                                }
                            },
                            ivs: format!(
                                "{}/{}/{}/{}/{}/{}",
                                state.ivs[0],
                                state.ivs[1],
                                state.ivs[2],
                                state.ivs[3],
                                state.ivs[4],
                                state.ivs[5]
                            ),
                        })
                        .collect()
                }
            });
            ui.vertical_centered_justified(|ui| {
                egui_extras::TableBuilder::new(ui)
                    .columns(Column::auto().clip(false), 3)
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
                        body.rows(15.0, self.results.len(), |i, mut row| {
                            let result = &self.results[i];
                            row.col(|ui| {
                                ui.label(&result.seed);
                            });
                            row.col(|ui| {
                                ui.label(result.method);
                            });
                            row.col(|ui| {
                                ui.label(&result.ivs);
                            });
                        })
                    });
                if ui.button("Close").clicked() {
                    close = true;
                }
            });
        });

        close
    }
}
