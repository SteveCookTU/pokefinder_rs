use crate::util::IVToPID;
use egui::Context;

#[derive(Default)]
pub struct Gen4Tools {
    pub ivs_to_pid: Option<Box<IVToPID>>,
}

impl Gen4Tools {
    pub fn show(&mut self, ctx: &Context) {
        let close = self
            .ivs_to_pid
            .as_mut()
            .map(|ivs_to_pid| ivs_to_pid.show(ctx))
            .unwrap_or_default();
        if close {
            self.ivs_to_pid = None;
        }
    }
}
