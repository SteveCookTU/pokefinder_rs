use crate::gen4::states::IDState4;
use crate::parents::filters::IDFilter;
use crate::parents::generators::IDGenerator;
use crate::rng::MTFast;

#[derive(Clone)]
pub struct IDGenerator4<'a> {
    pub base: IDGenerator<'a>,
    pub max_delay: u32,
    pub min_delay: u32,
    pub year: u16,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub month: u8,
}

impl<'a> IDGenerator4<'a> {
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
