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
//!   - [`Weight`][weight]: `Bold`, `Light`, `ResetWeight`
//!   - [`Slant`][slant]: `Italic`, `ResetSlant`
//!   - [`Blink`][blink]: `Slow`, `Fast`, `ResetBlink`
//!   - [`Invert`][invert]: `Inverted`, `ResetInvert`
//!   - [`Strike`][strike]: `Striked`, `ResetStrike`
//!   - [`Underline`][underline]: `Underlined`, `ResetUnderline`
//!   - [`Overline`][overline]: `Overlined`, `ResetOverline`
//!   - [`Border`][border]: `Frame`, `Circle`, `ResetBorder`
//!
//! All those types `Default` to their reset value: colors default to the user's
//! terminal default foreground/background color, attributes default to the
//! unsetting CSI. They `Display` the CSI they represent:
//!
//! ```
//! # use lay::*;
//! println!(
//!     "{}Hello, {}world{}!{}",
//!     Bold,
//!     Foreground(Red),
//!     Foreground(ResetColor),
//!     ResetWeight
//! );
//! ```
//!
//! In addition, we provide the [`Reset`][reset] type which represents the CSI
//! to reset all colors/attributes:
//!
//! ```
//! # use lay::*;
//! println!(
//!     "{}{}{}{}Hello, world!{} No styles here.",
//!     Foreground(Red),
//!     Bold,
//!     Italic,
//!     Slow,
//!     Reset
//! );
//! ```
//!
//! Note that the [`Color`][color] enum does not `Display` by itself.
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
//! The [`Styler`][styler] trait is at the heart of styles. It defines getters
//! and setters for types with `Option`al attributes:
//!
//! ```
//! # use lay::*;
//! // NOTE: Style implements Styler, see below
//! let style = Style::default() // All fields are None
//!     .red() // Red foreground
//!     .on_green() // Green background
//!     .bold() // Bold text
//!     .reset_blink(); // Reset blink
//!
//! assert_eq!(style.get_foreground(), Some(Foreground(Red)));
//! assert_eq!(style.get_background(), Some(Background(Green)));
//! assert_eq!(style.get_weight(), Some(Bold));
//! assert_eq!(style.get_blink(), Some(ResetBlink));
//! assert_eq!(style.get_slant(), None); // etc..
//! ```
//!
//! It also provides convenients methods for styles manipulation:
//! [`and`][styler_and] (`Option::and` fields), [`or`][styler_or] (`Option::or`
//! fields), [`xor`][styler_xor] (`Option::xor` fields),
//! [`dedup`][styler_dedup] (`None`s when identical fields),
//! [`reset`][styler_reset] (reset `Some` fields).
//!
//! [`Styler`][styler] can easily be implemented on your own types with the
//! [`impl_styler`][impl_styler] macro.
//!
//! ### Styling operations overloads
//!
//! For an easier use of styles, [`Styler`][styler] enables styling operations
//! overloads. The above example could be written with the `Add` (+), `Mul` (*)
//! and `Div` (/) setters:
//!
//! ```
//! # use lay::*;
//! // NOTE: Style implements Styler's operators overloads, see below
//! let style = Style::default()
//!     * Red // Mul for foreground
//!     / Green // Div for background
//!     + Foreground(Red) // Add for everything
//!     + Background(Green)
//!     + Bold
//!     + ResetBlink;
//! ```
//!
//! Moreover, `BitAnd` (&) is [`and`][styler_and], `BitOr` (|) is
//! [`or`][styler_or], `BitXor` (^) is [`xor`][styler_xor], `Rem` (%) is
//! [`dedup`][styler_dedup] and `Not` (!) is [`reset`][styler_reset].
//!
//! `lay` defines handy unit struct to `None` their corresponding attribute
//! fields:
//!
//! ```
//! # use lay::*;
//! let style = Style::default()
//!     + NoForeground // ~= * None
//!     + NoBackground // ~= / None
//!     + NoWeight
//!     + NoSlant
//!     + NoBlink
//!     + NoInvert
//!     + NoStrike
//!     + NoUnderline
//!     + NoOverline
//!     + NoBorder;
//! ```
//!
//! Those overloads are implemented on all `lay`'s `Styler` types. You can
//! generate the operators overloads on your own `Styler` types with the
//! [`impl_styler_ops`][impl_styler_ops] macro.
//!
//! Finally, similar overloads are implemented on attributes types. TODO
//!
//! You can disable styling operations overloads by opting out of the
//! `styler-ops` default feature.
//!
//! # `Style`
//!
//! The [`Style`][style] struct is the most simple implementation of `Styler`
//! you can think of: it has a field for each attribute wrapped in an `Option`.
//! It `Display`s as the CSIs of its `Some` fields.
//!
//! ```
//! # use lay::*;
//! let style = Style::default() * Red / Blue;
//! println!("Hello, {}world{}!", style, !style);
//! ```
//!
//! [color]: enum.Color.html
//! [foreground]: struct.Foreground.html
//! [background]: struct.Background.html
//! [weight]: enum.Weight.html
//! [slant]: enum.Slant.html
//! [blink]: enum.Blink.html
//! [invert]: enum.Invert.html
//! [strike]: enum.Strike.html
//! [underline]: enum.Underline.html
//! [overline]: enum.Overline.html
//! [border]: enum.Border.html
//! [reset]: struct.Reset.html
//! [styler]: trait.Styler.html
//! [styler_and]: trait.Styler.html#method.and
//! [styler_or]: trait.Styler.html#method.or
//! [styler_xor]: trait.Styler.html#method.xor
//! [styler_dedup]: trait.Styler.html#method.dedup
//! [styler_reset]: trait.Styler.html#method.reset
//! [style]: struct.Style.html
//! [styled]: struct.Styled.html
//! [impl_styler]: ../macro.impl_styler.html
//! [impl_styler_ops]: ../macro.impl_styler_ops.html

mod attrs;
mod color;
mod colors;
mod reset;
mod style;
mod styled;
mod styler;

/// Attributes (`Foreground`, `Background`, `Weight`, `Slant`, `Blink`,
/// `Invert`, `Strike`, `Underline`, `Overline`, `Border`).
pub mod attributes {
    pub use super::{attrs::*, colors::*};
}

pub use attributes::*;
pub use color::*;
pub use reset::*;
pub use style::*;
pub use styled::*;
pub use styler::*;

#[cfg(feature = "styler-ops")]
pub mod no;
#[cfg(feature = "styler-ops")]
pub use no::{
    Background as NoBackground,
    Blink as NoBlink,
    Border as NoBorder,
    Foreground as NoForeground,
    Invert as NoInvert,
    Overline as NoOverline,
    Slant as NoSlant,
    Strike as NoStrike,
    Underline as NoUnderline,
    Weight as NoWeight,
};

#[cfg(feature = "styler-idx")]
pub mod i;
#[cfg(feature = "styler-idx")]
pub use i::{
    Background as Bg,
    Blink as Blk,
    Border as Brd,
    Foreground as Fg,
    Invert as Inv,
    Overline as Ovl,
    Slant as Slt,
    Strike as Stk,
    Underline as Udl,
    Weight as Wgt,
};
