//! # Lay
//!
//! > Terminal graphics lol.
//!
//! This crate defines **styling** and **layering** utilities to print
//! "graphics" to the terminal.
//!
//! ## Styles
//!
//! The [`style`][mod_style] module exports the basic types
//! ([`Foreground`][foreground], [`Background`][background], [`Color`][color],
//! [`Weighted`][weighted], ...) you will need to manipulate terminal control
//! sequences at a rather low level.
//!
//! ```
//! use lay::*;
//!
//! fn main() {
//!     // Greet the world in red
//!     println!("Hello, {}world{}!", Foreground(Red), Foreground(Reset));
//! }
//! ```
//!
//! On top of thoses you will find the [`Styler`][styler] trait defining types
//! of styled data. We provide the [`Style`][style] and [`Styled<T>`][styled]
//! structs as simple implementations of this trait, as well as the
//! [`impl_styler`][impl_styler] macro as a convenience for implementing it on
//! your own types.
//!
//! ```
//! use lay::*;
//!
//! fn main() {
//!     let style = Style::default() * Red / Blue;
//!     println!("Hello, {}world{}!", style, !style);
//!
//!     let styled = Styled::from("world") + Bold;
//!     println!("Hello, {}!", styled);
//! }
//! ```
//!
//! See the [`style`][mod_style] module for details.
//!
//! ## Layer
//!
//! The [`layer`][mod_layer] module contains types useful for painting and
//! rendering in the terminal at a higher level. It is based on the
//! [`Layer`][layer] trait, which enable merging of layers, i.e. rectangle of
//! [`Cell`][cell]s.
//!
//! See and [`layer`][mod_layer] for details.
//!
//! ## Examples
//!
//! See [`examples`][gh_examples] for examples.
//!
//! [mod_style]: style/index.html
//! [mod_layer]: layer/index.html
//! [gh_examples]: https://github.com/truchi/lay/tree/master/examples
//! [foreground]: struct.Foreground.html
//! [background]: struct.Background.html
//! [color]: enum.Color.html
//! [weighted]: enum.Weighted.html
//! [styler]: trait.Styler.html
//! [style]: struct.Style.html
//! [styled]: struct.Styled.html
//! [impl_styler]: macro.impl_styler.html
//! [layer]: trait.Layer.html
//! [cell]: type.Cell.html

#[macro_use]
mod macros;

pub mod layer;
pub mod style;

pub use layer::*;
pub use style::*;
