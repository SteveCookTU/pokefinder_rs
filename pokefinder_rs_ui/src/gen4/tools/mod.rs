mod chained_sid;
mod search_coin_flips;
mod seed_to_time4;

pub use chained_sid::*;
pub use search_coin_flips::*;
pub use seed_to_time4::*;

use crate::util::IVToPID;
use egui::Context;

#[derive(Default)]
pub struct Gen4Tools {
    pub ivs_to_pid: Option<Box<IVToPID>>,
    pub chained_sid: Option<Box<ChainedSID>>,
    pub seed_to_time: Option<Box<SeedToTime4>>,
}

impl Gen4Tools {
    pub fn show(&mut self, ctx: &Context) {
        let mut close = self
            .ivs_to_pid
            .as_mut()
            .map(|ivs_to_pid| ivs_to_pid.show(ctx))
            .unwrap_or_default();
        if close {
            self.ivs_to_pid = None;
        }

        close = self
            .chained_sid
            .as_mut()
            .map(|chained_sid| chained_sid.show(ctx))
            .unwrap_or_default();
        if close {
            self.chained_sid = None;
        }

        close = self
            .seed_to_time
            .as_mut()
            .map(|seed_to_time| seed_to_time.show(ctx))
            .unwrap_or_default();
        if close {
            self.seed_to_time = None;
        }
    }
}
