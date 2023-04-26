use crate::DialogResult;
use egui::Context;
use egui_extras::RetainedImage;
use once_cell::sync::Lazy;
use pokefinder_rs_core::gen4::SeedTimeCalibrate4;

pub struct SearchCoinFlips {
    pub show: bool,
    possible_results: String,
    input: String,
    possible: Vec<bool>,
}

impl Default for SearchCoinFlips {
    fn default() -> Self {
        Self {
            show: false,
            possible_results: "Possible Results: ".to_string(),
            input: "".to_string(),
            possible: vec![],
        }
    }
}

static HEADS_IMAGE: Lazy<RetainedImage> = Lazy::new(|| {
    RetainedImage::from_image_bytes(
        "search_coin_flips_heads_img",
        include_bytes!("../../../images/heads.png"),
    )
    .unwrap()
});

static TAILS_IMAGE: Lazy<RetainedImage> = Lazy::new(|| {
    RetainedImage::from_image_bytes(
        "search_coin_flips_tails_img",
        include_bytes!("../../../images/tails.png"),
    )
    .unwrap()
});

impl SearchCoinFlips {
    pub fn reset(&mut self) {
        self.input = String::new();
        self.possible = vec![];
        self.possible_results = "Possible Results: ".to_string();
    }

    pub fn get_results(&self) -> &[bool] {
        &self.possible
    }

    pub fn show(&mut self, ctx: &Context, data: &[SeedTimeCalibrate4]) -> DialogResult {
        if self.show {
            let mut dialog_result = DialogResult::default();
            let mut text_changed = false;
            egui::Window::new("Search Coin Flips").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui
                        .add(egui::ImageButton::new(
                            HEADS_IMAGE.texture_id(ctx),
                            HEADS_IMAGE.size_vec2(),
                        ))
                        .clicked()
                    {
                        self.input += if self.input.is_empty() { "H" } else { ", H" };
                        text_changed = true;
                    }
                    if ui
                        .add(egui::ImageButton::new(
                            TAILS_IMAGE.texture_id(ctx),
                            HEADS_IMAGE.size_vec2(),
                        ))
                        .clicked()
                    {
                        self.input += if self.input.is_empty() { "T" } else { ", T" };
                        text_changed = true;
                    }
                });
                ui.vertical_centered_justified(|ui| {
                    let mut text_edit = ui.text_edit_singleline(&mut self.input);
                    if text_changed {
                        text_edit.mark_changed();
                    }
                    if ui.button("Clear").clicked() {
                        self.reset();
                        text_edit.mark_changed();
                    }
                    if ui.button("Close").clicked() {
                        dialog_result = DialogResult::Okay;
                    }
                    if text_edit.changed() && !self.input.is_empty() {
                        let mut num = 0;
                        self.possible.clear();
                        let result = self.input.clone().replace([' ', ','], "");

                        for dt in data {
                            let compare = dt.get_sequence().replace([' ', ','], "");
                            let pass = compare.contains(&result);
                            self.possible.push(pass);
                            if pass {
                                num += 1;
                            }
                        }

                        self.possible_results = format!("Possible Results: {}", num);
                    }
                });
            });
            dialog_result
        } else {
            DialogResult::Pending
        }
    }
}
