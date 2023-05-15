mod lcrng;
mod lcrng64;
/// Provides a way to compute origin seed given IVs or PID.
///
/// The PokeRNG attacks are a derivative of meet-in-the-middle attack (based on <https://crypto.stackexchange.com/a/10609>)
/// combined with patterns in modular arithmetic.
///
/// The Channel and XDRNG attacks are Euclidean divisor based (<https://crypto.stackexchange.com/a/10629>)
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

/// Trait containing common implementation for PRNGs
pub trait Rng {
    /// Common output of the implementing RNG
    type Output;

    /// Advances the implementing PRNG by 1 advance
    fn next(&mut self) -> Self::Output;
    /// Advances the implementing PRNG by a specified amount of advances
    fn advance(&mut self, advances: u32);
    /// Jumps the implementing PRNG by a specified amount of advances
    fn jump(&mut self, advances: u32);
}
