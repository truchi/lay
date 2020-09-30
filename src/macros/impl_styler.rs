#[macro_export]
macro_rules! impl_styler {
    ($(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:ident: $Type:ty) {
        $foreground:expr,
        $background:expr,
        $weight:expr,
        $slant:expr,
        $blink:expr,
        $invert:expr,
        $strike:expr,
        $underline:expr,
        $overline:expr,
        $border:expr,
    }) => {
        impl_styler!(impl [mode:field] $(<$($G $(: $($B)+)?,)+>)? ($self: $Type) {
            $foreground,
            $background,
            $weight,
            $slant,
            $blink,
            $invert,
            $strike,
            $underline,
            $overline,
            $border,
        });
    };
    ($(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:ident: $Type:ty) => $style:expr) => {
        impl_styler!(impl [mode:method] $(<$($G $(: $($B)+)?,)+>)? ($self: $Type) {
            $style,
            $style,
            $style,
            $style,
            $style,
            $style,
            $style,
            $style,
            $style,
            $style,
        });
    };

    // =====
    // Trait
    // =====
    (impl [mode:$mode:ident] $(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:ident: $Type:ty) {
        $foreground:expr,
        $background:expr,
        $weight:expr,
        $slant:expr,
        $blink:expr,
        $invert:expr,
        $strike:expr,
        $underline:expr,
        $overline:expr,
        $border:expr,
    }) => {
        impl $(<$($G $(: $($B +)+)?,)+>)? $crate::Styler for $Type {
            impl_styler!(impl [mode:$mode] Foreground get_foreground foreground_mut $self $foreground);
            impl_styler!(impl [mode:$mode] Background get_background background_mut $self $background);
            impl_styler!(impl [mode:$mode] Weighted get_weighted weighted_mut $self $weight);
            impl_styler!(impl [mode:$mode] Slanted get_slanted slanted_mut $self $slant);
            impl_styler!(impl [mode:$mode] Blinking get_blinking blinking_mut $self $blink);
            impl_styler!(impl [mode:$mode] Inverted get_inverted inverted_mut $self $invert);
            impl_styler!(impl [mode:$mode] Striked get_striked striked_mut $self $strike);
            impl_styler!(impl [mode:$mode] Underlined get_underlined underlined_mut $self $underline);
            impl_styler!(impl [mode:$mode] Overlined get_overlined overlined_mut $self $overline);
            impl_styler!(impl [mode:$mode] Bordered get_bordered bordered_mut $self $border);
        }

        impl_styler!(impl [ops] $Type: $(<$($G $(: $($B)+)?,)+>)?);
    };
    (impl [mode:field] $Attr:ident $get:ident $set_mut:ident $self:ident $field:expr) => {
        fn $get(&self) -> ::std::option::Option<$crate::$Attr> {
            let $self = self;
            $field
        }

        fn $set_mut(&mut self, arg: impl ::std::convert::Into<::std::option::Option<$crate::$Attr>>) {
            let $self = self;
            $field = arg.into();
        }
    };
    (impl [mode:method] $Attr:ident $get:ident $set_mut:ident $self:ident $field:expr) => {
        fn $get(&self) -> ::std::option::Option<$crate::$Attr> {
            let $self = self;
            $field.$get()
        }

        fn $set_mut(&mut self, arg: impl ::std::convert::Into<::std::option::Option<$crate::$Attr>>) {
            let $self = self;
            $field.$set_mut(arg);
        }
    };

    // ===
    // Ops
    // ===
    (impl [ops] $Type:ty: $(<$($G:ident $(: $($B:path)+)?,)+>)?) => {
        impl_styler!(impl [ops:ops] $Type {
            <$($($G $(: $($B)+)?,)+)?> Mul(mul) MulAssign(mul_assign) foreground_mut(foreground: Color NoColor) $crate::Foreground(foreground),
            <$($($G $(: $($B)+)?,)+)?> Div(div) DivAssign(div_assign) background_mut(background: Color NoColor) $crate::Background(background),
            <$($($G $(: $($B)+)?,)+)?> Add(add) AddAssign(add_assign) foreground_mut(foreground: Foreground NoForeground) foreground,
            <$($($G $(: $($B)+)?,)+)?> Add(add) AddAssign(add_assign) background_mut(background: Background NoBackground) background,
            <$($($G $(: $($B)+)?,)+)?> Add(add) AddAssign(add_assign) weighted_mut(weighted: Weighted NoWeight) weighted,
            <$($($G $(: $($B)+)?,)+)?> Add(add) AddAssign(add_assign) slanted_mut(slanted: Slanted NoSlant) slanted,
            <$($($G $(: $($B)+)?,)+)?> Add(add) AddAssign(add_assign) blinking_mut(blinking: Blinking NoBlink) blinking,
            <$($($G $(: $($B)+)?,)+)?> Add(add) AddAssign(add_assign) inverted_mut(inverted: Inverted NoInvert) inverted,
            <$($($G $(: $($B)+)?,)+)?> Add(add) AddAssign(add_assign) striked_mut(striked: Striked NoStrike) striked,
            <$($($G $(: $($B)+)?,)+)?> Add(add) AddAssign(add_assign) underlined_mut(underlined: Underlined NoUnderline) underlined,
            <$($($G $(: $($B)+)?,)+)?> Add(add) AddAssign(add_assign) overlined_mut(overlined: Overlined NoOverline) overlined,
            <$($($G $(: $($B)+)?,)+)?> Add(add) AddAssign(add_assign) bordered_mut(bordered: Bordered NoBorder) bordered,
        });
        impl_styler!(impl [ops:bit] $Type {
            <$($($G $(: $($B)+)?,)+)?> BitAnd(bitand) BitAndAssign(bitand_assign) and_mut,
            <$($($G $(: $($B)+)?,)+)?> BitOr (bitor ) BitOrAssign (bitor_assign ) or_mut,
            <$($($G $(: $($B)+)?,)+)?> BitXor(bitxor) BitXorAssign(bitxor_assign) xor_mut,
            <$($($G $(: $($B)+)?,)+)?> Rem   (rem   ) RemAssign   (rem_assign   ) dedup_mut,
        });

        impl_styler!(impl trait unary $Type {
            <$($($G $(: $($B)+)?,)+)?> Not(not) reset_mut
        });
    };

    (impl [ops:ops] $Type:ty { $(
        <$($G:ident $(: $($B:path)+)?,)*>
        $Op:ident($op:ident) $OpAssign:ident($op_assign:ident)
        $fn:ident($rhs:tt: $Rhs:ident $NoRhs:ident) $body:expr,
    )* }) => {
        impl_styler!(impl trait binary $Type { $(
            <$($G $(: $($B)+)?,)*>
            $Op($op) $OpAssign($op_assign) $fn($rhs: $crate::$Rhs) Some($body),
            <$($G $(: $($B)+)?,)*>
            $Op($op) $OpAssign($op_assign) $fn(_: $crate::$NoRhs) None,
        )* });
    };
   (impl [ops:bit] $Type:ty { $(
        <$($G:ident $(: $($B:path)+)?,)*>
        $Op:ident($op:ident) $OpAssign:ident($op_assign:ident)
        $fn:ident,
    )* }) => {
        impl_styler!(impl trait binary $Type { $(
            <$($G $(: $($B)+)?,)* Styler: $crate::Styler,>
            $Op($op) $OpAssign($op_assign) $fn(styler: &Styler) styler,
        )* });
    };

    (impl trait binary $Type:ty { $(
        <$($G:ident $(: $($B:path)+)?,)*>
        $Op:ident($op:ident) $OpAssign:ident($op_assign:ident)
        $fn:ident($rhs:tt: $($Rhs:tt)*) $body:expr,
    )* }) => {
        $(
            impl<$($G $(: $($B+)+,)?)*> ::std::ops::$Op<$($Rhs)*> for $Type {
                type Output = Self;

                fn $op(mut self, $rhs: $($Rhs)*) -> Self {
                    $crate::Styler::$fn(&mut self, $body);
                    self
                }
            }

            impl<$($G $(: $($B+)+,)?)*> ::std::ops::$OpAssign<$($Rhs)*> for $Type {
                fn $op_assign(&mut self, $rhs: $($Rhs)*) {
                    $crate::Styler::$fn(self, $body);
                }
            }
        )*
    };
    (impl trait unary $Type:ty { $(
        <$($G:ident $(: $($B:path)+)?,)*>
        $Op:ident($op:ident) $fn:ident
    )* }) => {
        $(
            impl<$($G $(: $($B+)+,)?)*> ::std::ops::$Op for $Type {
                type Output = Self;

                fn $op(mut self) -> Self {
                    $crate::Styler::$fn(&mut self);
                    self
                }
            }

            impl<$($G $(: $($B+)+,)?)*> ::std::ops::$Op for &mut $Type {
                type Output = ();

                fn $op(self) {
                    $crate::Styler::$fn(self);
                }
            }
        )*
    };
}
