use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq)]
    pub struct Lead: u8 {
        const NONE = 255;
        const SYNCHRONIZE = 0;
        const SYNCHRONIZE_END = 25;
        const CUTE_CHARM_F = 26;
        const CUTE_CHARM_M = 27;
        const MAGNET_PULL = 28;
        const STATIC = 29;
        const HARVEST = 30;
        const FLASH_FIRE = 31;
        const STORM_DRAIN = 32;
        const PRESSURE = 33;
        const HUSTLE = Self::PRESSURE.bits();
        const VITAL_SPIRIT = Self::PRESSURE.bits();
        const SUCTION_CUPS = 34;
        const STICKY_HOLD = Self::SUCTION_CUPS.bits();
        const COMPOUND_EYES = 35;
        const SUPER_LUCK = Self::COMPOUND_EYES.bits();
        const ARENA_TRAP = 36;
        const ILLUMINATE = Self::ARENA_TRAP.bits();
        const NO_GUARD = Self::ARENA_TRAP.bits();
    }
}
