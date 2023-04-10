use crate::enums::Game;

pub trait Profile: PartialEq {
    fn get_name(&self) -> &str;
    fn get_version(&self) -> Game;
    fn get_sid(&self) -> u16;
    fn get_tid(&self) -> u16;
}
