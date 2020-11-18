pub mod attributes;
mod backends;
mod color;
mod reset;
mod style;
mod styled;
mod styler;

pub use attributes::*;
pub use color::*;
pub use reset::*;
pub use style::*;
pub use styled::*;
pub use styler::*;

// @generate import_markers
#[cfg(feature = "styler-idx")] //  🚧
pub mod i; //                      🚧
#[cfg(feature = "styler-idx")] //  🚧
pub use i::{
    Background as Bg, //           🚧
    Blink as Blk,     //           🚧
    Border as Brd,    //           🚧
    Foreground as Fg, //           🚧
    Invert as Inv,    //           🚧
    Overline as Ovl,  //           🚧
    Slant as Slt,     //           🚧
    Strike as Str,    //           🚧
    Underline as Udl, //           🚧
    Weight as Wgt,    //           🚧
}; //                              🚧

#[cfg(feature = "styler-ops")] //  🚧
pub mod no; //                     🚧
#[cfg(feature = "styler-ops")] //  🚧
pub use no::{
    Background as NoBackground, // 🚧
    Blink as NoBlink,           // 🚧
    Border as NoBorder,         // 🚧
    Foreground as NoForeground, // 🚧
    Invert as NoInvert,         // 🚧
    Overline as NoOverline,     // 🚧
    Slant as NoSlant,           // 🚧
    Strike as NoStrike,         // 🚧
    Underline as NoUnderline,   // 🚧
    Weight as NoWeight,         // 🚧
}; //                              🚧

// /generate import_markers
