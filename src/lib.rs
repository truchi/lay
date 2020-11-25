//! # Lay
//!
//! > 📽  *Terminal graphics lol.*
//!
//! This crate defines **styling** and **layering** utilities to print
//! "graphics" to the terminal.

include!("style.doc.rs");

pub mod layer;
pub use layer::*;
