use crate::enums::Game;

/// Trait that contains common functions for Profile structs
pub trait Profile: PartialEq {
    /// Returns the name of the profile
    fn get_name(&self) -> &str;
    /// Returns the version of the profile
    fn get_version(&self) -> Game;
    /// Returns the SID of the profile
    fn get_sid(&self) -> u16;
    /// Returns the TID of the profile
    fn get_tid(&self) -> u16;
}
