use crate::ProfileEditorResult;
use egui::{Context, Widget};
use pokefinder_rs_core::enums::Game;
use pokefinder_rs_core::gen4::Profile4;
use pokefinder_rs_core::util::translator;

pub struct ProfileEditor4 {
    pub profile: Profile4,
    default_result: ProfileEditorResult,
}

impl Default for ProfileEditor4 {
    fn default() -> Self {
        Self {
            profile: Default::default(),
            default_result: ProfileEditorResult::New,
        }
    }
}

impl ProfileEditor4 {
    pub fn edit(profile: &Profile4) -> Self {
        Self {
            profile: profile.clone(),
            default_result: ProfileEditorResult::Edit,
        }
    }

    pub fn show(&mut self, ctx: &Context) -> ProfileEditorResult {
        let mut result = ProfileEditorResult::Pending;

        egui::Window::new("Profile 4 Editor").show(ctx, |ui| {
            egui::Grid::new("profile_4_editor_grid")
                .num_columns(5)
                .show(ui, |ui| {
                    ui.label("Profile Name");
                    ui.text_edit_singleline(&mut self.profile.name);
                    ui.label("TID");
                    egui::DragValue::new(&mut self.profile.tid)
                        .clamp_range(0..=65535)
                        .ui(ui);
                    if ui.add(egui::Button::new("Okay").wrap(false)).clicked() {
                        result = self.default_result;
                    }
                    ui.end_row();
                    ui.label("Version");
                    egui::ComboBox::new("profile_editor_4_version", "")
                        .wrap(false)
                        .selected_text(translator::get_game(self.profile.version))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.profile.version,
                                Game::DIAMOND,
                                "Diamond",
                            );
                            ui.selectable_value(&mut self.profile.version, Game::PEARL, "Pearl");
                            ui.selectable_value(
                                &mut self.profile.version,
                                Game::PLATINUM,
                                "Platinum",
                            );
                            ui.selectable_value(
                                &mut self.profile.version,
                                Game::HEART_GOLD,
                                "Heart Gold",
                            );
                            ui.selectable_value(
                                &mut self.profile.version,
                                Game::SOUL_SILVER,
                                "Soul Silver",
                            );
                        });
                    ui.label("SID");
                    egui::DragValue::new(&mut self.profile.sid)
                        .clamp_range(0..=65535)
                        .ui(ui);
                    if ui.add(egui::Button::new("Cancel").wrap(false)).clicked() {
                        result = ProfileEditorResult::Cancel;
                    }
                    ui.end_row();
                    ui.label("");
                    ui.label("");
                    ui.label("");
                    ui.checkbox(&mut self.profile.dex, "National Dex");
                    ui.label("");
                    ui.end_row();
                });
        });

        result
    }
}
