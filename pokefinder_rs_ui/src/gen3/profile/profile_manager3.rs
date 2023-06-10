use crate::gen3::profile::ProfileEditor3;
use crate::ProfileEditorResult;
use egui::Context;
use egui_extras::Column;
use pokefinder_rs_core::gen3::Profile3;
use pokefinder_rs_core::parents::{profile_loader_3, Profile};
use pokefinder_rs_core::util::translator;

pub struct ProfileManager3 {
    pub show: bool,
    pub profiles: Vec<(Profile3, bool)>,
    pub edit: usize,
    pub editor: Option<Box<ProfileEditor3>>,
}

static HEADERS: [&str; 5] = ["Profile Name", "Version", "TID", "SID", "Dead Battery"];

impl Default for ProfileManager3 {
    fn default() -> Self {
        Self {
            show: false,
            profiles: profile_loader_3::get_profiles()
                .into_iter()
                .map(|p| (p, false))
                .collect(),
            edit: 0,
            editor: None,
        }
    }
}

impl ProfileManager3 {
    pub fn show(&mut self, ctx: &Context) {
        if self.show {
            egui::Window::new("Profile Manager 3").show(ctx, |ui| {
                ui.style_mut().wrap = Some(false);
                ui.horizontal(|ui| {
                    if ui.button("New").clicked() {
                        self.editor = Some(Box::default());
                    }
                    if ui.button("Edit").clicked() && !self.profiles.is_empty() {
                        self.editor =
                            Some(Box::new(ProfileEditor3::edit(&self.profiles[self.edit].0)));
                    }
                    if ui.button("Delete").clicked() {
                        self.profiles.retain(|(profile, delete)| {
                            if *delete {
                                profile_loader_3::remove_profile(profile);
                                false
                            } else {
                                true
                            }
                        });
                    }
                });
                ui.vertical_centered_justified(|ui| {
                    egui_extras::TableBuilder::new(ui)
                        .columns(Column::auto(), HEADERS.len() + 2)
                        .resizable(true)
                        .max_scroll_height(150.0)
                        .auto_shrink([false; 2])
                        .striped(true)
                        .header(20.0, |mut header| {
                            for head in HEADERS {
                                header.col(|ui| {
                                    ui.heading(head);
                                });
                            }
                            header.col(|ui| {
                                ui.heading("Edit");
                            });
                            header.col(|ui| {
                                ui.heading("Delete");
                            });
                        })
                        .body(|body| {
                            body.rows(15.0, self.profiles.len(), |i, mut row| {
                                let (profile, delete) = &mut self.profiles[i];
                                row.col(|ui| {
                                    ui.label(profile.get_name());
                                });
                                row.col(|ui| {
                                    ui.label(translator::get_game(profile.get_version()));
                                });
                                row.col(|ui| {
                                    ui.label(profile.get_tid().to_string());
                                });
                                row.col(|ui| {
                                    ui.label(profile.get_sid().to_string());
                                });
                                row.col(|ui| {
                                    ui.label(if profile.get_dead_battery() {
                                        "Yes"
                                    } else {
                                        "No"
                                    });
                                });
                                row.col(|ui| {
                                    ui.radio_value(&mut self.edit, i, "");
                                });
                                row.col(|ui| {
                                    ui.checkbox(delete, "");
                                });
                            });
                        });
                    if ui.button("Done").clicked() {
                        self.show = false;
                    }
                });
            });

            let editor_result = self
                .editor
                .as_mut()
                .map(|editor| editor.show(ctx))
                .unwrap_or_default();
            match editor_result {
                ProfileEditorResult::Pending => {}
                ProfileEditorResult::Edit => {
                    profile_loader_3::remove_profile(&self.profiles[self.edit].0);
                    let editor = self.editor.take();
                    let profile = editor.unwrap().profile;
                    profile_loader_3::add_profile(profile.clone());
                    self.profiles[self.edit].0 = profile;
                }
                ProfileEditorResult::New => {
                    let editor = self.editor.take();
                    let profile = editor.unwrap().profile;
                    profile_loader_3::add_profile(profile.clone());
                    self.profiles.push((profile, false));
                }
                ProfileEditorResult::Cancel => {
                    self.editor = None;
                }
            }
        }
    }
}
