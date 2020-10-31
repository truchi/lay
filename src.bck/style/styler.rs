use super::*;
use std::fmt::{Display, Error, Formatter};

macro_rules! styler {
    (Colors { $(
        $Color:ident($color:ident) {
            $get_color:ident $get_mut_color:ident
            $set_color:ident $set_mut_color:ident
            $unset_color:ident $unset_mut_color:ident
            Rgb: $set_rgb:ident $set_rgb_mut:ident
            Ansi: $set_ansi:ident $set_ansi_mut:ident
            $reset_color:ident: $set_reset_color:ident $set_reset_mut_color:ident
            $($variant_color:ident: $set_variant_color:ident $set_variant_mut_color:ident)*
        } )*
    }
    Attributes { $(
        $Attr:ident($attr:ident) {
            $get_attr:ident $get_mut_attr:ident
            $set_attr:ident $set_mut_attr:ident
            $unset_attr:ident $unset_mut_attr:ident
            $reset_attr:ident: $set_reset_attr:ident $set_reset_mut_attr:ident
            $($variant_attr:ident: $set_variant_attr:ident $set_variant_mut_attr:ident)*
        } )*
    }) => {
        /// A trait for getting `Option`al attributes on styled types.
        pub trait StylerIndex {
            priv_styler!(
                $($get_color(&Self) -> Option<$Color>)*
                $($get_attr (&Self) -> Option<$Attr>)*
            );

            /// Returns a `Style`.
            fn get_style(&self) -> Style {
                Style {
                    $($color: self.$get_color(),)*
                    $($attr : self.$get_attr(),)*
                }
            }
        }

        /// A trait for getting `Option`al attributes on mutable styled types.
        pub trait StylerIndexMut {
            priv_styler!(
                $($get_mut_color(&mut Self) -> &mut Option<$Color>)*
                $($get_mut_attr (&mut Self) -> &mut Option<$Attr>)*
            );
        }

        /// A trait for setting `Option`al attributes on styled types.
        pub trait Styler: StylerIndex + Sized {
            /// The resulting type of the setters.
            type Output;

            priv_styler!((Self) -> Self::Output;
                $(($color: $Color) {
                    $set_color
                    $unset_color
                    { $set_rgb $set_ansi }
                    [$($set_variant_color($Color(Color::$variant_color)))*]
                    $set_reset_color($Color(Color::$reset_color))
                })*
                $(($attr: $Attr) {
                    $set_attr $unset_attr
                    [$($set_variant_attr($Attr::$variant_attr))*]
                    $set_reset_attr($Attr::$reset_attr)
                })*
            );

            priv_styler!((Self) (
                    $($get_color $set_color($Color(Color::$reset_color)))*
                    $($get_attr  $set_attr ($Attr::$reset_attr))*
                )
                style and or xor dedup reset
            );

            /// Formats the CSIs of `self`'s `Some` fields.
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                $(if let Some(t) = self.$get_color() { <$Color as Display>::fmt(&t, f)?; })*
                $(if let Some(t) = self.$get_attr () { <$Attr  as Display>::fmt(&t, f)?; })*
                Ok(())
            }
        }

        /// A trait for setting `Option`al attributes on mutable styled types.
        pub trait StylerMut: StylerIndex {
            priv_styler!((&mut Self) -> ();
                $(($color: $Color) {
                    $set_mut_color $unset_mut_color
                    { $set_rgb_mut $set_ansi_mut }
                    [$($set_variant_mut_color($Color(Color::$variant_color)))*]
                    $set_reset_mut_color($Color(Color::$reset_color))
                })*
                $(($attr: $Attr) {
                    $set_mut_attr $unset_mut_attr
                    [$($set_variant_mut_attr($Attr::$variant_attr))*]
                    $set_reset_mut_attr($Attr::$reset_attr)
                })*
            );

            priv_styler!((&mut Self) (
                    $($get_color $set_mut_color($Color(Color::$reset_color)))*
                    $($get_attr  $set_mut_attr ($Attr::$reset_attr))*
                )
                style_mut and_mut or_mut xor_mut dedup_mut reset_mut
            );
        }
    };
}

macro_rules! priv_styler {
    ($($get:ident($Self:ty) -> $Output:ty)*) => {
        $(doc!("Gets `" stringify!($Output) "`.",
        fn $get(self: $Self) -> $Output;);)*
    };

    (($Self:ty) -> $Output:ty;
        $(($attr:ident: $Attr:ident) {
            $set:ident $unset:ident
            $({ $set_rgb:ident $set_ansi:ident })?
            [$($set_variant:ident($body_variant:expr))*]
            $set_reset:ident($body_reset:expr)
        })*
    ) => {
        $(
            doc!("Sets `Option<" stringify!($Attr) ">`.",
            fn $set(self: $Self, $attr: impl Into<Option<$Attr>>) -> $Output;);

            doc!("`None`s `" stringify!($Attr) "`.",
            fn $unset(self: $Self) -> $Output {
                self.$set(None)
            });

            $(doc!("Sets `Some(" stringify!($body_variant) ")`.",
            fn $set_variant(self: $Self) -> $Output {
                self.$set(Some($body_variant))
            });)*

            $(doc!("Sets `Some(" stringify!($Attr) "(Color::Rgb(r, g, b)))`.",
            fn $set_rgb(self: $Self, r: u8, g: u8, b: u8) -> $Output {
                self.$set(Some($Attr(Color::Rgb(r, g, b))))
            });)?

            $(doc!("Sets `Some(" stringify!($Attr) "(Color::Ansi(ansi)))`.",
            fn $set_ansi(self: $Self, ansi: u8) -> $Output {
                self.$set(Some($Attr(Color::Ansi(ansi))))
            });)?

            doc!("Sets `Some(" stringify!($body_reset) ")`.",
            fn $set_reset(self: $Self) -> $Output {
                self.$set(Some($body_reset))
            });
        )*
    };
    ((Self) ($($get:ident $set:ident($reset_expr:expr))*)
        $style:ident $and:ident $or:ident $xor:ident $dedup:ident $reset:ident
    ) => {
        /// Applies `styler`'s styles.
        fn $style(self, styler: &impl StylerIndex) -> <Self::Output as Styler>::Output
        where
            Self::Output: Styler<Output = Self::Output>
        {
            let out = self;
            $(let out = out.$set(styler.$get());)*
            out
        }

        priv_styler!((Self) and $and $($get $set)*);
        priv_styler!((Self) or  $or  $($get $set)*);
        priv_styler!((Self) xor $xor $($get $set)*);

        /// Dedups (`None`s if identicals) fields.
        fn $dedup(mut self, before: &impl StylerIndex) -> Self
        where
            Self: Styler<Output = Self>
        {
            $(if self.$get() == before.$get() {
                self = self.$set(None);
            })*
            self
        }

        /// Resets (sets to reset value) fields which are `Some`.
        fn $reset(mut self) -> Self
        where
            Self: Styler<Output = Self>
        {
            $(if let Some(_) = self.$get() {
                self = self.$set(Some($reset_expr));
            })*
            self
        }
    };
    ((Self) $op:ident $fn:ident $($get:ident $set:ident)*) => {
        doc!("`Option::" stringify!($op) "` fields.",
        fn $fn(self, other: &impl StylerIndex) -> <Self::Output as Styler>::Output
        where
            Self::Output: Styler<Output = Self::Output>
        {
            let out = self;
            $(
                let op = out.$get().$op(other.$get());
                let out = out.$set(op);
            )*
            out
        });
    };
    ((&mut Self) ($($get:ident $set:ident($reset_expr:expr))*)
        $style:ident $and:ident $or:ident $xor:ident $dedup:ident $reset:ident
    ) => {
        /// Applies `styler`'s styles.
        fn $style(&mut self, styler: &impl StylerIndex) {
            $(self.$set(styler.$get());)*
        }

        priv_styler!((&mut Self) and $and $($get $set)*);
        priv_styler!((&mut Self) or  $or  $($get $set)*);
        priv_styler!((&mut Self) xor $xor $($get $set)*);

        /// Dedups (`None`s if identicals) fields.
        fn $dedup(&mut self, before: &impl StylerIndex) {
            $(if self.$get() == before.$get() {
                self.$set(None);
            })*
        }

        /// Resets (sets to reset value) fields which are `Some`.
        fn $reset(&mut self) {
            $(if let Some(_) = self.$get() {
                self.$set(Some($reset_expr));
            })*
        }
    };
    ((&mut Self) $op:ident $fn:ident $($get:ident $set:ident)*) => {
        doc!("`Option::" stringify!($op) "` fields.",
        fn $fn(&mut self, other: &impl StylerIndex) {
            $(self.$set(self.$get().$op(other.$get()));)*
        });
    };
}

styler!(
    Colors {
        Foreground(foreground) {
            get_foreground get_foreground_mut
            foreground foreground_mut
            no_foreground no_foreground_mut
                Rgb: rgb rgb_mut
                Ansi: ansi ansi_mut
                ResetColor: reset_color reset_color_mut
                White: white white_mut
                Black: black black_mut
                Grey: grey grey_mut
                DarkGrey: dark_grey dark_grey_mut
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
        Background(background) {
            get_background get_background_mut
            background background_mut
            no_background no_background_mut
                Rgb: on_rgb on_rgb_mut
                Ansi: on_ansi on_ansi_mut
                ResetColor: on_reset_color on_reset_color_mut
                White: on_white on_white_mut
                Black: on_black on_black_mut
                Grey: on_grey on_grey_mut
                DarkGrey: on_dark_grey on_dark_grey_mut
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
        Weight(weight) {
            get_weight get_weight_mut
            weight weight_mut
            no_weight no_weight_mut
                ResetWeight: reset_weight reset_weight_mut
                Bold: bold bold_mut
                Light: light light_mut
        }
        Slant(slant) {
            get_slant get_slant_mut
            slant slant_mut
            no_slant no_slant_mut
                ResetSlant: reset_slant reset_slant_mut
                Italic: italic italic_mut
        }
        Blink(blink) {
            get_blink get_blink_mut
            blink blink_mut
            no_blink no_blink_mut
                ResetBlink: reset_blink reset_blink_mut
                Slow: slow slow_mut
                Fast: fast fast_mut
        }
        Invert(invert) {
            get_invert get_invert_mut
            invert invert_mut
            no_invert no_invert_mut
                ResetInvert: reset_invert reset_invert_mut
                Inverted: inverted inverted_mut
        }
        Strike(strike) {
            get_strike get_strike_mut
            strike strike_mut
            no_strike no_strike_mut
                ResetStrike: reset_strike reset_strike_mut
                Striked: striked striked_mut
        }
        Underline(underline) {
            get_underline get_underline_mut
            underline underline_mut
            no_underline no_underline_mut
                ResetUnderline: reset_underline reset_underline_mut
                Underlined: underlined underlined_mut
        }
        Overline(overline) {
            get_overline get_overline_mut
            overline overline_mut
            no_overline no_overline_mut
                ResetOverline: reset_overline reset_overline_mut
                Overlined: overlined overlined_mut
        }
        Border(border) {
            get_border get_border_mut
            border border_mut
            no_border no_border_mut
                ResetBorder: reset_border reset_border_mut
                Frame: frame frame_mut
                Circle: circle circle_mut
        }
    }
);

impl_styler_index!(
    <('a)> (_: &'a str) {
        None, None, None, None, None, None, None, None, None, None,
    }
    (_: String) {
        None, None, None, None, None, None, None, None, None, None,
    }
);

impl_styler!(
    (content: String) -> Styled<String> {
        (foreground) Styled { content, style: Style::NONE.foreground(foreground) },
        (background) Styled { content, style: Style::NONE.background(background) },
        (weight)     Styled { content, style: Style::NONE.weight(weight) },
        (slant)      Styled { content, style: Style::NONE.slant(slant) },
        (blink)      Styled { content, style: Style::NONE.blink(blink) },
        (invert)     Styled { content, style: Style::NONE.invert(invert) },
        (strike)     Styled { content, style: Style::NONE.strike(strike) },
        (underline)  Styled { content, style: Style::NONE.underline(underline) },
        (overline)   Styled { content, style: Style::NONE.overline(overline) },
        (border)     Styled { content, style: Style::NONE.border(border) },
    }
    <('a)> (content: &'a str) -> Styled<&'a str> {
        (foreground) Styled { content, style: Style::NONE.foreground(foreground) },
        (background) Styled { content, style: Style::NONE.background(background) },
        (weight)     Styled { content, style: Style::NONE.weight(weight) },
        (slant)      Styled { content, style: Style::NONE.slant(slant) },
        (blink)      Styled { content, style: Style::NONE.blink(blink) },
        (invert)     Styled { content, style: Style::NONE.invert(invert) },
        (strike)     Styled { content, style: Style::NONE.strike(strike) },
        (underline)  Styled { content, style: Style::NONE.underline(underline) },
        (overline)   Styled { content, style: Style::NONE.overline(overline) },
        (border)     Styled { content, style: Style::NONE.border(border) },
    }
);
