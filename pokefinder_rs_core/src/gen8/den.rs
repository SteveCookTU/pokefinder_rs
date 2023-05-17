use crate::enums::Game;
use crate::gen8::Raid;

/// Contains all the Raid pokemon within a Den and the identifying hash value
#[derive(Copy, Clone)]
pub struct Den {
    /// Hash value of the den
    pub hash: u64,
    /// Sword raids
    pub shield: [Raid; 12],
    /// Shield raids
    pub sword: [Raid; 12],
}

impl Den {
    /// Construct a new [`Den`] struct
    pub const fn new(hash: u64, sword: [Raid; 12], shield: [Raid; 12]) -> Self {
        Self {
            hash,
            shield,
            sword,
        }
    }

    /// Returns the raid for the `index` and `version`
    pub const fn get_raid(&self, index: usize, version: Game) -> Raid {
        if version.bits() == Game::SWORD.bits() {
            self.sword[index]
        } else {
            self.shield[index]
        }
    }

    /// Returns the raids for the `version`
    pub const fn get_raids(&self, version: Game) -> [Raid; 12] {
        if version.bits() == Game::SWORD.bits() {
            self.sword
        } else {
            self.shield
        }
    }
}

/// Contains all the Raid pokemon within a Den event
#[derive(Clone, Default)]
pub struct DenEvent {
    /// Shield raids
    pub shield: Vec<Raid>,
    /// Sword raids
    pub sword: Vec<Raid>,
}

impl DenEvent {
    /// Construct a new [`DenEvent`] struct
    pub fn new(sword: Vec<Raid>, shield: Vec<Raid>) -> Self {
        Self { shield, sword }
    }

    /// Blank initializer for const contexts
    pub const fn blank() -> Self {
        Self {
            shield: vec![],
            sword: vec![],
        }
    }

    /// Returns the raid for the `index` and `version`
    pub fn get_raid(&self, index: usize, version: Game) -> Raid {
        if version == Game::SWORD {
            self.sword[index]
        } else {
            self.shield[index]
        }
    }

    /// Returns the raids for the `version`
    pub fn get_raids(&self, version: Game) -> &[Raid] {
        if version == Game::SWORD {
            &self.sword
        } else {
            &self.shield
        }
    }
}
