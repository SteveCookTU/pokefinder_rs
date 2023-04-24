use bitflags::bitflags;
bitflags! {
    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
    pub struct Buttons: u16 {
        const NONE = 0;
        const R = 1;
        const L = 1 << 1;
        const X = 1 << 2;
        const Y = 1 << 3;
        const A = 1 << 4;
        const B = 1 << 5;
        const SELECT = 1 << 6;
        const START = 1 << 7;
        const SELECT_START = Self::SELECT.bits() | Self::START.bits();
        const RIGHT = 1 << 8;
        const LEFT = 1 << 9;
        const UP = 1 << 10;
        const DOWN = 1 << 11;
        const RIGHT_UP = Self::RIGHT.bits() | Self::UP.bits();
        const LEFT_UP = Self::LEFT.bits() | Self::UP.bits();
        const RIGHT_DOWN = Self::RIGHT.bits() | Self::DOWN.bits();
        const LEFT_DOWN = Self::LEFT.bits() | Self::DOWN.bits();
    }
}
