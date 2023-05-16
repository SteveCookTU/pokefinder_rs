use serde::Deserialize;
use std::collections::HashMap;

pub mod dp;
pub mod hgss;
mod narc;
mod pack;
pub mod pt;
mod text;

#[derive(Deserialize)]
struct LocationModifiers<'a> {
    #[serde(borrow)]
    dppt: HashMap<&'a str, HashMap<&'a str, &'a str>>,
    #[serde(borrow)]
    hgss: HashMap<&'a str, HashMap<&'a str, &'a str>>,
}