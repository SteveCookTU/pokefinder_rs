use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
    pub struct Game: u32 {
        const NONE = 0;
        const RUBY = 1;
        const SAPPHIRE = 1 << 1;
        const RS = 1 | (1 << 1);
        const EMERALD = 1 << 2;
        const RSE = Self::RS.bits() | Self::EMERALD.bits();
        const FIRE_RED = 1 << 3;
        const LEAF_GREEN = 1 << 4;
        const FRLG = Self::FIRE_RED.bits() | Self::LEAF_GREEN.bits();
        const GEN3 = Self::RSE.bits() | Self::FRLG.bits();
        const GALES = 1 << 5;
        const COLOSSEUM = 1 << 6;
        const GC = Self::GALES.bits() | Self::COLOSSEUM.bits();
        const DIAMOND = 1 << 7;
        const PEARL = 1 << 8;
        const DP = Self::DIAMOND.bits() | Self::PEARL.bits();
        const PLATINUM = 1 << 9;
        const DPPT = Self::DIAMOND.bits() | Self::PEARL.bits() | Self::PLATINUM.bits();
        const HEART_GOLD = 1 << 10;
        const SOUL_SILVER = 1 << 11;
        const HGSS = Self::HEART_GOLD.bits() | Self::SOUL_SILVER.bits();
        const GEN4 = Self::DPPT.bits() | Self::HGSS.bits();
        const BLACK = 1 << 12;
        const WHITE = 1 << 13;
        const BW = Self::BLACK.bits() | Self::WHITE.bits();
        const BLACK2 = 1 << 14;
        const WHITE2 = 1 << 15;
        const BW2 = Self::BLACK2.bits() | Self::WHITE2.bits();
        const GEN5 = Self::BW.bits() | Self::BW2.bits();
        const X = 1 << 16;
        const Y = 1 << 17;
        const XY = Self::X.bits() | Self::Y.bits();
        const OR = 1 << 18;
        const AS = 1 << 19;
        const ORAS = Self::OR.bits() | Self::AS.bits();
        const GEN6 = Self::XY.bits() | Self::ORAS.bits();
        const SUN = 1 << 20;
        const MOON = 1 << 21;
        const SM = Self::SUN.bits() | Self::MOON.bits();
        const US = 1 << 22;
        const UM = 1 << 23;
        const USUM = Self::US.bits() | Self::UM.bits();
        const GEN7 = Self::SM.bits() | Self::USUM.bits();
        const SWORD = 1 << 24;
        const SHIELD = 1 << 25;
        const SWSH = Self::SWORD.bits() | Self::SHIELD.bits();
        const BD = 1 << 26;
        const SP = 1 << 27;
        const BDSP = Self::BD.bits() | Self::SP.bits();
        const GEN8 = Self::SWSH.bits() | Self::BDSP.bits();
    }
}
