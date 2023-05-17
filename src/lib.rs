#![warn(rust_2018_idioms)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! This app calculates the distances to nearby points of interest like grocery shops, bars, etc.

/// Responsible for distance calculations.
pub mod gmaps;

/// Helper functions.
pub mod utils;
