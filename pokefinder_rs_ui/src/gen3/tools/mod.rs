mod pid_to_iv;

use egui::Context;
pub use pid_to_iv::*;

#[derive(Default)]
pub struct Gen3Tools {
    pub pid_to_ivs: Option<Box<PIDToIV>>,
}
impl Gen3Tools {
    pub fn show(&mut self, ctx: &Context) {
        let mut close = self
            .pid_to_ivs
            .as_mut()
            .map(|pid_to_ivs| pid_to_ivs.show(ctx))
            .unwrap_or_default();
        if close {
            self.pid_to_ivs = None;
        }
    }
}
