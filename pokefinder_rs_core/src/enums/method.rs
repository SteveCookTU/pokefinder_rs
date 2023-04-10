use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum Method {
    #[num_enum(default)]
    None,

    Method1,
    Method1Reverse,
    Method2,
    Method4,

    XDColo,
    Channel,

    EBred,
    EBredSplit,
    EBredAlternate,
    EBredPID,
    RSFRLGBred,
    RSFRLGBredSplit,
    RSFRLGBredAlternate,
    RSFRLGBredMixed,

    CuteCharmDPPt,
    CuteCharmHGSS,
    MethodJ,
    MethodK,
    PokeRadar,
    WondercardIVs,

    Method5IVs,
    Method5CGear,
    Method5,
}
