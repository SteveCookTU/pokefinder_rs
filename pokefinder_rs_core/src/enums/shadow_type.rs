use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum ShadowType {
    #[num_enum(default)]
    SingleLock,
    FirstShadow,
    Salamence,
    SecondShadow,
    EReader,
}
