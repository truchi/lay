//! A convenient wrapper around `crossterm::style`.

mod attributes;
mod colors;
mod style;
// mod styled;
mod styler;

pub use attributes::*;
pub use colors::*;
pub use crossterm::style::Color;
pub use style::*;
// pub use styled::*;
pub use styler::*;
pub use Color::*;
