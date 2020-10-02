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
//! assert_eq!(style.get_slant(), None);
//! ```
//!
//! It also provides convenients methods for styles manipulation:
//! [`and`][styler_and], [`or`][styler_or], [`xor`][styler_xor],
//! [`dedup`][styler_dedup], [`reset`][styler_reset].
//!
//! For an easier use of styles, [`Styler`][styler] enables styling operations
//! overloads. The above example could be written:
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
//! `lay` defines handy unit struct to `None` their corresponding attribute
//! fields: [`NoColor`][no_color], [`NoForeground`][no_foreground],
//! [`NoBackground`][no_background], [`NoWeight`][no_weight],
//! [`NoSlant`][no_slant], [`NoBlink`][no_blink], [`NoInvert`][no_invert],
//! [`NoStrike`][no_strike], [`NoUnderline`][no_underline],
//! [`NoOverline`][no_overline], [`NoBorder`][no_border].
//!
//! ```
//! # use lay::*;
//! let style =
//!     Style::default() * None / None + NoForeground + NoBackground + NoWeight + NoSlant + NoBlink;
//! // etc...
//! ```
//!
//! [`Styler`][styler] can easily be implemented with the
//! [`impl_styler`][impl_styler] macro, and you can generate the operators
//! overloads with the [`impl_styler_ops`][impl_styler_ops] macro.
//!
//! You can disable styling operations overloads (and
//! [`impl_styler_ops`][impl_styler_ops]) by opting out of the `styler-ops`
//! default feature.
//!
//! # `Style`
//!
//! The [`Style`][style] struct is the most simple implementation of `Styler`
//! you can think of: it has a field for each attribute wrapped in an `Option`.
//! It implements styling operations overloads.
//!
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
//! [no_color]: struct.NoColor.html
//! [no_foreground]: struct.NoForeground.html
//! [no_background]: struct.NoBackground.html
//! [no_weight]: struct.NoWeight.html
//! [no_slant]: struct.NoSlant.html
//! [no_blink]: struct.NoBlink.html
//! [no_invert]: struct.NoInvert.html
//! [no_strike]: struct.NoStrike.html
//! [no_underline]: struct.NoUnderline.html
//! [no_overline]: struct.NoOverline.html
//! [no_border]: struct.NoBorder.html
//! [style]: struct.Style.html
//! [impl_styler]: ../macro.impl_styler.html
//! [impl_styler_ops]: ../macro.impl_styler_ops.html

#[macro_use]
mod attributes;
#[macro_use]
mod colors;
#[macro_use]
mod style;
mod styled;
#[macro_use]
mod styler;

pub use attributes::*;
pub use colors::*;
pub use style::*;
pub use styled::*;
pub use styler::*;

use std::fmt::{Display, Error, Formatter};

macro_rules! mod_style {
    (
        $(#[$meta_reset:meta])*
        $Reset:ident
        Colors { $(
            $(#[$meta_color:meta])*
            $Color:ident($color:ident) $NoColor:ident ($str_color:literal $str_reset_color:literal) {
                $get_color:ident
                $set_color:ident $set_mut_color:ident
                $unset_color:ident $unset_mut_color:ident
                $reset_color:ident: $set_reset_color:ident $set_reset_mut_color:ident
                Rgb: $set_rgb:ident $set_rgb_mut:ident
                Ansi: $set_ansi:ident $set_ansi_mut:ident
                $($variant_color:ident: $set_variant_color:ident $set_variant_mut_color:ident)*
            }
        )* }
        Attributes { $(
            $(#[$meta_attr:meta])*
            $Attr:ident($attr:ident) $NoAttr:ident {
                $get_attr:ident
                $set_attr:ident $set_mut_attr:ident
                $unset_attr:ident $unset_mut_attr:ident
                $reset_attr:ident($str_reset_attr:literal): $set_reset_attr:ident $set_reset_mut_attr:ident
                $($variant_attr:ident($str_attr:literal): $set_variant_attr:ident $set_variant_mut_attr:ident)*
            }
        )* }
    ) => {
        colors!($(
            $(#[$meta_color])*
            $Color ($str_color $str_reset_color)
        )*);

        attributes!($(
            $(#[$meta_attr])*
            $Attr: $($variant_attr($str_attr))* + $reset_attr($str_reset_attr)
        )*);

        style!(
            $(($color: $Color, $set_color, $Color(Color::$reset_color)))*
            $(($attr: $Attr, $set_attr, $Attr::$reset_attr))*
        );

        styler!(
            $(#[$meta_reset])*
            $Reset
            Colors { $(
                $Color($color) $NoColor {
                    $get_color
                    $set_color $set_mut_color
                    $unset_color $unset_mut_color
                    Rgb: $set_rgb $set_rgb_mut
                    Ansi: $set_ansi $set_ansi_mut
                    $reset_color: $set_reset_color $set_reset_mut_color
                    $($variant_color: $set_variant_color $set_variant_mut_color)*
                }
            )* }
            Attributes { $(
                $Attr($attr) $NoAttr {
                    $get_attr
                    $set_attr $set_mut_attr
                    $unset_attr $unset_mut_attr
                    $reset_attr: $set_reset_attr $set_reset_mut_attr
                    $($variant_attr: $set_variant_attr $set_variant_mut_attr)*
                }
            )* }
        );
    };
}

mod_style!(
    /// `Reset`s all terminal attributes.
    Reset
    Colors {
        /// A `Foreground` `Color`.
        ///
        /// Prints the corresponding CSI to the terminal when `Display`ed.
        ///
        /// `Default`s to `Foreground(Color::ResetColor)`, user's default terminal foreground color.
        Foreground(foreground) NoForeground ("38;" "39") {
            get_foreground
            foreground foreground_mut
            no_foreground no_foreground_mut
                ResetColor: reset_color reset_color_mut
                Rgb: rgb rgb_mut
                Ansi: ansi ansi_mut
                White: white white_mut
                Black: black black_mut
                Red: red red_mut
                DarkRed: dark_red dark_red_mut
                Green: green green_mut
                DarkGreen: dark_green dark_green_mut
                Yellow: yellow yellow_mut
                DarkYellow: dark_yellow dark_yellow_mut
                Blue: blue blue_mut
                DarkBlue: dark_blue dark_blue_mut
                Magenta: magenta magenta_mut
                DarkMagenta: dark_magenta dark_magenta_mut
                Cyan: cyan cyan_mut
                DarkCyan: dark_cyan dark_cyan_mut
        }
        /// A `Background` `Color`.
        ///
        /// Prints the corresponding CSI to the terminal when `Display`ed.
        ///
        /// `Default`s to `Background(Color::ResetColor)`, user's default terminal background color.
        Background(background) NoBackground ("48;" "49") {
            get_background
            background background_mut
            no_background no_background_mut
                ResetColor: on_reset_color on_reset_color_mut
                Rgb: on_rgb on_rgb_mut
                Ansi: on_ansi on_ansi_mut
                White: on_white on_white_mut
                Black: on_black on_black_mut
                Red: on_red on_red_mut
                DarkRed: on_dark_red on_dark_red_mut
                Green: on_green on_green_mut
                DarkGreen: on_dark_green on_dark_green_mut
                Yellow: on_yellow on_yellow_mut
                DarkYellow: on_dark_yellow on_dark_yellow_mut
                Blue: on_blue on_blue_mut
                DarkBlue: on_dark_blue on_dark_blue_mut
                Magenta: on_magenta on_magenta_mut
                DarkMagenta: on_dark_magenta on_dark_magenta_mut
                Cyan: on_cyan on_cyan_mut
                DarkCyan: on_dark_cyan on_dark_cyan_mut
        }
    }
    Attributes {
        /// `Weight` text (`Bold`, `Light`, `ResetBold`).
        ///
        /// Prints the corresponding CSI to the terminal when `Display`ed.
        ///
        /// `Default`s to `ResetWeight`, the weight unsetting CSI.
        Weight(weight) NoWeight {
            get_weight
            weight weight_mut
            no_weight no_weight_mut
                ResetWeight("22"): reset_weight reset_weight_mut
                Bold("1"): bold bold_mut
                Light("2"): light light_mut
        }
        /// `Slant` text (`Italic`, `ResetSlant`).
        ///
        /// Prints the corresponding CSI to the terminal when `Display`ed.
        ///
        /// `Default`s to `ResetSlant`, the slant unsetting CSI.
        Slant(slant) NoSlant {
            get_slant
            slant slant_mut
            no_slant no_slant_mut
                ResetSlant("23"): reset_slant reset_slant_mut
                Italic("3"): italic italic_mut
        }
        /// `Blink` text (`Slow`, `Fast`, `ResetBlink`).
        ///
        /// Prints the corresponding CSI to the terminal when `Display`ed.
        ///
        /// `Default`s to `ResetBlink`, the blink unsetting CSI.
        Blink(blink) NoBlink {
            get_blink
            blink blink_mut
            no_blink no_blink_mut
                ResetBlink("25"): reset_blink reset_blink_mut
                Slow("5"): slow slow_mut
                Fast("6"): fast fast_mut
        }
        /// `Invert` text (`Inverted`, `ResetInvert`).
        ///
        /// Prints the corresponding CSI to the terminal when `Display`ed.
        ///
        /// `Default`s to `ResetInvert`, the invert unsetting CSI.
        Invert(invert) NoInvert {
            get_invert
            invert invert_mut
            no_invert no_invert_mut
                ResetInvert("27"): reset_invert reset_invert_mut
                Inverted("7"): inverted inverted_mut
        }
        /// `Strik` text (`Striked`, `ResetStrike`).
        ///
        /// Prints the corresponding CSI to the terminal when `Display`ed.
        ///
        /// `Default`s to `ResetStrike`, the strike unsetting CSI.
        Strike(strike) NoStrike {
            get_strike
            strike strike_mut
            no_strike no_strike_mut
                ResetStrike("29"): reset_strike reset_strike_mut
                Striked("9"): striked striked_mut
        }
        /// `Underline` text (`Underlined`, `ResetUnderline`).
        ///
        /// Prints the corresponding CSI to the terminal when `Display`ed.
        ///
        /// `Default`s to `ResetUnderline`, the underline unsetting CSI.
        Underline(underline) NoUnderline {
            get_underline
            underline underline_mut
            no_underline no_underline_mut
                ResetUnderline("24"): reset_underline reset_underline_mut
                Underlined("4"): underlined underlined_mut
        }
        /// `Overlined` text (`Overlined`, `ResetOverline`).
        ///
        /// Prints the corresponding CSI to the terminal when `Display`ed.
        ///
        /// `Default`s to `ResetOverline`, the overline unsetting CSI.
        Overline(overline) NoOverline {
            get_overline
            overline overline_mut
            no_overline no_overline_mut
                ResetOverline("55"): reset_overline reset_overline_mut
                Overlined("53"): overlined overlined_mut
        }
        /// `Border` text (`Frame`, `Circle`, `ResetBorder`).
        ///
        /// Prints the corresponding CSI to the terminal when `Display`ed.
        ///
        /// `Default`s to `ResetBorder`, the border unsetting CSI.
        Border(border) NoBorder {
            get_border
            border border_mut
            no_border no_border_mut
                ResetBorder("54"): reset_border reset_border_mut
                Frame("51"): frame frame_mut
                Circle("52"): circle circle_mut
        }
    }
);
