//! # Lay
//!
//! > 📽  *Terminal graphics lol.*
//!
//! This crate defines **styling** and **layering** utilities to print
//! "graphics" to the terminal.

use std::{
    fmt::{self, Debug, Display, Formatter},
    io::{self, Write},
};

include!("style.doc.rs");

pub mod layer;
pub use layer::*;

pub mod geometry;
pub use geometry::*;
