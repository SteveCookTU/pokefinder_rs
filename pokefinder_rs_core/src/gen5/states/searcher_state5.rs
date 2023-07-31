use crate::enums::Buttons;
use crate::util::DateTime;

#[derive(Copy, Clone)]
pub struct SearcherState5<T> {
    pub initial_seed: u64,
    pub dt: DateTime,
    pub buttons: Buttons,
    pub timer0: u16,
    pub state: T,
}

impl<T> SearcherState5<T> {
    pub fn new(dt: DateTime, initial_seed: u64, buttons: Buttons, timer0: u16, state: T) -> Self {
        Self {
            initial_seed,
            dt,
            buttons,
            timer0,
            state,
        }
    }
}
