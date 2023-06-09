use crate::gen3::profile::ProfileManager3;
use crate::gen3::tools::Gen3Tools;
use crate::gen4::profile::ProfileManager4;
use crate::gen4::tools::Gen4Tools;
use crate::util::IVToPID;
use egui::Visuals;
#[cfg(not(target_arch = "wasm32"))]
use pokefinder_rs_core::parents::init_profile_loader;

#[derive(Default)]
pub struct PokeFinder {
    gen: Gen,
    ivs_to_pid: Option<Box<IVToPID>>,
    gen_3_tools: Gen3Tools,
    gen_4_tools: Gen4Tools,
    profile_manager3: ProfileManager3,
    profile_manager4: ProfileManager4,
}

#[derive(Default, PartialEq)]
enum Gen {
    #[default]
    Gen3,
    Gen4,
}

impl PokeFinder {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(Visuals::dark());
        #[cfg(not(target_arch = "wasm32"))]
        init_profile_loader(String::new());
        Default::default()
    }
}

impl eframe::App for PokeFinder {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            gen,
            ivs_to_pid,
            gen_3_tools,
            gen_4_tools,
            profile_manager3,
            profile_manager4,
        } = self;

        egui::Window::new("Main").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("Gen 3 Tools", |ui| {
                    if ui.button("IVs to PID").clicked() && ivs_to_pid.is_none() {
                        *ivs_to_pid = Some(Box::default());
                    }
                    if ui.button("PID to IVs").clicked() && gen_3_tools.pid_to_ivs.is_none() {
                        gen_3_tools.pid_to_ivs = Some(Box::default());
                    }
                    if ui.button("Profile Manager 3").clicked() {
                        profile_manager3.show = true;
                    }
                });
                ui.menu_button("Gen 4 Tools", |ui| {
                    if ui.button("IVs to PID").clicked() && ivs_to_pid.is_none() {
                        *ivs_to_pid = Some(Box::default());
                    }
                    if ui.button("Profile Manager 4").clicked() {
                        profile_manager4.show = true;
                    }
                    if ui.button("SID From Chained Shiny").clicked()
                        && gen_4_tools.chained_sid.is_none()
                    {
                        gen_4_tools.chained_sid = Some(Box::default());
                    }
                    if ui.button("Seed to Time").clicked() && gen_4_tools.seed_to_time.is_none() {
                        gen_4_tools.seed_to_time = Some(Box::default());
                    }
                });
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.selectable_value(gen, Gen::Gen3, "Gen 3");
                ui.selectable_value(gen, Gen::Gen4, "Gen 4");
            });
            ui.separator();
            ui.vertical_centered_justified(|ui| match gen {
                Gen::Gen3 => {
                    if ui.button("Egg").clicked() {}
                    ui.add_space(3.0);
                    if ui.button("GameCube").clicked() {}
                    ui.add_space(3.0);
                    if ui.button("IDs").clicked() {}
                    ui.add_space(3.0);
                    if ui.button("Static").clicked() {}
                    ui.add_space(3.0);
                    if ui.button("Wild").clicked() {}
                    ui.add_space(3.0);
                }
                Gen::Gen4 => {
                    if ui.button("Egg").clicked() {}
                    ui.add_space(3.0);
                    if ui.button("IDs").clicked() {}
                    ui.add_space(3.0);
                    if ui.button("Static").clicked() {}
                    ui.add_space(3.0);
                    if ui.button("Wild").clicked() {}
                    ui.add_space(3.0);
                }
            });
        });
        let close = ivs_to_pid
            .as_mut()
            .map(|ivs_to_pid| ivs_to_pid.show(ctx))
            .unwrap_or_default();
        if close {
            self.ivs_to_pid = None;
        }

        gen_3_tools.show(ctx);
        gen_4_tools.show(ctx);
        profile_manager3.show(ctx);
        profile_manager4.show(ctx);
    }
}
