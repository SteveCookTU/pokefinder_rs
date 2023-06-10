//! This crate is a Rust port of the PokeFinder core library written in C++.
//! This crate attempts to follow the original libraries usage along with being
//! maintained side by side.
//!
//! The original library can be found [here] along with a desktop application that uses it.
//!
//! [here]: https://github.com/Admiral-Fish/PokeFinder

/// Contains enums and bitflags used throughout the core library.
pub mod enums;
/// Contains tools and structures used for predicting Gen 3 games.
pub mod gen3;
/// Contains tools and structures used for predicting Gen 4 games.
pub mod gen4;
/// Contains tools and structures used for predicting Gen 5 games.
pub mod gen5;
/// Contains tools and structures used for predicting Gen 8 games.
pub mod gen8;
/// Contains parent structures that tools within the other modules share.
pub mod parents;
/// Contains resources such as text/translations and encounter data.
pub mod resources;
/// Contains the implementations for the PRNG modules used within the Pokémon games.
pub mod rng;
/// Contains general utility functions used across multiple modules.
pub mod util;
