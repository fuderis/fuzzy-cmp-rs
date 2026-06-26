#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
// pub mod prelude;

pub mod cmp;
pub use cmp::*;

#[cfg(feature = "lev")]
pub mod lev;
#[cfg(feature = "lev")]
pub use lev::*;

#[cfg(feature = "hybrid")]
pub mod hybrid;
#[cfg(feature = "hybrid")]
pub use hybrid::*;
