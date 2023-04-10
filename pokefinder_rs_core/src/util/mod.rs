use bzip2::read::BzDecoder;
use std::io::Read;

mod datetime;
pub mod encounter_slot;
pub mod nature;

pub use datetime::*;

pub fn decompress(compressed_data: &[u8]) -> Vec<u8> {
    let mut decompressor = BzDecoder::new(compressed_data);
    let mut contents = vec![];
    decompressor
        .read_to_end(&mut contents)
        .expect("Failed to decompress");
    contents
}
