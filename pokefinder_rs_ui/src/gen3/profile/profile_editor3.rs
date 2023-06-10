use crate::ProfileEditorResult;
use egui::{Context, Widget};
use pokefinder_rs_core::enums::Game;
use pokefinder_rs_core::gen3::Profile3;
use pokefinder_rs_core::util::translator;

pub struct ProfileEditor3 {
    pub profile: Profile3,
    default_result: ProfileEditorResult,
}

impl Default for ProfileEditor3 {
    fn default() -> Self {
        Self {
            profile: Default::default(),
            default_result: ProfileEditorResult::New,
        }
    }
}

impl ProfileEditor3 {
    pub fn edit(profile: &Profile3) -> Self {
        Self {
            profile: profile.clone(),
            default_result: ProfileEditorResult::Edit,
        }
    }

    pub fn show(&mut self, ctx: &Context) -> ProfileEditorResult {
        let mut result = ProfileEditorResult::Pending;

        egui::Window::new("Profile 3 Editor").show(ctx, |ui| {
            egui::Grid::new("profile_3_editor_grid")
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
                    egui::ComboBox::new("profile_editor_3_version", "")
                        .wrap(false)
                        .selected_text(translator::get_game(self.profile.version))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.profile.version, Game::RUBY, "Ruby");
                            ui.selectable_value(
                                &mut self.profile.version,
                                Game::SAPPHIRE,
                                translator::get_game(Game::SAPPHIRE),
                            );
                            ui.selectable_value(
                                &mut self.profile.version,
                                Game::FIRE_RED,
                                translator::get_game(Game::FIRE_RED),
                            );
                            ui.selectable_value(
                                &mut self.profile.version,
                                Game::LEAF_GREEN,
                                translator::get_game(Game::LEAF_GREEN),
                            );
                            ui.selectable_value(
                                &mut self.profile.version,
                                Game::EMERALD,
                                translator::get_game(Game::EMERALD),
                            );
                            ui.selectable_value(
                                &mut self.profile.version,
                                Game::GALES,
                                translator::get_game(Game::GALES),
                            );
                            ui.selectable_value(
                                &mut self.profile.version,
                                Game::COLOSSEUM,
                                translator::get_game(Game::COLOSSEUM),
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
                    if matches!(self.profile.version, Game::RUBY | Game::SAPPHIRE) {
                        ui.label("");
                        ui.label("");
                        ui.label("");
                        ui.checkbox(&mut self.profile.dead_battery, "Dead Battery");
                        ui.label("");
                        ui.end_row();
                    }
                });
        });

        result
    }
}
