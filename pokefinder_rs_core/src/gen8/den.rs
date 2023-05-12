use crate::enums::Game;
use crate::gen8::Raid;

#[derive(Copy, Clone)]
pub struct Den {
    pub hash: u64,
    pub shield: [Raid; 12],
    pub sword: [Raid; 12],
}

impl Den {
    pub const fn new(hash: u64, sword: [Raid; 12], shield: [Raid; 12]) -> Self {
        Self {
            hash,
            shield,
            sword,
        }
    }

    pub const fn get_raid(&self, index: usize, version: Game) -> Raid {
        if version.bits() == Game::SWORD.bits() {
            self.sword[index]
        } else {
            self.shield[index]
        }
    }

    pub const fn get_raids(&self, version: Game) -> [Raid; 12] {
        if version.bits() == Game::SWORD.bits() {
            self.sword
        } else {
            self.shield
        }
    }
}

#[derive(Clone, Default)]
pub struct DenEvent {
    pub shield: Vec<Raid>,
    pub sword: Vec<Raid>,
}

impl DenEvent {
    pub fn new(sword: Vec<Raid>, shield: Vec<Raid>) -> Self {
        Self { shield, sword }
    }

    pub const fn blank() -> Self {
        Self {
            shield: vec![],
            sword: vec![],
        }
    }

    pub fn get_raid(&self, index: usize, version: Game) -> Raid {
        if version == Game::SWORD {
            self.sword[index]
        } else {
            self.shield[index]
        }
    }

    pub fn get_raids(&self, version: Game) -> &[Raid] {
        if version == Game::SWORD {
            &self.sword
        } else {
            &self.shield
        }
    }
}
