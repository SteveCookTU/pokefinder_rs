use serde::Deserialize;
use std::collections::HashMap;

pub mod dp;
mod encounters4;
pub mod hgss;
mod pack;
pub mod pt;
mod text;

pub use encounters4::*;

#[derive(Deserialize)]
struct LocationModifiers<'a> {
    #[serde(borrow)]
    dppt: HashMap<&'a str, HashMap<&'a str, &'a str>>,
    #[serde(borrow)]
    hgss: HashMap<&'a str, HashMap<&'a str, &'a str>>,
}
