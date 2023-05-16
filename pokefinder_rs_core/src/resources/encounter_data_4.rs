use crate::enums::{Game, Method, Shiny};
use crate::gen4::StaticTemplate4;

pub(crate) static STARTERS: [StaticTemplate4; 12] = [
    StaticTemplate4::new(Game::DPPT, 387, 0, Shiny::Random, 5, Method::Method1),
    StaticTemplate4::new(Game::DPPT, 390, 0, Shiny::Random, 5, Method::Method1),
    StaticTemplate4::new(Game::DPPT, 393, 0, Shiny::Random, 5, Method::Method1),
    StaticTemplate4::new(Game::HGSS, 1, 0, Shiny::Random, 5, Method::Method1),
    StaticTemplate4::new(Game::HGSS, 4, 0, Shiny::Random, 5, Method::Method1),
    StaticTemplate4::new(Game::HGSS, 7, 0, Shiny::Random, 5, Method::Method1),
    StaticTemplate4::new(Game::HGSS, 152, 0, Shiny::Random, 5, Method::Method1),
    StaticTemplate4::new(Game::HGSS, 155, 0, Shiny::Random, 5, Method::Method1),
    StaticTemplate4::new(Game::HGSS, 158, 0, Shiny::Random, 5, Method::Method1),
    StaticTemplate4::new(Game::HGSS, 252, 0, Shiny::Random, 5, Method::Method1),
    StaticTemplate4::new(Game::HGSS, 255, 0, Shiny::Random, 5, Method::Method1),
    StaticTemplate4::new(Game::HGSS, 258, 0, Shiny::Random, 5, Method::Method1),
];

pub(crate) static FOSSILS: [StaticTemplate4; 7] = [
    StaticTemplate4::new(Game::GEN4, 138, 0, Shiny::Random, 20, Method::Method1),
    StaticTemplate4::new(Game::GEN4, 140, 0, Shiny::Random, 20, Method::Method1),
    StaticTemplate4::new(Game::GEN4, 142, 0, Shiny::Random, 20, Method::Method1),
    StaticTemplate4::new(Game::GEN4, 345, 0, Shiny::Random, 20, Method::Method1),
    StaticTemplate4::new(Game::GEN4, 347, 0, Shiny::Random, 20, Method::Method1),
    StaticTemplate4::new(Game::GEN4, 408, 0, Shiny::Random, 20, Method::Method1),
    StaticTemplate4::new(Game::GEN4, 410, 0, Shiny::Random, 20, Method::Method1),
];

pub(crate) static GIFTS: [StaticTemplate4; 14] = [
    StaticTemplate4::new(Game::DP, 133, 0, Shiny::Random, 5, Method::Method1),
    StaticTemplate4::new(Game::PLATINUM, 133, 0, Shiny::Random, 20, Method::Method1),
    StaticTemplate4::new(Game::PLATINUM, 137, 0, Shiny::Random, 25, Method::Method1),
    StaticTemplate4::new(Game::PLATINUM, 175, 0, Shiny::Random, 1, Method::Method1),
    StaticTemplate4::new(Game::DP, 440, 0, Shiny::Random, 1, Method::Method1),
    StaticTemplate4::new(Game::DPPT, 447, 0, Shiny::Random, 1, Method::Method1),
    StaticTemplate4::new(Game::HGSS, 72, 0, Shiny::Random, 15, Method::Method1),
    StaticTemplate4::new(Game::HGSS, 133, 0, Shiny::Random, 5, Method::Method1),
    StaticTemplate4::new(Game::HGSS, 147, 0, Shiny::Random, 15, Method::Method1),
    StaticTemplate4::new(Game::HGSS, 236, 0, Shiny::Random, 10, Method::Method1),
    StaticTemplate4::new(Game::HGSS, 175, 0, Shiny::Random, 1, Method::Method1),
    StaticTemplate4::new(Game::HGSS, 179, 0, Shiny::Random, 1, Method::Method1),
    StaticTemplate4::new(Game::HGSS, 194, 0, Shiny::Random, 1, Method::Method1),
    StaticTemplate4::new(Game::HGSS, 218, 0, Shiny::Random, 1, Method::Method1),
];

pub(crate) static GAME_CORNER: [StaticTemplate4; 7] = [
    StaticTemplate4::new(Game::HGSS, 122, 0, Shiny::Random, 15, Method::Method1),
    StaticTemplate4::new(Game::HGSS, 133, 0, Shiny::Random, 15, Method::Method1),
    StaticTemplate4::new(Game::HGSS, 137, 0, Shiny::Random, 15, Method::Method1),
    StaticTemplate4::new(Game::HGSS, 63, 0, Shiny::Random, 15, Method::Method1),
    StaticTemplate4::new(Game::HEART_GOLD, 23, 0, Shiny::Random, 15, Method::Method1),
    StaticTemplate4::new(Game::SOUL_SILVER, 27, 0, Shiny::Random, 15, Method::Method1),
    StaticTemplate4::new(Game::HGSS, 147, 0, Shiny::Random, 15, Method::Method1),
];

pub(crate) static STATIONARY: [StaticTemplate4; 13] = [
    StaticTemplate4::new(Game::DP, 425, 0, Shiny::Random, 22, Method::MethodJ),
    StaticTemplate4::new(Game::PLATINUM, 425, 0, Shiny::Random, 15, Method::MethodJ),
    StaticTemplate4::new(Game::DP, 479, 0, Shiny::Random, 15, Method::MethodJ),
    StaticTemplate4::new(Game::PLATINUM, 479, 0, Shiny::Random, 20, Method::MethodJ),
    StaticTemplate4::new(Game::DPPT, 442, 0, Shiny::Random, 25, Method::MethodJ),
    StaticTemplate4::new(Game::HGSS, 100, 0, Shiny::Random, 23, Method::MethodK),
    StaticTemplate4::new(Game::HGSS, 74, 0, Shiny::Random, 21, Method::MethodK),
    StaticTemplate4::new(Game::HGSS, 109, 0, Shiny::Random, 21, Method::MethodK),
    StaticTemplate4::new(Game::HGSS, 130, 0, Shiny::Always, 30, Method::Method1),
    StaticTemplate4::new(Game::HGSS, 131, 0, Shiny::Random, 20, Method::MethodK),
    StaticTemplate4::new(Game::HGSS, 101, 0, Shiny::Random, 23, Method::MethodK),
    StaticTemplate4::new(Game::HGSS, 143, 0, Shiny::Random, 50, Method::MethodK),
    StaticTemplate4::new(Game::HGSS, 185, 0, Shiny::Random, 20, Method::MethodK),
];

pub(crate) static LEGENDS: [StaticTemplate4; 33] = [
    StaticTemplate4::new(Game::PLATINUM, 377, 0, Shiny::Random, 30, Method::MethodJ),
    StaticTemplate4::new(Game::PLATINUM, 378, 0, Shiny::Random, 30, Method::MethodJ),
    StaticTemplate4::new(Game::PLATINUM, 379, 0, Shiny::Random, 30, Method::MethodJ),
    StaticTemplate4::new(Game::DPPT, 480, 0, Shiny::Random, 50, Method::MethodJ),
    StaticTemplate4::new(Game::DPPT, 482, 0, Shiny::Random, 50, Method::MethodJ),
    StaticTemplate4::new(Game::DIAMOND, 483, 0, Shiny::Random, 47, Method::MethodJ),
    StaticTemplate4::new(Game::PEARL, 484, 0, Shiny::Random, 47, Method::MethodJ),
    StaticTemplate4::new(Game::PLATINUM, 483, 0, Shiny::Random, 70, Method::MethodJ),
    StaticTemplate4::new(Game::PLATINUM, 484, 0, Shiny::Random, 70, Method::MethodJ),
    StaticTemplate4::new(Game::DP, 485, 0, Shiny::Random, 70, Method::MethodJ),
    StaticTemplate4::new(Game::PLATINUM, 485, 0, Shiny::Random, 50, Method::MethodJ),
    StaticTemplate4::new(Game::DP, 486, 0, Shiny::Random, 70, Method::MethodJ),
    StaticTemplate4::new(Game::PLATINUM, 486, 0, Shiny::Random, 1, Method::MethodJ),
    StaticTemplate4::new(Game::DP, 487, 0, Shiny::Random, 70, Method::MethodJ),
    StaticTemplate4::new(Game::PLATINUM, 487, 0, Shiny::Random, 47, Method::MethodJ),
    StaticTemplate4::new(Game::PLATINUM, 487, 1, Shiny::Random, 47, Method::MethodJ),
    StaticTemplate4::new(Game::HGSS, 144, 0, Shiny::Random, 50, Method::MethodK),
    StaticTemplate4::new(Game::HGSS, 145, 0, Shiny::Random, 50, Method::MethodK),
    StaticTemplate4::new(Game::HGSS, 146, 0, Shiny::Random, 50, Method::MethodK),
    StaticTemplate4::new(Game::HGSS, 150, 0, Shiny::Random, 70, Method::MethodK),
    StaticTemplate4::new(Game::HGSS, 245, 0, Shiny::Random, 40, Method::MethodK),
    StaticTemplate4::new(Game::HEART_GOLD, 249, 0, Shiny::Random, 70, Method::MethodK),
    StaticTemplate4::new(
        Game::SOUL_SILVER,
        249,
        0,
        Shiny::Random,
        45,
        Method::MethodK,
    ),
    StaticTemplate4::new(Game::HEART_GOLD, 250, 0, Shiny::Random, 45, Method::MethodK),
    StaticTemplate4::new(
        Game::SOUL_SILVER,
        250,
        0,
        Shiny::Random,
        70,
        Method::MethodK,
    ),
    StaticTemplate4::new(Game::HEART_GOLD, 381, 0, Shiny::Random, 40, Method::MethodK),
    StaticTemplate4::new(
        Game::SOUL_SILVER,
        380,
        0,
        Shiny::Random,
        40,
        Method::MethodK,
    ),
    StaticTemplate4::new(Game::HEART_GOLD, 382, 0, Shiny::Random, 50, Method::MethodK),
    StaticTemplate4::new(
        Game::SOUL_SILVER,
        383,
        0,
        Shiny::Random,
        50,
        Method::MethodK,
    ),
    StaticTemplate4::new(Game::HGSS, 384, 0, Shiny::Random, 50, Method::MethodK),
    StaticTemplate4::new(Game::HGSS, 483, 0, Shiny::Random, 1, Method::Method1),
    StaticTemplate4::new(Game::HGSS, 484, 0, Shiny::Random, 1, Method::Method1),
    StaticTemplate4::new(Game::HGSS, 487, 1, Shiny::Random, 1, Method::Method1),
];

pub(crate) static EVENTS: [StaticTemplate4; 3] = [
    StaticTemplate4::new(Game::GEN4, 490, 0, Shiny::Never, 1, Method::Method1),
    StaticTemplate4::new(Game::PLATINUM, 491, 0, Shiny::Random, 50, Method::MethodJ),
    StaticTemplate4::new(Game::PLATINUM, 492, 0, Shiny::Random, 30, Method::MethodJ),
];

pub(crate) static ROAMERS: [StaticTemplate4; 9] = [
    StaticTemplate4::new(Game::DPPT, 481, 0, Shiny::Random, 50, Method::Method1),
    StaticTemplate4::new(Game::DPPT, 488, 0, Shiny::Random, 50, Method::Method1),
    StaticTemplate4::new(Game::PLATINUM, 144, 0, Shiny::Random, 60, Method::Method1),
    StaticTemplate4::new(Game::PLATINUM, 145, 0, Shiny::Random, 60, Method::Method1),
    StaticTemplate4::new(Game::PLATINUM, 146, 0, Shiny::Random, 60, Method::Method1),
    StaticTemplate4::new(Game::HGSS, 243, 0, Shiny::Random, 40, Method::Method1),
    StaticTemplate4::new(Game::HGSS, 244, 0, Shiny::Random, 40, Method::Method1),
    StaticTemplate4::new(Game::HEART_GOLD, 380, 0, Shiny::Random, 35, Method::Method1),
    StaticTemplate4::new(
        Game::SOUL_SILVER,
        381,
        0,
        Shiny::Random,
        35,
        Method::Method1,
    ),
];

pub(crate) static DIAMOND: &[u8] = include_bytes!("diamond.bin");

pub(crate) static HEART_GOLD: &[u8] = include_bytes!("heartgold.bin");

pub(crate) static HG_HEADBUTT: &[u8] = include_bytes!("hg_headbutt.bin");

pub(crate) static HGSS_BUG: &[u8] = include_bytes!("hgss_bug.bin");

pub(crate) static HGSS_SAFARI: &[u8] = include_bytes!("hgss_safari.bin");

pub(crate) static PEARL: &[u8] = include_bytes!("pearl.bin");

pub(crate) static PLATINUM: &[u8] = include_bytes!("platinum.bin");

pub(crate) static SOUL_SILVER: &[u8] = include_bytes!("soulsilver.bin");

pub(crate) static SS_HEADBUTT: &[u8] = include_bytes!("ss_headbutt.bin");
