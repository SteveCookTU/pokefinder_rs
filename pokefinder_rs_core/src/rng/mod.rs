mod lcrng;
mod lcrng64;
pub mod lcrng_reverse;
mod mt;
mod mt_fast;
mod rng_list;
mod xoroshiro;
mod xorshift;

pub use lcrng::*;
pub use lcrng64::*;
pub use mt::*;
pub use mt_fast::*;
pub use rng_list::*;
pub use xoroshiro::*;
pub use xorshift::*;

pub trait Rng {
    type Output;

    fn next(&mut self) -> Self::Output;
    fn advance(&mut self, advances: u32);
    fn jump(&mut self, advances: u32);
}
