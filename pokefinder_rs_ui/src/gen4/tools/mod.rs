use crate::util::IVToPID;
use egui::{Context, Ui};

#[derive(Default)]
pub struct Gen4Tools {
    pub ivs_to_pid: IVToPID,
}

impl Gen4Tools {
    pub fn show(&mut self, ctx: &Context) {
        if self.ivs_to_pid.show {
            self.ivs_to_pid.show(ctx);
        }
    }
}
