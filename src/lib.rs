//! # Lay
//!
//! > 📽  *Terminal graphics lol.*
//!
//! This crate defines **styling** and **layering** utilities to print
//! "graphics" to the terminal.

// use std::{
// fmt::{self, Debug, Display, Formatter},
// io::{self, Write},
// };

// include!("style.doc.rs");
// pub mod style;
// pub use style::*;
//
// #[cfg(feature = "layer")]
// pub mod layer;
// #[cfg(feature = "layer")]
// pub use layer::*;
//
// #[cfg(feature = "layer")]
// pub mod geometry;
// #[cfg(feature = "layer")]
// pub use geometry::*;

mod grid;
pub use grid::*;
