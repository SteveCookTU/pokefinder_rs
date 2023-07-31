use crate::enums::Buttons;
use crate::gen5::Profile5;

const KEYS: [Buttons; 8] = [
    Buttons::R,
    Buttons::L,
    Buttons::X,
    Buttons::Y,
    Buttons::A,
    Buttons::B,
    Buttons::SELECT,
    Buttons::START,
];
const DIRECTIONS: [Buttons; 8] = [
    Buttons::RIGHT,
    Buttons::LEFT,
    Buttons::UP,
    Buttons::DOWN,
    Buttons::RIGHT_UP,
    Buttons::LEFT_UP,
    Buttons::RIGHT_DOWN,
    Buttons::LEFT_DOWN,
];

const BUTTON_VALUES: [u32; 12] = [
    0x10000, 0x20000, 0x40000, 0x80000, 0x1000000, 0x2000000, 0x4000000, 0x8000000, 0x10000000,
    0x20000000, 0x40000000, 0x80000000,
];

fn valid(button: Buttons, skip_lr: bool) -> bool {
    !(skip_lr && ((button & Buttons::L) != Buttons::NONE || (button & Buttons::R) != Buttons::NONE))
}

pub fn get_key_presses(profile: &Profile5) -> Vec<Buttons> {
    let mut buttons = vec![];

    let key_presses = profile.get_key_presses();
    let skip_lr = profile.get_skip_lr();

    if key_presses[0] {
        buttons.push(Buttons::NONE);
    }

    for (i, (key, direction)) in KEYS.into_iter().zip(DIRECTIONS).enumerate() {
        if key_presses[1] {
            if valid(key, skip_lr) {
                buttons.push(key);
            }

            buttons.push(direction);
        }

        if key_presses[2] {
            for key2 in KEYS.into_iter().skip(i) {
                let combo = key | key2;
                if valid(combo, skip_lr) {
                    buttons.push(combo);
                }
            }

            for direction in DIRECTIONS.into_iter() {
                let combo = key | direction;
                if valid(combo, skip_lr) {
                    buttons.push(combo);
                }
            }
        }

        if key_presses[3] {
            for (j, key2) in KEYS.into_iter().skip(i).enumerate() {
                for key3 in KEYS.into_iter().skip(j) {
                    let combo = key | key2 | key3;
                    if valid(combo, skip_lr) {
                        buttons.push(combo);
                    }
                }

                for direction in DIRECTIONS.into_iter() {
                    let combo = key | key2 | direction;
                    if valid(combo, skip_lr) {
                        buttons.push(combo);
                    }
                }
            }
        }
    }

    buttons
}

pub fn get_values(buttons: &[Buttons]) -> Vec<u32> {
    let mut values = vec![];
    for &button in buttons {
        let mut value = 0xff2f0000;

        for (i, button_value) in BUTTON_VALUES.into_iter().enumerate() {
            if (button & Buttons::from_bits_retain(1 << i)) != Buttons::NONE {
                value -= button_value;
            }
        }

        values.push(value);
    }

    values
}
