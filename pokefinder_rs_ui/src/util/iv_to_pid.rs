use egui::{Context, Ui};

#[derive(Default)]
pub struct IVToPID {
    pub show: bool,
    nature: u8,
    tid: u16,
    hp: u8,
    atk: u8,
    def: u8,
    spa: u8,
    spd: u8,
    spe: u8,
}

impl IVToPID {
    pub fn show(&mut self, ctx: &Context) {
        egui::Window::new("IVs to PID").show(ctx, |ui| {
            if ui.button("Close").clicked() {
                egui::ComboBox::new("ivs_to_pid_nature", "Nature").show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.nature, 0, "Hardy");
                });
                self.show = false;
            }
        });
    }
}
