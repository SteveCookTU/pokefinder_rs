use serde::Deserialize;
use std::collections::HashMap;

pub mod bw;
pub mod bw2;
mod encounters5;
mod pack;
mod text;

pub use encounters5::*;

#[derive(Deserialize)]
struct LocationModifiers<'a> {
    #[serde(borrow)]
    bw: HashMap<&'a str, HashMap<&'a str, &'a str>>,
    #[serde(borrow)]
    bw2: HashMap<&'a str, HashMap<&'a str, &'a str>>,
}
