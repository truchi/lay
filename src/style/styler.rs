use super::{
    Background,
    Blinking,
    Bordered,
    Color,
    Foreground,
    Inverted,
    Overlined,
    Slanted,
    Striked,
    Underlined,
    Weighted,
};
use std::fmt::{Display, Error, Formatter};

macro_rules! styler2 {
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
        pub trait Styler2: Sized {
            $(
                styler2!(impl Base "`Option<" stringify!($Color) ">`",
                    $get_color $set_color $set_color_mut
                    Option<$Color>: ($color)
                );
                styler2!(impl Set "`None`",
                    $unset_color $unset_color_mut [$set_color_mut]
                    () None
                );

                styler2!(impl Set "`Some(" stringify!($Color) "(Color::Rbg(r, g, b)))`",
                    $set_rgb $set_rgb_mut [$set_color_mut]
                    (r: u8, g: u8, b: u8,) Some($Color(Color::Rgb(r, g, b)))
                );
                styler2!(impl Set "`Some(" stringify!($Color) "(Color::AnsiValue(value)))`",
                    $set_ansi $set_ansi_mut [$set_color_mut]
                    (ansi: u8,) Some($Color(Color::Ansi(ansi)))
                );

                $(styler2!(impl Set "`Some(" stringify!($Color) "(Color::" stringify!($color_variant) "))`",
                    $set_color_variant $set_color_variant_mut [$set_color_mut]
                    () Some($Color(Color::$color_variant))
                );)*
            )*
            $(
                styler2!(impl Base "`Option<" stringify!($Attr) ">`",
                    $get_attr $set_attr $set_attr_mut
                    Option<$Attr>: ($attr)
                );
                styler2!(impl Set "`None`",
                    $unset_attr $unset_attr_mut [$set_attr_mut]
                    () None
                );

                $(styler2!(impl Set "`Some(" stringify!($Attr) "::" stringify!($attr_variant) ")`",
                    $set_attr_variant $set_attr_variant_mut [$set_attr_mut]
                    () Some(<$Attr>::$attr_variant)
                );)*
            )*

            styler2!(impl op and "Sets `None` if the field is `None`, otherwise sets `other`.",
                and and_mut [$($get_color $set_color_mut)* $($get_attr $set_attr_mut)*]
            );
            styler2!(impl op or "Sets the field if it contains a value, otherwise sets `other`.",
                or or_mut [$($get_color $set_color_mut)* $($get_attr $set_attr_mut)*]
            );
            styler2!(impl op xor "Sets `Some` if exactly one of `self`, `other` is `Some`, otherwise sets `None`.",
                xor xor_mut [$($get_color $set_color_mut)* $($get_attr $set_attr_mut)*]
            );

            /// Unsets fields if the value is identical to the corresponding one in
            /// `before`.
            fn dedup<T: Styler2>(mut self, before: &T) -> Self {
                $(self.$set_color_mut(dedup(self.$get_color(), before.$get_color()));)*
                $(self.$set_attr_mut (dedup(self.$get_attr(), before.$get_attr()));)*
                self
            }

            /// Unsets fields if the value is identical to the corresponding one in
            /// `before`.
            fn dedup_mut<T: Styler2>(&mut self, before: &T) {
                $(self.$set_color_mut(dedup(self.$get_color(), before.$get_color()));)*
                $(self.$set_attr_mut (dedup(self.$get_attr(), before.$get_attr()));)*
            }

           // /// Sets `Some` fields to their reset variant.
           // fn reset(self) -> Self {
           //     self.and(&Style::RESET)
           // }
           //
           // /// Sets `Some` fields to their reset variant.
           // fn reset_mut(&mut self) {
           //     self.and_mut(&Style::RESET);
           // }

            /// Formats the CSIs of `self` when `Some`.
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                $(if let Some(t) = self.$get_color() { t.fmt(f)?; })*
                $(if let Some(t) = self.$get_attr() { t.fmt(f)?; })*
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
            fn $fn<T: Styler2>(mut self, other: &T) -> Self {
                $(self.$set_mut(self.$get().$op(other.$get()));)*
                $(self.$set_mut(self.$get().$op(other.$get()));)*
                self
            }
        );
        doc!($($doc)*,
            fn $fn_mut<T: Styler2>(&mut self, other: &T) {
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

styler2!(
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
        get_weighted weighted weighted_mut: Weighted (weighted) {
            no_weight no_weight_mut
            bold bold_mut: Bold
            light light_mut: Light
            reset_weight reset_weight_mut: ResetWeight
        }
        get_slanted slanted slanted_mut: Slanted (slanted) {
            no_slant no_slant_mut
            italic italic_mut: Italic
            reset_slant reset_slant_mut: ResetSlant
        }
        get_blinking blinking blinking_mut: Blinking (blinking) {
            no_blink no_blink_mut
            slow slow_mut: Slow
            fast fast_mut: Fast
            reset_blink reset_blink_mut: ResetBlink
        }
        get_inverted inverted inverted_mut: Inverted (inverted) {
            no_invert no_invert_mut
            invert invert_mut: Invert
            reset_invert reset_invert_mut: ResetInvert
        }
        get_striked striked striked_mut: Striked (striked) {
            no_strike no_strike_mut
            strike strike_mut: Strike
            reset_strike reset_strike_mut: ResetStrike
        }
        get_underlined underlined underlined_mut: Underlined (underlined) {
            no_underline no_underline_mut
            underline underline_mut: Underline
            reset_underline reset_underline_mut: ResetUnderline
        }
        get_overlined overlined overlined_mut: Overlined (overlined) {
            no_overline no_overline_mut
            overline overline_mut: Overline
            reset_overline reset_overline_mut: ResetOverline
        }
        get_bordered bordered bordered_mut: Bordered (bordered) {
            no_border no_border_mut
            frame frame_mut: Frame
            circle circle_mut: Circle
            reset_border reset_border_mut: ResetBorder
        }
    }
);

// =======================================================================================
// =======================================================================================
// =======================================================================================
// =======================================================================================
// =======================================================================================
// =======================================================================================
// =======================================================================================
// =======================================================================================
// =======================================================================================
// =======================================================================================
// =======================================================================================
// =======================================================================================
// =======================================================================================
// =======================================================================================
// =======================================================================================
// =======================================================================================
// =======================================================================================
// =======================================================================================
// =======================================================================================
// =======================================================================================
// =======================================================================================
// =======================================================================================
// =======================================================================================
// =======================================================================================
// =======================================================================================

macro_rules! styler {
    (
        $(#[$meta_styler:meta])*
        $Styler:ident
        $(#[$meta_styler_mut:meta])*
        $StylerMut:ident
        Colors { $(
            $Color:ident($color:ident) $NoColor:ident [$IdxColor:ident]
            $OpColor:ident($op_color:ident) $OpAssignColor:ident($op_assign_color:ident) {
                $get_color:ident $get_mut_color:ident
                $set_color:ident $set_mut_color:ident
                $unset_color:ident $unset_mut_color:ident
                $reset_color:ident: $set_reset_color:ident $set_reset_mut_color:ident
                $($variant_color:ident: $set_variant_color:ident $set_variant_mut_color:ident)*
            }
        )* }
        Attributes { $(
            $Attr:ident($attr:ident) $NoAttr:ident [$IdxAttr:ident]
            $OpAttr:ident($op_attr:ident) $OpAssignAttr:ident($op_assign_attr:ident) {
                $get_attr:ident $get_mut_attr:ident
                $set_attr:ident $set_mut_attr:ident
                $unset_attr:ident $unset_mut_attr:ident
                $reset_attr:ident: $set_reset_attr:ident $set_reset_mut_attr:ident
                $($variant_attr:ident: $set_variant_attr:ident $set_variant_mut_attr:ident)*
            }
        )* }
    ) => {
        styler!(impl [No, Idx] $($Color $NoColor $IdxColor)* $($Attr $NoAttr $IdxAttr)*);

        $(#[$meta_styler])*
        pub trait $Styler<'a, T> where
            T: 'a + $Styler<'a, T>,
            Self: Sized,
            $(Self: Index<$IdxColor, Output = Option<$Color>>,)*
            $(Self: Index<$IdxAttr, Output = Option<$Attr>>,)*
            $(Self: $OpColor<$NoColor, Output = Self>,)*
            $(Self: $OpAttr<$NoAttr, Output = Self>,)*
            $(Self: $OpColor<Color, Output = Self>,)*
            $(Self: $OpAttr<$Attr, Output = Self>,)*
            Self: BitAnd<&'a T, Output = Self>,
            Self: BitOr<&'a T, Output = Self>,
            Self: BitXor<&'a T, Output = Self>,
            Self: Rem<&'a T, Output = Self>,
            Self: Not<Output = Self>,
        {
            $(styler!(impl [get] $Color($color) $IdxColor $get_color);)*
            $(styler!(impl [get] $Attr ($attr)  $IdxAttr  $get_attr);)*

            $(styler!(impl [unset] $Color($color) $unset_color $op_color($NoColor));)*
            $(styler!(impl [unset] $Attr ($attr)  $unset_attr  $op_attr ($NoAttr));)*

            $(styler!(impl [set] $Color($color) $set_color $op_color($color.0, $NoColor));)*
            $(styler!(impl [set] $Attr ($attr)  $set_attr  $op_attr ($attr,    $NoAttr));)*

            $(styler!(impl [variant]
                $(stringify!($Color) "(Color::" stringify!($variant_color) ")",
                    $set_variant_color $op_color(Color::$variant_color))*
                stringify!($Color) "(Color::" stringify!($reset_color) ")",
                    $set_reset_color $op_color(Color::$reset_color)
            );)*
            $(styler!(impl [variant]
                $(stringify!($Attr) "::" stringify!($variant_attr),
                    $set_variant_attr $op_attr($Attr::$variant_attr))*
                stringify!($Attr) "::" stringify!($reset_attr),
                    $set_reset_attr $op_attr($Attr::$reset_attr)
            );)*

            styler!(impl [op and] and $($get_color $set_color)* $($get_attr $set_attr)*);
            styler!(impl [op or ] or  $($get_color $set_color)* $($get_attr $set_attr)*);
            styler!(impl [op xor] xor $($get_color $set_color)* $($get_attr $set_attr)*);

            styler!(impl [op rem] dedup
                $($get_color $op_color($NoColor))*
                $($get_attr  $op_attr($NoAttr))*
            );

            styler!(impl [op not] reset
                $($get_color $op_color(Color::$reset_color))*
                $($get_attr  $op_attr ($Attr::$reset_attr))*
            );
        }

        $(#[$meta_styler_mut])*
        pub trait $StylerMut<'a, T> where
            T: 'a + $Styler<'a, T>,
            Self: 'a + $Styler<'a, T>,
            $(Self: IndexMut<$IdxColor>,)*
            $(Self: IndexMut<$IdxAttr>,)*
            $(Self: $OpAssignColor<$NoColor>,)*
            $(Self: $OpAssignAttr<$NoAttr>,)*
            $(Self: $OpAssignColor<Color>,)*
            $(Self: $OpAssignAttr<$Attr>,)*
            Self: BitAndAssign<&'a T>,
            Self: BitOrAssign<&'a T>,
            Self: BitXorAssign<&'a T>,
            Self: RemAssign<&'a T>,
            &'a mut Self: Not<Output = ()>,
        {
            $(styler!(impl [get mut] $Color($color) $IdxColor $get_mut_color);)*
            $(styler!(impl [get mut] $Attr ($attr)  $IdxAttr  $get_mut_attr);)*

            $(styler!(impl [unset mut] $Color($color) $unset_mut_color $op_assign_color($NoColor));)*
            $(styler!(impl [unset mut] $Attr ($attr)  $unset_mut_attr  $op_assign_attr ($NoAttr));)*

            $(styler!(impl [set mut] $Color($color) $set_mut_color $op_assign_color($color.0, $NoColor));)*
            $(styler!(impl [set mut] $Attr ($attr)  $set_mut_attr  $op_assign_attr ($attr,    $NoAttr));)*

            $(styler!(impl [variant mut]
                $(stringify!($Color) "(Color::" stringify!($variant_color) ")",
                    $set_variant_mut_color $op_assign_color(Color::$variant_color))*
                stringify!($Color) "(Color::" stringify!($reset_color) ")",
                    $set_reset_mut_color $op_assign_color(Color::$reset_color)
            );)*
            $(styler!(impl [variant mut]
                $(stringify!($Attr) "::" stringify!($variant_attr),
                    $set_variant_mut_attr $op_assign_attr($Attr::$variant_attr))*
                stringify!($Attr) "::" stringify!($reset_attr),
                    $set_reset_mut_attr $op_assign_attr($Attr::$reset_attr)
            );)*

            styler!(impl [op and mut] and   $($get_color $set_mut_color)*   $($get_attr $set_mut_attr)*);
            styler!(impl [op or  mut] or    $($get_color $set_mut_color)*   $($get_attr $set_mut_attr)*);
            styler!(impl [op xor mut] xor   $($get_color $set_mut_color)*   $($get_attr $set_mut_attr)*);

            styler!(impl [op rem mut] dedup
                $($get_color $op_assign_color($NoColor))*
                $($get_attr  $op_assign_attr ($NoAttr))*
            );

            styler!(impl [op not mut] reset_mut
                $($get_color $op_assign_color(Color::$reset_color))*
                $($get_attr  $op_assign_attr ($Attr::$reset_attr))*
            );
        }
    };

    (impl [No, Idx] $($Self:ident $No:ident $Idx:ident)*) => {
        $(
            doc!("Gets `Option<" stringify!($Self) ">`.",
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
            pub struct $Idx;);

            doc!("Sets `Option<" stringify!($Self) ">` to `None`.",
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
            pub struct $No;);
        )*
    };

    (impl [get] $Self:ident($self:ident) $Idx:ident $get:ident) => {
        doc!("Gets `Option<" stringify!($Self) ">`.",
        fn $get(&self) -> Option<$Self> {
            self[$Idx]
        });
    };
    (impl [get mut] $Self:ident($self:ident) $Idx:ident $get_mut:ident) => {
        doc!("Gets `&mut Option<" stringify!($Self) ">`.",
        fn $get_mut(&mut self) -> &mut Option<$Self> {
            &mut self[$Idx]
        });
    };

    (impl [set] $Self:ident($self:ident) $set:ident $op:ident($body:expr, $No:ident)) => {
        doc!("Sets `Option<" stringify!($Self) ">`.",
        fn $set(self, $self: impl Into<Option<$Self>>) -> Self {
            let $self = $self.into();

            if let Some($self) = $self {
                self.$op($body)
            } else {
                self.$op($No)
            }
        });
    };
    (impl [set mut] $Self:ident($self:ident) $set_mut:ident $op_assign:ident($body:expr, $No:ident)) => {
        doc!("Sets `Option<" stringify!($Self) ">` mutably.",
        fn $set_mut(&mut self, $self: impl Into<Option<$Self>>) {
            let $self = $self.into();

            if let Some($self) = $self {
                self.$op_assign($body);
            } else {
                self.$op_assign($No);
            }
        });
    };

    (impl [unset] $Self:ident($self:ident) $unset:ident $op:ident($No:ident)) => {
        doc!("Sets " stringify!($self) " to `None`.",
        fn $unset(self) -> Self {
            self.$op($No)
        });
    };
    (impl [unset mut] $Self:ident($self:ident) $unset_mut:ident $op_assign:ident($No:ident)) => {
        doc!("Sets " stringify!($self) " to `None` mutably.",
        fn $unset_mut(&mut self) {
            self.$op_assign($No);
        });
    };

    (impl [variant] $($($doc:expr)*, $set_variant:ident $op:ident($body:expr))*) => {
        $(doc!("Sets `Some(" $($doc)* ")`.",
        fn $set_variant(self) -> Self {
            self.$op($body)
        });)*
    };
    (impl [variant mut] $($($doc:expr)*, $set_variant_mut:ident $op_assign:ident($body:expr))*) => {
        $(doc!("Sets `Some(" $($doc)* ")` mutably.",
        fn $set_variant_mut(&mut self) {
            self.$op_assign($body);
        });)*
    };

    (impl [op rem] $fn:ident $($get:ident $op:ident($No:ident))*) => {
        /// Dedups (`None`s if identical) fields.
        fn $fn(mut self, before: &T) -> Self {
            $(if self.$get() == before.$get() {
                self = self.$op($No);
            })*
            self
        }
    };
    (impl [op rem mut] $fn_mut:ident $($get:ident $op_assign:ident($No:ident))*) => {
        /// Dedups (`None`s if identical) fields mutably.
        fn $fn_mut(&mut self, before: &T) {
            $(if self.$get() == before.$get() {
                self.$op_assign($No);
            })*
        }
    };

    (impl [op not] $fn:ident $($get:ident $op:ident($body:expr))*) => {
        /// Resets (sets to reset value/variant) fields which are `Some`.
        fn $fn(mut self) -> Self {
            $(if let Some(_) = self.$get() {
                self = self.$op($body);
            })*
            self
        }
    };
    (impl [op not mut] $fn_mut:ident $($get:ident $op_assign:ident($body:expr))*) => {
        /// Resets (sets to reset value/variant) fields which are `Some` mutably.
        fn $fn_mut(&mut self) {
            $(if let Some(_) = self.$get() {
                self.$op_assign($body);
            })*
        }
    };

    (impl [op $op:ident] $fn:ident $($get:ident $set:ident)*) => {
        doc!("`Option::" stringify!($op) "` fields.",
            fn $op(mut self, other: &'a T) -> Self {
                $(let x = self.$get().$op(other.$get());
                self = self.$set(x);)*
                self
            }
        );
    };
    (impl [op $op:ident mut] $fn_mut:ident $($get:ident $set_mut:ident)*) => {
        doc!("`Option::" stringify!($op) "` fields mutably.",
            fn $fn_mut(&mut self, other: &T) {
                $(self.$set_mut(self.$get().$op(other.$get()));)*
            }
        );
    };
}
