use crate::gen4::profile::ProfileManager4;
use crate::gen4::tools::Gen4Tools;
use egui::Visuals;
#[cfg(not(target_arch = "wasm32"))]
use pokefinder_rs_core::parents::init_profile_loader;

#[derive(Default)]
pub struct PokeFinder {
    gen: Gen,
    gen_4_tools: Gen4Tools,
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
            gen_4_tools,
            profile_manager4,
        } = self;

        egui::Window::new("Main").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("Gen 3 Tools", |_ui| {});
                ui.menu_button("Gen 4 Tools", |ui| {
                    if ui.button("IVs to PID").clicked() {
                        gen_4_tools.ivs_to_pid = Some(Box::default());
                    }
                    if ui.button("Profile Manager 4").clicked() {
                        profile_manager4.show = true;
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
        gen_4_tools.show(ctx);
        profile_manager4.show(ctx);
    }
}
