use crate::gen4::profile::ProfileEditor4;
use crate::ProfileEditorResult;
use egui::Context;
use egui_extras::Column;
use pokefinder_rs_core::gen4::Profile4;
use pokefinder_rs_core::parents::{profile_loader_4, Profile};
use pokefinder_rs_core::util::translator;

pub struct ProfileManager4 {
    pub show: bool,
    pub profiles: Vec<(Profile4, bool)>,
    pub edit: usize,
    pub editor: Option<Box<ProfileEditor4>>,
}

static HEADERS: [&str; 5] = ["Profile Name", "Version", "TID", "SID", "National Dex"];

impl Default for ProfileManager4 {
    fn default() -> Self {
        Self {
            profiles: profile_loader_4::get_profiles()
                .into_iter()
                .map(|p| (p, false))
                .collect(),
            show: false,
            edit: 0,
            editor: None,
        }
    }
}

impl ProfileManager4 {
    pub fn show(&mut self, ctx: &Context) {
        if self.show {
            egui::Window::new("Profile Manager 4").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("New").clicked() {
                        self.editor = Some(Box::default());
                    }
                    if ui.button("Edit").clicked() && !self.profiles.is_empty() {
                        self.editor =
                            Some(Box::new(ProfileEditor4::edit(&self.profiles[self.edit].0)));
                    }
                    if ui.button("Delete").clicked() {
                        self.profiles.retain(|(profile, delete)| {
                            if *delete {
                                profile_loader_4::remove_profile(profile);
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
                                    ui.label(if profile.get_national_dex() {
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
                    let editor = self.editor.take();
                    let profile = editor.unwrap().profile;
                    profile_loader_4::add_profile(profile.clone());
                    self.profiles[self.edit].0 = profile;
                }
                ProfileEditorResult::New => {
                    let editor = self.editor.take();
                    let profile = editor.unwrap().profile;
                    profile_loader_4::add_profile(profile.clone());
                    self.profiles.push((profile, false));
                }
                ProfileEditorResult::Cancel => {
                    self.editor = None;
                }
            }
        }
    }
}
