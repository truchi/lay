//! Styling utilities.
//!
//! This module contains utilities to work with terminal CSIs and styled types.
//!
//! # Colors and attributes
//!
//! You can use the following types to print CSIs to the terminal:
//! - Colors (tuple structs):
//!   - [`Foreground`][foreground]: `Foreground(Color)`
//!   - [`Background`][background]: `Background(Color)`
//! - Attributes (enums):
//!   - [`Weighted`][weighted]: `Bold`, `Light`, `ResetWeight`
//!   - [`Slanted`][slanted]: `Italic`, `ResetSlant`
//!   - [`Blinking`][blinking]: `Slow`, `Fast`, `ResetBlink`
//!   - [`Inverted`][inverted]: `Invert`, `ResetInvert`
//!   - [`Striked`][striked]: `Strike`, `ResetStrike`
//!   - [`Underlined`][underlined]: `Underline`, `ResetUnderline`
//!   - [`Overlined`][overlined]: `Overline`, `ResetOverline`
//!   - [`Bordered`][bordered]: `Frame`, `Circle`, `ResetBorder`
//!
//! All those types `Default` to their reset value/variant: colors default to
//! the user's terminal default foreground/background color, attributes default
//! to the unsetting CSI. They `Display` the CSI they represent:
//!
//! ```
//! use lay::*;
//!
//! fn main() {
//!     println!(
//!         "{}Hello, {}world{}!{}",
//!         Bold,
//!         Foreground(Red),
//!         Foreground(Reset),
//!         ResetWeight
//!     );
//! }
//! ```
//!
//! # Styling
//!
//! (We will refer to both colors and attributes as "attributes".)
//!
//! We want to individually wrap styling attributes with `Option`s to convey
//! ideas such as "undefined" (display no CSI) or "inherit" (inherit from some
//! parent attribute, if any).
//!
//! ## `Styler`
//!
//! The [`Styler`][styler] trait defines getters and setters for types with
//! `Option`al attributes, provides lots of convenient methods for a few
//! required methods and enables styling operations overloads. It can be easily
//! implemented with the [`impl_styler`][impl_styler] macro.
//!
//! ```
//! use lay::*;
//!
//! // NOTE: Style implements Styler, see below
//!
//! fn main() {
//!     // A red foreground
//!     let style = Style::default().red();
//!     // Both are equivalent
//!     let new_style = style.bold().or(&style.black().on_white());
//!     let new_style = style + Bold | style * Black / White;
//!     // Both are equivalent
//!     let reset = new_style.reset();
//!     let reset = !new_style;
//!
//!     assert_eq!(new_style, style * Red / White + Bold);
//!     assert_eq!(reset, style * Reset / Reset + ResetWeight);
//! }
//! ```
//!
//! # `Style`
//!
//! The [`Style`][style] struct is the most simple implementation of `Styler`
//! you can think of: it has a field for each attribute wrapped in an `Option`.
//!
//! [foreground]: struct.Foreground.html
//! [background]: struct.Background.html
//! [weighted]: enum.Weighted.html
//! [slanted]: enum.Slanted.html
//! [blinking]: enum.Blinking.html
//! [inverted]: enum.Inverted.html
//! [striked]: enum.Striked.html
//! [underlined]: enum.Underlined.html
//! [overlined]: enum.Overlined.html
//! [bordered]: enum.Bordered.html
//! [styler]: trait.Styler.html
//! [style]: struct.Style.html
//! [impl_styler]: ../macro.impl_styler.html

mod attributes;
mod colors;
mod style;
mod styled;
mod styler;

pub use attributes::*;
pub use colors::*;
pub use style::*;
pub use styled::*;
pub use styler::*;
