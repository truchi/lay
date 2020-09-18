use super::{
    Background,
    Blinking,
    Bordered,
    Color,
    Foreground,
    Inverted,
    OptionalStyle,
    Overlined,
    Slanted,
    Striked,
    Underlined,
    Weighted,
};
use std::fmt::{Display, Error, Formatter};

macro_rules! styler {
    (colors {
        $($get_color:ident $set_color:ident $set_color_mut:ident: $Color:ident ($color:ident) {
            $unset_color:ident $unset_color_mut:ident
            $set_rgb:ident $set_rgb_mut:ident $set_ansi:ident $set_ansi_mut:ident
            $($set_color_variant:ident $set_color_variant_mut:ident: $color_variant:ident)*
        })*
    }
    attributes {
        $($get_attr:ident $set_attr:ident $set_attr_mut:ident: $Attr:ident ($attr:ident) {
            $unset_attr:ident $unset_attr_mut:ident
            $($set_attr_variant:ident $set_attr_variant_mut:ident : $attr_variant:ident)*
        })*
    }) => {
        /// A trait for styled types.
        pub trait Styler: Sized {
            $(
                styler!(impl Base "`" stringify!($Color) "`",
                    $get_color $set_color $set_color_mut
                    $Color: ($color)
                );

                styler!(impl Set "`" stringify!($Color) "(Color::Rbg { r, g, b })`",
                    $set_rgb $set_rgb_mut [$set_color_mut]
                    (r: u8, g: u8, b: u8,) $Color(Color::Rgb { r, g, b })
                );
                styler!(impl Set "`" stringify!($Color) "(Color::AnsiValue(value))`",
                    $set_ansi $set_ansi_mut [$set_color_mut]
                    (value: u8,) $Color(Color::AnsiValue(value))
                );

                $(styler!(impl Set "`" stringify!($Color) "(Color::" stringify!($color_variant) ")`",
                    $set_color_variant $set_color_variant_mut [$set_color_mut]
                    () $Color(Color::$color_variant)
                );)*
            )*
            $(
                styler!(impl Base "`" stringify!($Attr) "`",
                    $get_attr $set_attr $set_attr_mut
                    $Attr: ($attr)
                );

                $(styler!(impl Set "`" stringify!($Attr) "::" stringify!($attr_variant) "`",
                    $set_attr_variant $set_attr_variant_mut [$set_attr_mut]
                    () <$Attr>::$attr_variant
                );)*
            )*

            /// Sets fields to their reset variant.
            fn reset(self) -> Self {
                self
                    $(.$set_color(Default::default()))*
                    $(.$set_attr (Default::default()))*
           }

            /// Sets fields to their reset variant.
            fn reset_mut(&mut self) {
                $(self.$set_color_mut(Default::default());)*
                $(self.$set_attr_mut (Default::default());)*
            }

            /// Formats the CSIs of `self`.
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                $(self.$get_color().fmt(f)?;)*
                $(self.$get_attr() .fmt(f)?;)*
                Ok(())
            }
        }

        /// A trait for `Option`al styled types.
        pub trait OptionalStyler: Sized {
            $(
                styler!(impl Base "`Option<" stringify!($Color) ">`",
                    $get_color $set_color $set_color_mut
                    Option<$Color>: ($color)
                );
                styler!(impl Set "`None`",
                    $unset_color $unset_color_mut [$set_color_mut]
                    () None
                );

                styler!(impl Set "`Some(" stringify!($Color) "(Color::Rbg { r, g, b }))`",
                    $set_rgb $set_rgb_mut [$set_color_mut]
                    (r: u8, g: u8, b: u8,) Some($Color(Color::Rgb { r, g, b }))
                );
                styler!(impl Set "`Some(" stringify!($Color) "(Color::AnsiValue(value)))`",
                    $set_ansi $set_ansi_mut [$set_color_mut]
                    (value: u8,) Some($Color(Color::AnsiValue(value)))
                );

                $(styler!(impl Set "`Some(" stringify!($Color) "(Color::" stringify!($color_variant) "))`",
                    $set_color_variant $set_color_variant_mut [$set_color_mut]
                    () Some($Color(Color::$color_variant))
                );)*
            )*
            $(
                styler!(impl Base "`Option<" stringify!($Attr) ">`",
                    $get_attr $set_attr $set_attr_mut
                    Option<$Attr>: ($attr)
                );
                styler!(impl Set "`None`",
                    $unset_attr $unset_attr_mut [$set_attr_mut]
                    () None
                );

                $(styler!(impl Set "`Some(" stringify!($Attr) "::" stringify!($attr_variant) ")`",
                    $set_attr_variant $set_attr_variant_mut [$set_attr_mut]
                    () Some(<$Attr>::$attr_variant)
                );)*
            )*

            styler!(impl op and "Sets `None` if the field is `None`, otherwise sets `other`.",
                and and_mut [$($get_color $set_color_mut)* $($get_attr $set_attr_mut)*]
            );
            styler!(impl op or "Sets the field if it contains a value, otherwise sets `other`.",
                or or_mut [$($get_color $set_color_mut)* $($get_attr $set_attr_mut)*]
            );
            styler!(impl op xor "Sets `Some` if exactly one of `self`, `other` is `Some`, otherwise sets `None`.",
                xor xor_mut [$($get_color $set_color_mut)* $($get_attr $set_attr_mut)*]
            );

            /// Unsets fields if the value is identical to the corresponding one in
            /// `before`.
            fn dedup<T: OptionalStyler>(mut self, before: &T) -> Self {
                $(self.$set_color_mut(dedup(self.$get_color(), before.$get_color()));)*
                $(self.$set_attr_mut (dedup(self.$get_attr() , before.$get_attr() ));)*
                self
            }

            /// Unsets fields if the value is identical to the corresponding one in
            /// `before`.
            fn dedup_mut<T: OptionalStyler>(&mut self, before: &T) {
                $(self.$set_color_mut(dedup(self.$get_color(), before.$get_color()));)*
                $(self.$set_attr_mut (dedup(self.$get_attr() , before.$get_attr() ));)*
            }

            /// Sets `Some` fields to their reset variant.
            fn reset(self) -> Self {
                self.and(&OptionalStyle::RESET)
            }

            /// Sets `Some` fields to their reset variant.
            fn reset_mut(&mut self) {
                self.and_mut(&OptionalStyle::RESET);
            }

            /// Formats the CSIs of `self` when `Some`.
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                $(if let Some(t) = self.$get_color() { t.fmt(f)?; })*
                $(if let Some(t) = self.$get_attr()  { t.fmt(f)?; })*
                Ok(())
            }
        }
    };
    (impl Base
        $($doc:expr)*,
        $get:ident $set:ident $set_mut:ident
        $Type:ty: ($arg:ident)
    ) => {
        doc!("Gets " $($doc)*,
            fn $get(&self) -> $Type;
        );
        doc!("Sets " $($doc)*,
            fn $set_mut(&mut self, $arg: $Type);
        );
        doc!("Sets " $($doc)*,
            fn $set(mut self, $arg: $Type) -> Self {
                self.$set_mut($arg);
                self
            }
        );
    };
    (impl Set
        $($doc:expr)*,
        $fn:ident $fn_mut:ident [$set_mut:ident]
        ($($arg:ident: $ArgType:ident,)*)
        $body:expr
    ) => {
        doc!("Sets " $($doc)*,
            fn $fn(mut self $(, $arg: $ArgType)*) -> Self {
                self.$set_mut($body);
                self
            }
        );
        doc!("Sets " $($doc)*,
            fn $fn_mut(&mut self $(, $arg: $ArgType)*) {
                self.$set_mut($body);
            }
        );
    };
    (impl op $op:ident $($doc:expr)*, $fn:ident $fn_mut:ident [$($get:ident $set_mut:ident)*]) => {
        doc!($($doc)*,
            fn $fn<T: OptionalStyler>(mut self, other: &T) -> Self {
                $(self.$set_mut(self.$get().$op(other.$get()));)*
                $(self.$set_mut(self.$get().$op(other.$get()));)*
                self
            }
        );
        doc!($($doc)*,
            fn $fn_mut<T: OptionalStyler>(&mut self, other: &T) {
                $(self.$set_mut(self.$get().$op(other.$get()));)*
                $(self.$set_mut(self.$get().$op(other.$get()));)*
            }
        );
    }
}

/// Dedup helper.
fn dedup<T: PartialEq>(a: Option<T>, before: Option<T>) -> Option<T> {
    match (&a, &before) {
        (Some(a), Some(before)) if a == before => None,
        _ => a,
    }
}

styler!(
    colors {
        get_foreground foreground foreground_mut: Foreground (foreground) {
            no_foreground no_foreground_mut
            rgb rgb_mut ansi ansi_mut
            white white_mut: White
            black black_mut: Black
            red red_mut: Red
            dark_red dark_red_mut: DarkRed
            green green_mut: Green
            dark_green dark_green_mut: DarkGreen
            yellow yellow_mut: Yellow
            dark_yellow dark_yellow_mut: DarkYellow
            blue blue_mut: Blue
            dark_blue dark_blue_mut: DarkBlue
            magenta magenta_mut: Magenta
            dark_magenta dark_magenta_mut: DarkMagenta
            cyan cyan_mut: Cyan
            dark_cyan dark_cyan_mut: DarkCyan
        }
        get_background background background_mut: Background (background) {
            no_background no_background_mut
            on_rgb on_rgb_mut on_ansi on_ansi_mut
            on_white on_white_mut: White
            on_black on_black_mut: Black
            on_red on_red_mut: Red
            on_dark_red on_dark_red_mut: DarkRed
            on_green on_green_mut: Green
            on_dark_green on_dark_green_mut: DarkGreen
            on_yellow on_yellow_mut: Yellow
            on_dark_yellow on_dark_yellow_mut: DarkYellow
            on_blue on_blue_mut: Blue
            on_dark_blue on_dark_blue_mut: DarkBlue
            on_magenta on_magenta_mut: Magenta
            on_dark_magenta on_dark_magenta_mut: DarkMagenta
            on_cyan on_cyan_mut: Cyan
            on_dark_cyan on_dark_cyan_mut: DarkCyan
        }
    }
    attributes {
        get_weighted weighted weighted_mut: Weighted (weight) {
            no_weight no_weight_mut
            bold bold_mut: Bold
            light light_mut: Light
            reset_weight reset_weight_mut: ResetWeight
        }
        get_slanted slanted slanted_mut: Slanted (slant) {
            no_slant no_slant_mut
            italic italic_mut: Italic
            reset_slant reset_slant_mut: ResetSlant
        }
        get_blinking blinking blinking_mut: Blinking (blink) {
            no_blink no_blink_mut
            slow slow_mut: Slow
            fast fast_mut: Fast
            reset_blink reset_blink_mut: ResetBlink
        }
        get_inverted inverted inverted_mut: Inverted (invert) {
            no_invert no_invert_mut
            invert invert_mut: Invert
            reset_invert reset_invert_mut: ResetInvert
        }
        get_striked striked striked_mut: Striked (strike) {
            no_strike no_strike_mut
            strike strike_mut: Strike
            reset_strike reset_strike_mut: ResetStrike
        }
        get_underlined underlined underlined_mut: Underlined (underline) {
            no_underline no_underline_mut
            underline underline_mut: Underline
            reset_underline reset_underline_mut: ResetUnderline
        }
        get_overlined overlined overlined_mut: Overlined (overline) {
            no_overline no_overline_mut
            overline overline_mut: Overline
            reset_overline reset_overline_mut: ResetOverline
        }
        get_bordered bordered bordered_mut: Bordered (border) {
            no_border no_border_mut
            frame frame_mut: Frame
            circle circle_mut: Circle
            reset_border reset_border_mut: ResetBorder
        }
    }
);
