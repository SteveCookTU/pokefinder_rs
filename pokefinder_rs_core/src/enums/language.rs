use num_enum::{FromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};

#[derive(
    Copy,
    Clone,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    IntoPrimitive,
    FromPrimitive,
    Serialize,
    Deserialize,
)]
#[repr(u8)]
pub enum Language {
    #[num_enum(default)]
    English,
    French,
    German,
    Italian,
    Japanese,
    Korean,
    Spanish,
}
