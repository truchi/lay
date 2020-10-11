//! # Lay
//!
//! > Terminal graphics lol.
//!
//! This crate defines **styling** and **layering** utilities to print
//! "graphics" to the terminal.
//!
//! (All examples prefixed with `use lay::*`.)
//!
//! ## Styles
//!
//! The [`style`][mod_style] module exports the basic types
//! ([`Color`][color], [`Foreground`][foreground], [`Background`][background],
//! [`Weight`][weight], ...) you will need to manipulate terminal control
//! sequences manually.
//!
//! ```
//! # use lay::*;
//! // Greet the world in red
//! println!("Hello, {}world{}!", Foreground(Red), Foreground(ResetColor));
//! ```
//!
//! On top of thoses you will find the [`Styler`][styler] trait defining types
//! of styled data. We provide the [`Style`][style] and [`Styled<T>`][styled]
//! structs as simple implementations of this trait, as well as the
//! [`impl_styler`][impl_styler] macro as a convenience for implementing it on
//! your own types.
//!
//! ```
//! # use lay::*;
//! let style = Style::default() * Red / Blue;
//! println!("Hello, {}world{}!", style, !style);
//!
//! let styled = Styled::from("world") + Bold;
//! println!("Hello, {}!", styled);
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
//! TODO
//!
//! See the [`layer`][mod_layer] module for details.
//!
//! ## Examples
//!
//! See the [`examples`][gh_examples] folder for examples.
//!
//! [mod_style]: style/index.html
//! [foreground]: style/struct.Foreground.html
//! [background]: style/struct.Background.html
//! [color]: style/enum.Color.html
//! [weight]: style/enum.Weight.html
//! [styler]: style/trait.Styler.html
//! [style]: style/struct.Style.html
//! [styled]: style/struct.Styled.html
//! [mod_layer]: layer/index.html
//! [layer]: layer/rait.Layer.html
//! [cell]: layer/type.Cell.html
//! [impl_styler]: macro.impl_styler.html
//! [gh_examples]: https://github.com/truchi/lay/tree/master/examples

// #[macro_use]
// mod macros;
//
// pub mod layer;
// pub mod style;
//
// pub use layer::*;
// pub use style::*;

pub mod new {
    use super::*;
    use lay_macros::*;

    make_color!();
    make_foreground!();
    make_background!();
    make_attrs!();
    // make_style!();

    /// LALALA  
    /// ```
    /// fn main() {
    ///      let lol = "lol";
    /// }
    /// assert_eq!(12, 12, #prout);
    /// ```
    #[color(prout, lol = caca)]
    _item! {}

    test!(
        /// LALALA
        ///
        /// ```
        /// fn main() {
        ///     let lol = "lol";
        /// }
        /// assert_eq!(12, 12, #prout);
        /// ```
    );
}
