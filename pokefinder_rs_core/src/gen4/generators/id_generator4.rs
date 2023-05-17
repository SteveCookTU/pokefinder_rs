use crate::gen4::states::IDState4;
use crate::parents::filters::IDFilter;
use crate::parents::generators::IDGenerator;
use crate::rng::MTFast;

/// TID/SID generator for Gen4
#[derive(Clone)]
pub struct IDGenerator4<'a> {
    /// Base ID generator data
    pub base: IDGenerator<'a>,
    /// Maximum delay
    pub max_delay: u32,
    /// Minimum delay
    pub min_delay: u32,
    /// Search year
    pub year: u16,
    /// Search day
    pub day: u8,
    /// Search hour
    pub hour: u8,
    /// Search minute
    pub minute: u8,
    /// Search month
    pub month: u8,
}

impl<'a> IDGenerator4<'a> {
    /// Construct a new [`IDGenerator4`] struct
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        min_delay: u32,
        max_delay: u32,
        year: u16,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        filter: &'a IDFilter,
    ) -> Self {
        Self {
            base: IDGenerator::new(0, 0, filter),
            max_delay,
            min_delay,
            year,
            day,
            hour,
            minute,
            month,
        }
    }

    /// Generates [`IDState4`] states
    pub fn generate(&self) -> Vec<IDState4> {
        let mut states = vec![];

        for second in 0..60 {
            for efgh in self.min_delay..=self.max_delay {
                let seed = ((((self.month as u32) * (self.day as u32)
                    + ((self.minute as u32) + second))
                    & 0xFF)
                    << 24)
                    | ((self.hour as u32) << 16).wrapping_add(efgh);

                let mut mt = MTFast::<2, 8, false>::new(seed, 1);

                let sid_tid = mt.next();
                let tid = sid_tid & 0xFFFF;
                let sid = sid_tid >> 16;
                let state = IDState4::new_with_seconds(
                    seed,
                    efgh.wrapping_add(2000).wrapping_sub(self.year as u32),
                    tid as u16,
                    sid as u16,
                    second as u8,
                );
                if self.base.filter.compare(&state) {
                    states.push(state);
                }
            }
        }

        states
    }
}
