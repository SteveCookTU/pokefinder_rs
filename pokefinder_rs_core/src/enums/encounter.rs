use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum Encounter {
    #[num_enum(default)]
    Grass,
    DoubleGrass,
    SpecialGrass,
    RockSmash,
    Surfing,
    SpecialSurf,
    OldRod,
    GoodRod,
    SuperRod,
    SpecialSuperRod,
    Static,
    BugCatchingContest,
    Headbutt,
    HeadbuttAlt,
    HeadbuttSpecial,
    Roamer,
    Gift,
    EntraLink,
    GiftEgg,
    HiddenGrotto,
}
