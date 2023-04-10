use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum Shiny {
    #[num_enum(default)]
    Random, // PID is random, not locked
    Never,  // PID is random, forced to not be shiny
    Always, // PID is random, forced to shiny
    Star,   // PID is random, forced to be star shiny
    Square, // PID is random, forced to be square shiny
    Static, // PID is set to a fixed value
}
