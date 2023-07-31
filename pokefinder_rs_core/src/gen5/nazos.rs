use crate::enums::{DSType, Game, Language};

const fn change_endian(val: u32) -> u32 {
    let val = ((val << 8) & 0xFF00FF00) | ((val >> 8) & 0xFF00FF);
    (val << 16) | (val >> 16)
}

const fn compute_nazo_bw(nazo: u32) -> [u32; 5] {
    const OFFSET1: u32 = 0xFC;
    const OFFSET2: u32 = OFFSET1 + 0x4C;

    let mut nazos = [0; 5];

    nazos[0] = change_endian(nazo);
    nazos[1] = change_endian(nazo + OFFSET1);
    nazos[2] = change_endian(nazo + OFFSET1);
    nazos[3] = change_endian(nazo + OFFSET2);
    nazos[4] = change_endian(nazo + OFFSET2);

    nazos
}

const fn compute_nazo_bw2(nazo: u32, nazo0: u32, nazo1: u32) -> [u32; 5] {
    const OFFSET: u32 = 0x54;

    let mut nazos = [0; 5];

    nazos[0] = change_endian(nazo0);
    nazos[1] = change_endian(nazo1);
    nazos[2] = change_endian(nazo);
    nazos[3] = change_endian(nazo + OFFSET);
    nazos[4] = change_endian(nazo + OFFSET);

    nazos
}

const ENGLISH_BLACK: [u32; 5] = compute_nazo_bw(0x022160b0);
const ENGLISH_WHITE: [u32; 5] = compute_nazo_bw(0x022160d0);
const ENGLISH_BLACK_DSI: [u32; 5] = compute_nazo_bw(0x02760190);
const ENGLISH_WHITE_DSI: [u32; 5] = compute_nazo_bw(0x027601b0);
const ENGLISH_BLACK2: [u32; 5] = compute_nazo_bw2(0x02200010, 0x0209aee8, 0x02039de9);
const ENGLISH_WHITE2: [u32; 5] = compute_nazo_bw2(0x02200050, 0x0209af28, 0x02039e15);
const ENGLISH_BLACK2_DSI: [u32; 5] = compute_nazo_bw2(0x027a5f70, 0x0209aee8, 0x02039de9);
const ENGLISH_WHITE2_DSI: [u32; 5] = compute_nazo_bw2(0x027a5e90, 0x0209af28, 0x02039e15);

const JAPANESE_BLACK: [u32; 5] = compute_nazo_bw(0x02215f10);
const JAPANESE_WHITE: [u32; 5] = compute_nazo_bw(0x02215f30);
const JAPANESE_BLACK_DSI: [u32; 5] = compute_nazo_bw(0x02761150);
const JAPANESE_WHITE_DSI: [u32; 5] = compute_nazo_bw(0x02761150);
const JAPANESE_BLACK2: [u32; 5] = compute_nazo_bw2(0x021ff9b0, 0x0209a8dc, 0x02039ac9);
const JAPANESE_WHITE2: [u32; 5] = compute_nazo_bw2(0x021ff9d0, 0x0209a8fc, 0x02039af5);
const JAPANESE_BLACK2_DSI: [u32; 5] = compute_nazo_bw2(0x027aa730, 0x0209a8dc, 0x02039ac9);
const JAPANESE_WHITE2_DSI: [u32; 5] = compute_nazo_bw2(0x027aa5f0, 0x0209a8fc, 0x02039af5);

const GERMAN_BLACK: [u32; 5] = compute_nazo_bw(0x02215ff0);
const GERMAN_WHITE: [u32; 5] = compute_nazo_bw(0x02216010);
const GERMAN_BLACK_DSI: [u32; 5] = compute_nazo_bw(0x027602f0);
const GERMAN_WHITE_DSI: [u32; 5] = compute_nazo_bw(0x027602f0);
const GERMAN_BLACK2: [u32; 5] = compute_nazo_bw2(0x021fff50, 0x0209ae28, 0x02039d69);
const GERMAN_WHITE2: [u32; 5] = compute_nazo_bw2(0x021fff70, 0x0209ae48, 0x02039d95);
const GERMAN_BLACK2_DSI: [u32; 5] = compute_nazo_bw2(0x027a6110, 0x0209ae28, 0x02039d69);
const GERMAN_WHITE2_DSI: [u32; 5] = compute_nazo_bw2(0x027a6010, 0x0209ae48, 0x02039d95);

const SPANISH_BLACK: [u32; 5] = compute_nazo_bw(0x02216070);
const SPANISH_WHITE: [u32; 5] = compute_nazo_bw(0x02216070);
const SPANISH_BLACK_DSI: [u32; 5] = compute_nazo_bw(0x027601f0);
const SPANISH_WHITE_DSI: [u32; 5] = compute_nazo_bw(0x027601f0);
const SPANISH_BLACK2: [u32; 5] = compute_nazo_bw2(0x021fffd0, 0x0209aea8, 0x02039db9);
const SPANISH_WHITE2: [u32; 5] = compute_nazo_bw2(0x021ffff0, 0x0209aec8, 0x02039de5);
const SPANISH_BLACK2_DSI: [u32; 5] = compute_nazo_bw2(0x027a6070, 0x0209aea8, 0x02039db9);
const SPANISH_WHITE2_DSI: [u32; 5] = compute_nazo_bw2(0x027a5fb0, 0x0209aec8, 0x02039de5);

const FRENCH_BLACK: [u32; 5] = compute_nazo_bw(0x02216030);
const FRENCH_WHITE: [u32; 5] = compute_nazo_bw(0x02216050);
const FRENCH_BLACK_DSI: [u32; 5] = compute_nazo_bw(0x02760230);
const FRENCH_WHITE_DSI: [u32; 5] = compute_nazo_bw(0x02760250);
const FRENCH_BLACK2: [u32; 5] = compute_nazo_bw2(0x02200030, 0x0209af08, 0x02039df9);
const FRENCH_WHITE2: [u32; 5] = compute_nazo_bw2(0x02200050, 0x0209af28, 0x02039e25);
const FRENCH_BLACK2_DSI: [u32; 5] = compute_nazo_bw2(0x027a5f90, 0x0209af08, 0x02039df9);
const FRENCH_WHITE2_DSI: [u32; 5] = compute_nazo_bw2(0x027a5ef0, 0x0209af28, 0x02039e25);

const ITALIAN_BLACK: [u32; 5] = compute_nazo_bw(0x02215fb0);
const ITALIAN_WHITE: [u32; 5] = compute_nazo_bw(0x02215fd0);
const ITALIAN_BLACK_DSI: [u32; 5] = compute_nazo_bw(0x027601d0);
const ITALIAN_WHITE_DSI: [u32; 5] = compute_nazo_bw(0x027601d0);
const ITALIAN_BLACK2: [u32; 5] = compute_nazo_bw2(0x021fff10, 0x0209ade8, 0x02039d69);
const ITALIAN_WHITE2: [u32; 5] = compute_nazo_bw2(0x021fff50, 0x0209ae28, 0x02039d95);
const ITALIAN_BLACK2_DSI: [u32; 5] = compute_nazo_bw2(0x027a5f70, 0x0209ade8, 0x02039d69);
const ITALIAN_WHITE2_DSI: [u32; 5] = compute_nazo_bw2(0x027a5ed0, 0x0209ae28, 0x02039d95);

const KOREAN_BLACK: [u32; 5] = compute_nazo_bw(0x022167b0);
const KOREAN_WHITE: [u32; 5] = compute_nazo_bw(0x022167b0);
const KOREAN_BLACK_DSI: [u32; 5] = compute_nazo_bw(0x02761150);
const KOREAN_WHITE_DSI: [u32; 5] = compute_nazo_bw(0x02761150);
const KOREAN_BLACK2: [u32; 5] = compute_nazo_bw2(0x02200750, 0x0209b60c, 0x0203a4d5);
const KOREAN_WHITE2: [u32; 5] = compute_nazo_bw2(0x02200770, 0x0209b62c, 0x0203a501);
const KOREAN_BLACK2_DSI: [u32; 5] = compute_nazo_bw2(0x02200770, 0x0209b60c, 0x0203a4d5);
const KOREAN_WHITE2_DSI: [u32; 5] = compute_nazo_bw2(0x027a57b0, 0x0209b62c, 0x0203a501);

pub fn get_nazos(version: Game, language: Language, ds_type: DSType) -> [u32; 5] {
    match language {
        Language::English => match version {
            Game::BLACK => {
                if ds_type == DSType::DS {
                    ENGLISH_BLACK
                } else {
                    ENGLISH_BLACK_DSI
                }
            }
            Game::WHITE => {
                if ds_type == DSType::DS {
                    ENGLISH_WHITE
                } else {
                    ENGLISH_WHITE_DSI
                }
            }
            Game::BLACK2 => {
                if ds_type == DSType::DS {
                    ENGLISH_BLACK2
                } else {
                    ENGLISH_BLACK2_DSI
                }
            }
            Game::WHITE2 => {
                if ds_type == DSType::DS {
                    ENGLISH_WHITE2
                } else {
                    ENGLISH_WHITE2_DSI
                }
            }
            _ => [0; 5],
        },
        Language::Japanese => match version {
            Game::BLACK => {
                if ds_type == DSType::DS {
                    JAPANESE_BLACK
                } else {
                    JAPANESE_BLACK_DSI
                }
            }
            Game::WHITE => {
                if ds_type == DSType::DS {
                    JAPANESE_WHITE
                } else {
                    JAPANESE_WHITE_DSI
                }
            }
            Game::BLACK2 => {
                if ds_type == DSType::DS {
                    JAPANESE_BLACK2
                } else {
                    JAPANESE_BLACK2_DSI
                }
            }
            Game::WHITE2 => {
                if ds_type == DSType::DS {
                    JAPANESE_WHITE2
                } else {
                    JAPANESE_WHITE2_DSI
                }
            }
            _ => [0; 5],
        },
        Language::German => match version {
            Game::BLACK => {
                if ds_type == DSType::DS {
                    GERMAN_BLACK
                } else {
                    GERMAN_BLACK_DSI
                }
            }
            Game::WHITE => {
                if ds_type == DSType::DS {
                    GERMAN_WHITE
                } else {
                    GERMAN_WHITE_DSI
                }
            }
            Game::BLACK2 => {
                if ds_type == DSType::DS {
                    GERMAN_BLACK2
                } else {
                    GERMAN_BLACK2_DSI
                }
            }
            Game::WHITE2 => {
                if ds_type == DSType::DS {
                    GERMAN_WHITE2
                } else {
                    GERMAN_WHITE2_DSI
                }
            }
            _ => [0; 5],
        },
        Language::Spanish => match version {
            Game::BLACK => {
                if ds_type == DSType::DS {
                    SPANISH_BLACK
                } else {
                    SPANISH_BLACK_DSI
                }
            }
            Game::WHITE => {
                if ds_type == DSType::DS {
                    SPANISH_WHITE
                } else {
                    SPANISH_WHITE_DSI
                }
            }
            Game::BLACK2 => {
                if ds_type == DSType::DS {
                    SPANISH_BLACK2
                } else {
                    SPANISH_BLACK2_DSI
                }
            }
            Game::WHITE2 => {
                if ds_type == DSType::DS {
                    SPANISH_WHITE2
                } else {
                    SPANISH_WHITE2_DSI
                }
            }
            _ => [0; 5],
        },
        Language::French => match version {
            Game::BLACK => {
                if ds_type == DSType::DS {
                    FRENCH_BLACK
                } else {
                    FRENCH_BLACK_DSI
                }
            }
            Game::WHITE => {
                if ds_type == DSType::DS {
                    FRENCH_WHITE
                } else {
                    FRENCH_WHITE_DSI
                }
            }
            Game::BLACK2 => {
                if ds_type == DSType::DS {
                    FRENCH_BLACK2
                } else {
                    FRENCH_BLACK2_DSI
                }
            }
            Game::WHITE2 => {
                if ds_type == DSType::DS {
                    FRENCH_WHITE2
                } else {
                    FRENCH_WHITE2_DSI
                }
            }
            _ => [0; 5],
        },
        Language::Italian => match version {
            Game::BLACK => {
                if ds_type == DSType::DS {
                    ITALIAN_BLACK
                } else {
                    ITALIAN_BLACK_DSI
                }
            }
            Game::WHITE => {
                if ds_type == DSType::DS {
                    ITALIAN_WHITE
                } else {
                    ITALIAN_WHITE_DSI
                }
            }
            Game::BLACK2 => {
                if ds_type == DSType::DS {
                    ITALIAN_BLACK2
                } else {
                    ITALIAN_BLACK2_DSI
                }
            }
            Game::WHITE2 => {
                if ds_type == DSType::DS {
                    ITALIAN_WHITE2
                } else {
                    ITALIAN_WHITE2_DSI
                }
            }
            _ => [0; 5],
        },
        Language::Korean => match version {
            Game::BLACK => {
                if ds_type == DSType::DS {
                    KOREAN_BLACK
                } else {
                    KOREAN_BLACK_DSI
                }
            }
            Game::WHITE => {
                if ds_type == DSType::DS {
                    KOREAN_WHITE
                } else {
                    KOREAN_WHITE_DSI
                }
            }
            Game::BLACK2 => {
                if ds_type == DSType::DS {
                    KOREAN_BLACK2
                } else {
                    KOREAN_BLACK2_DSI
                }
            }
            Game::WHITE2 => {
                if ds_type == DSType::DS {
                    KOREAN_WHITE2
                } else {
                    KOREAN_WHITE2_DSI
                }
            }
            _ => [0; 5],
        },
    }
}
