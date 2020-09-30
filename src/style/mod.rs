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
//!         Foreground(ResetColor),
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
//!     assert_eq!(reset, style * ResetColor / ResetColor + ResetWeight);
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

#[macro_use]
mod attributes;
#[macro_use]
mod colors;
#[macro_use]
mod style;
// mod styled;
#[macro_use]
mod styler;

pub use attributes::*;
pub use colors::*;
pub use style::*;
// pub use styled::*;
pub use styler::*;

use std::fmt::{Display, Error, Formatter};

macro_rules! mod_style {
    (
        Colors { $(
            $(#[$meta_color:meta])*
            $Color:ident($color:ident) $NoColor:ident ($str_color:literal $str_reset_color:literal) {
                $get_color:ident $get_mut_color:ident
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
                $get_attr:ident $get_mut_attr:ident
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
            $(($color: $Color, $Color(Color::$reset_color)))*
            $(($attr: $Attr, $Attr::$reset_attr))*
        );

        styler!(
            Colors { $(
                $Color($color) $NoColor {
                    $get_color $get_mut_color
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
                    $get_attr $get_mut_attr
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
    Colors {
        /// A `Foreground` `Color`.
        ///
        /// Prints the corresponding CSI to the terminal when `Display`ed.
        ///
        /// `Default`s to `Foreground(Color::ResetColor)`, user's default terminal foreground color.
        Foreground(foreground) NoForeground ("38;" "39") {
            get_foreground get_foreground_mut
            foreground foreground_mut
            no_foreground no_foreground_mut
                ResetColor: fg_reset fg_reset_mut
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
            get_background get_background_mut
            background background_mut
            no_background no_background_mut
                ResetColor: on_reset on_reset_mut
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
        /// `Weighted` text (`Bold`, `Light`, `ResetBold`).
        ///
        /// Prints the corresponding CSI to the terminal when `Display`ed.
        ///
        /// `Default`s to `ResetWeight`, the weight unsetting CSI.
        Weighted(weighted) NoWeight {
            get_weighted get_weighted_mut
            weighted weighted_mut
            no_weight no_weight_mut
                ResetWeight("22"): reset_weight reset_weight_mut
                Bold("1"): bold bold_mut
                Light("2"): light light_mut
        }
        /// `Slanted` text (`Italic`, `ResetSlant`).
        ///
        /// Prints the corresponding CSI to the terminal when `Display`ed.
        ///
        /// `Default`s to `ResetSlant`, the slant unsetting CSI.
        Slanted(slanted) NoSlant {
            get_slanted get_slanted_mut
            slanted slanted_mut
            no_slant no_slant_mut
                ResetSlant("23"): reset_slant reset_slant_mut
                Italic("3"): italic italic_mut
        }
        /// `Blinking` text (`Slow`, `Fast`, `ResetBlink`).
        ///
        /// Prints the corresponding CSI to the terminal when `Display`ed.
        ///
        /// `Default`s to `ResetBlink`, the blink unsetting CSI.
        Blinking(blinking) NoBlink {
            get_blinking get_blinking_mut
            blinking blinking_mut
            no_blink no_blink_mut
                ResetBlink("25"): reset_blink reset_blink_mut
                Slow("5"): slow slow_mut
                Fast("6"): fast fast_mut
        }
        /// `Inverted` text (`Invert`, `ResetInvert`).
        ///
        /// Prints the corresponding CSI to the terminal when `Display`ed.
        ///
        /// `Default`s to `ResetInvert`, the invert unsetting CSI.
        Inverted(inverted) NoInvert {
            get_inverted get_inverted_mut
            inverted inverted_mut
            no_invert no_invert_mut
                ResetInvert("27"): reset_invert reset_invert_mut
                Invert("7"): invert invert_mut
        }
        /// `Striked` text (`Strike`, `ResetStrike`).
        ///
        /// Prints the corresponding CSI to the terminal when `Display`ed.
        ///
        /// `Default`s to `ResetStrike`, the strike unsetting CSI.
        Striked(striked) NoStrike {
            get_striked get_striked_mut
            striked striked_mut
            no_strike no_strike_mut
                ResetStrike("29"): reset_strike reset_strike_mut
                Strike("9"): strike strike_mut
        }
        /// `Underlined` text (`Underline`, `ResetUnderline`).
        ///
        /// Prints the corresponding CSI to the terminal when `Display`ed.
        ///
        /// `Default`s to `ResetUnderline`, the underline unsetting CSI.
        Underlined(underlined) NoUnderline {
            get_underlined get_underlined_mut
            underlined underlined_mut
            no_underline no_underline_mut
                ResetUnderline("24"): reset_underline reset_underline_mut
                Underline("4"): underline underline_mut
        }
        /// `Overlined` text (`Overline`, `ResetOverline`).
        ///
        /// Prints the corresponding CSI to the terminal when `Display`ed.
        ///
        /// `Default`s to `ResetOverline`, the overline unsetting CSI.
        Overlined(overlined) NoOverline {
            get_overlined get_overlined_mut
            overlined overlined_mut
            no_overline no_overline_mut
                ResetOverline("55"): reset_overline reset_overline_mut
                Overline("53"): overline overline_mut
        }
        /// `Bordered` text (`Frame`, `Circle`, `ResetBorder`).
        ///
        /// Prints the corresponding CSI to the terminal when `Display`ed.
        ///
        /// `Default`s to `ResetBorder`, the border unsetting CSI.
        Bordered(bordered) NoBorder {
            get_bordered get_bordered_mut
            bordered bordered_mut
            no_border no_border_mut
                ResetBorder("54"): reset_border reset_border_mut
                Frame("51"): frame frame_mut
                Circle("52"): circle circle_mut
        }
    }
);
